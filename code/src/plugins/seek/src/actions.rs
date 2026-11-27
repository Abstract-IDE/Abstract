//! What happens to a chosen result.

use std::path::Path;
use std::rc::Rc;

use wp_ui::nvim::Buffer;
use wp_ui::{Result, lua};

use crate::entry::{Entry, Target};
use crate::nvim;

/// Where to open a result.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum How {
    Edit,
    Split,
    VSplit,
    Tab,
}

impl How {
    fn file_cmd(self) -> &'static str {
        match self {
            How::Edit => "edit",
            How::Split => "split",
            How::VSplit => "vsplit",
            How::Tab => "tabedit",
        }
    }

    fn buffer_cmd(self) -> &'static str {
        match self {
            How::Edit => "buffer",
            How::Split => "sbuffer",
            How::VSplit => "vertical sbuffer",
            How::Tab => "tab sbuffer",
        }
    }
}

/// A path for Ex commands: relative to the cwd when possible, escaped.
fn ex_path(root: &Path, rel: &str) -> Result<String> {
    let abs = root.join(rel);
    let shown: String = lua::call_fn("fnamemodify", (abs.to_string_lossy().to_string(), ":~:."))?;
    lua::call_fn("fnameescape", (shown,))
}

/// Open `entry` in the current window (or a split/tab).
/// `lines_buf` is the buffer lines-mode entries belong to.
pub fn open(entry: &Entry, how: How, root: &Path, lines_buf: Option<Buffer>) -> Result<()> {
    match &entry.target {
        Target::File { path } => lua::cmd(&format!("{} {}", how.file_cmd(), ex_path(root, path)?)),
        Target::Match { path, lnum, col } => {
            lua::cmd(&format!("{} {}", how.file_cmd(), ex_path(root, path)?))?;
            nvim::call("jump", (*lnum, *col))
        },
        Target::Buffer { bufnr, .. } => lua::cmd(&format!("{} {bufnr}", how.buffer_cmd())),
        Target::Line { lnum, indent } => {
            if let Some(buf) = lines_buf.filter(|b| b.is_valid()) {
                let current: i64 = lua::call_api("nvim_get_current_buf", ())?;
                if how != How::Edit || current != buf.id() {
                    lua::cmd(&format!("{} {}", how.buffer_cmd(), buf.id()))?;
                }
            }
            nvim::call("jump", (*lnum, *indent))
        },
    }
}

/// Send every result to the quickfix list and open it.
pub fn quickfix(entries: &[Rc<Entry>], title: &str, root: &Path, lines_buf: Option<Buffer>) -> Result<()> {
    let l = lua::lua()?;
    let items = l.create_table()?;
    for (i, e) in entries.iter().enumerate() {
        let item = l.create_table()?;
        match &e.target {
            Target::File { path } => {
                item.set("filename", root.join(path).to_string_lossy().to_string())?;
                item.set("lnum", 1)?;
                item.set("text", path.as_str())?;
            },
            Target::Match { path, lnum, col } => {
                item.set("filename", root.join(path).to_string_lossy().to_string())?;
                item.set("lnum", *lnum)?;
                item.set("col", col + 1)?;
                item.set("text", e.text.as_str())?;
            },
            Target::Buffer { bufnr, .. } => {
                item.set("bufnr", *bufnr)?;
                item.set("lnum", nvim::call::<usize>("last_line", *bufnr)?)?;
                item.set("text", e.text.as_str())?;
            },
            Target::Line { lnum, .. } => {
                item.set("bufnr", lines_buf.map(|b| b.id()).unwrap_or(0))?;
                item.set("lnum", *lnum)?;
                item.set("text", e.text.as_str())?;
            },
        }
        items.set(i + 1, item)?;
    }
    nvim::call("quickfix", (title, items))
}
