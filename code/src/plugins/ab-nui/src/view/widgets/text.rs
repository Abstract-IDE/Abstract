//! `Text` — styled, optionally wrapping text. The leaf of most trees.
//!
//! A `Text` holds one styled [`Line`] (any number of highlighted spans). With
//! [`Text::wrap`] it reflows across rows — this is the library's paragraph
//! widget; there is no separate one.

use crate::text::{Line, Span, Wrap, truncate_line, wrap_line};
use crate::view::canvas::Canvas;
use crate::view::widget::{Area, Constraints, Cx, Size, Widget};

/// Horizontal alignment of each rendered row within the widget's area.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TextAlign {
    #[default]
    Left,
    Center,
    Right,
}

/// What happens to a row that exceeds the width when not wrapping.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Overflow {
    /// Hard clip at the edge (the canvas clips anyway).
    #[default]
    Clip,
    /// Truncate with a trailing `…`.
    Ellipsis,
}

pub struct Text {
    line: Line,
    wrap: Wrap,
    align: TextAlign,
    overflow: Overflow,
    max_lines: Option<u16>,
}

impl Text {
    pub fn new(text: impl Into<String>) -> Self {
        Self::from_line(Line::raw(text))
    }

    /// Build from a pre-styled [`Line`] (mixed highlights within one text).
    pub fn from_line(line: Line) -> Self {
        Self { line, wrap: Wrap::None, align: TextAlign::Left, overflow: Overflow::Clip, max_lines: None }
    }

    /// Append a styled span (builder style).
    pub fn span(mut self, text: impl Into<String>, group: &str) -> Self {
        self.line.spans.push(Span::hl(text, group));
        self
    }

    /// Append an unstyled span (builder style).
    pub fn raw(mut self, text: impl Into<String>) -> Self {
        self.line.spans.push(Span::raw(text));
        self
    }

    /// Paint the whole text with the given highlight group (e.g. `"Title"`).
    /// Overrides any per-span groups set earlier.
    pub fn fg(mut self, group: &str) -> Self {
        for span in &mut self.line.spans {
            span.hl = Some(group.to_string());
        }
        self
    }

    /// Reflow across rows when wider than the area.
    pub fn wrap(mut self, wrap: Wrap) -> Self {
        self.wrap = wrap;
        self
    }

    /// Horizontal alignment of each row.
    pub fn align(mut self, align: TextAlign) -> Self {
        self.align = align;
        self
    }

    /// Truncate an overflowing row with `…` instead of hard clipping.
    pub fn ellipsis(mut self) -> Self {
        self.overflow = Overflow::Ellipsis;
        self
    }

    /// Cap the number of rendered rows when wrapping.
    pub fn max_lines(mut self, n: u16) -> Self {
        self.max_lines = Some(n);
        self
    }

    /// The rows this text resolves to at the given width.
    fn rows(&self, width: u16) -> Vec<Line> {
        let mut rows = wrap_line(&self.line, width, self.wrap);
        if let Some(max) = self.max_lines {
            rows.truncate(max.max(1) as usize);
        }
        rows
    }
}

impl Widget for Text {
    fn measure(&self, c: Constraints) -> Size {
        if self.wrap == Wrap::None {
            let w = self.line.width().min(c.max_w);
            return Size::new(w.max(c.min_w), 1.clamp(c.min_h, c.max_h.max(1)));
        }
        let rows = self.rows(c.max_w);
        let w = rows.iter().map(Line::width).max().unwrap_or(0).min(c.max_w);
        let h = (rows.len() as u16).min(c.max_h);
        c.clamp(Size::new(w, h))
    }

    fn paint(&self, _cx: &mut Cx, area: Area, canvas: &mut Canvas) {
        let rows = self.rows(area.w);
        for (i, row) in rows.iter().enumerate() {
            if i as u16 >= area.h {
                break;
            }
            let row = match self.overflow {
                Overflow::Ellipsis if row.width() > area.w => truncate_line(row, area.w),
                _ => row.clone(),
            };
            let rw = row.width().min(area.w);
            let dx = match self.align {
                TextAlign::Left => 0,
                TextAlign::Center => (area.w - rw) / 2,
                TextAlign::Right => area.w - rw,
            };
            canvas.put_line(area.x + dx, area.y + i as u16, &row);
        }
    }
}

// Keep the widget usable with plain strings in layout children.
impl From<&str> for Text {
    fn from(s: &str) -> Self {
        Text::new(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn measure_wraps_to_height() {
        let t = Text::new("hello brave new world").wrap(Wrap::Word);
        let s = t.measure(Constraints::loose(11, 10));
        assert_eq!(s.h, 2);
        assert!(s.w <= 11);
    }

    #[test]
    fn paint_centers_rows() {
        let t = Text::new("ab").align(TextAlign::Center);
        let mut canvas = Canvas::new(6, 1);
        let mut cx = Cx::new(0);
        t.paint(&mut cx, Area { x: 0, y: 0, w: 6, h: 1 }, &mut canvas);
        assert_eq!(canvas.to_lines()[0].text(), "  ab  ");
    }

    #[test]
    fn ellipsis_truncates() {
        let t = Text::new("hello world").ellipsis();
        let mut canvas = Canvas::new(7, 1);
        let mut cx = Cx::new(0);
        t.paint(&mut cx, Area { x: 0, y: 0, w: 7, h: 1 }, &mut canvas);
        assert_eq!(canvas.to_lines()[0].text(), "hello …");
    }

    #[test]
    fn spans_keep_highlights() {
        let t = Text::new("a").span("b", "Title");
        let mut canvas = Canvas::new(2, 1);
        let mut cx = Cx::new(0);
        t.paint(&mut cx, Area { x: 0, y: 0, w: 2, h: 1 }, &mut canvas);
        let line = &canvas.to_lines()[0];
        assert_eq!(line.spans.len(), 2);
        assert_eq!(line.spans[1].hl.as_deref(), Some("Title"));
    }

}
