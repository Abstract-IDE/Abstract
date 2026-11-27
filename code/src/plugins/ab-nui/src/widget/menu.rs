//! `Menu` — a selectable list. Reactive selection state drives the rendering;
//! `j`/`k` (or any motion) move, `<CR>` confirms, `<Esc>`/`q` cancel.

use std::cell::Cell;
use std::rc::Rc;

use crate::error::Result;
use crate::geometry::{Position, Size};
use crate::nvim::{self, Border, TitlePos};
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
        Self { title: None, size: Size::ratio(0.4, 0.4), position: Position::Center, border: Border::Rounded }
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
            // Focus leaving the menu dismisses it (see `Menu::open`).
            close_on_leave: true,
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

    /// Open the menu. `on_done` is called exactly once: with the chosen value
    /// on `<CR>`, or with `None` when the menu is dismissed — `<Esc>`/`q`,
    /// `<CR>` on an empty list, the cursor leaving the menu window, or the
    /// window being closed by anything else (`:q`, `<C-w>c`, another plugin).
    /// The menu is fully closed before `on_done` runs. (If `open` itself
    /// fails, `on_done` is never called.)
    ///
    /// The cursor row is the selection: `j`/`k`/arrows move it (wrapping, with
    /// counts), and any other motion (`G`, `gg`, search, the mouse) selects
    /// the row it lands on.
    pub fn open(self, on_done: impl Fn(Option<T>) + 'static) -> Result<()> {
        let popup = self.popup.clone();
        let result = self.open_inner(on_done);
        if result.is_err() {
            popup.close();
        }
        result
    }

    fn open_inner(self, on_done: impl Fn(Option<T>) + 'static) -> Result<()> {
        // Opened from insert mode (an insert-mode mapping, an LSP reply while
        // typing), the cursor would land in the list still inserting: leave
        // insert mode so the normal-mode keys below work.
        let mode = nvim::mode()?;
        self.popup.open()?;
        if mode.starts_with('i') || mode.starts_with('R') {
            crate::lua::cmd("stopinsert")?;
        }

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
            self.popup.own_effect(crate::reactive::unowned(|| {
                crate::reactive::effect(move || {
                    let i = sel.get();
                    let _ = popup.set_cursor(i as i64 + 1, 0);
                })
            }));
        }

        // The list is display-only; render_reactive already drew it once.
        self.popup.buffer().lock()?;

        let count = self.items.len();
        let values: Vec<T> = self.items.into_iter().map(|i| i.value).collect();

        // ...and the other way round: wherever the cursor goes, the selection
        // follows.
        {
            let popup = self.popup.clone();
            let sel = self.selected.clone();
            let group = self.popup.augroup()?;
            nvim::buf_autocmd(&["CursorMoved"], self.popup.buffer(), Some(group), move || {
                if let Some(row) = popup.cursor_row()
                    && count > 0
                {
                    sel.set_if_changed(row.min(count - 1));
                }
            })?;
        }

        // Navigation (wrapping; `3j` moves three).
        for (key, forward) in [("j", true), ("<Down>", true), ("k", false), ("<Up>", false)] {
            let sel = self.selected.clone();
            self.popup.on_key("n", key, move || {
                if count == 0 {
                    return;
                }
                let step = nvim::count1().unwrap_or(1) % count;
                sel.update(|s| *s = if forward { (*s + step) % count } else { (*s + count - step) % count });
            })?;
        }

        // Exactly once: whichever way the menu ends first wins.
        let fired = Rc::new(Cell::new(false));
        let finish: Rc<dyn Fn(Option<usize>)> = {
            let popup = self.popup.clone();
            Rc::new(move |idx: Option<usize>| {
                if fired.replace(true) {
                    return;
                }
                popup.close();
                on_done(idx.and_then(|i| values.get(i).cloned()));
            })
        };

        // Confirm the row under the cursor (the source of truth, even if no
        // `CursorMoved` has synced the signal yet).
        {
            let popup = self.popup.clone();
            let sel = self.selected.clone();
            let finish = finish.clone();
            self.popup.on_key("n", "<CR>", move || {
                if count == 0 {
                    return finish(None);
                }
                let idx = popup.cursor_row().unwrap_or_else(|| sel.get_untracked());
                finish(Some(idx.min(count - 1)));
            })?;
        }

        // Cancel.
        for key in ["<Esc>", "q"] {
            let finish = finish.clone();
            self.popup.on_key("n", key, move || finish(None))?;
        }

        // Closed or left some other way: that is a cancel too.
        self.popup.set_on_dismiss(move || finish(None));
        Ok(())
    }
}
