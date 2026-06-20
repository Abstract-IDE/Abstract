//! Layout math: sizes, rectangles, and how to place a floating window.

use crate::error::Result;
use crate::lua;

/// A length, either an absolute number of cells or a fraction of the parent.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Dim {
    /// Exactly this many cells.
    Cells(u32),
    /// A fraction (0.0–1.0) of the available space.
    Ratio(f64),
}

impl Dim {
    /// Resolve against a total length, clamped to `[1, total]`.
    pub fn resolve(self, total: u32) -> u32 {
        let raw = match self {
            Dim::Cells(n) => n,
            Dim::Ratio(r) => (total as f64 * r).round() as u32,
        };
        raw.clamp(1, total.max(1))
    }
}

/// A desired width × height for a widget.
#[derive(Clone, Copy, Debug)]
pub struct Size {
    pub width: Dim,
    pub height: Dim,
}

impl Size {
    pub fn cells(width: u32, height: u32) -> Self {
        Self { width: Dim::Cells(width), height: Dim::Cells(height) }
    }

    pub fn ratio(width: f64, height: f64) -> Self {
        Self { width: Dim::Ratio(width), height: Dim::Ratio(height) }
    }
}

/// Where to anchor a floating widget.
#[derive(Clone, Copy, Debug)]
pub enum Position {
    /// Centered in the editor.
    Center,
    /// Absolute editor coordinates (top-left of the window).
    At { row: f64, col: f64 },
}

/// A resolved rectangle in editor cells.
#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub row: f64,
    pub col: f64,
    pub width: u32,
    pub height: u32,
}

impl Rect {
    /// Resolve a [`Size`] + [`Position`] against the current editor dimensions.
    pub fn resolve(size: Size, position: Position) -> Result<Self> {
        let (cols, lines) = editor_size()?;
        let width = size.width.resolve(cols);
        let height = size.height.resolve(lines);
        let (row, col) = match position {
            Position::Center => {
                let row = (lines.saturating_sub(height)) as f64 / 2.0;
                let col = (cols.saturating_sub(width)) as f64 / 2.0;
                (row, col)
            }
            Position::At { row, col } => (row, col),
        };
        Ok(Rect { row, col, width, height })
    }

    /// Split this rect into two columns: `(left, right)`, separated by `gap`
    /// cells. `left_width` is taken from the left.
    pub fn split_h(self, left_width: u32, gap: u32) -> (Rect, Rect) {
        let left_width = left_width.min(self.width);
        let right_width = self.width.saturating_sub(left_width + gap);
        let left = Rect { width: left_width, ..self };
        let right = Rect { col: self.col + (left_width + gap) as f64, width: right_width, ..self };
        (left, right)
    }

    /// Split this rect into two rows: `(top, bottom)`, separated by `gap` cells.
    pub fn split_v(self, top_height: u32, gap: u32) -> (Rect, Rect) {
        let top_height = top_height.min(self.height);
        let bottom_height = self.height.saturating_sub(top_height + gap);
        let top = Rect { height: top_height, ..self };
        let bottom = Rect { row: self.row + (top_height + gap) as f64, height: bottom_height, ..self };
        (top, bottom)
    }
}

/// Current editor size as `(columns, lines)`.
pub fn editor_size() -> Result<(u32, u32)> {
    let cols: i64 = lua::get_option("columns")?;
    let lines: i64 = lua::get_option("lines")?;
    Ok((cols.max(1) as u32, lines.max(1) as u32))
}
