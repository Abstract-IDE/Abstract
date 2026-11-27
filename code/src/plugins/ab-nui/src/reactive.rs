//! Fine-grained reactive state — the "state management" layer for UIs.
//!
//! The primitives, modelled after Solid/Leptos but tiny and synchronous:
//!
//! * [`Signal<T>`] — a mutable reactive value. Reading it inside a reactive
//!   scope subscribes that scope; writing it re-runs every subscriber.
//! * [`effect`] — runs a closure now, and again whenever any signal it read
//!   changes. Dependencies are tracked automatically, no manual wiring.
//! * [`memo`] — a cached value derived from other signals.
//! * [`batch`] — group several writes so subscribers re-run once, afterwards.
//!
//! Everything is single-threaded (Neovim's Lua thread), so the dependency graph
//! lives in a thread-local and uses `Rc`/`RefCell` — no locks.
//!
//! ```ignore
//! let count = Signal::new(0);
//! effect({
//!     let count = count.clone();
//!     move || println!("count is {}", count.get())
//! }); // prints "count is 0"
//! count.set(1); // prints "count is 1"
//! ```
//!
//! ## How updates propagate
//!
//! Propagation is push-pull ("dirty/check"), like Solid and Leptos:
//!
//! * A write marks the signal's direct subscribers *dirty* and everything
//!   downstream of them *check* (maybe stale), and queues the effects it
//!   reached. Nothing is recomputed yet.
//! * Memos are lazy: reading one (anywhere — in an effect, in a [`batch`],
//!   untracked) first brings it up to date, recomputing it only if one of its
//!   inputs really changed. A memo read therefore always reflects the current
//!   signal values, and no memo or effect ever observes a mix of old and new
//!   inputs (no glitches).
//! * Queued effects run when the outermost write (or [`batch`]) finishes;
//!   outside of any batch, `signal.set(..)` has re-run its subscribers by the
//!   time it returns. An effect that was only reached through memos whose
//!   values came out equal is skipped.
//! * Queued effects run in creation order (oldest first), whatever order they
//!   subscribed or were re-queued in, so behaviour is deterministic and a
//!   parent effect runs before the effects it created.
//! * An effect that writes a signal it also reads is re-run after it finishes,
//!   until it settles. One that re-triggers itself more than
//!   10 000 times in one drain is disposed (and reported)
//!   instead of hanging the editor; other effects are unaffected.
//! * Writes made while a [`Signal::update`] or [`Signal::with`] closure runs
//!   are propagated after it returns, so subscribers never run while a signal
//!   is borrowed.
//!
//! ## Lifetimes
//!
//! Signals are reference counted: when the last clone goes away, so does its
//! bookkeeping. An [`effect`] lives until it is disposed (see [`EffectHandle`]);
//! a [`Memo`] disposes its computation when its last clone is dropped.
//!
//! Effects and memos created while another effect or memo is running are
//! *owned* by it: they are disposed when it re-runs or is disposed, so an
//! effect that creates effects does not leak one set per run.

use std::cell::{Cell, Ref, RefCell};
use std::collections::{BTreeMap, HashMap};
use std::mem::ManuallyDrop;
use std::rc::{Rc, Weak};

type Id = u64;

/// How often one effect may re-run within a single drain of the queue before
/// it is considered a cycle that never settles and is disposed.
const MAX_RUNS_PER_EFFECT: u32 = 10_000;

/// The nodes subscribed to one signal (or memo). Shared between the signal and
/// its subscribers (which need it to unsubscribe).
struct Subscribers {
    list: RefCell<Vec<Weak<EffectNode>>>,
    /// The memo computing the value behind this list (dangling for a plain
    /// signal). Checking a subscriber means bringing these up to date first.
    memo: Weak<EffectNode>,
}

impl Default for Subscribers {
    fn default() -> Self {
        Self { list: RefCell::default(), memo: Weak::new() }
    }
}

impl Subscribers {
    /// The live subscribers. Entries of nodes that are gone or disposed are
    /// pruned on the way, so the list cannot grow without bound.
    fn live(&self) -> Vec<Rc<EffectNode>> {
        let mut list = self.list.borrow_mut();
        let mut out = Vec::with_capacity(list.len());
        list.retain(|w| match w.upgrade() {
            Some(node) if !node.disposed.get() => {
                out.push(node);
                true
            },
            _ => false,
        });
        out
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    Effect,
    /// The computation behind a [`Memo`]: pulled lazily, never queued.
    Memo,
}

/// How up to date a node is. Ordered: a node is only ever raised by a write.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum State {
    Clean,
    /// Some memo upstream may have changed; verify before deciding to run.
    Check,
    /// A direct input changed; must re-run.
    Dirty,
}

/// Node closure. Returns whether the node's output changed (memos only;
/// effects have no readers and the value is ignored).
type NodeFn = Box<dyn FnMut() -> bool>;

struct EffectNode {
    id: Id,
    kind: Kind,
    /// The closure; taken out while it runs and dropped on dispose.
    f: RefCell<Option<NodeFn>>,
    /// The signals/memos read during the last run.
    sources: RefCell<Vec<Rc<Subscribers>>>,
    /// A memo's readers (`None` for effects).
    subs: Option<Rc<Subscribers>>,
    /// Effects and memos created during the last run.
    children: RefCell<Vec<Id>>,
    state: Cell<State>,
    queued: Cell<bool>,
    running: Cell<bool>,
    /// A memo whose last recomputation panicked: recompute on the next read.
    retry: Cell<bool>,
    disposed: Cell<bool>,
}

impl EffectNode {
    /// Detach from every signal this node is subscribed to.
    fn unsubscribe(self: &Rc<Self>) {
        let me = Rc::as_ptr(self);
        let sources = std::mem::take(&mut *self.sources.borrow_mut());
        for source in sources {
            source.list.borrow_mut().retain(|w| !std::ptr::eq(w.as_ptr(), me));
        }
    }

    /// Dispose every effect/memo created during the last run.
    fn dispose_children(&self) {
        let children = std::mem::take(&mut *self.children.borrow_mut());
        for id in children {
            dispose_effect(id);
        }
    }
}

