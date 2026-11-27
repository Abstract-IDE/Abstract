//! The seek window's frame: a rounded box split into a header row, a body and
//! an optional footer row, in one or two columns, with proper junctions
//! (`├ ┼ ┤ ┬ ┴`) where the rules meet.
//!
//! ```text
//! ╭──────────┬──────────────╮
//! │ header   │ header       │
//! ├──────────┼──────────────┤
//! │ body     │ body         │
//! ├──────────┼──────────────┤
//! │ footer   │ footer       │
//! ╰──────────┴──────────────╯
//! ```

use std::rc::Rc;

use wp_ui::theme::groups;
use wp_ui::view::canvas::Canvas;
use wp_ui::view::{Area, Constraints, Cx, Element, Size, Widget};

/// One column's widgets. `footer` is ignored unless every column has one.
pub struct Column {
    pub header: Element,
    pub body: Element,
    pub footer: Option<Element>,
}

pub struct Frame {
    columns: Vec<Column>,
    /// Width share of the right column when there are two.
    right_ratio: f64,
}

impl Frame {
    /// A frame of one or two columns (more are ignored).
    pub fn new(columns: Vec<Column>, right_ratio: f64) -> Self {
        Self { columns, right_ratio }
    }
}

/// Characters for a horizontal rule: (left end, middle junction, right end).
type Rule = (char, char, char);
const TOP: Rule = ('╭', '┬', '╮');
const MID: Rule = ('├', '┼', '┤');
const BOTTOM: Rule = ('╰', '┴', '╯');

impl Widget for Frame {
    fn measure(&self, c: Constraints) -> Size {
        // Fill whatever the surface gives us.
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
        // two share it around a middle rule.
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
        rule(canvas, header_rule, MID);
        if let Some(y) = footer_rule {
            rule(canvas, y, MID);
        }
        rule(canvas, bottom, BOTTOM);
        for y in (top + 1..bottom).filter(|&y| y != header_rule && Some(y) != footer_rule) {
            canvas.set(area.x, y, '│', hl.clone());
            canvas.set(area.x + area.w - 1, y, '│', hl.clone());
            if let Some(x) = split_x {
                canvas.set(x, y, '│', hl.clone());
            }
        }

        // Children, column by column and top to bottom — that is also the
        // order interactive widgets register in, so the first column's header
        // is focus index 0.
        for (i, (col, &w)) in cols.iter().zip(&widths).enumerate() {
            let x = col_x(i);
            col.header.paint(cx, Area { x, y: header_y, w, h: 1 }, canvas);
            let body = Area { x, y: header_rule + 1, w, h: body_end - header_rule - 1 };
            col.body.paint(cx, body, canvas);
            if let (Some(footer), Some(rule_y)) = (&col.footer, footer_rule) {
                footer.paint(cx, Area { x, y: rule_y + 1, w, h: 1 }, canvas);
            }
        }
    }
}
