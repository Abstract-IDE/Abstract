//! `Text` — a styled string. The leaf of most trees.

use std::rc::Rc;

use crate::view::canvas::Canvas;
use crate::view::widget::{Area, Constraints, Cx, Size, Widget};

pub struct Text {
    text: String,
    hl: Option<Rc<str>>,
}

impl Text {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into(), hl: None }
    }

    /// Paint with the given highlight group (e.g. `"Title"`, `"Comment"`).
    pub fn fg(mut self, group: &str) -> Self {
        self.hl = Some(Rc::from(group));
        self
    }
}

impl Widget for Text {
    fn measure(&self, c: Constraints) -> Size {
        let w = (self.text.chars().count() as u16).min(c.max_w);
        Size::new(w, 1.min(c.max_h))
    }

    fn paint(&self, _cx: &mut Cx, area: Area, canvas: &mut Canvas) {
        canvas.put_str(area.x, area.y, &self.text, self.hl.clone());
    }
}
