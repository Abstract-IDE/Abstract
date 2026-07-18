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

/// Create a per-instance autocommand group (`"{prefix}_{n}"` with a
/// process-unique counter), so concurrent widgets never clear each other's
/// autocmds. Returns the group id.
pub fn unique_augroup(prefix: &str) -> Result<i64> {
    thread_local! {
        static NEXT: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
    }
    let n = NEXT.with(|c| {
        let n = c.get();
        c.set(n + 1);
        n
    });
    augroup(&format!("{prefix}_{n}"))
}

/// Delete an autocommand group (and all its autocmds) by id.
pub fn del_augroup(id: i64) -> Result<()> {
    lua::call_api("nvim_del_augroup_by_id", id)
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

/// What Neovim passed to an autocommand callback.
#[derive(Clone, Debug, Default)]
pub struct AutocmdArgs {
    /// The fired event name (e.g. `"VimResized"`).
    pub event: String,
    /// The buffer, when the event carries one.
    pub buf: Option<i64>,
    /// Expanded `<afile>`.
    pub file: String,
    /// The pattern the event matched.
    pub matched: String,
}

/// Register a global (non-buffer) autocommand calling a Rust callback with the
/// event's args. Returns the autocmd id.
///
/// `pattern = None` matches everything (`"*"`).
pub fn autocmd(
    events: &[&str],
    pattern: Option<&str>,
    group: Option<i64>,
    callback: impl Fn(&AutocmdArgs) + 'static,
) -> Result<i64> {
    let l = lua::lua()?;
    let func: Function = l.create_function(move |_, args: mlua::Table| {
        let parsed = AutocmdArgs {
            event: args.get("event").unwrap_or_default(),
            buf: args.get("buf").ok(),
            file: args.get("file").unwrap_or_default(),
            matched: args.get("match").unwrap_or_default(),
        };
        callback(&parsed);
        Ok(())
    })?;

    let events: Vec<String> = events.iter().map(|s| s.to_string()).collect();
    let opts = l.create_table()?;
    opts.set("pattern", pattern.unwrap_or("*").to_string())?;
    opts.set("callback", func)?;
    if let Some(g) = group {
        opts.set("group", g)?;
    }

    lua::call_api("nvim_create_autocmd", (events, opts))
}

/// Like [`autocmd`], but the autocommand removes itself after firing once.
pub fn autocmd_once(
    events: &[&str],
    pattern: Option<&str>,
    group: Option<i64>,
    callback: impl Fn(&AutocmdArgs) + 'static,
) -> Result<i64> {
    let l = lua::lua()?;
    let func: Function = l.create_function(move |_, args: mlua::Table| {
        let parsed = AutocmdArgs {
            event: args.get("event").unwrap_or_default(),
            buf: args.get("buf").ok(),
            file: args.get("file").unwrap_or_default(),
            matched: args.get("match").unwrap_or_default(),
        };
        callback(&parsed);
        Ok(())
    })?;

    let events: Vec<String> = events.iter().map(|s| s.to_string()).collect();
    let opts = l.create_table()?;
    opts.set("pattern", pattern.unwrap_or("*").to_string())?;
    opts.set("callback", func)?;
    opts.set("once", true)?;
    if let Some(g) = group {
        opts.set("group", g)?;
    }

    lua::call_api("nvim_create_autocmd", (events, opts))
}

/// Delete one autocommand by id.
pub fn del_autocmd(id: i64) -> Result<()> {
    lua::call_api("nvim_del_autocmd", id)
}

/// Remove a keymap set with [`buf_keymap`] (or any `vim.keymap.set` map).
pub fn del_keymap(buf: Option<Buffer>, mode: &str, lhs: &str) -> Result<()> {
    let l = lua::lua()?;
    let opts = l.create_table()?;
    if let Some(b) = buf {
        opts.set("buffer", b.id())?;
    }
    let del: Function = lua::vim()?.get::<mlua::Table>("keymap")?.get("del")?;
    del.call::<()>((mode.to_string(), lhs.to_string(), opts))?;
    Ok(())
}

/// Options for a user command created with [`user_command`].
#[derive(Clone, Debug, Default)]
pub struct CommandOpts {
    /// `nargs` spec: `"0"`, `"1"`, `"*"`, `"?"`, or `"+"`.
    pub nargs: Option<&'static str>,
    pub desc: Option<String>,
    pub bang: bool,
    pub range: bool,
    /// Completion spec (e.g. `"file"`, `"buffer"`).
    pub complete: Option<String>,
}

/// What Neovim passed to a user command callback.
#[derive(Clone, Debug, Default)]
pub struct CommandArgs {
    /// The raw argument string.
    pub args: String,
    /// The split argument list.
    pub fargs: Vec<String>,
    pub bang: bool,
    pub line1: i64,
    pub line2: i64,
}

/// Create a user command (`:Name ...`) backed by a Rust callback.
pub fn user_command(name: &str, opts: &CommandOpts, callback: impl Fn(CommandArgs) + 'static) -> Result<()> {
    let l = lua::lua()?;
    let func: Function = l.create_function(move |_, args: mlua::Table| {
        let parsed = CommandArgs {
            args: args.get("args").unwrap_or_default(),
            fargs: args.get("fargs").unwrap_or_default(),
            bang: args.get("bang").unwrap_or(false),
            line1: args.get("line1").unwrap_or(0),
            line2: args.get("line2").unwrap_or(0),
        };
        callback(parsed);
        Ok(())
    })?;

    let o = l.create_table()?;
    if let Some(nargs) = opts.nargs {
        o.set("nargs", nargs)?;
    }
    if let Some(desc) = &opts.desc {
        o.set("desc", desc.clone())?;
    }
    if opts.bang {
        o.set("bang", true)?;
    }
    if opts.range {
        o.set("range", true)?;
    }
    if let Some(complete) = &opts.complete {
        o.set("complete", complete.clone())?;
    }

    lua::call_api("nvim_create_user_command", (name.to_string(), func, o))
}

/// Delete a user command by name.
pub fn del_user_command(name: &str) -> Result<()> {
    lua::call_api("nvim_del_user_command", name.to_string())
}
