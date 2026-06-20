//! Keymaps and autocommands that call back into Rust.
//!
//! mlua (without the `send` feature) wants `Fn` callbacks, so handlers are
//! `Fn() + 'static`; capture `Rc`/`Signal`s for any mutable state.

use mlua::Function;

use crate::error::Result;
use crate::lua;
use crate::nvim::buffer::Buffer;

/// Options for a buffer-local keymap.
#[derive(Clone, Copy, Debug)]
pub struct KeymapOpts {
    pub noremap: bool,
    pub silent: bool,
    pub nowait: bool,
}

impl Default for KeymapOpts {
    fn default() -> Self {
        Self { noremap: true, silent: true, nowait: true }
    }
}

/// Bind `lhs` in `mode` for `buf` to a Rust callback (via `vim.keymap.set`).
pub fn buf_keymap(buf: Buffer, mode: &str, lhs: &str, opts: KeymapOpts, callback: impl Fn() + 'static) -> Result<()> {
    let l = lua::lua()?;
    let func: Function = l.create_function(move |_, ()| {
        callback();
        Ok(())
    })?;

    let o = l.create_table()?;
    o.set("buffer", buf.id())?;
    o.set("noremap", opts.noremap)?;
    o.set("silent", opts.silent)?;
    o.set("nowait", opts.nowait)?;

    let set: Function = lua::vim()?.get::<mlua::Table>("keymap")?.get("set")?;
    set.call::<()>((mode.to_string(), lhs.to_string(), func, o))?;
    Ok(())
}

/// Create an autocommand group, clearing any previous definition with the same
/// name. Returns the group id.
pub fn augroup(name: &str) -> Result<i64> {
    let opts = lua::table()?;
    opts.set("clear", true)?;
    lua::call_api("nvim_create_augroup", (name.to_string(), opts))
}

/// Register a buffer-local autocommand calling a Rust callback. Returns the
/// autocmd id.
pub fn buf_autocmd(
    events: &[&str],
    buf: Buffer,
    group: Option<i64>,
    callback: impl Fn() + 'static,
) -> Result<i64> {
    let l = lua::lua()?;
    let func: Function = l.create_function(move |_, _args: mlua::Variadic<mlua::Value>| {
        callback();
        Ok(())
    })?;

    let events: Vec<String> = events.iter().map(|s| s.to_string()).collect();
    let opts = l.create_table()?;
    opts.set("buffer", buf.id())?;
    opts.set("callback", func)?;
    if let Some(g) = group {
        opts.set("group", g)?;
    }

    lua::call_api("nvim_create_autocmd", (events, opts))
}
