//! `Select` — a dropdown. Closed it shows `▾ value`; open, it paints its
//! option list as a [`Cx::overlay`] above the surrounding widgets.

use std::rc::Rc;

use crate::reactive::Signal;
use crate::text::{Line, display_width, truncate_line};
use crate::theme::groups;
use crate::view::canvas::Canvas;
use crate::view::widget::{Area, Constraints, Cx, Key, Size, Widget};

/// The signals a [`Select`] needs across rebuilds.
#[derive(Clone)]
pub struct SelectState {
    pub selected: Signal<usize>,
    pub open: Signal<bool>,
    /// The row highlighted while the dropdown is open.
    hover: Signal<usize>,
}

impl SelectState {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { selected: Signal::new(0), open: Signal::new(false), hover: Signal::new(0) }
    }
}

/// A dropdown select. Focused: `<CR>`/`<Space>` opens; open: `j`/`k` move,
/// `<CR>` chooses, `<Esc>` closes (consumed — the surface stays up).
pub struct Select {
    options: Vec<String>,
    state: SelectState,
    on_change: Rc<dyn Fn(usize)>,
}

impl Select {
    pub fn new(options: Vec<String>, state: &SelectState) -> Self {
        Self { options, state: state.clone(), on_change: Rc::new(|_| {}) }
    }
    pub fn on_change(mut self, f: impl Fn(usize) + 'static) -> Self {
        self.on_change = Rc::new(f);
        self
    }

    fn widest(&self) -> u16 {
        self.options.iter().map(|o| display_width(o)).max().unwrap_or(0)
    }
}

/// Where the open dropdown goes for an anchor at `anchor` (root-canvas
/// coordinates) on a `full` canvas: `(x0, y0, w, h)`. Below the anchor when the
/// options fit there (or there is at least as much room below as above),
/// otherwise above it.
fn dropdown_rect(anchor: Area, full: Area, n: usize, widest: u16) -> (u16, u16, u16, u16) {
    let n = n.min(u16::MAX as usize) as u16;
    let w = widest.saturating_add(4).min(full.w);
    let room_below = full.h.saturating_sub(anchor.y.saturating_add(1));
    let room_above = anchor.y.min(full.h);
    let (y0, h) = if n <= room_below || room_below >= room_above {
        (anchor.y.saturating_add(1), n.min(room_below))
    } else {
        let h = n.min(room_above);
        (anchor.y - h, h)
    };
    let x0 = anchor.x.min(full.w.saturating_sub(w));
    (x0, y0, w, h)
}

/// The first option shown in a dropdown `h` rows tall so `hover` is visible.
fn dropdown_top(hover: usize, n: usize, h: usize) -> usize {
    if h == 0 { 0 } else { hover.saturating_sub(h - 1).min(n.saturating_sub(h)) }
}

impl Widget for Select {
    fn measure(&self, c: Constraints) -> Size {
        c.clamp(Size::new(self.widest().saturating_add(2), 1))
    }

    fn paint(&self, cx: &mut Cx, area: Area, canvas: &mut Canvas) {
        let focused = cx.will_focus();
        let n = self.options.len();
        let selected = self.state.selected.get().min(n.saturating_sub(1));
        let open = self.state.open.get();

        let label = self.options.get(selected).cloned().unwrap_or_default();
        let hl = if focused { groups::SELECTION } else { groups::FIELD };
        canvas.put_str_clipped(area.x, area.y, area.w, &format!("▾ {label}"), Some(Rc::from(hl)));

        // The dropdown list, painted above everything after the main tree.
        // Overlays paint on the root canvas, so the anchor is mapped to
        // on-screen coordinates (it may sit on a scrolled, offscreen canvas).
        if open && n > 0 {
            let options = self.options.clone();
            let hover = self.state.hover.clone();
            let widest = self.widest();
            let screen = canvas.screen_map();
            cx.overlay(move |_, full, canvas| {
                // Scrolled out of view: no dropdown either.
                let Some(anchor) = screen.map(area) else { return };
                let (x0, y0, w, h) = dropdown_rect(anchor, full, n, widest);
                let cur = hover.get().min(n - 1); // tracked: moving the hover repaints
                let top = dropdown_top(cur, n, h as usize);
                let inner = w.saturating_sub(2);
                for (row, i) in (top..n.min(top + h as usize)).enumerate() {
                    let hl = if i == cur { groups::SELECTION } else { groups::FIELD };
                    // Pad/truncate by display width, not chars.
                    let label = truncate_line(&Line::raw(options[i].as_str()), inner);
                    let pad = " ".repeat(inner.saturating_sub(label.width()) as usize);
                    let text = format!(" {}{pad} ", label.text());
                    canvas.put_str_clipped(x0, y0 + row as u16, w, &text, Some(Rc::from(hl)));
                }
            });
        }

        // One focusable that behaves differently while open.
        let state = self.state.clone();
        let on_change = self.on_change.clone();
        cx.register(
            area,
            Rc::new(move |k| {
                if n == 0 {
                    return false;
                }
                if state.open.get_untracked() {
                    let cur = state.hover.get_untracked().min(n - 1);
                    match k {
                        Key::Char('j') | Key::Down => state.hover.set((cur + 1) % n),
                        Key::Char('k') | Key::Up => state.hover.set((cur + n - 1) % n),
                        Key::Enter | Key::Char(' ') => {
                            state.selected.set(cur);
                            state.open.set(false);
                            on_change(cur);
                        },
                        Key::Esc => state.open.set(false),
                        _ => return false,
                    }
                    true
                } else {
                    match k {
                        Key::Enter | Key::Char(' ') => {
                            state.hover.set(state.selected.get_untracked().min(n - 1));
                            state.open.set(true);
                            true
                        },
                        _ => false,
                    }
                }
            }),
        );
    }
}
