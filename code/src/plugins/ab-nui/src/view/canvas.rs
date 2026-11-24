//! A cell grid that widgets paint onto, which compiles down to the rows the
//! renderer diffs and sends to Neovim. This is the bridge between the
//! declarative widget tree and the rendering core.
//!
//! The grid is display-cell-correct: a double-width glyph (CJK, emoji) claims
//! two cells — a leader holding the char and a continuation cell — so column
//! math stays aligned with what Neovim actually renders. Overwriting either
//! half of a wide char blanks the orphaned other half.
//!
//! Highlight groups are interned per canvas: a cell stores a small index
//! rather than a reference-counted string, which keeps cells `Copy` and makes
//! "same highlight as the previous cell" an integer comparison.

use std::rc::Rc;

use crate::nvim::render::{Mark, Row};
use crate::text::{Line, Span, char_width};
use crate::view::widget::Area;

/// Index into [`Canvas::styles`], offset by one; `0` means "no highlight".
type StyleId = u16;

/// One terminal cell: a character and an (interned) highlight group.
#[derive(Clone, Copy)]
struct Cell {
    ch: char,
    style: StyleId,
    /// True if this cell is the trailing half of a double-width glyph; the
    /// leader cell to its left holds the char. Skipped when emitting rows.
    cont: bool,
}

const BLANK: Cell = Cell { ch: ' ', style: 0, cont: false };

/// A fixed-size grid of cells (row-major).
pub struct Canvas {
    pub w: u16,
    pub h: u16,
    cells: Vec<Cell>,
    /// The highlight groups used so far; a cell's `style` is its index + 1.
    styles: Vec<Rc<str>>,
}

impl Canvas {
    pub fn new(w: u16, h: u16) -> Self {
        let len = (w as usize) * (h as usize);
        Self { w, h, cells: vec![BLANK; len], styles: Vec::new() }
    }

    /// Clear the canvas and give it a new size, reusing its allocations.
    pub fn reset(&mut self, w: u16, h: u16) {
        self.w = w;
        self.h = h;
        self.cells.clear();
        self.cells.resize((w as usize) * (h as usize), BLANK);
        self.styles.clear();
    }

    fn index(&self, x: u16, y: u16) -> Option<usize> {
        if x < self.w && y < self.h {
            Some(y as usize * self.w as usize + x as usize)
        } else {
            None
        }
    }

    /// Intern a highlight group by name. Widgets tend to paint the same group
    /// cell after cell, so the most recently added styles are checked first.
    fn intern_name(&mut self, name: &str) -> StyleId {
        if let Some(pos) = self.styles.iter().rposition(|s| **s == *name) {
            return pos as StyleId + 1;
        }
        self.push_style(Rc::from(name))
    }

    /// Intern a highlight group, reusing the caller's `Rc` when it is new.
    fn intern(&mut self, hl: Option<&Rc<str>>) -> StyleId {
        let Some(hl) = hl else { return 0 };
        if let Some(pos) = self.styles.iter().rposition(|s| Rc::ptr_eq(s, hl) || **s == **hl) {
            return pos as StyleId + 1;
        }
        self.push_style(hl.clone())
    }

    fn push_style(&mut self, hl: Rc<str>) -> StyleId {
        if self.styles.len() >= StyleId::MAX as usize - 1 {
            return 0; // out of ids: paint unstyled rather than wrap around
        }
        self.styles.push(hl);
        self.styles.len() as StyleId
    }

    fn style_name(&self, style: StyleId) -> Option<&Rc<str>> {
        (style != 0).then(|| &self.styles[style as usize - 1])
    }

    /// Blank a cell, repairing a wide char it may have been part of: if it was
    /// a leader, its continuation is blanked too; if it was a continuation, its
    /// leader is blanked. Called before every overwrite.
    fn clear_wide(&mut self, x: u16, y: u16) {
        let Some(i) = self.index(x, y) else { return };
        if self.cells[i].cont {
            if x > 0
                && let Some(l) = self.index(x - 1, y)
            {
                self.cells[l] = BLANK;
            }
            self.cells[i] = BLANK;
        } else if char_width(self.cells[i].ch) == 2 {
            if let Some(r) = self.index(x + 1, y)
                && self.cells[r].cont
            {
                self.cells[r] = BLANK;
            }
            self.cells[i] = BLANK;
        }
    }

