//! `Button` — pressable label. The pattern source for interactive widgets:
//! builder + `Rc<dyn Fn>` callbacks + `will_focus` styling + `register`.

use std::rc::Rc;

use crate::theme::groups;
use crate::view::canvas::Canvas;
use crate::view::widget::{Area, Constraints, Cx, Key, Size, Widget};

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
        let w = (crate::text::display_width(&self.label) + 4).min(c.max_w);
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
