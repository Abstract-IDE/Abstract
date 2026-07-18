//! The bridge to Neovim.
//!
//! Everything this library does to Neovim goes through the global `vim` table
//! (`vim.api`, `vim.fn`, `vim.keymap`, ...). We never touch nvim-oxi: the host
//! plugin hands us its live [`mlua::Lua`] once via [`init`], and every call here
//! reuses it.
//!
//! Neovim's Lua runs on a single thread, so the handle lives in a thread-local.

use std::cell::RefCell;

use mlua::{Function, IntoLua, IntoLuaMulti, Lua, Table};

use crate::error::{Error, Result};

thread_local! {
    static LUA: RefCell<Option<Lua>> = const { RefCell::new(None) };
}

/// Register the host's `Lua` handle. Call this exactly once during setup,
/// e.g. `ui::init(&nvim_oxi::mlua::lua())`.
pub fn init(lua: &Lua) {
    LUA.with(|cell| *cell.borrow_mut() = Some(lua.clone()));
}

/// `true` once [`init`] has been called.
pub fn is_initialized() -> bool {
    LUA.with(|cell| cell.borrow().is_some())
}

/// Get a clone of the live `Lua` handle.
///
/// Cheap (the handle is reference-counted). Returns an error rather than
/// panicking so callers can surface a friendly message.
pub fn lua() -> Result<Lua> {
    LUA.with(|cell| cell.borrow().clone()).ok_or(Error::NotInitialized)
}

/// Fresh, empty Lua table.
pub fn table() -> Result<Table> {
    Ok(lua()?.create_table()?)
}

/// The global `vim` table.
pub fn vim() -> Result<Table> {
    Ok(lua()?.globals().get::<Table>("vim")?)
}

/// `vim.api`.
pub fn api() -> Result<Table> {
    Ok(vim()?.get::<Table>("api")?)
}

/// `vim.fn`.
pub fn vfn() -> Result<Table> {
    Ok(vim()?.get::<Table>("fn")?)
}

/// Call a `vim.api.*` function by name.
///
/// ```ignore
/// let bufnr: i64 = ui::lua::call_api("nvim_create_buf", (false, true))?;
/// ```
pub fn call_api<R: mlua::FromLuaMulti>(name: &str, args: impl IntoLuaMulti) -> Result<R> {
    let f: Function = api()?.get(name)?;
    Ok(f.call(args)?)
}

/// Call a `vim.fn.*` (vimscript) function by name.
pub fn call_fn<R: mlua::FromLuaMulti>(name: &str, args: impl IntoLuaMulti) -> Result<R> {
    let f: Function = vfn()?.get(name)?;
    Ok(f.call(args)?)
}

/// Run an Ex command string (`vim.cmd(...)`).
pub fn cmd(command: &str) -> Result<()> {
    let f: Function = vim()?.get("cmd")?;
    f.call::<()>(command.to_string())?;
    Ok(())
}

/// Read a global option via `nvim_get_option_value(name, {})`.
pub fn get_option<R: mlua::FromLua>(name: &str) -> Result<R> {
    let opts = table()?;
    call_api("nvim_get_option_value", (name.to_string(), opts))
}

/// Read an option with an explicit scope table (`{ buf = .. }`, `{ win = .. }`)
/// via `nvim_get_option_value`.
pub fn get_option_scoped<R: mlua::FromLua>(name: &str, scope: Table) -> Result<R> {
    call_api("nvim_get_option_value", (name.to_string(), scope))
}

/// Set an option with an explicit scope table (`{ buf = .. }`, `{ win = .. }`,
/// or empty for global) via `nvim_set_option_value`.
pub fn set_option_scoped(name: &str, value: impl IntoLua, scope: Table) -> Result<()> {
    call_api::<()>("nvim_set_option_value", (name.to_string(), value, scope))
}
