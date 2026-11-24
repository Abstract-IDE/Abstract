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
    AutocmdArgs, CommandArgs, CommandOpts, KeymapOpts, augroup, autocmd, autocmd_once, buf_autocmd,
    buf_keymap, buf_keymap_table, del_augroup, del_autocmd, del_keymap, del_user_command, unique_augroup, user_command,
};
pub use extmark::{ExtmarkInfo, ExtmarkOpts, Namespace, VirtTextPos};
pub use render::{Mark, Renderer, Row, render};
pub use timer::{Timer, defer, schedule};
pub use window::{
    Anchor, Border, FloatConfig, Relative, SplitConfig, SplitDir, TitlePos, WinConfigInfo, Window,
};
