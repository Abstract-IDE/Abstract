//! Thin, safe wrappers over the bits of the Neovim API the UI layer needs.
//!
//! Nothing here touches nvim-oxi — every call goes through [`crate::lua`].

pub mod buffer;
pub mod event;
pub mod extmark;
pub mod render;
pub mod timer;
pub mod window;

pub use buffer::Buffer;
pub use event::{
    AutocmdArgs, CommandArgs, CommandOpts, KeymapOpts, augroup, autocmd, autocmd_once, buf_autocmd, buf_keymap,
    buf_keymap_or, buf_keymap_table, del_augroup, del_autocmd, del_keymap, del_user_command, unique_augroup,
    user_command,
};
pub use extmark::{ExtmarkInfo, ExtmarkOpts, Namespace, VirtTextPos};
pub use render::{LINE_BREAK, Mark, Renderer, Row, render, single_line};
pub use timer::{Timer, defer, schedule};
pub use window::{Anchor, Border, FloatConfig, Relative, SplitConfig, SplitDir, TitlePos, WinConfigInfo, Window};

/// The current mode, as `nvim_get_mode().mode` reports it (`"n"`, `"i"`,
/// `"no"`, ...).
pub fn mode() -> crate::Result<String> {
    let info: mlua::Table = crate::lua::call_api("nvim_get_mode", ())?;
    Ok(info.get("mode")?)
}

/// `v:count1`: the count typed before the key being handled (1 without one).
pub fn count1() -> crate::Result<usize> {
    let v: mlua::Table = crate::lua::vim()?.get("v")?;
    Ok(v.get::<i64>("count1")?.max(1) as usize)
}
