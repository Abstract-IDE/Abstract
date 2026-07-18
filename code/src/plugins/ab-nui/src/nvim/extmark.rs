//! Highlight namespaces and extmark decorations: range highlights, virtual
//! text, virtual lines, signs, and conceal — everything `nvim_buf_set_extmark`
//! offers that a UI needs.

use mlua::Table;

use crate::error::Result;
use crate::lua;
use crate::nvim::buffer::Buffer;
use crate::text::Line;

/// Where inline virtual text is placed.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum VirtTextPos {
    /// After the end of the line (the default).
    #[default]
    Eol,
    /// Over the text at the mark position.
    Overlay,
    /// Right-aligned in the window.
    RightAlign,
    /// Inline at the mark position, shifting the text.
    Inline,
}

impl VirtTextPos {
    fn as_str(self) -> &'static str {
        match self {
            VirtTextPos::Eol => "eol",
            VirtTextPos::Overlay => "overlay",
            VirtTextPos::RightAlign => "right_align",
            VirtTextPos::Inline => "inline",
        }
    }
}

/// Options for [`Namespace::set_extmark`]. All fields optional; a [`Line`]
/// maps 1:1 onto a virt-text chunk list (`[[text, hl], ...]`).
#[derive(Clone, Debug, Default)]
pub struct ExtmarkOpts {
    /// Reuse/replace an existing mark id.
    pub id: Option<i64>,
    pub end_row: Option<i64>,
    pub end_col: Option<i64>,
    /// Highlight the marked range with this group.
    pub hl_group: Option<String>,
    /// Continue the highlight to the end of the line.
    pub hl_eol: bool,
    /// Highlight the whole line with this group.
    pub line_hl_group: Option<String>,
    /// Virtual text chunks shown at [`ExtmarkOpts::virt_text_pos`].
    pub virt_text: Option<Line>,
    pub virt_text_pos: Option<VirtTextPos>,
    /// Whole virtual lines below (or above) the marked row.
    pub virt_lines: Option<Vec<Line>>,
    pub virt_lines_above: bool,
    /// Sign-column text (1–2 cells) and its highlight.
    pub sign_text: Option<String>,
    pub sign_hl_group: Option<String>,
    /// Conceal the marked range, showing this (possibly empty) char instead.
    pub conceal: Option<String>,
    pub priority: Option<i64>,
}

/// Convert a [`Line`] to a virt-text chunk array: `{{text, hl?}, ...}`.
fn line_to_chunks(line: &Line) -> Result<Table> {
    let chunks = lua::table()?;
    for (i, span) in line.spans.iter().enumerate() {
        let chunk = lua::table()?;
        chunk.set(1, span.text.clone())?;
        if let Some(hl) = &span.hl {
            chunk.set(2, hl.clone())?;
        }
        chunks.set(i + 1, chunk)?;
    }
    Ok(chunks)
}

impl ExtmarkOpts {
    fn to_table(&self) -> Result<Table> {
        let t = lua::table()?;
        if let Some(id) = self.id {
            t.set("id", id)?;
        }
        if let Some(end_row) = self.end_row {
            t.set("end_row", end_row)?;
        }
        if let Some(end_col) = self.end_col {
            t.set("end_col", end_col)?;
        }
        if let Some(group) = &self.hl_group {
            t.set("hl_group", group.clone())?;
        }
        if self.hl_eol {
            t.set("hl_eol", true)?;
        }
        if let Some(group) = &self.line_hl_group {
            t.set("line_hl_group", group.clone())?;
        }
        if let Some(line) = &self.virt_text {
            t.set("virt_text", line_to_chunks(line)?)?;
            t.set("virt_text_pos", self.virt_text_pos.unwrap_or_default().as_str())?;
        }
        if let Some(lines) = &self.virt_lines {
            let arr = lua::table()?;
            for (i, line) in lines.iter().enumerate() {
                arr.set(i + 1, line_to_chunks(line)?)?;
            }
            t.set("virt_lines", arr)?;
            if self.virt_lines_above {
                t.set("virt_lines_above", true)?;
            }
        }
        if let Some(sign) = &self.sign_text {
            t.set("sign_text", sign.clone())?;
            if let Some(group) = &self.sign_hl_group {
                t.set("sign_hl_group", group.clone())?;
            }
        }
        if let Some(conceal) = &self.conceal {
            t.set("conceal", conceal.clone())?;
        }
        if let Some(priority) = self.priority {
            t.set("priority", priority)?;
        }
        Ok(t)
    }
}

/// A placed extmark, as returned by [`Namespace::get_extmarks`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExtmarkInfo {
    pub id: i64,
    pub row: i64,
    pub col: i64,
}

/// A highlight namespace — an isolated layer of extmarks we can clear wholesale.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Namespace(pub i64);

impl Namespace {
    /// Create (or fetch) a named namespace.
    pub fn create(name: &str) -> Result<Self> {
        let id: i64 = lua::call_api("nvim_create_namespace", (name.to_string(),))?;
        Ok(Namespace(id))
    }

    pub fn id(self) -> i64 {
        self.0
    }

    /// Remove all extmarks this namespace owns in `buf`.
    pub fn clear(self, buf: Buffer) -> Result<()> {
        lua::call_api::<()>("nvim_buf_clear_namespace", (buf.id(), self.0, 0i64, -1i64))
    }

    /// Place an extmark at `(row, col)` (0-based) with the given decorations.
    /// Returns the mark id.
    pub fn set_extmark(self, buf: Buffer, row: i64, col: i64, opts: &ExtmarkOpts) -> Result<i64> {
        lua::call_api("nvim_buf_set_extmark", (buf.id(), self.0, row, col, opts.to_table()?))
    }

    /// Delete one extmark by id.
    pub fn del_extmark(self, buf: Buffer, id: i64) -> Result<()> {
        lua::call_api::<bool>("nvim_buf_del_extmark", (buf.id(), self.0, id))?;
        Ok(())
    }

    /// List this namespace's extmarks in `buf` between two `(row, col)`
    /// positions (inclusive; use `(0, 0)` and `(-1, -1)` for all).
    pub fn get_extmarks(self, buf: Buffer, start: (i64, i64), end: (i64, i64)) -> Result<Vec<ExtmarkInfo>> {
        let s = lua::table()?;
        s.set(1, start.0)?;
        s.set(2, start.1)?;
        let e = lua::table()?;
        e.set(1, end.0)?;
        e.set(2, end.1)?;
        let marks: Vec<Vec<i64>> =
            lua::call_api("nvim_buf_get_extmarks", (buf.id(), self.0, s, e, lua::table()?))?;
        Ok(marks
            .into_iter()
            .filter(|m| m.len() >= 3)
            .map(|m| ExtmarkInfo { id: m[0], row: m[1], col: m[2] })
            .collect())
    }

    /// Highlight a byte range on `row` (0-based) with `group`.
    pub fn highlight(self, buf: Buffer, row: i64, start_col: i64, end_col: i64, group: &str) -> Result<()> {
        let opts = ExtmarkOpts {
            end_col: Some(end_col),
            hl_group: Some(group.to_string()),
            ..Default::default()
        };
        self.set_extmark(buf, row, start_col, &opts)?;
        Ok(())
    }
}
