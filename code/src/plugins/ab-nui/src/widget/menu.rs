//! `Menu` — a selectable list. Reactive selection state drives the rendering;
//! `j`/`k` move, `<CR>` confirms, `<Esc>`/`q` cancel.

use crate::error::Result;
use crate::geometry::{Position, Size};
use crate::nvim::{Border, TitlePos};
use crate::reactive::Signal;
use crate::text::{Line, Span};
use crate::theme::groups;
use crate::widget::popup::{Popup, PopupOptions};

/// A single menu entry: a label plus an opaque user value.
pub struct MenuItem<T> {
    pub label: String,
    pub value: T,
}

impl<T> MenuItem<T> {
    pub fn new(label: impl Into<String>, value: T) -> Self {
        Self { label: label.into(), value }
    }
}

/// Builder/configuration for a [`Menu`].
pub struct MenuOptions {
    pub title: Option<String>,
    pub size: Size,
    pub position: Position,
    pub border: Border,
}

impl Default for MenuOptions {
    fn default() -> Self {
        Self {
            title: None,
            size: Size::ratio(0.4, 0.4),
            position: Position::Center,
            border: Border::Rounded,
        }
    }
}

/// A reactive selectable menu.
pub struct Menu<T: Clone + 'static> {
    popup: Popup,
    items: Vec<MenuItem<T>>,
    selected: Signal<usize>,
}

impl<T: Clone + 'static> Menu<T> {
    /// Build a menu over `items`. Does not open it yet.
    pub fn new(items: Vec<MenuItem<T>>, opts: MenuOptions) -> Result<Self> {
        let popup = Popup::new(PopupOptions {
            title: opts.title,
            title_pos: TitlePos::Center,
            size: opts.size,
            position: opts.position,
            border: opts.border,
            ..Default::default()
        })?;
        Ok(Self { popup, items, selected: Signal::new(0) })
    }

    /// The reactive index of the highlighted item.
    pub fn selected(&self) -> Signal<usize> {
        self.selected.clone()
    }

    /// The underlying popup (for extra keymaps, styling, etc).
    pub fn popup(&self) -> &Popup {
        &self.popup
    }

    /// Open the menu. `on_choose` fires with the chosen value when the user
    /// confirms; the menu closes first.
    pub fn open(self, on_choose: impl Fn(T) + 'static) -> Result<()> {
        self.open_with(on_choose, || {})
    }

    /// Like [`Menu::open`], but `on_cancel` fires when the user dismisses the
    /// menu (`<Esc>`/`q`) instead of choosing. Exactly one of the two callbacks
    /// runs, at most once.
    pub fn open_with(self, on_choose: impl Fn(T) + 'static, on_cancel: impl Fn() + 'static) -> Result<()> {
        self.popup.open()?;

        // Reactive rendering driven by the selection signal.
        let labels: Vec<String> = self.items.iter().map(|i| i.label.clone()).collect();
        let sel = self.selected.clone();
        self.popup.render_reactive(move || {
            let cur = sel.get();
            labels
                .iter()
                .enumerate()
                .map(|(i, label)| {
                    if i == cur {
                        Line::from_spans([Span::hl(format!("▌ {label}"), groups::SELECTION)])
                    } else {
                        Line::from_spans([Span::raw(format!("  {label}"))])
                    }
                })
                .collect()
        });

        // Keep the window cursor on the selected row, so `cursorline` agrees
        // with the ▌ marker and long lists scroll to follow the selection.
        {
            let popup = self.popup.clone();
            let sel = self.selected.clone();
            self.popup.own_effect(crate::reactive::effect(move || {
                let i = sel.get();
                let _ = popup.set_cursor(i as i64 + 1, 0);
            }));
        }

        // The list is display-only; render_reactive already drew it once.
        self.popup.buffer().lock()?;

        let count = self.items.len();
        let values: Vec<T> = self.items.into_iter().map(|i| i.value).collect();

        // Navigation.
        let down = self.selected.clone();
        self.popup.on_key("n", "j", move || {
            if count > 0 {
                down.update(|s| *s = (*s + 1) % count);
            }
        })?;
        let up = self.selected.clone();
        self.popup.on_key("n", "k", move || {
            if count > 0 {
                up.update(|s| *s = (*s + count - 1) % count);
            }
        })?;
        self.popup.on_key("n", "<Down>", {
            let s = self.selected.clone();
            move || {
                if count > 0 {
                    s.update(|s| *s = (*s + 1) % count);
                }
            }
        })?;
        self.popup.on_key("n", "<Up>", {
            let s = self.selected.clone();
            move || {
                if count > 0 {
                    s.update(|s| *s = (*s + count - 1) % count);
                }
            }
        })?;

        // Choose/cancel are mutually exclusive and fire at most once.
        let fired = std::rc::Rc::new(std::cell::Cell::new(false));

        // Confirm.
        {
            let close = self.popup.closer();
            let sel = self.selected.clone();
            let fired = fired.clone();
            self.popup.on_key("n", "<CR>", move || {
                if fired.replace(true) {
                    return;
                }
                let idx = sel.get_untracked();
                close();
                if let Some(value) = values.get(idx) {
                    on_choose(value.clone());
                }
            })?;
        }

        // Cancel.
        let on_cancel = std::rc::Rc::new(on_cancel);
        for key in ["<Esc>", "q"] {
            let popup = self.popup.clone();
            let on_cancel = on_cancel.clone();
            let fired = fired.clone();
            self.popup.on_key("n", key, move || {
                if fired.replace(true) {
                    return;
                }
                popup.close();
                on_cancel();
            })?;
        }
        Ok(())
    }
}
