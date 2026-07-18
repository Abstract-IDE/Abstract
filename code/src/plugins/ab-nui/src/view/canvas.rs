//! A cell grid that widgets paint onto, which compiles down to buffer
//! [`Line`]s + highlights. This is the bridge between the declarative widget
//! tree and the existing rendering core.
//!
//! The grid is display-cell-correct: a double-width glyph (CJK, emoji) claims
//! two cells — a leader holding the char and a continuation cell — so column
//! math stays aligned with what Neovim actually renders. Overwriting either
//! half of a wide char blanks the orphaned other half.

use std::rc::Rc;

use crate::text::{Line, Span, char_width};
use crate::view::widget::Area;

/// One terminal cell: a character and an optional highlight group.
#[derive(Clone)]
pub struct Cell {
    pub ch: char,
    pub hl: Option<Rc<str>>,
    /// True if this cell is the trailing half of a double-width glyph; the
    /// leader cell to its left holds the char. Skipped by [`Canvas::to_lines`].
    cont: bool,
}

impl Default for Cell {
    fn default() -> Self {
        Self { ch: ' ', hl: None, cont: false }
    }
}

/// A fixed-size grid of cells (row-major).
pub struct Canvas {
    pub w: u16,
    pub h: u16,
    cells: Vec<Cell>,
}

impl Canvas {
    pub fn new(w: u16, h: u16) -> Self {
        let len = (w as usize) * (h as usize);
        Self { w, h, cells: vec![Cell::default(); len] }
    }

    fn index(&self, x: u16, y: u16) -> Option<usize> {
        if x < self.w && y < self.h {
            Some(y as usize * self.w as usize + x as usize)
        } else {
            None
        }
    }

    /// Blank a cell, repairing a wide char it may have been part of: if it was
    /// a leader, its continuation is blanked too; if it was a continuation, its
    /// leader is blanked. Called before every overwrite.
    fn clear_wide(&mut self, x: u16, y: u16) {
        let Some(i) = self.index(x, y) else { return };
        if self.cells[i].cont {
            if x > 0
                && let Some(l) = self.index(x - 1, y) {
                    self.cells[l] = Cell::default();
                }
            self.cells[i] = Cell::default();
        } else if char_width(self.cells[i].ch) == 2 {
            if let Some(r) = self.index(x + 1, y)
                && self.cells[r].cont {
                    self.cells[r] = Cell::default();
                }
            self.cells[i] = Cell::default();
        }
    }

    /// Paint one glyph. A double-width glyph claims two cells; if it would be
    /// clipped by the right edge it is dropped (a blank is painted instead).
    /// Zero-width chars are ignored. Returns the display cells consumed.
    pub fn set(&mut self, x: u16, y: u16, ch: char, hl: Option<Rc<str>>) -> u16 {
        let width = char_width(ch);
        if width == 0 {
            return 0;
        }
        let Some(i) = self.index(x, y) else { return 0 };

        if width == 2 {
            if x + 1 >= self.w {
                // Wide char clipped at the edge: paint a blank in the last column.
                self.clear_wide(x, y);
                self.cells[i] = Cell { ch: ' ', hl, cont: false };
                return 1;
            }
            self.clear_wide(x, y);
            self.clear_wide(x + 1, y);
            self.cells[i] = Cell { ch, hl: hl.clone(), cont: false };
            let j = self.index(x + 1, y).expect("checked above");
            self.cells[j] = Cell { ch: ' ', hl, cont: true };
            2
        } else {
            self.clear_wide(x, y);
            self.cells[i] = Cell { ch, hl, cont: false };
            1
        }
    }

    /// Paint a string starting at `(x, y)`, clipped to the canvas width.
    /// Returns the display cells consumed.
    pub fn put_str(&mut self, x: u16, y: u16, s: &str, hl: Option<Rc<str>>) -> u16 {
        let mut col = x;
        for ch in s.chars() {
            if col >= self.w || y >= self.h {
                break;
            }
            col += self.set(col, y, ch, hl.clone());
        }
        col - x
    }

    /// Paint a styled [`Line`] (each span with its own highlight). Returns the
    /// display cells consumed.
    pub fn put_line(&mut self, x: u16, y: u16, line: &Line) -> u16 {
        let mut col = x;
        for span in &line.spans {
            let hl: Option<Rc<str>> = span.hl.as_deref().map(Rc::from);
            col += self.put_str(col, y, &span.text, hl);
            if col >= self.w {
                break;
            }
        }
        col - x
    }

    /// Fill a rectangular area with a character.
    pub fn fill(&mut self, area: Area, ch: char, hl: Option<Rc<str>>) {
        for y in area.y..area.y.saturating_add(area.h) {
            let mut x = area.x;
            while x < area.x.saturating_add(area.w) {
                let consumed = self.set(x, y, ch, hl.clone());
                x += consumed.max(1);
            }
        }
    }

