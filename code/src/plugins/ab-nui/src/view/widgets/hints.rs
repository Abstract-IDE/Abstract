//! `KeyHints` — the ubiquitous "Tab focus · CR select · q quit" footer line.

use crate::text::{Line, Span, Wrap};
use crate::theme::groups;
use crate::view::canvas::Canvas;
use crate::view::widget::{Area, Constraints, Cx, Size, Widget};
use crate::view::widgets::text::Text;

/// A row of `key label` pairs separated by `·`, keys accented, labels muted.
/// Wraps onto extra rows when the area is narrow.
pub struct KeyHints {
    text: Text,
}

impl KeyHints {
    pub fn new(pairs: Vec<(&str, &str)>) -> Self {
        let mut line = Line::empty();
        for (i, (key, label)) in pairs.iter().enumerate() {
            if i > 0 {
                line.spans.push(Span::hl(" · ", groups::MUTED));
            }
            line.spans.push(Span::hl(key.to_string(), groups::ACCENT));
            line.spans.push(Span::hl(format!(" {label}"), groups::MUTED));
        }
        Self { text: Text::from_line(line).wrap(Wrap::Word) }
    }
}

impl Widget for KeyHints {
    fn measure(&self, c: Constraints) -> Size {
        self.text.measure(c)
    }
    fn paint(&self, cx: &mut Cx, area: Area, canvas: &mut Canvas) {
        self.text.paint(cx, area, canvas);
    }
}
