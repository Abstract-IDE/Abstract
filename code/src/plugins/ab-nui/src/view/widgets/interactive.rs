//! Interactive widgets. Each registers a key handler during paint via [`Cx`];
//! the runtime routes events to whichever is focused. Focus styling is driven
//! by [`Cx::will_focus`], so widgets paint themselves differently when focused.

use std::rc::Rc;

use crate::reactive::Signal;
use crate::theme::groups;
use crate::view::canvas::Canvas;
use crate::view::widget::{Area, Constraints, Cx, Key, Size, Widget};

// -----------------------------------------------------------------------------
// Button
// -----------------------------------------------------------------------------

/// A pressable button. Activated with `<CR>` or `<Space>` while focused.
pub struct Button {
    label: String,
    on_press: Rc<dyn Fn()>,
    hl: Option<Rc<str>>,
}

impl Button {
    pub fn new(label: impl Into<String>) -> Self {
        Self { label: label.into(), on_press: Rc::new(|| {}), hl: None }
    }
    pub fn on_press(mut self, f: impl Fn() + 'static) -> Self {
        self.on_press = Rc::new(f);
        self
    }
    pub fn fg(mut self, group: &str) -> Self {
        self.hl = Some(Rc::from(group));
        self
    }
}

impl Widget for Button {
    fn measure(&self, c: Constraints) -> Size {
        let w = (self.label.chars().count() as u16 + 4).min(c.max_w);
        Size::new(w, 1)
    }

    fn paint(&self, cx: &mut Cx, area: Area, canvas: &mut Canvas) {
        let focused = cx.will_focus();
        let text = format!("[ {} ]", self.label);
        let hl = if focused {
            Some(Rc::from(groups::SELECTION))
        } else {
            self.hl.clone()
        };
        canvas.put_str(area.x, area.y, &text, hl);

        let press = self.on_press.clone();
        cx.register(
            area,
            Rc::new(move |k| match k {
                Key::Enter | Key::Char(' ') => {
                    press();
                    true
                }
                _ => false,
            }),
        );
    }
}

// -----------------------------------------------------------------------------
// ListView
// -----------------------------------------------------------------------------

/// A selectable, scrolling list backed by a `Signal<usize>` selection.
/// `j`/`k` (or arrows) move; `<CR>` confirms.
pub struct ListView {
    items: Vec<String>,
    selected: Signal<usize>,
    on_select: Rc<dyn Fn(usize)>,
}

impl ListView {
    pub fn new(items: Vec<String>, selected: Signal<usize>) -> Self {
        Self { items, selected, on_select: Rc::new(|_| {}) }
    }
    pub fn on_select(mut self, f: impl Fn(usize) + 'static) -> Self {
        self.on_select = Rc::new(f);
        self
    }
}

impl Widget for ListView {
    fn measure(&self, c: Constraints) -> Size {
        let w = self.items.iter().map(|i| i.chars().count() as u16 + 2).max().unwrap_or(0);
        let h = self.items.len() as u16;
        Size::new(w.min(c.max_w), h.min(c.max_h))
    }

    fn paint(&self, cx: &mut Cx, area: Area, canvas: &mut Canvas) {
        let focused = cx.will_focus();
        let cur = self.selected.get();
        let n = self.items.len();
        let h = area.h as usize;

        // Scroll so the selection stays visible.
        let start = if h == 0 || cur < h { 0 } else { cur - h + 1 };
        for (row, i) in (start..n.min(start + h)).enumerate() {
            let sel = i == cur;
            let prefix = if sel { "▌ " } else { "  " };
            let line = format!("{}{}", prefix, self.items[i]);
            let hl = if sel {
                Some(Rc::from(if focused { groups::SELECTION } else { groups::FOCUS }))
            } else {
                None
            };
            canvas.put_str(area.x, area.y + row as u16, &line, hl);
        }

        let sel = self.selected.clone();
        let on = self.on_select.clone();
        cx.register(
            area,
            Rc::new(move |k| match k {
                Key::Char('j') | Key::Down => {
                    if n > 0 {
                        sel.update(|s| *s = (*s + 1) % n);
                    }
                    true
                }
                Key::Char('k') | Key::Up => {
                    if n > 0 {
                        sel.update(|s| *s = (*s + n - 1) % n);
                    }
                    true
                }
                Key::Enter => {
                    on(sel.get_untracked());
                    true
                }
                _ => false,
            }),
        );
    }
}

// -----------------------------------------------------------------------------
// TextField
// -----------------------------------------------------------------------------

/// A single-line text field backed by a `Signal<String>`. Input is captured by
/// the runtime's key routing (no Neovim insert mode), so it composes like any
/// other widget. Shows a block cursor when focused.
pub struct TextField {
    value: Signal<String>,
    placeholder: String,
}

impl TextField {
    pub fn new(value: Signal<String>) -> Self {
        Self { value, placeholder: String::new() }
    }
    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = text.into();
        self
    }
}

impl Widget for TextField {
    fn measure(&self, c: Constraints) -> Size {
        Size::new(c.max_w, 1)
    }

    fn paint(&self, cx: &mut Cx, area: Area, canvas: &mut Canvas) {
        let focused = cx.will_focus();
        let text = self.value.get();
        canvas.fill(area, ' ', Some(Rc::from(groups::FIELD)));

        if text.is_empty() && !focused {
            canvas.put_str(area.x, area.y, &self.placeholder, Some(Rc::from(groups::MUTED)));
        } else {
            canvas.put_str(area.x, area.y, &text, Some(Rc::from(groups::FIELD)));
        }

        if focused && area.w > 0 {
            let cursor_x = area.x + (text.chars().count() as u16).min(area.w - 1);
            canvas.set(cursor_x, area.y, ' ', Some(Rc::from(groups::SELECTION)));
        }

        let value = self.value.clone();
        cx.register(
            area,
            Rc::new(move |k| match k {
                Key::Char(c) => {
                    value.update(|s| s.push(c));
                    true
                }
                Key::Backspace => {
                    value.update(|s| {
                        s.pop();
                    });
                    true
                }
                _ => false,
            }),
        );
    }
}