    /// Copy a rectangle from another canvas onto this one at `(dst_x, dst_y)`.
    /// Used by scrolling viewports to blit their visible window.
    pub fn blit(&mut self, src: &Canvas, src_area: Area, dst_x: u16, dst_y: u16) {
        for dy in 0..src_area.h {
            let sy = src_area.y + dy;
            let ty = dst_y + dy;
            if sy >= src.h || ty >= self.h {
                break;
            }
            let mut dx = 0;
            while dx < src_area.w {
                let sx = src_area.x + dx;
                let tx = dst_x + dx;
                if sx >= src.w || tx >= self.w {
                    break;
                }
                let cell = &src.cells[sy as usize * src.w as usize + sx as usize];
                if cell.cont {
                    // Continuation of a leader outside the source rect: blank.
                    if dx == 0 {
                        self.set(tx, ty, ' ', cell.hl.clone());
                    }
                    dx += 1;
                    continue;
                }
                let consumed = self.set(tx, ty, cell.ch, cell.hl.clone());
                dx += consumed.max(1);
            }
        }
    }

    /// Convert the grid into one styled [`Line`] per row, merging adjacent
    /// cells that share a highlight into a single [`Span`]. Continuation cells
    /// are skipped (their leader already spans two display columns).
    pub fn to_lines(&self) -> Vec<Line> {
        let mut lines = Vec::with_capacity(self.h as usize);
        for y in 0..self.h {
            let mut spans: Vec<Span> = Vec::new();
            let mut run = String::new();
            let mut run_hl: Option<Rc<str>> = None;
            let mut started = false;

            for x in 0..self.w {
                let cell = &self.cells[y as usize * self.w as usize + x as usize];
                if cell.cont {
                    continue;
                }
                if !started {
                    run.push(cell.ch);
                    run_hl = cell.hl.clone();
                    started = true;
                } else if same_hl(&run_hl, &cell.hl) {
                    run.push(cell.ch);
                } else {
                    spans.push(make_span(&run, &run_hl));
                    run.clear();
                    run.push(cell.ch);
                    run_hl = cell.hl.clone();
                }
            }
            if started {
                spans.push(make_span(&run, &run_hl));
            }
            lines.push(Line { spans });
        }
        lines
    }
}

fn same_hl(a: &Option<Rc<str>>, b: &Option<Rc<str>>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(x), Some(y)) => x.as_ref() == y.as_ref(),
        _ => false,
    }
}

fn make_span(text: &str, hl: &Option<Rc<str>>) -> Span {
    Span { text: text.to_string(), hl: hl.as_ref().map(|r| r.to_string()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wide_char_claims_two_cells() {
        let mut c = Canvas::new(6, 1);
        assert_eq!(c.put_str(0, 0, "日本", None), 4);
        let lines = c.to_lines();
        assert_eq!(lines[0].text(), "日本  ");
    }

    #[test]
    fn overwriting_wide_half_blanks_orphan() {
        let mut c = Canvas::new(4, 1);
        c.put_str(0, 0, "日", None);
        // Overwrite the continuation cell: the leader must be blanked.
        c.set(1, 0, 'x', None);
        assert_eq!(c.to_lines()[0].text(), " x  ");
    }

    #[test]
    fn wide_char_clipped_at_edge_becomes_blank() {
        let mut c = Canvas::new(2, 1);
        assert_eq!(c.put_str(0, 0, "a日", None), 2);
        assert_eq!(c.to_lines()[0].text(), "a ");
    }

    #[test]
    fn to_lines_merges_hl_runs() {
        let mut c = Canvas::new(4, 1);
        let hl: Rc<str> = Rc::from("Title");
        c.put_str(0, 0, "ab", Some(hl.clone()));
        c.put_str(2, 0, "cd", None);
        let line = &c.to_lines()[0];
        assert_eq!(line.spans.len(), 2);
        assert_eq!(line.spans[0].text, "ab");
        assert_eq!(line.spans[0].hl.as_deref(), Some("Title"));
        assert_eq!(line.spans[1].text, "cd");
    }

    #[test]
    fn blit_copies_rect() {
        let mut src = Canvas::new(4, 2);
        src.put_str(0, 0, "abcd", None);
        src.put_str(0, 1, "efgh", None);
        let mut dst = Canvas::new(4, 1);
        dst.blit(&src, Area { x: 1, y: 1, w: 2, h: 1 }, 0, 0);
        assert_eq!(dst.to_lines()[0].text(), "fg  ");
    }
}
