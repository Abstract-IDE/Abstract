//! `Frame`: a rounded box split into a header row, a body and an optional
//! footer row, in one or two columns. The column divider only runs through
//! the body, meeting the rules with proper junctions, and can carry a
//! scrollbar thumb — so a list next to it needs no scrollbar column of its own:
//!
//! ```text
//! ╭─────────────────────────╮
//! │ header     header       │
//! ├──────────┬──────────────┤
//! │ body     ┃ body         │
//! │          │              │
//! ├──────────┴──────────────┤
//! │ footer     footer       │
//! ╰─────────────────────────╯
//! ```
//!
//! ```ignore
//! Frame::new(vec![
//!     FrameColumn::new(prompt, list).footer(counts),
//!     FrameColumn::new(title, preview).footer(info),
//! ])
//! .split(0.6)                        // the right column's width share
//! .scrollbar(total, state.offset.clone())
//! ```

use std::rc::Rc;

use crate::reactive::Signal;
use crate::theme::groups;
use crate::view::canvas::Canvas;
use crate::view::widget::{Area, Constraints, Cx, Element, Size, Widget};

/// One column: a header cell, a body and (optionally) a footer cell.
pub struct FrameColumn {
    header: Element,
    body: Element,
    footer: Option<Element>,
}

impl FrameColumn {
    pub fn new(header: impl Widget + 'static, body: impl Widget + 'static) -> Self {
        Self { header: Box::new(header), body: Box::new(body), footer: None }
    }

    /// The footer cell (shown only when every column has one).
    pub fn footer(mut self, footer: impl Widget + 'static) -> Self {
        self.footer = Some(Box::new(footer));
        self
    }
}

pub struct Frame {
    columns: Vec<FrameColumn>,
    right_ratio: f64,
    scroll: Option<(usize, Signal<usize>)>,
}

impl Frame {
    /// One or two columns (more are ignored).
    pub fn new(columns: Vec<FrameColumn>) -> Self {
        Self { columns, right_ratio: 0.5, scroll: None }
    }

    /// The right column's share of the width (two columns).
    pub fn split(mut self, right_ratio: f64) -> Self {
        self.right_ratio = right_ratio.clamp(0.05, 0.95);
        self
    }

    /// A scrollbar for the first column's body, drawn on the divider (or on
    /// the right border with one column): `total` rows, the first visible
    /// one in `offset` (read after the body paints, so a list that scrolls
    /// itself to its selection is shown where it ends up).
    pub fn scrollbar(mut self, total: usize, offset: Signal<usize>) -> Self {
        self.scroll = Some((total, offset));
        self
    }
}

/// The thumb's (top, height) on a track of `track` rows showing `view` of
/// `total` rows from `offset`; `None` when everything fits.
pub fn thumb(total: usize, view: usize, offset: usize, track: usize) -> Option<(usize, usize)> {
    if track == 0 || total <= view {
        return None;
    }
    let height = (track * view / total).clamp(1, track);
    let max_top = track - height;
    let top = (offset.min(total - view) * max_top).div_ceil(total - view).min(max_top);
    Some((top, height))
}

/// Characters for a horizontal rule: (left end, where the divider meets it, right end).
type Rule = (char, char, char);
const TOP: Rule = ('╭', '─', '╮');
const HEADER: Rule = ('├', '┬', '┤');
const FOOTER: Rule = ('├', '┴', '┤');
const BOTTOM: Rule = ('╰', '─', '╯');

impl Widget for Frame {
    fn measure(&self, c: Constraints) -> Size {
        // Fills whatever it is given.
        Size::new(c.max_w, c.max_h)
    }

