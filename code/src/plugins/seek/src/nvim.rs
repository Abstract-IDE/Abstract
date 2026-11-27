//! Calls into the embedded Lua helpers (`helpers.lua`).

use std::cell::RefCell;
use std::mem::ManuallyDrop;

use mlua::{FromLuaMulti, Function, IntoLuaMulti, Table};
use wp_ui::{Result, lua};

const HELPERS: &str = include_str!("helpers.lua");

thread_local! {
    // `ManuallyDrop`: never release Lua values from a thread-exit destructor.
    static MODULE: ManuallyDrop<RefCell<Option<Table>>> = const { ManuallyDrop::new(RefCell::new(None)) };
}

fn module() -> Result<Table> {
    if let Some(m) = MODULE.with(|m| m.borrow().clone()) {
        return Ok(m);
    }
    let m: Table = lua::lua()?.load(HELPERS).set_name("seek/helpers.lua").eval()?;
    MODULE.with(|slot| *slot.borrow_mut() = Some(m.clone()));
    Ok(m)
}

/// Call `helpers.<name>(args...)`.
pub fn call<R: FromLuaMulti>(name: &str, args: impl IntoLuaMulti) -> Result<R> {
    let f: Function = module()?.get(name)?;
    Ok(f.call(args)?)
}
