//! Scheduling: `vim.schedule`, `vim.defer_fn`, and libuv timers — the pieces
//! that make spinners, debouncing, auto-dismiss and animation possible.
//!
//! uv timer callbacks run in a *fast event* context where most of the API is
//! off-limits, so every callback here is wrapped in `vim.schedule_wrap` before
//! it reaches libuv.

use std::cell::RefCell;

use mlua::{Function, Value};

use crate::error::{Error, Result};
use crate::lua;

/// Run `f` soon on the main loop (`vim.schedule`). Safe to call from anywhere
/// on the Lua thread.
pub fn schedule(f: impl FnOnce() + 'static) -> Result<()> {
    let l = lua::lua()?;
    let slot = RefCell::new(Some(f));
    let func: Function = l.create_function(move |_, ()| {
        if let Some(f) = slot.borrow_mut().take() {
            f();
        }
        Ok(())
    })?;
    let schedule: Function = lua::vim()?.get("schedule")?;
    schedule.call::<()>(func)?;
    Ok(())
}

/// Run `f` after `ms` milliseconds (`vim.defer_fn`).
pub fn defer(ms: u64, f: impl FnOnce() + 'static) -> Result<()> {
    let l = lua::lua()?;
    let slot = RefCell::new(Some(f));
    let func: Function = l.create_function(move |_, ()| {
        if let Some(f) = slot.borrow_mut().take() {
            f();
        }
        Ok(())
    })?;
    let defer_fn: Function = lua::vim()?.get("defer_fn")?;
    defer_fn.call::<()>((func, ms as i64))?;
    Ok(())
}

/// The `vim.uv` (or legacy `vim.loop`) table.
fn uv() -> Result<mlua::Table> {
    let vim = lua::vim()?;
    if let Ok(t) = vim.get::<mlua::Table>("uv") {
        return Ok(t);
    }
    vim.get::<mlua::Table>("loop").map_err(Error::from)
}

/// An owned libuv timer. Stopped and closed on [`Timer::stop`], [`Timer::close`]
/// or drop — so a repeating timer dies with whatever owns it.
pub struct Timer {
    handle: Value,
    closed: std::cell::Cell<bool>,
}

impl Timer {
    /// Start a timer: first fire after `delay_ms`, then every `repeat_ms`
    /// (`repeat_ms = 0` fires once). The callback runs on the main loop.
    pub fn start(delay_ms: u64, repeat_ms: u64, f: impl Fn() + 'static) -> Result<Timer> {
        let l = lua::lua()?;
        let cb: Function = l.create_function(move |_, ()| {
            f();
            Ok(())
        })?;
        let schedule_wrap: Function = lua::vim()?.get("schedule_wrap")?;
        let wrapped: Function = schedule_wrap.call(cb)?;

        let new_timer: Function = uv()?.get("new_timer")?;
        let handle: Value = new_timer.call(())?;

        // `timer:start(delay, repeat, cb)` — a shim avoids fighting mlua's
        // method-call API for foreign (luv) userdata.
        let starter: Function = l
            .load("return function(t, delay, rep, cb) t:start(delay, rep, cb) end")
            .call(())?;
        starter.call::<()>((handle.clone(), delay_ms as i64, repeat_ms as i64, wrapped))?;

        Ok(Timer { handle, closed: std::cell::Cell::new(false) })
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

    /// Stop the timer without releasing the handle (it can be re-started from
    /// Lua, but from Rust prefer creating a fresh one).
    pub fn stop(&self) -> Result<()> {
        let l = lua::lua()?;
        let stopper: Function = l.load("return function(t) t:stop() end").call(())?;
        stopper.call::<()>(self.handle.clone())?;
        Ok(())
    }

    /// Stop and close the timer, releasing the libuv handle. Idempotent.
    pub fn close(&self) -> Result<()> {
        if self.closed.replace(true) {
            return Ok(());
        }
        let l = lua::lua()?;
        let closer: Function = l
            .load("return function(t) t:stop() if not t:is_closing() then t:close() end end")
            .call(())?;
        closer.call::<()>(self.handle.clone())?;
        Ok(())
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        let _ = self.close();
    }
}