    fn paint(&self, cx: &mut Cx, area: Area, canvas: &mut Canvas) {
        let cols = &self.columns[..self.columns.len().min(2)];
        let has_footer = !cols.is_empty() && cols.iter().all(|c| c.footer.is_some());
        let min_h = if has_footer { 7 } else { 5 };
        if cols.is_empty() || area.w < 4 || area.h < min_h {
            return;
        }

        // Inner widths: one column takes everything between the side borders;
        // two share it around the divider.
        let inner = area.w - 2;
        let widths: Vec<u16> = if cols.len() == 2 && inner >= 3 {
            let right = ((inner - 1) as f64 * self.right_ratio) as u16;
            vec![inner - 1 - right, right]
        } else {
            vec![inner]
        };
        let split_x = (widths.len() == 2).then(|| area.x + 1 + widths[0]);
        let col_x = |i: usize| if i == 0 { area.x + 1 } else { split_x.unwrap_or(area.x) + 1 };

        // Rows: top rule, header, rule, body…, [rule, footer], bottom rule.
        let top = area.y;
        let bottom = area.y + area.h - 1;
        let header_y = top + 1;
        let header_rule = top + 2;
        let footer_rule = has_footer.then(|| bottom - 2);
        let body_end = footer_rule.unwrap_or(bottom); // exclusive

        let hl: Option<Rc<str>> = Some(Rc::from(groups::BORDER));
        let rule = |canvas: &mut Canvas, y: u16, (l, m, r): Rule| {
            for x in area.x..area.x + area.w {
                canvas.set(x, y, '─', hl.clone());
            }
            canvas.set(area.x, y, l, hl.clone());
            canvas.set(area.x + area.w - 1, y, r, hl.clone());
            if let Some(x) = split_x {
                canvas.set(x, y, m, hl.clone());
            }
        };
        rule(canvas, top, TOP);
        rule(canvas, header_rule, HEADER);
        if let Some(y) = footer_rule {
            rule(canvas, y, FOOTER);
        }
        rule(canvas, bottom, BOTTOM);
        for y in (top + 1..bottom).filter(|&y| y != header_rule && Some(y) != footer_rule) {
            canvas.set(area.x, y, '│', hl.clone());
            canvas.set(area.x + area.w - 1, y, '│', hl.clone());
            // The divider belongs to the body only.
            if let Some(x) = split_x
                && y > header_rule
                && y < body_end
            {
                canvas.set(x, y, '│', hl.clone());
            }
        }

        // Children, column by column and top to bottom (also the order
        // interactive widgets register in), each clipped to its cell.
        let paint = |child: &Element, cell: Area, canvas: &mut Canvas, cx: &mut Cx| {
            let prev = canvas.push_clip(cell);
            child.paint(cx, cell, canvas);
            canvas.restore_clip(prev);
        };
        for (i, (col, &w)) in cols.iter().zip(&widths).enumerate() {
            let x = col_x(i);
            paint(&col.header, Area { x, y: header_y, w, h: 1 }, canvas, cx);
            paint(&col.body, Area { x, y: header_rule + 1, w, h: body_end - header_rule - 1 }, canvas, cx);
            if let (Some(footer), Some(rule_y)) = (&col.footer, footer_rule) {
                paint(footer, Area { x, y: rule_y + 1, w, h: 1 }, canvas, cx);
            }
        }

        // The scrollbar thumb, on the divider (or the right border).
        if let Some((total, offset)) = &self.scroll {
            let x = split_x.unwrap_or(area.x + area.w - 1);
            let (y0, track) = (header_rule + 1, (body_end - header_rule - 1) as usize);
            // Tracked: a scroll on its own (no other change) still repaints.
            if let Some((top, height)) = thumb(*total, track, offset.get(), track) {
                let thumb_hl: Option<Rc<str>> = Some(Rc::from(groups::SCROLLBAR_THUMB));
                for y in top..top + height {
                    canvas.set(x, y0 + y as u16, '┃', thumb_hl.clone());
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::thumb;

    #[test]
    fn thumb_size_and_position() {
        assert_eq!(thumb(10, 20, 0, 20), None);
        assert_eq!(thumb(100, 10, 0, 10), Some((0, 1)));
        assert_eq!(thumb(100, 10, 90, 10), Some((9, 1)));
        assert_eq!(thumb(40, 20, 0, 20), Some((0, 10)));
        assert_eq!(thumb(40, 20, 20, 20), Some((10, 10)));
        // Any scroll at all moves the thumb off the top.
        assert_eq!(thumb(1000, 20, 1, 20).map(|t| t.0), Some(1));
    }
}