struct Runtime {
    next_id: Cell<Id>,
    /// The node currently executing whose reads should be tracked
    /// (cleared by [`untrack`]).
    observer: RefCell<Option<Rc<EffectNode>>>,
    /// The node currently executing, which owns effects created meanwhile
    /// (not affected by [`untrack`]).
    owner: RefCell<Option<Rc<EffectNode>>>,
    /// Every live effect and memo. This is what keeps an effect alive:
    /// handles are plain ids, so `effect(..)` can be fire-and-forget.
    effects: RefCell<HashMap<Id, Rc<EffectNode>>>,
    /// Effects waiting to run, keyed (and so ordered) by creation id.
    queue: RefCell<BTreeMap<Id, Rc<EffectNode>>>,
    /// Depth of nested [`batch`]es (and initial effect runs).
    hold: Cell<u32>,
    flushing: Cell<bool>,
}

// `ManuallyDrop`: effects own UI state (windows, timers) whose destructors
// call into Neovim. Running those while the process is exiting is at best
// pointless, so the graph is simply left alone at thread exit.
thread_local! {
    static RT: ManuallyDrop<Runtime> = ManuallyDrop::new(Runtime {
        next_id: Cell::new(1),
        observer: RefCell::new(None),
        owner: RefCell::new(None),
        effects: RefCell::new(HashMap::new()),
        queue: RefCell::new(BTreeMap::new()),
        hold: Cell::new(0),
        flushing: Cell::new(false),
    });
}

impl Runtime {
    fn fresh_id(&self) -> Id {
        let id = self.next_id.get();
        self.next_id.set(id + 1);
        id
    }

    fn enqueue(&self, node: &Rc<EffectNode>) {
        if node.disposed.get() || node.queued.replace(true) {
            return;
        }
        self.queue.borrow_mut().insert(node.id, node.clone());
    }

    /// Raise every subscriber in `subs` to (at least) `state`.
    fn mark(&self, subs: &Subscribers, state: State) {
        for node in subs.live() {
            self.stale(&node, state);
        }
    }

    fn stale(&self, node: &Rc<EffectNode>, state: State) {
        let raised = node.state.get() < state;
        if raised {
            node.state.set(state);
        }
        match node.kind {
            // `enqueue` dedupes; queueing on every mark (not only when raised)
            // means an effect can never be left stale but unqueued.
            Kind::Effect => {
                if node.state.get() != State::Clean {
                    self.enqueue(node);
                }
            },
            Kind::Memo => {
                if raised && let Some(subs) = &node.subs {
                    self.mark(subs, State::Check);
                }
            },
        }
    }

    /// Bring `node` up to date: if it might be stale, verify its memo inputs
    /// (recomputing those that changed, upstream first), and re-run it if any
    /// input really changed.
    fn refresh(&self, node: &Rc<EffectNode>) {
        if node.disposed.get() || node.running.get() {
            // A memo read from inside its own computation sees its last value.
            return;
        }
        if node.retry.replace(false) {
            node.state.set(State::Dirty);
        }
        if node.state.get() == State::Clean {
            return;
        }

        // If verification or the run panics, leave the node in a state that
        // still propagates: clean, and (for a memo) recomputed on next read.
        struct Unwind<'a>(&'a EffectNode);
        impl Drop for Unwind<'_> {
            fn drop(&mut self) {
                if !std::thread::panicking() {
                    return;
                }
                if self.0.kind == Kind::Memo {
                    self.0.state.set(State::Clean);
                    self.0.retry.set(true);
                } else if !self.0.queued.get() {
                    self.0.state.set(State::Clean);
                }
            }
        }
        let _unwind = Unwind(node);

        if node.state.get() == State::Check {
            let sources = node.sources.borrow().clone();
            for source in sources {
                if let Some(memo) = source.memo.upgrade() {
                    self.refresh(&memo);
                }
                if node.state.get() == State::Dirty {
                    break;
                }
            }
        }
        if node.state.get() == State::Dirty {
            self.run(node);
        } else {
            node.state.set(State::Clean);
        }
    }

    /// Execute one node: dispose what its last run created, detach its old
    /// dependencies, run it as the active observer/owner (so reads
    /// re-subscribe), then restore the previous observer/owner.
    fn run(&self, node: &Rc<EffectNode>) {
        if node.disposed.get() {
            return;
        }
        // Taking the closure out means no `RefCell` borrow is held while user
        // code runs (it may dispose this very effect).
        let Some(f) = node.f.borrow_mut().take() else { return };

        struct Guard<'a> {
            rt: &'a Runtime,
            node: &'a Rc<EffectNode>,
            f: Option<NodeFn>,
            prev_observer: Option<Rc<EffectNode>>,
            prev_owner: Option<Rc<EffectNode>>,
        }
        impl Drop for Guard<'_> {
            fn drop(&mut self) {
                let observer = self.rt.observer.replace(self.prev_observer.take());
                let owner = self.rt.owner.replace(self.prev_owner.take());
                drop((observer, owner));
                self.node.running.set(false);
                let f = self.f.take();
                if self.node.disposed.get() {
                    // Disposed while running: finish the teardown now.
                    self.node.unsubscribe();
                    self.node.dispose_children();
                    drop(f);
                } else {
                    // Put the closure back even when unwinding, so a panic
                    // does not silently kill the effect.
                    *self.node.f.borrow_mut() = f;
                }
            }
        }
        let mut guard = Guard { rt: self, node, f: Some(f), prev_observer: None, prev_owner: None };

        node.dispose_children();
        node.unsubscribe();
        // Clean *before* running: a write to one of its own inputs during the
        // run marks it again (and re-queues an effect).
        node.state.set(State::Clean);
        node.running.set(true);
        guard.prev_observer = self.observer.replace(Some(node.clone()));
        guard.prev_owner = self.owner.replace(Some(node.clone()));

        let changed = (guard.f.as_mut().expect("closure present"))();
        drop(guard);

        if changed && let Some(subs) = &node.subs {
            self.mark(subs, State::Dirty);
        }
    }

    /// Drain the queue, unless something further up the stack will.
    fn flush(&self) {
        if self.hold.get() > 0 || self.flushing.replace(true) {
            return;
        }
        // Reset the flag even if an effect panics.
        struct Done<'a>(&'a Cell<bool>);
        impl Drop for Done<'_> {
            fn drop(&mut self) {
                self.0.set(false);
            }
        }
        let _done = Done(&self.flushing);

        let mut runs: HashMap<Id, u32> = HashMap::new();
        loop {
            let next = self.queue.borrow_mut().pop_first();
            let Some((id, node)) = next else { break };
            node.queued.set(false);
            let count = runs.entry(id).or_insert(0);
            *count += 1;
            if *count > MAX_RUNS_PER_EFFECT {
                // A cycle that never settles: kill only the runaway effect.
                dispose_effect(id);
                report_runaway(id);
                continue;
            }
            self.refresh(&node);
        }
    }

    /// Run `f` with propagation held back, then drain the queue.
    fn held<R>(&self, f: impl FnOnce() -> R) -> R {
        struct Release<'a>(&'a Runtime);
        impl Drop for Release<'_> {
            fn drop(&mut self) {
                self.0.hold.set(self.0.hold.get() - 1);
                if !std::thread::panicking() {
                    self.0.flush();
                }
            }
        }
        self.hold.set(self.hold.get() + 1);
        let _release = Release(self);
        f()
    }
}

