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
//! A write never runs subscribers recursively. It *queues* them, and the queue
//! is drained as soon as the outermost write (or [`batch`]) finishes:
//!
//! * Outside of any batch, `signal.set(..)` has re-run its subscribers by the
//!   time it returns — plain synchronous behaviour.
//! * Inside a [`batch`] (every Neovim callback this library registers is one),
//!   an effect that depends on several changed signals runs **once**.
//! * An effect that writes a signal it also reads is re-run after it finishes,
//!   until it settles, rather than being silently skipped.
//! * Memos are recomputed before ordinary effects, so an effect never sees a
//!   memo that is stale with respect to the signals it was computed from.
//! * Subscribers run in the order they subscribed, so behaviour is
//!   deterministic.
//!
//! ## Lifetimes
//!
//! Signals are reference counted: when the last clone goes away, so does its
//! bookkeeping. An [`effect`] lives until it is disposed (see [`EffectHandle`]);
//! a [`Memo`] disposes its computation when its last clone is dropped.

use std::cell::{Cell, RefCell};
use std::collections::{HashMap, VecDeque};
use std::mem::ManuallyDrop;
use std::rc::{Rc, Weak};

type Id = u64;

/// Hard stop for one drain of the queue: effects that keep re-triggering each
/// other forever would otherwise freeze the editor.
const MAX_RUNS_PER_FLUSH: u32 = 100_000;

/// The effects subscribed to one signal, in subscription order. Shared between
/// the signal and its subscribers (which need it to unsubscribe).
#[derive(Default)]
struct Subscribers {
    list: RefCell<Vec<Weak<EffectNode>>>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    Effect,
    /// The computation behind a [`Memo`]; drained before ordinary effects.
    Memo,
}

struct EffectNode {
    id: Id,
    kind: Kind,
    /// The closure; taken out while it runs and dropped on dispose.
    f: RefCell<Option<Box<dyn FnMut()>>>,
    /// The signals read during the last run.
    sources: RefCell<Vec<Rc<Subscribers>>>,
    queued: Cell<bool>,
    disposed: Cell<bool>,
}

impl EffectNode {
    /// Detach from every signal this effect is subscribed to.
    fn unsubscribe(self: &Rc<Self>) {
        let me = Rc::as_ptr(self);
        let sources = std::mem::take(&mut *self.sources.borrow_mut());
        for source in sources {
            source.list.borrow_mut().retain(|w| !std::ptr::eq(w.as_ptr(), me));
        }
    }
}

struct Runtime {
    next_id: Cell<Id>,
    /// The effect currently executing (whose reads should be tracked).
    observer: RefCell<Option<Rc<EffectNode>>>,
    /// Every live effect. This is what keeps an effect alive: handles are
    /// plain ids, so `effect(..)` can be fire-and-forget.
    effects: RefCell<HashMap<Id, Rc<EffectNode>>>,
    pending_memos: RefCell<VecDeque<Rc<EffectNode>>>,
    pending_effects: RefCell<VecDeque<Rc<EffectNode>>>,
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
        effects: RefCell::new(HashMap::new()),
        pending_memos: RefCell::new(VecDeque::new()),
        pending_effects: RefCell::new(VecDeque::new()),
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

    fn enqueue(&self, node: Rc<EffectNode>) {
        if node.disposed.get() || node.queued.replace(true) {
            return;
        }
        match node.kind {
            Kind::Memo => self.pending_memos.borrow_mut().push_back(node),
            Kind::Effect => self.pending_effects.borrow_mut().push_back(node),
        }
    }

    fn next_pending(&self) -> Option<Rc<EffectNode>> {
        let memo = self.pending_memos.borrow_mut().pop_front();
        memo.or_else(|| self.pending_effects.borrow_mut().pop_front())
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

        let mut budget = MAX_RUNS_PER_FLUSH;
        while let Some(node) = self.next_pending() {
            node.queued.set(false);
            self.run(&node);
            budget -= 1;
            if budget == 0 {
                // A cycle that never settles: drop what is left rather than
                // hang Neovim.
                while let Some(node) = self.next_pending() {
                    node.queued.set(false);
                }
                break;
            }
        }
    }

