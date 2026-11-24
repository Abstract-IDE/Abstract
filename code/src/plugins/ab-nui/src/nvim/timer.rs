//! Scheduling: `vim.schedule`, `vim.defer_fn`, and libuv timers — the pieces
//! that make spinners, debouncing, auto-dismiss and animation possible.
//!
//! uv timer callbacks run in a *fast event* context where most of the API is
//! off-limits, so every timer callback is wrapped in `vim.schedule_wrap`
//! before it reaches libuv (see the driver in [`crate::lua`]).
//!
//! Like every callback this library registers, these run inside a
//! [`batch`](crate::reactive::batch).

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use mlua::{Function, Value};

use crate::error::Result;
use crate::lua;
use crate::reactive::batch;

/// A Lua function that runs `f` the first time it is called.
fn once_fn(f: impl FnOnce() + 'static) -> Result<Function> {
    let slot = RefCell::new(Some(f));
    Ok(lua::lua()?.create_function(move |_, ()| {
        // Take it out first: `f` may schedule something that ends up here.
        let f = slot.borrow_mut().take();
        if let Some(f) = f {
            batch(f);
        }
        Ok(())
    })?)
}

/// Run `f` soon on the main loop (`vim.schedule`). Safe to call from anywhere
/// on the Lua thread.
pub fn schedule(f: impl FnOnce() + 'static) -> Result<()> {
    lua::vim_fn("schedule")?.call::<()>(once_fn(f)?)?;
    Ok(())
}

/// Run `f` after `ms` milliseconds (`vim.defer_fn`).
pub fn defer(ms: u64, f: impl FnOnce() + 'static) -> Result<()> {
    lua::vim_fn("defer_fn")?.call::<()>((once_fn(f)?, ms as i64))?;
    Ok(())
}

/// An owned libuv timer. Stopped and closed on [`Timer::stop`], [`Timer::close`]
/// or drop — so a repeating timer dies with whatever owns it.
///
/// Once stopped or closed, the callback is never invoked again — not even for
/// a tick that had already been queued on the main loop.
pub struct Timer {
    handle: Value,
    /// Shared with the callback: cleared on stop/close.
    active: Rc<Cell<bool>>,
    closed: Cell<bool>,
}

impl Timer {
    /// Start a timer: first fire after `delay_ms`, then every `repeat_ms`
    /// (`repeat_ms = 0` fires once). The callback runs on the main loop.
    pub fn start(delay_ms: u64, repeat_ms: u64, f: impl Fn() + 'static) -> Result<Timer> {
        let active = Rc::new(Cell::new(true));
        let cb: Function = lua::lua()?.create_function({
            let active = active.clone();
            move |_, ()| {
                if active.get() {
                    batch(&f);
                }
                Ok(())
            }
        })?;
        let handle: Value = lua::driver_fn("timer_start")?.call((delay_ms as i64, repeat_ms as i64, cb))?;
        Ok(Timer { handle, active, closed: Cell::new(false) })
    }

    /// Convenience: run `f` once after `ms` milliseconds on an owned timer
    /// (unlike [`defer`], this can be cancelled by dropping the handle).
    pub fn once(ms: u64, f: impl Fn() + 'static) -> Result<Timer> {
        Self::start(ms, 0, f)
    }

    /// Convenience: run `f` every `ms` milliseconds.
    pub fn interval(ms: u64, f: impl Fn() + 'static) -> Result<Timer> {
        Self::start(ms, ms.max(1), f)
    }

    /// Stop the timer without releasing the handle. A stopped timer does not
    /// fire again; create a fresh one to restart.
    pub fn stop(&self) -> Result<()> {
        self.active.set(false);
        if !self.closed.get() {
            lua::driver_fn("timer_stop")?.call::<()>(&self.handle)?;
        }
        Ok(())
    }

    /// Stop and close the timer, releasing the libuv handle. Idempotent.
    pub fn close(&self) -> Result<()> {
        self.active.set(false);
        if self.closed.replace(true) {
            return Ok(());
        }
        lua::driver_fn("timer_close")?.call::<()>(&self.handle)?;
        Ok(())
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        let _ = self.close();
    }
}
