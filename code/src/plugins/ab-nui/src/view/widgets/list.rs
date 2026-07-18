//! `List` — the full-featured selectable list: custom row rendering, filtering,
//! multi-select, scrolling with a scrollbar. `ListView` remains as the simple
//! string-list wrapper it always was.

use std::rc::Rc;

use crate::reactive::Signal;
use crate::text::{Line, Span};
use crate::theme::groups;
use crate::view::canvas::Canvas;
use crate::view::widget::{Area, Constraints, Cx, Key, Size, Widget};
use crate::view::widgets::scroll::draw_scrollbar;

/// The signals a list needs across rebuilds. Create ONE (outside the build
/// closure) and hand it to every rebuild's `List`.
#[derive(Clone)]
pub struct ListState {
    /// Selected item, as an index into the ORIGINAL (unfiltered) items.
    pub selected: Signal<usize>,
    /// First visible row (in filtered/visible rows).
    pub offset: Signal<usize>,
    /// Multi-selected original indices (with [`List::multi_select`]).
    pub marked: Signal<Vec<usize>>,
}

impl ListState {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { selected: Signal::new(0), offset: Signal::new(0), marked: Signal::new(Vec::new()) }
    }

    /// Adopt an existing selection signal (e.g. from old `ListView` code).
    pub fn from_selected(selected: Signal<usize>) -> Self {
        Self { selected, offset: Signal::new(0), marked: Signal::new(Vec::new()) }
    }
}

/// Row-rendering context passed to a custom [`List::render`] closure.
pub struct ItemCx {
    /// Index into the original items.
    pub index: usize,
    pub selected: bool,
    pub marked: bool,
    /// Whether the list itself is the focused widget.
    pub focused: bool,
    /// Cells available for the row (after the selection gutter).
    pub width: u16,
}

type RenderFn<T> = Rc<dyn Fn(&T, &ItemCx) -> Line>;
type FilterFn<T> = Rc<dyn Fn(&T) -> bool>;

/// A selectable, scrolling list over arbitrary items.
///
/// Keys (focused): `j`/`k`/arrows move, `g`/`G` home/end, `<C-d>`/`<C-u>` and
/// `PageUp`/`PageDown` page, `<Space>` toggles a mark (multi-select), `<CR>`
/// confirms. Callback indices are always ORIGINAL item indices, also under a
/// [`List::filter`].
pub struct List<T: Clone + 'static> {
    items: Rc<Vec<T>>,
    state: ListState,
    render: RenderFn<T>,
    filter: Option<FilterFn<T>>,
    multi: bool,
    show_scrollbar: bool,
    on_select: Rc<dyn Fn(usize)>,
    on_change: Rc<dyn Fn(usize)>,
}

impl<T: Clone + ToString + 'static> List<T> {
    /// A list rendering items via `ToString`. Use [`List::render`] to replace
    /// the row rendering (or [`List::with_render`] for non-`ToString` items).
    pub fn new(items: Vec<T>, state: &ListState) -> Self {
        Self::with_render(items, state, |item, _| Line::raw(item.to_string()))
    }
}

impl<T: Clone + 'static> List<T> {
    /// A list with a custom row renderer (no `ToString` requirement).
    pub fn with_render(items: Vec<T>, state: &ListState, render: impl Fn(&T, &ItemCx) -> Line + 'static) -> Self {
        Self {
            items: Rc::new(items),
            state: state.clone(),
            render: Rc::new(render),
            filter: None,
            multi: false,
            show_scrollbar: true,
            on_select: Rc::new(|_| {}),
            on_change: Rc::new(|_| {}),
        }
    }

    /// Replace the row renderer.
    pub fn render(mut self, f: impl Fn(&T, &ItemCx) -> Line + 'static) -> Self {
        self.render = Rc::new(f);
        self
    }

    /// Show only items passing `f`. Selection and callbacks keep reporting
    /// original indices.
    pub fn filter(mut self, f: impl Fn(&T) -> bool + 'static) -> Self {
        self.filter = Some(Rc::new(f));
        self
    }

    /// Enable multi-select: `<Space>` toggles the mark on the selected item.
    pub fn multi_select(mut self) -> Self {
        self.multi = true;
        self
    }

    pub fn scrollbar(mut self, show: bool) -> Self {
        self.show_scrollbar = show;
        self
    }

    /// `<CR>` on an item (original index).
    pub fn on_select(mut self, f: impl Fn(usize) + 'static) -> Self {
        self.on_select = Rc::new(f);
        self
    }

    /// Selection moved (original index).
    pub fn on_change(mut self, f: impl Fn(usize) + 'static) -> Self {
        self.on_change = Rc::new(f);
        self
    }

    /// Original indices passing the filter, in order.
    fn visible(&self) -> Vec<usize> {
        match &self.filter {
            Some(f) => self.items.iter().enumerate().filter(|(_, it)| f(it)).map(|(i, _)| i).collect(),
            None => (0..self.items.len()).collect(),
        }
    }

    fn bar_w(&self) -> u16 {
        if self.show_scrollbar { 1 } else { 0 }
    }
}

