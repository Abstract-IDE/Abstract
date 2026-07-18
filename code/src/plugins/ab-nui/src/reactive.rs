//! Fine-grained reactive state — the "state management" layer for UIs.
//!
//! Three primitives, modelled after Solid/Leptos but tiny and synchronous:
//!
//! * [`Signal<T>`] — a mutable reactive value. Reading it inside a reactive
//!   scope subscribes that scope; writing it re-runs every subscriber.
//! * [`effect`] — runs a closure now, and again whenever any signal it read
//!   changes. Dependencies are tracked automatically, no manual wiring.
//! * [`memo`] — a cached value derived from other signals.
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

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

type Id = u64;

thread_local! {
    static RT: RefCell<Runtime> = RefCell::new(Runtime::new());
}

/// The reactive graph. Signals and effects are identified by integer ids; the
/// graph only stores ids, so values can live anywhere.
struct Runtime {
    next: Id,
    /// The effect currently executing (whose reads should be tracked).
    observer: Option<Id>,
    /// effect id -> its closure.
    effects: HashMap<Id, Rc<RefCell<dyn FnMut()>>>,
    /// effect id -> the signals it currently depends on.
    sources: HashMap<Id, HashSet<Id>>,
    /// signal id -> the effects subscribed to it.
    subscribers: HashMap<Id, HashSet<Id>>,
    /// effects mid-execution, to break write→run→write cycles.
    running: HashSet<Id>,
}

impl Runtime {
    fn new() -> Self {
        Self {
            next: 1,
            observer: None,
            effects: HashMap::new(),
            sources: HashMap::new(),
            subscribers: HashMap::new(),
            running: HashSet::new(),
        }
    }

    fn fresh_id(&mut self) -> Id {
        let id = self.next;
        self.next += 1;
        id
    }
}

fn with_rt<R>(f: impl FnOnce(&mut Runtime) -> R) -> R {
    RT.with(|cell| f(&mut cell.borrow_mut()))
}

/// Record that the current observer (if any) read the given signal.
fn track(signal: Id) {
    with_rt(|rt| {
        if let Some(obs) = rt.observer {
            rt.subscribers.entry(signal).or_default().insert(obs);
            rt.sources.entry(obs).or_default().insert(signal);
        }
    });
}

/// Re-run every effect subscribed to the given signal.
fn notify(signal: Id) {
    let observers: Vec<Id> =
        with_rt(|rt| rt.subscribers.get(&signal).map(|s| s.iter().copied().collect()).unwrap_or_default());
    for id in observers {
        run_effect(id);
    }
}

/// Execute one effect: detach its old dependencies, run it as the active
/// observer (so reads re-subscribe), then restore the previous observer.
fn run_effect(id: Id) {
    // Phase 1 (graph locked): clean old deps, take the closure, set observer.
    let prepared = with_rt(|rt| {
        if rt.running.contains(&id) {
            return None; // already on the stack — skip to avoid a cycle
        }
        if let Some(old) = rt.sources.remove(&id) {
            for sig in old {
                if let Some(set) = rt.subscribers.get_mut(&sig) {
                    set.remove(&id);
                }
            }
        }
        let cb = rt.effects.get(&id).cloned()?;
        let prev = rt.observer;
        rt.observer = Some(id);
        rt.running.insert(id);
        Some((cb, prev))
    });

    // Phase 2 (graph unlocked): run user code. Its signal reads call back into
    // `track`, and its writes call `notify` — both re-borrow the runtime fine.
    if let Some((cb, prev)) = prepared {
        (cb.borrow_mut())();
        with_rt(|rt| {
            rt.observer = prev;
            rt.running.remove(&id);
        });
    }
}

// -----------------------------------------------------------------------------
// Signal
// -----------------------------------------------------------------------------

/// A reactive value. Clone is cheap and shares the same underlying cell, so a
/// `Signal` can be freely captured into callbacks.
pub struct Signal<T> {
    id: Id,
    value: Rc<RefCell<T>>,
}

impl<T> Clone for Signal<T> {
    fn clone(&self) -> Self {
        Self { id: self.id, value: self.value.clone() }
    }
}

