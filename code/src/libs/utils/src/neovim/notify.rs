//! `vim.notify` from Rust.
//!
//! Neovim 0.12 deprecated `nvim_notify()` (and nvim-oxi dropped its binding),
//! so this goes through `vim.notify` itself — which also means notification
//! plugins that override `vim.notify` keep working.

use nvim_oxi::{
    Array, Object,
    api::{self, types::LogLevel},
};

/// Show `msg` through `vim.notify` at the given level.
pub fn notify(msg: &str, level: LogLevel) -> Result<(), api::Error> {
    let args = Array::from_iter([Object::from(msg), Object::from(level as i64)]);
    api::call_function::<_, Object>("luaeval", ("vim.notify(_A[1], _A[2])", args)).map(|_| ())
}