    /// Execute one effect: detach its old dependencies, run it as the active
    /// observer (so reads re-subscribe), then restore the previous observer.
    fn run(&self, node: &Rc<EffectNode>) {
        if node.disposed.get() {
            return;
        }
        // Taking the closure out means no `RefCell` borrow is held while user
        // code runs (it may dispose this very effect).
        let Some(mut f) = node.f.borrow_mut().take() else { return };
        node.unsubscribe();

        struct Restore<'a> {
            rt: &'a Runtime,
            prev: Option<Rc<EffectNode>>,
        }
        impl Drop for Restore<'_> {
            fn drop(&mut self) {
                *self.rt.observer.borrow_mut() = self.prev.take();
            }
        }
        let prev = self.observer.borrow_mut().replace(node.clone());
        let restore = Restore { rt: self, prev };
        f();
        drop(restore);

        if node.disposed.get() {
            drop(f);
        } else {
            *node.f.borrow_mut() = Some(f);
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

/// Record that the current observer (if any) read the given signal.
fn track(subs: &Rc<Subscribers>) {
    RT.with(|rt| {
        let observer = rt.observer.borrow();
        let Some(observer) = observer.as_ref() else { return };
        let mut sources = observer.sources.borrow_mut();
        if sources.iter().rev().any(|s| Rc::ptr_eq(s, subs)) {
            return;
        }
        sources.push(subs.clone());
        subs.list.borrow_mut().push(Rc::downgrade(observer));
    });
}

/// Queue every effect subscribed to the given signal, and drain the queue
/// unless a batch (or a running effect) is holding it back.
fn notify(subs: &Subscribers) {
    if subs.list.borrow().is_empty() {
        return;
    }
    let targets: Vec<Rc<EffectNode>> = subs.list.borrow().iter().filter_map(Weak::upgrade).collect();
    RT.with(|rt| {
        for node in targets {
            rt.enqueue(node);
        }
        rt.flush();
    });
}

/// Group several writes: effects that depend on any of the signals written
/// inside `f` run once, after `f` returns (after the outermost `batch`, if
/// nested).
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

/// Run `f` without subscribing the current reactive scope to anything it
/// reads.
pub fn untrack<R>(f: impl FnOnce() -> R) -> R {
    RT.with(|rt| {
        let rt: &Runtime = rt;
        struct Restore<'a> {
            rt: &'a Runtime,
            prev: Option<Rc<EffectNode>>,
        }
        impl Drop for Restore<'_> {
            fn drop(&mut self) {
                *self.rt.observer.borrow_mut() = self.prev.take();
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

impl<T: 'static> Signal<T> {
    /// Create a new signal holding `value`.
    pub fn new(value: T) -> Self {
        Self { inner: Rc::new(SignalInner { value: RefCell::new(value), subs: Rc::default() }) }
    }

    /// Read and clone the value, subscribing the current reactive scope.
    pub fn get(&self) -> T
    where
        T: Clone,
    {
        track(&self.inner.subs);
        self.inner.value.borrow().clone()
    }

    /// Borrow the value (subscribing the current scope) without cloning.
    pub fn with<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        track(&self.inner.subs);
        f(&self.inner.value.borrow())
    }

    /// Read without subscribing — use inside callbacks that must not re-trigger.
    pub fn get_untracked(&self) -> T
    where
        T: Clone,
    {
        self.inner.value.borrow().clone()
    }

    /// Borrow the value without subscribing.
    pub fn with_untracked<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        f(&self.inner.value.borrow())
    }

    /// Replace the value and notify subscribers.
    pub fn set(&self, value: T) {
        // The old value is dropped after the borrow is released, in case its
        // `Drop` reads this signal.
        let old = self.inner.value.replace(value);
        drop(old);
        notify(&self.inner.subs);
    }

    /// Replace the value and notify subscribers, unless it is equal to the
    /// current one (then nothing happens). Returns whether it changed.
    pub fn set_if_changed(&self, value: T) -> bool
    where
        T: PartialEq,
    {
        if *self.inner.value.borrow() == value {
            return false;
        }
        self.set(value);
        true
    }

    /// Mutate the value in place and notify subscribers.
    pub fn update(&self, f: impl FnOnce(&mut T)) {
        {
            let mut v = self.inner.value.borrow_mut();
            f(&mut v);
        }
        notify(&self.inner.subs);
    }

    /// Replace the value WITHOUT notifying subscribers. For runtime-internal
    /// corrections (e.g. clamping focus during a repaint) where the caller is
    /// already about to show the new value.
    pub fn set_silent(&self, value: T) {
        let old = self.inner.value.replace(value);
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

fn create_effect(kind: Kind, f: impl FnMut() + 'static) -> EffectHandle {
    RT.with(|rt| {
        let node = Rc::new(EffectNode {
            id: rt.fresh_id(),
            kind,
            f: RefCell::new(Some(Box::new(f))),
            sources: RefCell::new(Vec::new()),
            queued: Cell::new(false),
            disposed: Cell::new(false),
        });
        rt.effects.borrow_mut().insert(node.id, node.clone());
        // The first run happens right here, even inside a batch. Anything it
        // writes is propagated once it has finished.
        rt.held(|| rt.run(&node));
        EffectHandle { id: node.id }
    })
}

/// Run `f` immediately, and again whenever a signal it read changes.
pub fn effect(f: impl FnMut() + 'static) -> EffectHandle {
    create_effect(Kind::Effect, f)
}

/// Tear down an effect by id: drop its closure and detach it from every signal
/// it was subscribed to. Idempotent.
pub fn dispose_effect(id: u64) {
    // `try_with`: this also runs from destructors, possibly during thread
    // teardown.
    let Ok(Some(node)) = RT.try_with(|rt| rt.effects.borrow_mut().remove(&id)) else { return };
    node.disposed.set(true);
    node.unsubscribe();
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
    signal: Signal<T>,
    _guard: Rc<MemoGuard>,
}

impl<T> Clone for Memo<T> {
    fn clone(&self) -> Self {
        Self { signal: self.signal.clone(), _guard: self._guard.clone() }
    }
}

impl<T: Clone + 'static> Memo<T> {
    /// Read the derived value, subscribing the current scope.
    pub fn get(&self) -> T {
        self.signal.get()
    }

    /// Read the derived value without subscribing.
    pub fn get_untracked(&self) -> T {
        self.signal.get_untracked()
    }

    /// Borrow the derived value, subscribing the current scope.
    pub fn with<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        self.signal.with(f)
    }
}

/// Create a [`Memo`] computed from `f`. It recomputes only when one of the
/// signals `f` reads changes, and stops for good once the last clone of the
/// memo is dropped.
pub fn memo<T: Clone + 'static>(mut f: impl FnMut() -> T + 'static) -> Memo<T> {
    let cell: Rc<RefCell<Option<Signal<T>>>> = Rc::new(RefCell::new(None));
    let inner = cell.clone();
    let handle = create_effect(Kind::Memo, move || {
        let value = f();
        let existing = inner.borrow().clone();
        match existing {
            Some(sig) => sig.set(value),
            None => *inner.borrow_mut() = Some(Signal::new(value)),
        }
    });
    let signal = cell.borrow().clone().expect("memo effect runs synchronously on creation");
    Memo { signal, _guard: Rc::new(MemoGuard(handle.id)) }
}

