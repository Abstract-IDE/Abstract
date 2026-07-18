//! `Keyed` — attach a stable focus key and/or custom key handlers to ANY
//! interactive widget, without every widget needing its own builders.
//!
//! ```ignore
//! Keyed::new(Button::new("Save").on_press(save))
//!     .focus_key("save-btn")            // focus survives tree-shape changes
//!     .on_key(Key::Char('s'), save2)    // checked before the button's own keys
//! ```

use std::rc::Rc;

use crate::view::canvas::Canvas;
use crate::view::widget::{Area, Constraints, Cx, Element, Key, Size, Widget};

/// Wraps a child; every focusable the child registers gets this key and these
/// extra handlers (checked before the child's own handling).
pub struct Keyed {
    key: Option<Rc<str>>,
    map: Vec<(Key, Rc<dyn Fn()>)>,
    child: Element,
}

impl Keyed {
    pub fn new(child: impl Widget + 'static) -> Self {
        Self { key: None, map: Vec::new(), child: Box::new(child) }
    }

    /// A stable focus identity: after a rebuild changes the tree's shape, focus
    /// re-attaches to the widget with the same key instead of the same index.
    pub fn focus_key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(Rc::from(key.into().as_str()));
        self
    }

    /// Run `f` when `key` is pressed while the child is focused (consumes the
    /// key; checked before the child's own handler).
    pub fn on_key(mut self, key: Key, f: impl Fn() + 'static) -> Self {
        self.map.push((key, Rc::new(f)));
        self
    }
}

impl Widget for Keyed {
    fn measure(&self, c: Constraints) -> Size {
        self.child.measure(c)
    }

    fn flex(&self) -> u16 {
        self.child.flex()
    }

    fn paint(&self, cx: &mut Cx, area: Area, canvas: &mut Canvas) {
        let before = cx.focusables.len();
        self.child.paint(cx, area, canvas);
        for f in &mut cx.focusables[before..] {
            if let Some(key) = &self.key {
                f.key = Some(key.clone());
            }
            if !self.map.is_empty() {
                let map = self.map.clone();
                let inner = f.handle.clone();
                f.handle = Rc::new(move |k| {
                    for (mk, mf) in &map {
                        if *mk == k {
                            mf();
                            return true;
                        }
                    }
                    inner(k)
                });
            }
        }
    }
}