fn report_runaway(id: Id) {
    let msg = format!(
        "ab-nui: effect {id} re-triggered itself more than {MAX_RUNS_PER_EFFECT} times in one \
         update (a reactive cycle that never settles) and was disposed"
    );
    if crate::lua::is_initialized()
        && let Ok(notify) = crate::lua::vim_fn("notify")
        && notify.call::<()>((msg.as_str(), 4)).is_ok()
    {
        return;
    }
    eprintln!("{msg}");
}

/// Record that the current observer (if any) read the given signal.
fn track(subs: &Rc<Subscribers>) {
    RT.with(|rt| {
        let observer = rt.observer.borrow();
        let Some(observer) = observer.as_ref() else { return };
        if observer.disposed.get() {
            return;
        }
        let mut sources = observer.sources.borrow_mut();
        if sources.iter().rev().any(|s| Rc::ptr_eq(s, subs)) {
            return;
        }
        sources.push(subs.clone());
        subs.list.borrow_mut().push(Rc::downgrade(observer));
    });
}

/// Mark everything downstream of a written signal, and drain the queue unless
/// a batch (or a running effect) is holding it back.
fn notify(subs: &Subscribers) {
    if subs.list.borrow().is_empty() {
        return;
    }
    RT.with(|rt| {
        rt.mark(subs, State::Dirty);
        rt.flush();
    });
}

/// Group several writes: effects that depend on any of the signals written
/// inside `f` run once, after `f` returns (after the outermost `batch`, if
/// nested). Memos read inside the batch are still up to date.
///
/// Every callback this library hands to Neovim (keymaps, autocmds, timers,
/// user commands) already runs inside a batch, so this is only needed when
/// writing signals from somewhere else.
///
/// ```ignore
/// batch(|| {
///     first.set("Ada".into());
///     last.set("Lovelace".into());
/// }); // one repaint, not two
/// ```
pub fn batch<R>(f: impl FnOnce() -> R) -> R {
    RT.with(|rt| rt.held(f))
}

/// Run `f` holding effects back until Neovim's next tick (not just until `f`
/// returns, like [`batch`]): for code running where effects can't touch the
/// UI, such as an expr mapping under the text lock.
pub(crate) fn held_until_next_tick<R>(f: impl FnOnce() -> R) -> R {
    struct Release;
    impl Drop for Release {
        fn drop(&mut self) {
            let release = || {
                RT.with(|rt| {
                    rt.hold.set(rt.hold.get() - 1);
                    rt.flush();
                })
            };
            if crate::nvim::schedule(release).is_err() {
                release();
            }
        }
    }
    RT.with(|rt| rt.hold.set(rt.hold.get() + 1));
    let _release = Release;
    f()
}

/// Run `f` with no owner: effects and memos created inside are NOT disposed
/// when the effect that happens to be running re-runs or is disposed — only
/// their own handle ends them. UI that owns its lifetime through a handle (a
/// popup, a surface, a toast) creates its effects this way, so opening a
/// window from inside an effect doesn't tie the window to that effect.
pub fn unowned<R>(f: impl FnOnce() -> R) -> R {
    RT.with(|rt| {
        let rt: &Runtime = rt;
        struct Restore<'a> {
            rt: &'a Runtime,
            prev: Option<Rc<EffectNode>>,
        }
        impl Drop for Restore<'_> {
            fn drop(&mut self) {
                let current = self.rt.owner.replace(self.prev.take());
                drop(current);
            }
        }
        let prev = rt.owner.borrow_mut().take();
        let _restore = Restore { rt, prev };
        f()
    })
}

/// Run `f` without subscribing the current reactive scope to anything it
/// reads. (Effects created inside are still owned by the running effect; see
/// [`unowned`].)
pub fn untrack<R>(f: impl FnOnce() -> R) -> R {
    RT.with(|rt| {
        let rt: &Runtime = rt;
        struct Restore<'a> {
            rt: &'a Runtime,
            prev: Option<Rc<EffectNode>>,
        }
        impl Drop for Restore<'_> {
            fn drop(&mut self) {
                let current = self.rt.observer.replace(self.prev.take());
                drop(current);
            }
        }
        let prev = rt.observer.borrow_mut().take();
        let _restore = Restore { rt, prev };
        f()
    })
}

// -----------------------------------------------------------------------------
// Signal
// -----------------------------------------------------------------------------

struct SignalInner<T> {
    value: RefCell<T>,
    subs: Rc<Subscribers>,
}

/// A reactive value. Clone is cheap and shares the same underlying cell, so a
/// `Signal` can be freely captured into callbacks.
pub struct Signal<T> {
    inner: Rc<SignalInner<T>>,
}

impl<T> Clone for Signal<T> {
    fn clone(&self) -> Self {
        Self { inner: self.inner.clone() }
    }
}

#[cold]
#[track_caller]
fn read_conflict() -> ! {
    panic!("ab-nui: a Signal was read inside its own `update` closure (use the `&mut` it hands you)")
}

#[cold]
#[track_caller]
fn write_conflict() -> ! {
    panic!("ab-nui: a Signal was written inside its own `with`/`update` closure")
}

