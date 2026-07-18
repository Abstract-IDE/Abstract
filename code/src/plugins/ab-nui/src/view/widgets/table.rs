//! `Table` — columns with headers, mixed sizing, and list-style selection.

use std::rc::Rc;

use crate::text::{Line, truncate_line};
use crate::theme::groups;
use crate::view::canvas::Canvas;
use crate::view::widget::{Area, Constraints, Cx, Key, Size, Widget};
use crate::view::widgets::list::ListState;
use crate::view::widgets::scroll::draw_scrollbar;
use crate::view::widgets::text::TextAlign;

/// How a column's width is resolved against the table's area.
#[derive(Clone, Copy, Debug)]
pub enum ColWidth {
    /// Exactly this many cells.
    Fixed(u16),
    /// This fraction of the table width.
    Fraction(f32),
    /// A weighted share of the leftover space (like flex).
    Flex(u16),
}

/// One column: header, sizing, alignment.
#[derive(Clone)]
pub struct TableColumn {
    pub header: Line,
    pub width: ColWidth,
    pub align: TextAlign,
}

impl TableColumn {
    pub fn new(header: impl Into<String>) -> Self {
        Self { header: Line::raw(header), width: ColWidth::Flex(1), align: TextAlign::Left }
    }
    pub fn width(mut self, width: ColWidth) -> Self {
        self.width = width;
        self
    }
    pub fn align(mut self, align: TextAlign) -> Self {
        self.align = align;
        self
    }
}

/// Resolve column widths for a total of `avail` cells with `gap` between them.
fn resolve_widths(columns: &[TableColumn], avail: u16, gap: u16) -> Vec<u16> {
    let gaps = gap * columns.len().saturating_sub(1) as u16;
    let usable = avail.saturating_sub(gaps);

    let mut widths: Vec<u16> = Vec::with_capacity(columns.len());
    let mut fixed_total: u16 = 0;
    let mut flex_total: u16 = 0;
    for col in columns {
        match col.width {
            ColWidth::Fixed(w) => {
                widths.push(w);
                fixed_total = fixed_total.saturating_add(w);
            }
            ColWidth::Fraction(f) => {
                let w = ((usable as f32) * f.clamp(0.0, 1.0)).round() as u16;
                widths.push(w);
                fixed_total = fixed_total.saturating_add(w);
            }
            ColWidth::Flex(weight) => {
                widths.push(0); // placeholder
                flex_total += weight;
            }
        }
    }
    let leftover = usable.saturating_sub(fixed_total);
    for (i, col) in columns.iter().enumerate() {
        if let ColWidth::Flex(weight) = col.width {
            widths[i] = (leftover * weight).checked_div(flex_total).unwrap_or(0);
        }
    }
    widths
}

/// A scrolling, selectable table. Rows are pre-styled cells (`Vec<Line>` per
/// row); reuse the [`ListState`] machinery for selection/marking. Keys match
/// [`crate::view::List`].
pub struct Table {
    columns: Vec<TableColumn>,
    rows: Rc<Vec<Vec<Line>>>,
    state: ListState,
    header: bool,
    gap: u16,
    show_scrollbar: bool,
    on_select: Rc<dyn Fn(usize)>,
}

impl Table {
    pub fn new(columns: Vec<TableColumn>, rows: Vec<Vec<Line>>, state: &ListState) -> Self {
        Self {
            columns,
            rows: Rc::new(rows),
            state: state.clone(),
            header: true,
            gap: 1,
            show_scrollbar: true,
            on_select: Rc::new(|_| {}),
        }
    }
    /// Hide the header row.
    pub fn headerless(mut self) -> Self {
        self.header = false;
        self
    }
    /// Cells between columns.
    pub fn gap(mut self, gap: u16) -> Self {
        self.gap = gap;
        self
    }
    pub fn scrollbar(mut self, show: bool) -> Self {
        self.show_scrollbar = show;
        self
    }
    /// `<CR>` on a row (row index).
    pub fn on_select(mut self, f: impl Fn(usize) + 'static) -> Self {
        self.on_select = Rc::new(f);
        self
    }

    fn header_rows(&self) -> u16 {
        if self.header { 2 } else { 0 } // header + separator
    }

    /// Paint one row of cells into `y`, aligned within resolved widths.
    fn paint_cells(
        &self,
        canvas: &mut Canvas,
        area: Area,
        y: u16,
        cells: &[Line],
        widths: &[u16],
        hl_override: Option<&str>,
    ) {
        let mut x = area.x;
        for (i, w) in widths.iter().enumerate() {
            let empty = Line::empty();
            let cell = cells.get(i).unwrap_or(&empty);
            let mut cell = truncate_line(cell, *w);
            if let Some(hl) = hl_override {
                for span in &mut cell.spans {
                    span.hl = Some(hl.to_string());
                }
            }
            let cw = cell.width().min(*w);
            let dx = match self.columns[i].align {
                TextAlign::Left => 0,
                TextAlign::Center => (*w - cw) / 2,
                TextAlign::Right => *w - cw,
            };
            canvas.put_line(x + dx, y, &cell);
            x += w + self.gap;
            if x >= area.x + area.w {
                break;
            }
        }
    }
}

