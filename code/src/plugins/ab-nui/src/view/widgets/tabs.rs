//! `TabBar` — a horizontal tab strip. Content switching is up to the build
//! closure: `match active.get() { 0 => page_a(), 1 => page_b(), .. }`.

use std::rc::Rc;

use crate::reactive::Signal;
use crate::text::display_width;
use crate::theme::groups;
use crate::view::canvas::Canvas;
use crate::view::widget::{Area, Constraints, Cx, Key, Size, Widget};

/// ` tab1 │ tab2 │ tab3 ` — `h`/`l` (or arrows) switch while focused.
pub struct TabBar {
    labels: Vec<String>,
    active: Signal<usize>,
    on_change: Rc<dyn Fn(usize)>,
}

impl TabBar {
    pub fn new(labels: Vec<String>, active: Signal<usize>) -> Self {
        Self { labels, active, on_change: Rc::new(|_| {}) }
    }
    pub fn on_change(mut self, f: impl Fn(usize) + 'static) -> Self {
        self.on_change = Rc::new(f);
        self
    }
}

impl Widget for TabBar {
    fn measure(&self, c: Constraints) -> Size {
        let w: u16 = self
            .labels
            .iter()
            .map(|l| display_width(l) + 2)
            .sum::<u16>()
            .saturating_add(self.labels.len().saturating_sub(1) as u16);
        Size::new(w.min(c.max_w), 1)
    }

    fn paint(&self, cx: &mut Cx, area: Area, canvas: &mut Canvas) {
        let focused = cx.will_focus();
        let n = self.labels.len();
        let active = self.active.get().min(n.saturating_sub(1));

        let mut x = area.x;
        for (i, label) in self.labels.iter().enumerate() {
            if i > 0 {
                x += canvas.put_str(x, area.y, "│", Some(Rc::from(groups::BORDER)));
            }
            let hl = if i == active {
                if focused { groups::SELECTION } else { groups::TAB_ACTIVE }
            } else {
                groups::TAB_INACTIVE
            };
            x += canvas.put_str(x, area.y, &format!(" {label} "), Some(Rc::from(hl)));
            if x >= area.x + area.w {
                break;
            }
        }

        let sig = self.active.clone();
        let on_change = self.on_change.clone();
        cx.register(
            area,
            Rc::new(move |k| {
                if n == 0 {
                    return false;
                }
                let cur = sig.get_untracked().min(n - 1);
                let next = match k {
                    Key::Char('l') | Key::Right => (cur + 1) % n,
                    Key::Char('h') | Key::Left => (cur + n - 1) % n,
                    _ => return false,
                };
                sig.set(next);
                on_change(next);
                true
            }),
        );
    }
}