impl<T: 'static> Signal<T> {
    /// Create a new signal holding `value`.
    pub fn new(value: T) -> Self {
        Self { inner: Rc::new(SignalInner { value: RefCell::new(value), subs: Rc::default() }) }
    }

    #[track_caller]
    fn read(&self) -> Ref<'_, T> {
        match self.inner.value.try_borrow() {
            Ok(v) => v,
            Err(_) => read_conflict(),
        }
    }

    /// Read and clone the value, subscribing the current reactive scope.
    #[track_caller]
    pub fn get(&self) -> T
    where
        T: Clone,
    {
        track(&self.inner.subs);
        self.read().clone()
    }

    /// Borrow the value (subscribing the current scope) without cloning.
    ///
    /// Signals written inside `f` propagate after `f` returns.
    #[track_caller]
    pub fn with<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        track(&self.inner.subs);
        self.with_untracked(f)
    }

    /// Read without subscribing — use inside callbacks that must not re-trigger.
    #[track_caller]
    pub fn get_untracked(&self) -> T
    where
        T: Clone,
    {
        self.read().clone()
    }

    /// Borrow the value without subscribing.
    ///
    /// Signals written inside `f` propagate after `f` returns.
    #[track_caller]
    pub fn with_untracked<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        let value = self.read();
        // Held: nothing may run (and read this signal) while it is borrowed.
        batch(move || f(&value))
    }

    /// Replace the value and notify subscribers.
    #[track_caller]
    pub fn set(&self, value: T) {
        // The old value is dropped after the borrow is released, in case its
        // `Drop` reads this signal.
        let old = match self.inner.value.try_borrow_mut() {
            Ok(mut v) => std::mem::replace(&mut *v, value),
            Err(_) => write_conflict(),
        };
        drop(old);
        notify(&self.inner.subs);
    }

    /// Replace the value and notify subscribers, unless it is equal to the
    /// current one (then nothing happens). Returns whether it changed.
    #[track_caller]
    pub fn set_if_changed(&self, value: T) -> bool
    where
        T: PartialEq,
    {
        if *self.read() == value {
            return false;
        }
        self.set(value);
        true
    }

    /// Mutate the value in place and notify subscribers.
    ///
    /// Signals written inside `f` propagate after `f` returns (together with
    /// this one), so their subscribers may read this signal.
    #[track_caller]
    pub fn update(&self, f: impl FnOnce(&mut T)) {
        let mut value = match self.inner.value.try_borrow_mut() {
            Ok(v) => v,
            Err(_) => write_conflict(),
        };
        batch(move || {
            f(&mut value);
            drop(value);
            notify(&self.inner.subs);
        });
    }

    /// Replace the value WITHOUT notifying subscribers. For runtime-internal
    /// corrections (e.g. clamping focus during a repaint) where the caller is
    /// already about to show the new value.
    #[track_caller]
    pub fn set_silent(&self, value: T) {
        let old = match self.inner.value.try_borrow_mut() {
            Ok(mut v) => std::mem::replace(&mut *v, value),
            Err(_) => write_conflict(),
        };
        drop(old);
    }
}

// -----------------------------------------------------------------------------
// Effect & Memo
// -----------------------------------------------------------------------------

/// A token for an [`effect`] that lets you tear it down later. Dropping it does
/// **not** dispose the effect (so `effect(..)` can stay fire-and-forget); call
/// [`EffectHandle::dispose`] or [`dispose_effect`] explicitly.
#[derive(Clone, Copy, Debug)]
pub struct EffectHandle {
    id: Id,
}

impl EffectHandle {
    /// The effect's internal id.
    pub fn id(self) -> u64 {
        self.id
    }
    /// Tear down the effect and remove all its subscriptions.
    pub fn dispose(self) {
        dispose_effect(self.id);
    }
}

fn create_node(kind: Kind, f: NodeFn) -> Rc<EffectNode> {
    RT.with(|rt| {
        let id = rt.fresh_id();
        let node = Rc::new_cyclic(|weak| EffectNode {
            id,
            kind,
            f: RefCell::new(Some(f)),
            sources: RefCell::new(Vec::new()),
            subs: (kind == Kind::Memo).then(|| Rc::new(Subscribers { list: RefCell::default(), memo: weak.clone() })),
            children: RefCell::new(Vec::new()),
            state: Cell::new(State::Clean),
            queued: Cell::new(false),
            running: Cell::new(false),
            retry: Cell::new(false),
            disposed: Cell::new(false),
        });
        rt.effects.borrow_mut().insert(id, node.clone());
        if let Some(owner) = rt.owner.borrow().as_ref() {
            owner.children.borrow_mut().push(id);
        }

        // If the first run panics the caller never gets a handle: tear the
        // node down rather than leak it.
        struct Abort(Id);
        impl Drop for Abort {
            fn drop(&mut self) {
                dispose_effect(self.0);
            }
        }
        let abort = Abort(id);
        // The first run happens right here, even inside a batch. Anything it
        // writes is propagated once it has finished.
        rt.held(|| rt.run(&node));
        std::mem::forget(abort);
        node
    })
}

/// Run `f` immediately, and again whenever a signal it read changes.
pub fn effect(mut f: impl FnMut() + 'static) -> EffectHandle {
    let node = create_node(
        Kind::Effect,
        Box::new(move || {
            f();
            false
        }),
    );
    EffectHandle { id: node.id }
}

/// Tear down an effect by id: drop its closure, detach it from every signal
/// it was subscribed to, and dispose the effects it created. Idempotent.
pub fn dispose_effect(id: u64) {
    // `try_with`: this also runs from destructors, possibly during thread
    // teardown.
    let Ok(Some(node)) = RT.try_with(|rt| {
        let queued = rt.queue.borrow_mut().remove(&id);
        drop(queued);
        rt.effects.borrow_mut().remove(&id)
    }) else {
        return;
    };
    node.disposed.set(true);
    node.unsubscribe();
    node.dispose_children();
    // If the effect is mid-run its closure is out of the cell and gets dropped
    // when the run returns; otherwise drop it now (outside any runtime borrow,
    // as dropping user state can re-enter).
    let f = node.f.borrow_mut().take();
    drop(f);
}

/// Disposes a memo's computation when the last [`Memo`] clone goes away.
struct MemoGuard(Id);