// -----------------------------------------------------------------------------
// Store
// -----------------------------------------------------------------------------

/// A reactive value holder — the recommended type for shared/app-level state
/// that several widgets read and react to. It's a thin, clone-shareable wrapper
/// over a [`Signal`]; cloning yields another handle to the same value.
///
/// For *process-global* values, see the [`store!`](crate::store) macro, which
/// declares a lazily-initialized global `Store` accessor.
pub struct Store<T> {
    signal: Signal<T>,
}

impl<T> Clone for Store<T> {
    fn clone(&self) -> Self {
        Self { signal: self.signal.clone() }
    }
}

impl<T: 'static> Store<T> {
    pub fn new(value: T) -> Self {
        Self { signal: Signal::new(value) }
    }

    /// Reactive read (subscribes the current scope).
    pub fn get(&self) -> T
    where
        T: Clone,
    {
        self.signal.get()
    }

    /// Read without subscribing.
    pub fn get_untracked(&self) -> T
    where
        T: Clone,
    {
        self.signal.get_untracked()
    }

    /// Borrow the value (subscribing the current scope).
    pub fn with<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        self.signal.with(f)
    }

    pub fn set(&self, value: T) {
        self.signal.set(value);
    }

    pub fn update(&self, f: impl FnOnce(&mut T)) {
        self.signal.update(f);
    }

    /// The underlying signal, for APIs that take a `Signal`.
    pub fn signal(&self) -> Signal<T> {
        self.signal.clone()
    }
}

/// Declare a process-global reactive value (per thread, lazily initialized).
///
/// ```ignore
/// wp_ui::store!(pub static THEME: u8 = 0);
///
/// THEME::get();             // reactive read — widgets reading this re-render
/// THEME::set(1);            // notify all readers
/// THEME::update(|t| *t ^= 1);
/// let sig = THEME::signal(); // pass to widgets that take a Signal
/// ```
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
            $vis fn get() -> $ty
            where
                $ty: ::std::clone::Clone,
            {
                Self::__signal().get()
            }
            $vis fn set(value: $ty) {
                Self::__signal().set(value);
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
                if cond.get() { a.get() } else { b.get() };
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
}
