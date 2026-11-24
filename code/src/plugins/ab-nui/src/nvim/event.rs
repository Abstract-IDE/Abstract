//! Keymaps, autocommands and user commands that call back into Rust.
//!
//! mlua (without the `send` feature) wants `Fn` callbacks, so handlers are
//! `Fn() + 'static`; capture `Rc`/`Signal`s for any mutable state.
//!
//! Every callback registered here runs inside a [`batch`], so a handler that
//! writes several signals triggers one repaint rather than one per write.

use mlua::{Function, Table};

use crate::error::Result;
use crate::lua;
use crate::nvim::buffer::Buffer;
use crate::reactive::batch;

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
        batch(&callback);
        Ok(())
    })?;

    let o = l.create_table()?;
    o.set("buffer", buf.id())?;
    o.set("noremap", opts.noremap)?;
    o.set("silent", opts.silent)?;
    o.set("nowait", opts.nowait)?;

    lua::vim_fn("keymap.set")?.call::<()>((mode, lhs, func, o))?;
    Ok(())
}

/// Bind many keys of `buf` (in `mode`) to a single Rust dispatcher: pressing
/// `lhs` calls `dispatch(code)` with the code paired with it.
///
/// This is what a [`Surface`](crate::view::Surface) uses for its key table —
/// one Rust callback and one trip into Lua for the whole set, instead of a
/// callback and a `vim.keymap.set` call per key.
pub fn buf_keymap_table(
    buf: Buffer,
    mode: &str,
    keys: impl IntoIterator<Item = (String, i64)>,
    dispatch: impl Fn(i64) + 'static,
) -> Result<()> {
    let l = lua::lua()?;
    let func: Function = l.create_function(move |_, code: i64| {
        batch(|| dispatch(code));
        Ok(())
    })?;
    let table = l.create_table()?;
    for (lhs, code) in keys {
        table.raw_push(lhs)?;
        table.raw_push(code)?;
    }
    lua::driver_fn("map_keys")?.call::<()>((buf.id(), mode, table, func))?;
    Ok(())
}

/// Create an autocommand group, clearing any previous definition with the same
/// name. Returns the group id.
pub fn augroup(name: &str) -> Result<i64> {
    let opts = lua::table()?;
    opts.set("clear", true)?;
    lua::call_api("nvim_create_augroup", (name, opts))
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

impl AutocmdArgs {
    fn parse(args: &Table) -> Self {
        Self {
            event: args.get("event").unwrap_or_default(),
            buf: args.get("buf").ok(),
            file: args.get("file").unwrap_or_default(),
            matched: args.get("match").unwrap_or_default(),
        }
    }
}

/// What an autocommand is attached to.
enum Target<'a> {
    Buffer(Buffer),
    Pattern(&'a str),
}

/// The one place autocommands are created.
fn create_autocmd(events: &[&str], target: Target, group: Option<i64>, once: bool, func: Function) -> Result<i64> {
    let l = lua::lua()?;
    let opts = l.create_table()?;
    match target {
        Target::Buffer(buf) => opts.set("buffer", buf.id())?,
        Target::Pattern(pattern) => opts.set("pattern", pattern)?,
    }
    opts.set("callback", func)?;
    if once {
        opts.set("once", true)?;
    }
    if let Some(g) = group {
        opts.set("group", g)?;
    }
    lua::call_api("nvim_create_autocmd", (l.create_sequence_from(events.iter().copied())?, opts))
}

/// Register a buffer-local autocommand calling a Rust callback. Returns the
/// autocmd id.
pub fn buf_autocmd(events: &[&str], buf: Buffer, group: Option<i64>, callback: impl Fn() + 'static) -> Result<i64> {
    // The callback's return value is discarded: returning a truthy value from
    // an autocmd callback would delete the autocommand.
    let func = lua::lua()?.create_function(move |_, _args: mlua::Variadic<mlua::Value>| {
        batch(&callback);
        Ok(())
    })?;
    create_autocmd(events, Target::Buffer(buf), group, false, func)
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
    let func = lua::lua()?.create_function(move |_, args: Table| {
        let args = AutocmdArgs::parse(&args);
        batch(|| callback(&args));
        Ok(())
    })?;
    create_autocmd(events, Target::Pattern(pattern.unwrap_or("*")), group, false, func)
}

/// Like [`autocmd`], but the autocommand removes itself after firing once.
pub fn autocmd_once(
    events: &[&str],
    pattern: Option<&str>,
    group: Option<i64>,
    callback: impl Fn(&AutocmdArgs) + 'static,
) -> Result<i64> {
    let func = lua::lua()?.create_function(move |_, args: Table| {
        let args = AutocmdArgs::parse(&args);
        batch(|| callback(&args));
        Ok(())
    })?;
    create_autocmd(events, Target::Pattern(pattern.unwrap_or("*")), group, true, func)
}

/// Delete one autocommand by id.
pub fn del_autocmd(id: i64) -> Result<()> {
    lua::call_api("nvim_del_autocmd", id)
}

/// Remove a keymap set with [`buf_keymap`] (or any `vim.keymap.set` map).
pub fn del_keymap(buf: Option<Buffer>, mode: &str, lhs: &str) -> Result<()> {
    let opts = lua::table()?;
    if let Some(b) = buf {
        opts.set("buffer", b.id())?;
    }
    lua::vim_fn("keymap.del")?.call::<()>((mode, lhs, opts))?;
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
    let func: Function = l.create_function(move |_, args: Table| {
        let parsed = CommandArgs {
            args: args.get("args").unwrap_or_default(),
            fargs: args.get("fargs").unwrap_or_default(),
            bang: args.get("bang").unwrap_or(false),
            line1: args.get("line1").unwrap_or(0),
            line2: args.get("line2").unwrap_or(0),
        };
        batch(|| callback(parsed));
        Ok(())
    })?;

    let o = l.create_table()?;
    if let Some(nargs) = opts.nargs {
        o.set("nargs", nargs)?;
    }
    if let Some(desc) = &opts.desc {
        o.set("desc", desc.as_str())?;
    }
    if opts.bang {
        o.set("bang", true)?;
    }
    if opts.range {
        o.set("range", true)?;
    }
    if let Some(complete) = &opts.complete {
        o.set("complete", complete.as_str())?;
    }

    lua::call_api("nvim_create_user_command", (name, func, o))
}

/// Delete a user command by name.
pub fn del_user_command(name: &str) -> Result<()> {
    lua::call_api("nvim_del_user_command", name)
}
