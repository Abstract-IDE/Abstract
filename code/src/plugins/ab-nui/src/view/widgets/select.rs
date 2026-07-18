//! `Select` — a dropdown. Closed it shows `▾ value`; open, it paints its
//! option list as a [`Cx::overlay`] above the surrounding widgets.

use std::rc::Rc;

use crate::reactive::Signal;
use crate::text::display_width;
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

impl Widget for Select {
    fn measure(&self, c: Constraints) -> Size {
        Size::new((self.widest() + 2).min(c.max_w), 1)
    }

    fn paint(&self, cx: &mut Cx, area: Area, canvas: &mut Canvas) {
        let focused = cx.will_focus();
        let n = self.options.len();
        let selected = self.state.selected.get().min(n.saturating_sub(1));
        let open = self.state.open.get();

        let label = self.options.get(selected).cloned().unwrap_or_default();
        let hl = if focused { groups::SELECTION } else { groups::FIELD };
        canvas.put_str(area.x, area.y, &format!("▾ {label}"), Some(Rc::from(hl)));

        // The dropdown list, painted above everything after the main tree.
        if open && n > 0 {
            let options = self.options.clone();
            let hover = self.state.hover.clone();
            let widest = self.widest();
            cx.overlay(move |_, full, canvas| {
                let w = (widest + 4).min(full.w);
                let h = (n as u16).min(full.h.saturating_sub(1));
                // Below the anchor if there's room, else above.
                let below = area.y + 1 + h <= full.h;
                let y0 = if below { area.y + 1 } else { area.y.saturating_sub(h) };
                let x0 = area.x.min(full.w.saturating_sub(w));
                let cur = hover.get_untracked().min(n - 1);
                for (row, option) in options.iter().enumerate().take(h as usize) {
                    let hl = if row == cur { groups::SELECTION } else { groups::FIELD };
                    let text = format!(" {option:<width$} ", width = (w as usize).saturating_sub(2));
                    canvas.put_str(x0, y0 + row as u16, &text, Some(Rc::from(hl)));
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
                        }
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
                        }
                        _ => false,
                    }
                }
            }),
        );
    }
}