impl Drop for MemoGuard {
    fn drop(&mut self) {
        dispose_effect(self.0);
    }
}

/// A cached, read-only value derived from other signals.
pub struct Memo<T> {
    value: Rc<RefCell<Option<T>>>,
    node: Rc<EffectNode>,
    _guard: Rc<MemoGuard>,
}

impl<T> Clone for Memo<T> {
    fn clone(&self) -> Self {
        Self { value: self.value.clone(), node: self.node.clone(), _guard: self._guard.clone() }
    }
}

impl<T: 'static> Memo<T> {
    /// Recompute if an input changed since the last computation.
    fn refresh(&self) {
        RT.with(|rt| rt.held(|| rt.refresh(&self.node)));
    }

    fn subscribe(&self) {
        // After `refresh`: recomputing marks this memo's readers dirty, which
        // must not include the reader that is pulling it right now.
        if let Some(subs) = &self.node.subs {
            track(subs);
        }
    }

    fn value(&self) -> Ref<'_, T> {
        Ref::map(self.value.borrow(), |v| v.as_ref().expect("memo computes on creation"))
    }

    /// Borrow the derived value without subscribing.
    pub fn with_untracked<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        self.refresh();
        let value = self.value();
        batch(move || f(&value))
    }

    /// Borrow the derived value, subscribing the current scope.
    pub fn with<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        self.refresh();
        self.subscribe();
        let value = self.value();
        batch(move || f(&value))
    }
}

impl<T: Clone + 'static> Memo<T> {
    /// Read the derived value, subscribing the current scope.
    pub fn get(&self) -> T {
        self.refresh();
        self.subscribe();
        self.value().clone()
    }

    /// Read the derived value without subscribing.
    pub fn get_untracked(&self) -> T {
        self.refresh();
        self.value().clone()
    }
}

/// Create a [`Memo`] computed from `f`. It is computed once now, then lazily:
/// a read recomputes it only if one of the signals `f` reads changed. Readers
/// are notified only when the new value differs (`!=`) from the previous one.
/// It stops for good once the last clone of the memo is dropped.
pub fn memo<T: Clone + PartialEq + 'static>(mut f: impl FnMut() -> T + 'static) -> Memo<T> {
    let value: Rc<RefCell<Option<T>>> = Rc::new(RefCell::new(None));
    let slot = value.clone();
    let node = create_node(
        Kind::Memo,
        Box::new(move || {
            let new = f();
            if slot.borrow().as_ref() == Some(&new) {
                return false;
            }
            let old = slot.borrow_mut().replace(new);
            drop(old);
            true
        }),
    );
    let guard = Rc::new(MemoGuard(node.id));
    Memo { value, node, _guard: guard }
}

// -----------------------------------------------------------------------------
// store!
// -----------------------------------------------------------------------------