    /// Paint one glyph with an already-interned style.
    fn set_styled(&mut self, x: u16, y: u16, ch: char, style: StyleId) -> u16 {
        let width = char_width(ch);
        if width == 0 {
            return 0;
        }
        let Some(i) = self.index(x, y) else { return 0 };

        if width == 2 {
            if x + 1 >= self.w {
                // Wide char clipped at the edge: paint a blank in the last column.
                self.clear_wide(x, y);
                self.cells[i] = Cell { ch: ' ', style, cont: false };
                return 1;
            }
            self.clear_wide(x, y);
            self.clear_wide(x + 1, y);
            self.cells[i] = Cell { ch, style, cont: false };
            self.cells[i + 1] = Cell { ch: ' ', style, cont: true };
            2
        } else {
            self.clear_wide(x, y);
            self.cells[i] = Cell { ch, style, cont: false };
            1
        }
    }

    /// Paint one glyph. A double-width glyph claims two cells; if it would be
    /// clipped by the right edge it is dropped (a blank is painted instead).
    /// Zero-width chars are ignored. Returns the display cells consumed.
    pub fn set(&mut self, x: u16, y: u16, ch: char, hl: Option<Rc<str>>) -> u16 {
        let style = self.intern(hl.as_ref());
        self.set_styled(x, y, ch, style)
    }

    fn put_str_styled(&mut self, x: u16, y: u16, s: &str, style: StyleId) -> u16 {
        if y >= self.h {
            return 0;
        }
        let mut col = x;
        for ch in s.chars() {
            if col >= self.w {
                break;
            }
            col += self.set_styled(col, y, ch, style);
        }
        col - x
    }

    /// Paint a string starting at `(x, y)`, clipped to the canvas width.
    /// Returns the display cells consumed.
    pub fn put_str(&mut self, x: u16, y: u16, s: &str, hl: Option<Rc<str>>) -> u16 {
        let style = self.intern(hl.as_ref());
        self.put_str_styled(x, y, s, style)
    }

    /// Paint a styled [`Line`] (each span with its own highlight). Returns the
    /// display cells consumed.
    pub fn put_line(&mut self, x: u16, y: u16, line: &Line) -> u16 {
        let mut col = x;
        for span in &line.spans {
            let style = span.hl.as_deref().map_or(0, |name| self.intern_name(name));
            col += self.put_str_styled(col, y, &span.text, style);
            if col >= self.w {
                break;
            }
        }
        col - x
    }

    /// Fill a rectangular area with a character.
    pub fn fill(&mut self, area: Area, ch: char, hl: Option<Rc<str>>) {
        let style = self.intern(hl.as_ref());
        for y in area.y..area.y.saturating_add(area.h) {
            let mut x = area.x;
            while x < area.x.saturating_add(area.w) {
                let consumed = self.set_styled(x, y, ch, style);
                x += consumed.max(1);
            }
        }
    }