impl Widget for Table {
    fn measure(&self, c: Constraints) -> Size {
        let h = self.rows.len() as u16 + self.header_rows();
        c.clamp(Size::new(c.max_w, h.min(c.max_h)))
    }

    fn paint(&self, cx: &mut Cx, area: Area, canvas: &mut Canvas) {
        let focused = cx.will_focus();
        let bar_w: u16 = if self.show_scrollbar { 1 } else { 0 };
        let widths = resolve_widths(&self.columns, area.w.saturating_sub(bar_w), self.gap);
        let n = self.rows.len();

        // Header.
        let mut y = area.y;
        if self.header {
            let headers: Vec<Line> = self.columns.iter().map(|c| c.header.clone()).collect();
            self.paint_cells(canvas, area, y, &headers, &widths, Some(groups::HEADER));
            y += 1;
            for x in area.x..area.x + area.w.saturating_sub(bar_w) {
                canvas.set(x, y, '─', Some(Rc::from(groups::BORDER)));
            }
            y += 1;
        }

        // Body with list-style scroll/selection.
        let body_h = (area.h.saturating_sub(self.header_rows())) as usize;
        let selected = self.state.selected.get().min(n.saturating_sub(1));
        self.state.selected.set_silent(selected);
        let mut offset = self.state.offset.get_untracked().min(n.saturating_sub(1));
        if selected < offset {
            offset = selected;
        } else if body_h > 0 && selected >= offset + body_h {
            offset = selected - body_h + 1;
        }
        self.state.offset.set_silent(offset);

        for (screen_row, row_idx) in (offset..n.min(offset + body_h)).enumerate() {
            let is_sel = row_idx == selected;
            let hl = is_sel.then_some(if focused { groups::SELECTION } else { groups::FOCUS });
            if is_sel {
                // Paint the selection bar under the whole row first.
                for x in area.x..area.x + area.w.saturating_sub(bar_w) {
                    canvas.set(x, y + screen_row as u16, ' ', hl.map(Rc::from));
                }
            }
            self.paint_cells(canvas, area, y + screen_row as u16, &self.rows[row_idx], &widths, hl);
        }

        if self.show_scrollbar {
            draw_scrollbar(
                canvas,
                Area {
                    x: area.x + area.w.saturating_sub(1),
                    y,
                    w: 1,
                    h: area.h.saturating_sub(self.header_rows()),
                },
                n,
                body_h,
                offset,
            );
        }

        // Keys.
        let state = self.state.clone();
        let on_select = self.on_select.clone();
        let page = body_h.max(1);
        cx.register(
            area,
            Rc::new(move |k| {
                if n == 0 {
                    return false;
                }
                let cur = state.selected.get_untracked().min(n - 1);
                match k {
                    Key::Char('j') | Key::Down => {
                        state.selected.set((cur + 1) % n);
                        true
                    }
                    Key::Char('k') | Key::Up => {
                        state.selected.set((cur + n - 1) % n);
                        true
                    }
                    Key::Char('g') | Key::Home => {
                        state.selected.set(0);
                        true
                    }
                    Key::Char('G') | Key::End => {
                        state.selected.set(n - 1);
                        true
                    }
                    Key::PageDown | Key::Ctrl('f') => {
                        state.selected.set((cur + page).min(n - 1));
                        true
                    }
                    Key::PageUp | Key::Ctrl('b') => {
                        state.selected.set(cur.saturating_sub(page));
                        true
                    }
                    Key::Enter => {
                        on_select(cur);
                        true
                    }
                    _ => false,
                }
            }),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn widths_mix_fixed_fraction_flex() {
        let cols = vec![
            TableColumn::new("a").width(ColWidth::Fixed(10)),
            TableColumn::new("b").width(ColWidth::Fraction(0.25)),
            TableColumn::new("c").width(ColWidth::Flex(1)),
            TableColumn::new("d").width(ColWidth::Flex(3)),
        ];
        // avail 83, gap 1 → gaps 3, usable 80. fixed 10 + fraction 20 → leftover 50.
        let w = resolve_widths(&cols, 83, 1);
        assert_eq!(w[0], 10);
        assert_eq!(w[1], 20);
        assert_eq!(w[2], 12); // 50 * 1/4
        assert_eq!(w[3], 37); // 50 * 3/4
    }
}