/// Declare a process-global reactive value (per thread, lazily initialized).
///
/// ```ignore
/// wp_ui::store!(pub static THEME: u8 = 0);
///
/// THEME::get();             // reactive read — widgets reading this re-render
/// THEME::set(1);            // notify all readers
/// THEME::update(|t| *t ^= 1);
/// THEME::with(|t| *t > 0);   // borrow (also works for non-`Clone` types)
/// THEME::get_untracked();    // read without subscribing
/// THEME::set_if_changed(1);  // write only if different
/// let sig = THEME::signal(); // pass to widgets that take a Signal
/// ```
///
/// `get`/`get_untracked` need `T: Clone` and `set_if_changed` needs
/// `T: PartialEq`; the rest work for any type.
#[macro_export]
macro_rules! store {
    ($vis:vis static $name:ident : $ty:ty = $init:expr $(;)?) => {
        #[allow(non_camel_case_types)]
        $vis struct $name;
        #[allow(dead_code)]
        impl $name {
            fn __signal() -> $crate::reactive::Signal<$ty> {
                thread_local! {
                    static __STORE: ::std::cell::RefCell<::std::option::Option<$crate::reactive::Signal<$ty>>> =
                        const { ::std::cell::RefCell::new(::std::option::Option::None) };
                }
                __STORE.with(|c| {
                    let mut slot = c.borrow_mut();
                    if slot.is_none() {
                        *slot = ::std::option::Option::Some($crate::reactive::Signal::new($init));
                    }
                    slot.clone().unwrap()
                })
            }
            // The `for<'__a>` bounds are deliberately higher-ranked: a plain
            // `where $ty: Clone` on a concrete type is a hard error (E0277)
            // for a non-`Clone` type, while this form just makes the method
            // unavailable.
            $vis fn get() -> $ty
            where
                for<'__a> $ty: ::std::clone::Clone,
            {
                Self::__signal().get()
            }
            $vis fn get_untracked() -> $ty
            where
                for<'__a> $ty: ::std::clone::Clone,
            {
                Self::__signal().get_untracked()
            }
            $vis fn with<__R>(f: impl ::std::ops::FnOnce(&$ty) -> __R) -> __R {
                Self::__signal().with(f)
            }
            $vis fn set(value: $ty) {
                Self::__signal().set(value);
            }
            $vis fn set_if_changed(value: $ty) -> bool
            where
                for<'__a> $ty: ::std::cmp::PartialEq,
            {
                Self::__signal().set_if_changed(value)
            }
            $vis fn update(f: impl ::std::ops::FnOnce(&mut $ty)) {
                Self::__signal().update(f);
            }
            $vis fn signal() -> $crate::reactive::Signal<$ty> {
                Self::__signal()
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A shared run counter.
    fn counter() -> Rc<Cell<u32>> {
        Rc::new(Cell::new(0))
    }

    #[test]
    fn effect_reruns_on_write_and_stops_after_dispose() {
        let sig = Signal::new(0);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let handle = {
            let sig = sig.clone();
            let seen = seen.clone();
            effect(move || seen.borrow_mut().push(sig.get()))
        };
        sig.set(1);
        sig.set(2);
        assert_eq!(*seen.borrow(), vec![0, 1, 2]);

        handle.dispose();
        sig.set(3);
        assert_eq!(*seen.borrow(), vec![0, 1, 2], "disposed effect must not re-run");
    }

    #[test]
    fn set_silent_does_not_notify() {
        let sig = Signal::new(0);
        let runs = counter();
        let handle = {
            let sig = sig.clone();
            let runs = runs.clone();
            effect(move || {
                sig.get();
                runs.set(runs.get() + 1);
            })
        };
        sig.set_silent(5);
        assert_eq!(runs.get(), 1);
        assert_eq!(sig.get_untracked(), 5);
        handle.dispose();
    }

    #[test]
    fn set_if_changed_skips_equal_values() {
        let sig = Signal::new(1);
        let runs = counter();
        let handle = {
            let (sig, runs) = (sig.clone(), runs.clone());
            effect(move || {
                sig.get();
                runs.set(runs.get() + 1);
            })
        };
        assert!(!sig.set_if_changed(1));
        assert_eq!(runs.get(), 1);
        assert!(sig.set_if_changed(2));
        assert_eq!(runs.get(), 2);
        handle.dispose();
    }

    #[test]
    fn memo_recomputes_only_on_dependency_change() {
        let a = Signal::new(2);
        let computes = counter();
        let m = {
            let a = a.clone();
            let computes = computes.clone();
            memo(move || {
                computes.set(computes.get() + 1);
                a.get() * 10
            })
        };
        assert_eq!(m.get(), 20);
        assert_eq!(computes.get(), 1);
        a.set(3);
        assert_eq!(m.get(), 30);
        assert_eq!(computes.get(), 2);
        // Reading the memo again does not recompute.
        let _ = m.get();
        assert_eq!(computes.get(), 2);
    }

    #[test]
    fn dropping_the_last_memo_clone_stops_its_computation() {
        let a = Signal::new(1);
        let computes = counter();
        let m = {
            let (a, computes) = (a.clone(), computes.clone());
            memo(move || {
                computes.set(computes.get() + 1);
                a.get()
            })
        };
        let m2 = m.clone();
        drop(m);
        a.set(2);
        assert_eq!(m2.get_untracked(), 2);
        assert_eq!(computes.get(), 2, "still alive through the clone");
        drop(m2);
        a.set(3);
        assert_eq!(computes.get(), 2, "disposed with the last clone");
    }

    #[test]
    fn an_effect_writing_its_own_dependency_settles() {
        let sig = Signal::new(0);
        let runs = counter();
        let last_seen = Rc::new(Cell::new(-1));
        let handle = {
            let (sig, runs, last_seen) = (sig.clone(), runs.clone(), last_seen.clone());
            effect(move || {
                runs.set(runs.get() + 1);
                let v = sig.get();
                last_seen.set(v);
                if v < 3 {
                    sig.set(v + 1);
                }
            })
        };
        // It re-ran until it stopped writing, and saw the final value.
        assert_eq!(sig.get_untracked(), 3);
        assert_eq!(last_seen.get(), 3);
        assert_eq!(runs.get(), 4);
        handle.dispose();
    }

    #[test]
    fn a_cycle_that_never_settles_is_cut_off() {
        let sig = Signal::new(0u64);
        let handle = {
            let sig = sig.clone();
            effect(move || {
                let v = sig.get();
                sig.set(v + 1);
            })
        };
        // Reaching this line at all is the assertion.
        assert!(sig.get_untracked() > 0);
        handle.dispose();
    }

    #[test]
    fn each_run_rebuilds_exact_dependencies() {
        let cond = Signal::new(true);
        let a = Signal::new(0);
        let b = Signal::new(0);
        let runs = counter();
        let handle = {
            let (cond, a, b, runs) = (cond.clone(), a.clone(), b.clone(), runs.clone());
            effect(move || {
                runs.set(runs.get() + 1);
                if cond.get() {
                    a.get()
                } else {
                    b.get()
                };
            })
        };
        assert_eq!(runs.get(), 1);
        b.set(9); // not a dependency while cond is true
        assert_eq!(runs.get(), 1);
        cond.set(false); // now b is, a isn't
        assert_eq!(runs.get(), 2);
        a.set(9);
        assert_eq!(runs.get(), 2);
        b.set(10);
        assert_eq!(runs.get(), 3);
        handle.dispose();
    }

    #[test]
    fn batch_runs_each_effect_once() {
        let a = Signal::new(1);
        let b = Signal::new(10);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let handle = {
            let (a, b, seen) = (a.clone(), b.clone(), seen.clone());
            effect(move || seen.borrow_mut().push(a.get() + b.get()))
        };
        batch(|| {
            a.set(2);
            b.set(20);
            batch(|| a.set(3)); // nested: still held by the outer batch
            assert_eq!(*seen.borrow(), vec![11], "nothing runs inside the batch");
        });
        assert_eq!(*seen.borrow(), vec![11, 23]);
        handle.dispose();
    }

    #[test]
    fn effects_run_in_subscription_order() {
        let sig = Signal::new(0);
        let order = Rc::new(RefCell::new(Vec::new()));
        let handles: Vec<_> = (0..5)
            .map(|i| {
                let (sig, order) = (sig.clone(), order.clone());
                effect(move || {
                    sig.get();
                    order.borrow_mut().push(i);
                })
            })
            .collect();
        order.borrow_mut().clear();
        sig.set(1);
        assert_eq!(*order.borrow(), vec![0, 1, 2, 3, 4]);
        handles.into_iter().for_each(EffectHandle::dispose);
    }

    #[test]
    fn effects_see_fresh_memos() {
        // `view` reads both `a` and a memo derived from `a`, and subscribed
        // to `a` first. It must still only ever observe consistent pairs.
        let a = Signal::new(1);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let doubled_slot: Rc<RefCell<Option<Memo<i32>>>> = Rc::default();
        let handle = {
            let (a, seen, slot) = (a.clone(), seen.clone(), doubled_slot.clone());
            effect(move || {
                let v = a.get();
                if let Some(doubled) = slot.borrow().as_ref() {
                    seen.borrow_mut().push((v, doubled.get()));
                }
            })
        };
        *doubled_slot.borrow_mut() = Some({
            let a = a.clone();
            memo(move || a.get() * 2)
        });
        a.set(2);
        a.set(3);
        assert_eq!(*seen.borrow(), vec![(2, 4), (3, 6)]);
        handle.dispose();
        doubled_slot.borrow_mut().take();
    }

    #[test]
    fn nested_writes_are_not_recursive() {
        // `first` writes `b`; `second` (subscribed to `b`) must run after
        // `first` has finished, not in the middle of it.
        let a = Signal::new(0);
        let b = Signal::new(0);
        let log = Rc::new(RefCell::new(Vec::new()));
        let second = {
            let (b, log) = (b.clone(), log.clone());
            effect(move || log.borrow_mut().push(format!("second:{}", b.get())))
        };
        let first = {
            let (a, b, log) = (a.clone(), b.clone(), log.clone());
            effect(move || {
                let v = a.get();
                log.borrow_mut().push(format!("first:{v}:start"));
                b.set(v);
                log.borrow_mut().push(format!("first:{v}:end"));
            })
        };
        log.borrow_mut().clear();
        a.set(7);
        assert_eq!(*log.borrow(), vec!["first:7:start", "first:7:end", "second:7"]);
        first.dispose();
        second.dispose();
    }

    #[test]
    fn an_effect_can_dispose_itself_while_running() {
        let sig = Signal::new(0);
        let runs = counter();
        let slot: Rc<Cell<Option<EffectHandle>>> = Rc::default();
        let handle = {
            let (sig, runs, slot) = (sig.clone(), runs.clone(), slot.clone());
            effect(move || {
                runs.set(runs.get() + 1);
                if sig.get() == 1 {
                    slot.take().expect("handle stored").dispose();
                }
            })
        };
        slot.set(Some(handle));
        sig.set(1);
        sig.set(2);
        assert_eq!(runs.get(), 2);
    }

    #[test]
    fn untrack_reads_without_subscribing() {
        let a = Signal::new(0);
        let b = Signal::new(0);
        let runs = counter();
        let handle = {
            let (a, b, runs) = (a.clone(), b.clone(), runs.clone());
            effect(move || {
                runs.set(runs.get() + 1);
                a.get();
                untrack(|| b.get());
            })
        };
        b.set(1);
        assert_eq!(runs.get(), 1);
        a.set(1);
        assert_eq!(runs.get(), 2);
        handle.dispose();
    }

    #[test]
    fn a_dropped_signal_releases_its_value_and_subscribers() {
        let payload = Rc::new(());
        let sig = Signal::new(payload.clone());
        let handle = {
            let sig = sig.clone();
            effect(move || {
                sig.with(|_| ());
            })
        };
        assert_eq!(Rc::strong_count(&payload), 2);
        handle.dispose(); // drops the closure's clone of the signal
        drop(sig);
        assert_eq!(Rc::strong_count(&payload), 1);
    }

    // --- regression tests ------------------------------------------------

    /// Number of subscriber entries (live or dead) on a signal.
    fn sub_entries<T>(sig: &Signal<T>) -> usize {
        sig.inner.subs.list.borrow().len()
    }

    fn live_nodes() -> usize {
        RT.with(|rt| rt.effects.borrow().len())
    }

    #[test]
    fn memo_is_fresh_inside_a_batch() {
        let a = Signal::new(1);
        let m = {
            let a = a.clone();
            memo(move || a.get() * 2)
        };
        let seen = batch(|| {
            a.set(5);
            (m.get_untracked(), m.get(), m.with(|v| *v))
        });
        assert_eq!(seen, (10, 10, 10));
        assert_eq!(m.get_untracked(), 10);
    }

    #[test]
    fn memo_is_fresh_inside_an_effect_that_wrote_its_input() {
        let a = Signal::new(1);
        let trigger = Signal::new(0);
        let m = {
            let a = a.clone();
            memo(move || a.get() * 2)
        };
        let seen = Rc::new(RefCell::new(Vec::new()));
        let handle = {
            let (a, m, trigger, seen) = (a.clone(), m.clone(), trigger.clone(), seen.clone());
            effect(move || {
                let t = trigger.get();
                if t > 0 {
                    a.set(7);
                    seen.borrow_mut().push(m.get());
                }
            })
        };
        trigger.set(1);
        assert_eq!(*seen.borrow(), vec![14]);
        handle.dispose();
    }

    #[test]
    fn memo_chains_are_glitch_free() {
        let s = Signal::new(0);
        let t = Signal::new(0);
        let a0 = {
            let (s, t) = (s.clone(), t.clone());
            memo(move || s.get() + t.get())
        };
        let a = {
            let a0 = a0.clone();
            memo(move || a0.get())
        };
        let inputs = Rc::new(RefCell::new(Vec::new()));
        let c = {
            let (a, s, inputs) = (a.clone(), s.clone(), inputs.clone());
            memo(move || {
                let pair = (a.get(), s.get());
                inputs.borrow_mut().push(pair);
                pair
            })
        };
        let seen = Rc::new(RefCell::new(Vec::new()));
        let handle = {
            let (c, s, seen) = (c.clone(), s.clone(), seen.clone());
            effect(move || seen.borrow_mut().push((s.get(), c.get())))
        };
        inputs.borrow_mut().clear();
        seen.borrow_mut().clear();
        s.set(5);
        assert_eq!(*inputs.borrow(), vec![(5, 5)], "c never sees a stale `a`");
        assert_eq!(*seen.borrow(), vec![(5, (5, 5))], "the effect runs once, consistently");
        handle.dispose();
    }

    #[test]
    fn memo_with_an_equal_value_does_not_notify() {
        let a = Signal::new(0);
        let parity = {
            let a = a.clone();
            memo(move || a.get() % 2)
        };
        let runs = counter();
        let handle = {
            let (parity, runs) = (parity.clone(), runs.clone());
            effect(move || {
                parity.get();
                runs.set(runs.get() + 1);
            })
        };
        a.set(2);
        assert_eq!(runs.get(), 1, "parity unchanged: reader skipped");
        a.set(3);
        assert_eq!(runs.get(), 2);
        handle.dispose();
    }

    #[test]
    fn update_closure_may_write_other_signals_read_by_subscribers() {
        let a = Signal::new(0);
        let b = Signal::new(0);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let handle = {
            let (a, b, seen) = (a.clone(), b.clone(), seen.clone());
            effect(move || seen.borrow_mut().push((b.get(), a.get())))
        };
        a.update(|v| {
            *v += 1;
            b.set(10); // the subscriber reads `a`: must not run while it is borrowed
        });
        assert_eq!(*seen.borrow(), vec![(0, 0), (10, 1)]);
        // Same for `with`.
        a.with(|v| b.set(*v + 20));
        assert_eq!(seen.borrow().last(), Some(&(21, 1)));
        handle.dispose();
    }

    #[test]
    #[should_panic(expected = "read inside its own `update`")]
    fn reading_a_signal_inside_its_own_update_panics_clearly() {
        let a = Signal::new(0);
        a.update(|_| {
            a.get();
        });
    }

    #[test]
    fn an_effect_disposed_mid_run_stops_subscribing() {
        let sig = Signal::new(0);
        let other = Signal::new(0);
        let slot: Rc<Cell<Option<EffectHandle>>> = Rc::default();
        let handle = {
            let (sig, other, slot) = (sig.clone(), other.clone(), slot.clone());
            effect(move || {
                if sig.get() == 1 {
                    slot.take().expect("handle stored").dispose();
                    other.get(); // read after disposal: must not subscribe
                }
            })
        };
        slot.set(Some(handle));
        sig.set(1);
        assert_eq!(sub_entries(&sig), 0);
        assert_eq!(sub_entries(&other), 0);
        for i in 0..10 {
            other.set(i);
        }
        assert_eq!(sub_entries(&other), 0, "no dead entries accumulate");
    }

    #[test]
    fn an_effect_survives_a_panic() {
        let sig = Signal::new(0);
        let runs = counter();
        let handle = {
            let (sig, runs) = (sig.clone(), runs.clone());
            effect(move || {
                runs.set(runs.get() + 1);
                if sig.get() == 1 {
                    panic!("boom");
                }
            })
        };
        let caught = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| sig.set(1)));
        assert!(caught.is_err());
        assert_eq!(runs.get(), 2);
        sig.set(2);
        assert_eq!(runs.get(), 3, "still runs on the next trigger");
        handle.dispose();
    }

    #[test]
    fn a_panicking_first_run_does_not_leak_the_effect() {
        let sig = Signal::new(0);
        let before = live_nodes();
        let caught = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let sig = sig.clone();
            effect(move || {
                sig.get();
                panic!("boom");
            })
        }));
        assert!(caught.is_err());
        assert_eq!(live_nodes(), before);
        assert_eq!(sub_entries(&sig), 0);
    }

    #[test]
    fn a_memo_recovers_from_a_panic() {
        let a = Signal::new(0);
        let m = {
            let a = a.clone();
            memo(move || {
                let v = a.get();
                assert!(v != 1, "boom");
                v * 2
            })
        };
        a.set(1);
        let caught = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| m.get_untracked()));
        assert!(caught.is_err());
        a.set(2);
        assert_eq!(m.get_untracked(), 4);
    }

    #[test]
    fn effects_keep_creation_order_after_reruns() {
        let sig = Signal::new(0);
        let extra = Signal::new(0);
        let order = Rc::new(RefCell::new(Vec::new()));
        let handles: Vec<_> = (0..3)
            .map(|i| {
                let (sig, extra, order) = (sig.clone(), extra.clone(), order.clone());
                effect(move || {
                    if i == 0 {
                        extra.get();
                    }
                    sig.get();
                    order.borrow_mut().push(i);
                })
            })
            .collect();
        extra.set(1); // effect 0 re-runs and re-subscribes to `sig` last
        order.borrow_mut().clear();
        sig.set(1);
        assert_eq!(*order.borrow(), vec![0, 1, 2]);
        handles.into_iter().for_each(EffectHandle::dispose);
    }

    #[test]
    fn a_runaway_effect_is_disposed_alone() {
        let looping = Signal::new(0u64);
        let other = Signal::new(0);
        let other_runs = counter();
        let runaway = {
            let looping = looping.clone();
            effect(move || {
                let v = looping.get();
                looping.set(v + 1);
            })
        };
        assert_eq!(sub_entries(&looping), 0, "the runaway was unsubscribed");
        let handle = {
            let (other, other_runs) = (other.clone(), other_runs.clone());
            effect(move || {
                other.get();
                other_runs.set(other_runs.get() + 1);
            })
        };
        batch(|| {
            looping.set(0);
            other.set(1);
        });
        assert_eq!(other_runs.get(), 2, "unrelated effects still run");
        runaway.dispose();
        handle.dispose();
    }

    #[test]
    fn child_effects_are_disposed_when_the_parent_reruns() {
        let parent_sig = Signal::new(0);
        let child_sig = Signal::new(0);
        let child_runs = counter();
        let before = live_nodes();
        let parent = {
            let (parent_sig, child_sig, child_runs) = (parent_sig.clone(), child_sig.clone(), child_runs.clone());
            effect(move || {
                parent_sig.get();
                let (child_sig, child_runs) = (child_sig.clone(), child_runs.clone());
                effect(move || {
                    child_sig.get();
                    child_runs.set(child_runs.get() + 1);
                });
            })
        };
        parent_sig.set(1);
        parent_sig.set(2);
        assert_eq!(live_nodes(), before + 2, "one parent, one (current) child");
        child_runs.set(0);
        child_sig.set(1);
        assert_eq!(child_runs.get(), 1, "only the current child reacts");
        parent.dispose();
        assert_eq!(live_nodes(), before);
        child_sig.set(2);
        assert_eq!(child_runs.get(), 1);
    }

    #[derive(Debug, PartialEq)]
    struct NotClone(u32);

    crate::store!(static NOT_CLONE: NotClone = NotClone(1));
    crate::store!(static CLONE_COUNT: u32 = 0);

    #[test]
    fn store_macro_works_for_clone_and_non_clone_types() {
        assert_eq!(NOT_CLONE::with(|v| v.0), 1);
        NOT_CLONE::set(NotClone(2));
        assert!(!NOT_CLONE::set_if_changed(NotClone(2)));
        NOT_CLONE::update(|v| v.0 += 1);
        assert_eq!(NOT_CLONE::signal().with_untracked(|v| v.0), 3);

        assert_eq!(CLONE_COUNT::get(), 0);
        assert!(CLONE_COUNT::set_if_changed(4));
        assert_eq!(CLONE_COUNT::get_untracked(), 4);
        assert_eq!(CLONE_COUNT::with(|v| *v + 1), 5);
    }
}
