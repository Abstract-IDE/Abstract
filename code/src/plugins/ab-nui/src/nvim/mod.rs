//! Thin, safe wrappers over the bits of the Neovim API the UI layer needs.
//!
//! Nothing here touches nvim-oxi — every call goes through [`crate::lua`].

pub mod buffer;
pub mod event;
pub mod extmark;
pub mod timer;
pub mod window;

pub use buffer::Buffer;
pub use event::{
    AutocmdArgs, CommandArgs, CommandOpts, KeymapOpts, augroup, autocmd, autocmd_once, buf_autocmd,
    buf_keymap, del_augroup, del_autocmd, del_keymap, del_user_command, unique_augroup, user_command,
};
pub use extmark::{ExtmarkInfo, ExtmarkOpts, Namespace, VirtTextPos};
pub use timer::{Timer, defer, schedule};
pub use window::{
    Anchor, Border, FloatConfig, Relative, SplitConfig, SplitDir, TitlePos, WinConfigInfo, Window,
};

use crate::error::Result;
use crate::text::Line;

/// Render styled [`Line`]s into `buf`, painting span highlights via `ns`.
///
/// Replaces the buffer contents, clears the namespace, then re-applies one
/// extmark per styled span. Highlight columns are byte offsets, matching the
/// plain text written to the buffer.
pub fn render(buf: Buffer, ns: Namespace, lines: &[Line]) -> Result<()> {
    let plain: Vec<String> = lines.iter().map(Line::text).collect();
    buf.set_all(plain)?;
    ns.clear(buf)?;

    for (row, line) in lines.iter().enumerate() {
        let mut col: usize = 0;
        for span in &line.spans {
            let len = span.text.len();
            if let Some(group) = &span.hl
                && len > 0
            {
                ns.highlight(buf, row as i64, col as i64, (col + len) as i64, group)?;
            }
            col += len;
        }
    }
    Ok(())
}
