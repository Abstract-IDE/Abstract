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
    /// A fraction (0.0–1.0) of the space left over once the window is placed:
    /// `0.0` hugs the top/left edge, `1.0` the bottom/right, `0.5` centers.
    /// Unlike [`Position::At`], this stays put relative to the editor when it
    /// is resized.
    Ratio { row: f64, col: f64 },
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
    /// Resolve a [`Size`] + [`Position`] for a window without a border; see
    /// [`Rect::resolve_framed`].
    pub fn resolve(size: Size, position: Position) -> Result<Self> {
        Self::resolve_framed(size, position, (0, 0))
    }

    /// Resolve a [`Size`] + [`Position`] for a float whose border adds
    /// `frame = (columns, rows)` cells around its content, against the
    /// [`usable_area`] of the editor.
    ///
    /// `size` is the CONTENT size: the returned `width`/`height` are what the
    /// window's text area gets (what `nvim_open_win` takes), clamped so that
    /// content + border fits in the usable area. `row`/`col` are the top-left
    /// of the whole box, border included (what `nvim_open_win` takes too);
    /// centering and [`Position::Ratio`] place the whole box.
    pub fn resolve_framed(size: Size, position: Position, frame: (u32, u32)) -> Result<Self> {
        Ok(Self::layout_in(usable_area()?, size, position, frame))
    }

    /// [`Rect::resolve_framed`] against an explicit `area` (no Neovim calls).
    pub fn layout_in(area: Rect, size: Size, position: Position, frame: (u32, u32)) -> Self {
        let (frame_w, frame_h) = frame;
        let width = size.width.resolve(area.width).min(area.width.saturating_sub(frame_w).max(1));
        let height = size.height.resolve(area.height).min(area.height.saturating_sub(frame_h).max(1));
        let (outer_w, outer_h) = (width + frame_w, height + frame_h);
        let free = |total: u32, used: u32, r: f64| (total.saturating_sub(used) as f64 * r.clamp(0.0, 1.0)).floor();
        let (row, col) = match position {
            Position::Center => (area.row + free(area.height, outer_h, 0.5), area.col + free(area.width, outer_w, 0.5)),
            Position::At { row, col } => (row, col),
            Position::Ratio { row, col } => {
                (area.row + free(area.height, outer_h, row), area.col + free(area.width, outer_w, col))
            },
        };
        Rect { row, col, width, height }
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

/// The part of the editor floats are laid out in, in editor coordinates:
/// every row except the command line (`cmdheight`) and, when it is shown, the
/// tabline. Floats may cover statuslines, so those are not subtracted.
pub fn usable_area() -> Result<Rect> {
    let (cols, lines) = editor_size()?;
    let cmdheight: i64 = lua::get_option("cmdheight")?;
    let tabline = match lua::get_option::<i64>("showtabline")? {
        0 => false,
        1 => lua::call_fn::<i64>("tabpagenr", ("$",))? > 1,
        _ => true,
    };
    let top = u32::from(tabline);
    let rows = lines.saturating_sub(cmdheight.max(0) as u32 + top).max(1);
    Ok(Rect { row: top as f64, col: 0.0, width: cols, height: rows })
}

#[cfg(test)]
mod tests {
    use super::*;

    const AREA: Rect = Rect { row: 0.0, col: 0.0, width: 80, height: 23 };

    #[test]
    fn full_size_content_leaves_room_for_the_border() {
        let r = Rect::layout_in(AREA, Size::ratio(1.0, 1.0), Position::Center, (2, 2));
        assert_eq!((r.width, r.height, r.row, r.col), (78, 21, 0.0, 0.0));
        let r = Rect::layout_in(AREA, Size::cells(500, 500), Position::Center, (0, 0));
        assert_eq!((r.width, r.height), (80, 23));
    }

    #[test]
    fn center_places_the_whole_box() {
        let r = Rect::layout_in(AREA, Size::cells(20, 6), Position::Center, (2, 2));
        assert_eq!((r.row, r.col), (7.0, 29.0));
        let top = Rect { row: 1.0, ..AREA };
        let r = Rect::layout_in(top, Size::cells(20, 6), Position::Center, (2, 2));
        assert_eq!(r.row, 8.0);
    }

    #[test]
    fn ratio_position_uses_the_free_space_around_the_box() {
        let r = Rect::layout_in(AREA, Size::cells(20, 5), Position::Ratio { row: 1.0, col: 1.0 }, (2, 2));
        assert_eq!((r.row, r.col), (16.0, 58.0));
    }
}