impl<T: 'static> Signal<T> {
    /// Create a new signal holding `value`.
    pub fn new(value: T) -> Self {
        let id = with_rt(|rt| rt.fresh_id());
        Self { id, value: Rc::new(RefCell::new(value)) }
    }

    /// Read and clone the value, subscribing the current reactive scope.
    pub fn get(&self) -> T
    where
        T: Clone,
    {
        track(self.id);
        self.value.borrow().clone()
    }

    /// Borrow the value (subscribing the current scope) without cloning.
    pub fn with<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        track(self.id);
        f(&self.value.borrow())
    }

    /// Read without subscribing — use inside callbacks that must not re-trigger.
    pub fn get_untracked(&self) -> T
    where
        T: Clone,
    {
        self.value.borrow().clone()
    }

    /// Replace the value and notify subscribers.
    pub fn set(&self, value: T) {
        *self.value.borrow_mut() = value;
        notify(self.id);
    }

    /// Mutate the value in place and notify subscribers.
    pub fn update(&self, f: impl FnOnce(&mut T)) {
        {
            let mut v = self.value.borrow_mut();
            f(&mut v);
        }
        notify(self.id);
    }

    /// Replace the value WITHOUT notifying subscribers. For runtime-internal
    /// corrections (e.g. clamping focus during a repaint) where notifying
    /// would re-run the effect that is currently running.
    pub fn set_silent(&self, value: T) {
        *self.value.borrow_mut() = value;
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

/// Run `f` immediately, and again whenever a signal it read changes.
pub fn effect(f: impl FnMut() + 'static) -> EffectHandle {
    let id = with_rt(|rt| {
        let id = rt.fresh_id();
        let boxed: Rc<RefCell<dyn FnMut()>> = Rc::new(RefCell::new(f));
        rt.effects.insert(id, boxed);
        id
    });
    run_effect(id);
    EffectHandle { id }
}

/// Tear down an effect by id: drop its closure and detach it from every signal
/// it was subscribed to. Idempotent.
pub fn dispose_effect(id: u64) {
    with_rt(|rt| {
        rt.effects.remove(&id);
        rt.running.remove(&id);
        if let Some(sources) = rt.sources.remove(&id) {
            for sig in sources {
                if let Some(set) = rt.subscribers.get_mut(&sig) {
                    set.remove(&id);
                }
            }
        }
    });
}

/// A cached, read-only value derived from other signals.
pub struct Memo<T> {
    signal: Signal<T>,
}

impl<T> Clone for Memo<T> {
    fn clone(&self) -> Self {
        Self { signal: self.signal.clone() }
    }
}

impl<T: Clone + 'static> Memo<T> {
    /// Read the derived value, subscribing the current scope.
    pub fn get(&self) -> T {
        self.signal.get()
    }

    /// Borrow the derived value, subscribing the current scope.
    pub fn with<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        self.signal.with(f)
    }
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

/// Create a [`Memo`] computed from `f`. It recomputes only when one of the
/// signals `f` reads changes.
pub fn memo<T: Clone + 'static>(mut f: impl FnMut() -> T + 'static) -> Memo<T> {
    let cell: Rc<RefCell<Option<Signal<T>>>> = Rc::new(RefCell::new(None));
    let inner = cell.clone();
    effect(move || {
        let value = f();
        let existing = inner.borrow().clone();
        match existing {
            Some(sig) => sig.set(value),
            None => *inner.borrow_mut() = Some(Signal::new(value)),
        }
    });
    let signal = cell.borrow().clone().expect("memo effect runs synchronously on creation");
    Memo { signal }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let runs = Rc::new(std::cell::Cell::new(0));
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
    fn memo_recomputes_only_on_dependency_change() {
        let a = Signal::new(2);
        let computes = Rc::new(std::cell::Cell::new(0));
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
    fn writing_a_signal_inside_its_own_effect_does_not_recurse() {
        let sig = Signal::new(0);
        let runs = Rc::new(std::cell::Cell::new(0));
        let handle = {
            let sig = sig.clone();
            let runs = runs.clone();
            effect(move || {
                runs.set(runs.get() + 1);
                let v = sig.get();
                if v < 1 {
                    sig.set(v + 1); // re-entrancy guard: must not loop forever
                }
            })
        };
        assert!(runs.get() >= 1);
        handle.dispose();
    }

    #[test]
    fn each_run_rebuilds_exact_dependencies() {
        let cond = Signal::new(true);
        let a = Signal::new(0);
        let b = Signal::new(0);
        let runs = Rc::new(std::cell::Cell::new(0));
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
}