impl<T: Clone + 'static> Widget for List<T> {
    fn measure(&self, c: Constraints) -> Size {
        let visible = self.visible();
        let probe = ItemCx { index: 0, selected: false, marked: false, focused: false, width: c.max_w };
        let w = visible
            .iter()
            .map(|&i| (self.render)(&self.items[i], &probe).width() + 2 + self.bar_w())
            .max()
            .unwrap_or(0);
        let h = visible.len() as u16;
        c.clamp(Size::new(w.min(c.max_w), h.min(c.max_h)))
    }

    fn paint(&self, cx: &mut Cx, area: Area, canvas: &mut Canvas) {
        let focused = cx.will_focus();
        let visible = self.visible();
        let n = visible.len();
        let h = area.h as usize;

        // Clamp selection to the visible set (silently: we're mid-repaint).
        let selected = self.state.selected.get(); // tracked: repaint on move
        let pos = visible.iter().position(|&i| i == selected).unwrap_or_else(|| {
            if let Some(&first) = visible.first() {
                self.state.selected.set_silent(first);
            }
            0
        });

        // Scroll window: keep the selection visible.
        let mut offset = self.state.offset.get_untracked().min(n.saturating_sub(1));
        if pos < offset {
            offset = pos;
        } else if h > 0 && pos >= offset + h {
            offset = pos - h + 1;
        }
        self.state.offset.set_silent(offset);

        let marked = self.state.marked.get(); // tracked with multi-select
        let row_w = area.w.saturating_sub(2 + self.bar_w());
        for (row, vis_pos) in (offset..n.min(offset + h)).enumerate() {
            let index = visible[vis_pos];
            let is_sel = vis_pos == pos;
            let icx = ItemCx {
                index,
                selected: is_sel,
                marked: marked.contains(&index),
                focused,
                width: row_w,
            };
            let mut line = Line::empty();
            let gutter = match (is_sel, icx.marked) {
                (true, _) => Span::hl("▌ ", if focused { groups::SELECTION } else { groups::FOCUS }),
                (false, true) => Span::hl("✓ ", groups::ACCENT),
                (false, false) => Span::raw("  "),
            };
            line.spans.push(gutter);
            let rendered = (self.render)(&self.items[index], &icx);
            if is_sel {
                // Highlight the whole selected row.
                for span in rendered.spans {
                    line.spans.push(Span {
                        text: span.text,
                        hl: Some(if focused { groups::SELECTION } else { groups::FOCUS }.to_string()),
                    });
                }
            } else {
                line.spans.extend(rendered.spans);
            }
            canvas.put_line(area.x, area.y + row as u16, &line);
        }

        if self.show_scrollbar {
            draw_scrollbar(
                canvas,
                Area { x: area.x + area.w.saturating_sub(1), y: area.y, w: 1, h: area.h },
                n,
                h,
                offset,
            );
        }

        // Key handling.
        let state = self.state.clone();
        let on_select = self.on_select.clone();
        let on_change = self.on_change.clone();
        let multi = self.multi;
        let page = h.max(1);
        cx.register(
            area,
            Rc::new(move |k| {
                if visible.is_empty() {
                    return false;
                }
                let cur = state.selected.get_untracked();
                let pos = visible.iter().position(|&i| i == cur).unwrap_or(0);
                let select_pos = |p: usize| {
                    let idx = visible[p.min(visible.len() - 1)];
                    state.selected.set(idx);
                    on_change(idx);
                };
                match k {
                    Key::Char('j') | Key::Down => {
                        select_pos((pos + 1) % visible.len());
                        true
                    }
                    Key::Char('k') | Key::Up => {
                        select_pos((pos + visible.len() - 1) % visible.len());
                        true
                    }
                    Key::Char('g') | Key::Home => {
                        select_pos(0);
                        true
                    }
                    Key::Char('G') | Key::End => {
                        select_pos(visible.len() - 1);
                        true
                    }
                    Key::PageDown | Key::Ctrl('f') => {
                        select_pos((pos + page).min(visible.len() - 1));
                        true
                    }
                    Key::PageUp | Key::Ctrl('b') => {
                        select_pos(pos.saturating_sub(page));
                        true
                    }
                    Key::Ctrl('d') => {
                        select_pos((pos + page / 2).min(visible.len() - 1));
                        true
                    }
                    Key::Ctrl('u') => {
                        select_pos(pos.saturating_sub(page / 2));
                        true
                    }
                    Key::Char(' ') if multi => {
                        state.marked.update(|m| {
                            if let Some(at) = m.iter().position(|&i| i == cur) {
                                m.remove(at);
                            } else {
                                m.push(cur);
                            }
                        });
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

// -----------------------------------------------------------------------------
// ListView — the original simple string list, now a thin wrapper
// -----------------------------------------------------------------------------

/// A selectable, scrolling list of strings backed by a `Signal<usize>`
/// selection. `j`/`k` (or arrows) move; `<CR>` confirms.
pub struct ListView {
    inner: List<String>,
}

impl ListView {
    pub fn new(items: Vec<String>, selected: Signal<usize>) -> Self {
        let state = ListState::from_selected(selected);
        Self { inner: List::new(items, &state).scrollbar(false) }
    }
    pub fn on_select(mut self, f: impl Fn(usize) + 'static) -> Self {
        self.inner = self.inner.on_select(f);
        self
    }
}

impl Widget for ListView {
    fn measure(&self, c: Constraints) -> Size {
        self.inner.measure(c)
    }
    fn paint(&self, cx: &mut Cx, area: Area, canvas: &mut Canvas) {
        self.inner.paint(cx, area, canvas);
    }
}
