//! Boolean and choice inputs: [`Checkbox`], [`Toggle`], [`RadioGroup`].

use std::rc::Rc;

use crate::reactive::Signal;
use crate::text::display_width;
use crate::theme::groups;
use crate::view::canvas::Canvas;
use crate::view::widget::{Area, Constraints, Cx, Key, Size, Widget};

/// `[x] label` — toggled with `<CR>`/`<Space>` while focused.
pub struct Checkbox {
    label: String,
    checked: Signal<bool>,
    on_change: Rc<dyn Fn(bool)>,
}

impl Checkbox {
    pub fn new(label: impl Into<String>, checked: Signal<bool>) -> Self {
        Self { label: label.into(), checked, on_change: Rc::new(|_| {}) }
    }
    pub fn on_change(mut self, f: impl Fn(bool) + 'static) -> Self {
        self.on_change = Rc::new(f);
        self
    }
}

impl Widget for Checkbox {
    fn measure(&self, c: Constraints) -> Size {
        c.clamp(Size::new(display_width(&self.label).saturating_add(4), 1))
    }

    fn paint(&self, cx: &mut Cx, area: Area, canvas: &mut Canvas) {
        let focused = cx.will_focus();
        let checked = self.checked.get();
        let text = format!("[{}] {}", if checked { "x" } else { " " }, self.label);
        let hl = if focused { Some(Rc::from(groups::SELECTION)) } else { None };
        canvas.put_str_clipped(area.x, area.y, area.w, &text, hl);

        let sig = self.checked.clone();
        let on_change = self.on_change.clone();
        cx.register(
            area,
            Rc::new(move |k| match k {
                Key::Enter | Key::Char(' ') => {
                    sig.update(|v| *v = !*v);
                    on_change(sig.get_untracked());
                    true
                },
                _ => false,
            }),
        );
    }
}

/// A pill switch (`●──` / `──●`) — same interaction as [`Checkbox`].
pub struct Toggle {
    label: String,
    on: Signal<bool>,
    on_change: Rc<dyn Fn(bool)>,
}

impl Toggle {
    pub fn new(label: impl Into<String>, on: Signal<bool>) -> Self {
        Self { label: label.into(), on, on_change: Rc::new(|_| {}) }
    }
    pub fn on_change(mut self, f: impl Fn(bool) + 'static) -> Self {
        self.on_change = Rc::new(f);
        self
    }
}

impl Widget for Toggle {
    fn measure(&self, c: Constraints) -> Size {
        c.clamp(Size::new(display_width(&self.label).saturating_add(5), 1))
    }

    fn paint(&self, cx: &mut Cx, area: Area, canvas: &mut Canvas) {
        let focused = cx.will_focus();
        let on = self.on.get();
        let pill = if on { "──●" } else { "●──" };
        let pill_hl = if on { groups::SUCCESS } else { groups::MUTED };
        let label_hl: Option<Rc<str>> = if focused { Some(Rc::from(groups::SELECTION)) } else { None };
        let consumed = canvas.put_str_clipped(area.x, area.y, area.w, pill, Some(Rc::from(pill_hl)));
        let label_x = consumed.saturating_add(1).min(area.w);
        canvas.put_str_clipped(area.x + label_x, area.y, area.w - label_x, &self.label, label_hl);

        let sig = self.on.clone();
        let on_change = self.on_change.clone();
        cx.register(
            area,
            Rc::new(move |k| match k {
                Key::Enter | Key::Char(' ') => {
                    sig.update(|v| *v = !*v);
                    on_change(sig.get_untracked());
                    true
                },
                _ => false,
            }),
        );
    }
}

/// A single-focus group of radio options (`(•)` / `( )`). `j`/`k` (vertical)
/// or `h`/`l` (horizontal) and arrows move; the selection follows the cursor.
pub struct RadioGroup {
    options: Vec<String>,
    selected: Signal<usize>,
    horizontal: bool,
    on_change: Rc<dyn Fn(usize)>,
}

impl RadioGroup {
    pub fn new(options: Vec<String>, selected: Signal<usize>) -> Self {
        Self { options, selected, horizontal: false, on_change: Rc::new(|_| {}) }
    }
    pub fn horizontal(mut self) -> Self {
        self.horizontal = true;
        self
    }
    pub fn on_change(mut self, f: impl Fn(usize) + 'static) -> Self {
        self.on_change = Rc::new(f);
        self
    }

    fn option_text(&self, i: usize, selected: usize) -> String {
        format!("({}) {}", if i == selected { "•" } else { " " }, self.options[i])
    }
}

impl Widget for RadioGroup {
    fn measure(&self, c: Constraints) -> Size {
        let size = if self.horizontal {
            let w = self.options.iter().fold(0u16, |acc, o| acc.saturating_add(display_width(o).saturating_add(6)));
            Size::new(w, 1)
        } else {
            let w = self.options.iter().map(|o| display_width(o).saturating_add(4)).max().unwrap_or(0);
            Size::new(w, self.options.len().min(u16::MAX as usize) as u16)
        };
        c.clamp(size)
    }

    fn paint(&self, cx: &mut Cx, area: Area, canvas: &mut Canvas) {
        let focused = cx.will_focus();
        let selected = self.selected.get().min(self.options.len().saturating_sub(1));

        let saved_clip = canvas.push_clip(area);
        if self.horizontal {
            let mut x = area.x;
            for i in 0..self.options.len() {
                if x >= area.x.saturating_add(area.w) {
                    break;
                }
                let text = self.option_text(i, selected);
                let hl: Option<Rc<str>> = (focused && i == selected).then(|| Rc::from(groups::SELECTION));
                x = x.saturating_add(canvas.put_str(x, area.y, &text, hl)).saturating_add(2);
            }
        } else {
            // Scroll so the selected option stays visible when they don't all fit.
            let h = area.h as usize;
            let top = if h == 0 { 0 } else { selected.saturating_sub(h - 1) };
            for (row, i) in (top..self.options.len().min(top + h)).enumerate() {
                let text = self.option_text(i, selected);
                let hl: Option<Rc<str>> = (focused && i == selected).then(|| Rc::from(groups::SELECTION));
                canvas.put_str(area.x, area.y + row as u16, &text, hl);
            }
        }
        canvas.restore_clip(saved_clip);

        let sig = self.selected.clone();
        let on_change = self.on_change.clone();
        let n = self.options.len();
        let horizontal = self.horizontal;
        cx.register(
            area,
            Rc::new(move |k| {
                if n == 0 {
                    return false;
                }
                let cur = sig.get_untracked().min(n - 1);
                let next = match (k, horizontal) {
                    (Key::Char('j') | Key::Down, false) | (Key::Char('l') | Key::Right, true) => (cur + 1) % n,
                    (Key::Char('k') | Key::Up, false) | (Key::Char('h') | Key::Left, true) => (cur + n - 1) % n,
                    _ => return false,
                };
                sig.set(next);
                on_change(next);
                true
            }),
        );
    }
}