    /// Copy a rectangle from another canvas onto this one at `(dst_x, dst_y)`.
    /// Used by scrolling viewports to blit their visible window.
    pub fn blit(&mut self, src: &Canvas, src_area: Area, dst_x: u16, dst_y: u16) {
        // Source style id → our style id, filled in on first use.
        let mut remap: Vec<Option<StyleId>> = vec![None; src.styles.len() + 1];
        remap[0] = Some(0);
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
                let cell = src.cells[sy as usize * src.w as usize + sx as usize];
                let style = match remap[cell.style as usize] {
                    Some(style) => style,
                    None => {
                        let style = self.intern(src.style_name(cell.style));
                        remap[cell.style as usize] = Some(style);
                        style
                    }
                };
                if cell.cont {
                    // Continuation of a leader outside the source rect: blank.
                    if dx == 0 {
                        self.set_styled(tx, ty, ' ', style);
                    }
                    dx += 1;
                    continue;
                }
                let consumed = self.set_styled(tx, ty, cell.ch, style);
                dx += consumed.max(1);
            }
        }
    }

    /// The cells of row `y` that carry a glyph (continuation halves skipped).
    fn glyphs(&self, y: u16) -> impl Iterator<Item = &Cell> {
        let start = y as usize * self.w as usize;
        self.cells[start..start + self.w as usize].iter().filter(|c| !c.cont)
    }

    /// Convert the grid into one [`Row`] per line for the renderer: the row's
    /// text, plus a [`Mark`] for every run of cells sharing a highlight.
    pub fn to_rows(&self) -> Vec<Row> {
        let mut rows = Vec::with_capacity(self.h as usize);
        for y in 0..self.h {
            let mut text = String::with_capacity(self.w as usize);
            let mut marks: Vec<Mark> = Vec::new();
            // The open run: (style, start byte).
            let mut run: Option<(StyleId, u32)> = None;
            for cell in self.glyphs(y) {
                if run.map(|(style, _)| style) != Some(cell.style) {
                    if let Some((style, start)) = run
                        && let Some(hl) = self.style_name(style)
                    {
                        marks.push(Mark { start, end: text.len() as u32, hl: hl.clone() });
                    }
                    run = Some((cell.style, text.len() as u32));
                }
                text.push(cell.ch);
            }
            if let Some((style, start)) = run
                && let Some(hl) = self.style_name(style)
            {
                marks.push(Mark { start, end: text.len() as u32, hl: hl.clone() });
            }
            rows.push(Row { text, marks });
        }
        rows
    }

    /// Convert the grid into one styled [`Line`] per row, merging adjacent
    /// cells that share a highlight into a single [`Span`]. Continuation cells
    /// are skipped (their leader already spans two display columns).
    pub fn to_lines(&self) -> Vec<Line> {
        let mut lines = Vec::with_capacity(self.h as usize);
        for y in 0..self.h {
            let mut spans: Vec<Span> = Vec::new();
            let mut run: Option<(StyleId, String)> = None;
            for cell in self.glyphs(y) {
                match &mut run {
                    Some((style, text)) if *style == cell.style => text.push(cell.ch),
                    _ => {
                        if let Some((style, text)) = run.take() {
                            spans.push(self.make_span(style, text));
                        }
                        run = Some((cell.style, cell.ch.to_string()));
                    }
                }
            }
            if let Some((style, text)) = run {
                spans.push(self.make_span(style, text));
            }
            lines.push(Line { spans });
        }
        lines
    }

    fn make_span(&self, style: StyleId, text: String) -> Span {
        Span { text, hl: self.style_name(style).map(|hl| hl.to_string()) }
    }
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
    fn to_rows_emits_byte_ranged_marks() {
        let mut c = Canvas::new(8, 2);
        c.put_str(0, 0, "ab", Some(Rc::from("Title")));
        c.put_str(2, 0, "日", Some(Rc::from("Error")));
        // A different `Rc` naming the same group continues the same run.
        c.put_str(4, 0, "x", Some(Rc::from("Error")));
        let rows = c.to_rows();
        assert_eq!(rows[0].text, "ab日x   ");
        let marks: Vec<(u32, u32, &str)> = rows[0].marks.iter().map(|m| (m.start, m.end, &*m.hl)).collect();
        assert_eq!(marks, vec![(0, 2, "Title"), (2, 6, "Error")]);
        assert_eq!(rows[1].text, "        ");
        assert!(rows[1].marks.is_empty());
    }

    #[test]
    fn reset_clears_cells_and_styles() {
        let mut c = Canvas::new(3, 1);
        c.put_str(0, 0, "abc", Some(Rc::from("Title")));
        c.reset(2, 2);
        let rows = c.to_rows();
        assert_eq!(rows.len(), 2);
        assert!(rows.iter().all(|r| r.text == "  " && r.marks.is_empty()));
    }

    #[test]
    fn blit_carries_highlights_across_canvases() {
        let mut src = Canvas::new(4, 1);
        src.put_str(0, 0, "ab", Some(Rc::from("Src")));
        let mut dst = Canvas::new(4, 1);
        dst.put_str(3, 0, "z", Some(Rc::from("Dst")));
        dst.blit(&src, Area { x: 0, y: 0, w: 2, h: 1 }, 0, 0);
        let marks: Vec<(u32, u32, String)> =
            dst.to_rows()[0].marks.iter().map(|m| (m.start, m.end, m.hl.to_string())).collect();
        assert_eq!(marks, vec![(0, 2, "Src".to_string()), (3, 4, "Dst".to_string())]);
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
