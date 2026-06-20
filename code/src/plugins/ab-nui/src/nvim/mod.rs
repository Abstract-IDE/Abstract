//! Thin, safe wrappers over the bits of the Neovim API the UI layer needs.
//!
//! Nothing here touches nvim-oxi — every call goes through [`crate::lua`].

pub mod buffer;
pub mod event;
pub mod extmark;
pub mod window;

pub use buffer::Buffer;
pub use event::{KeymapOpts, augroup, buf_autocmd, buf_keymap};
pub use extmark::Namespace;
pub use window::{Border, FloatConfig, TitlePos, Window};

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
