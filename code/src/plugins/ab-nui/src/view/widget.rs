//! The `Widget` trait and the layout/interaction plumbing every widget shares.
//!
//! Everything in the view layer is a [`Widget`]: text, layout, spacing,
//! buttons, lists — all the same trait, all composable. Widgets are laid out
//! with a two-phase pass ([`Widget::measure`] then [`Widget::paint`]) and
//! register their interactions during paint via the [`Cx`] context.

use std::rc::Rc;

use crate::view::canvas::Canvas;

/// A measured size in cells.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Size {
    pub w: u16,
    pub h: u16,
}

impl Size {
    pub fn new(w: u16, h: u16) -> Self {
        Self { w, h }
    }
}

/// Upper bounds a widget must lay out within.
#[derive(Clone, Copy, Debug)]
pub struct Constraints {
    pub max_w: u16,
    pub max_h: u16,
}

/// A placed rectangle in canvas cells.
#[derive(Clone, Copy, Debug, Default)]
pub struct Area {
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16,
}

impl Area {
    /// Shrink on every side by the given insets.
    pub fn shrink(self, left: u16, top: u16, right: u16, bottom: u16) -> Area {
        Area {
            x: self.x.saturating_add(left),
            y: self.y.saturating_add(top),
            w: self.w.saturating_sub(left + right),
            h: self.h.saturating_sub(top + bottom),
        }
    }

    pub fn constraints(self) -> Constraints {
        Constraints { max_w: self.w, max_h: self.h }
    }

    pub fn contains(self, x: u16, y: u16) -> bool {
        x >= self.x && x < self.x + self.w && y >= self.y && y < self.y + self.h
    }
}

/// A key event routed to the focused widget.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Key {
    Char(char),
    Enter,
    Backspace,
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    Esc,
}

/// Main-axis (along the layout direction) distribution.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MainAxis {
    #[default]
    Start,
    Center,
    End,
    SpaceBetween,
}

/// Cross-axis (perpendicular) alignment.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CrossAxis {
    #[default]
    Start,
    Center,
    End,
    Stretch,
}

/// 2D alignment for [`crate::view::Align`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alignment {
    TopLeft,
    Top,
    TopRight,
    Left,
    Center,
    Right,
    BottomLeft,
    Bottom,
    BottomRight,
}

impl Alignment {
    /// Offset to place a `child` of size `(cw, ch)` inside `(w, h)`.
    pub fn offset(self, w: u16, h: u16, cw: u16, ch: u16) -> (u16, u16) {
        let dx = w.saturating_sub(cw);
        let dy = h.saturating_sub(ch);
        let (fx, fy): (f32, f32) = match self {
            Alignment::TopLeft => (0.0, 0.0),
            Alignment::Top => (0.5, 0.0),
            Alignment::TopRight => (1.0, 0.0),
            Alignment::Left => (0.0, 0.5),
            Alignment::Center => (0.5, 0.5),
            Alignment::Right => (1.0, 0.5),
            Alignment::BottomLeft => (0.0, 1.0),
            Alignment::Bottom => (0.5, 1.0),
            Alignment::BottomRight => (1.0, 1.0),
        };
        ((dx as f32 * fx) as u16, (dy as f32 * fy) as u16)
    }
}

/// A registered interactive region: where it is and how it handles keys.
pub struct Focusable {
    pub area: Area,
    pub handle: Rc<dyn Fn(Key) -> bool>,
}

/// Paint-time context: tracks focus and collects interactions.
///
/// A widget that wants to be interactive calls [`Cx::will_focus`] (to style
/// itself) and [`Cx::register`] (to receive key events when focused). Focus ids
/// are assigned by registration order, which is stable across rebuilds.
pub struct Cx {
    pub focused: usize,
    pub focusables: Vec<Focusable>,
}

impl Cx {
    pub fn new(focused: usize) -> Self {
        Self { focused, focusables: Vec::new() }
    }

    /// Whether the *next* widget to register will be the focused one.
    pub fn will_focus(&self) -> bool {
        self.focusables.len() == self.focused
    }

    /// Register an interactive region; returns its focus id.
    pub fn register(&mut self, area: Area, handle: Rc<dyn Fn(Key) -> bool>) -> usize {
        let id = self.focusables.len();
        self.focusables.push(Focusable { area, handle });
        id
    }
}

/// The one trait everything in the view layer implements.
pub trait Widget {
    /// Desired size within `c` (no side effects).
    fn measure(&self, c: Constraints) -> Size;

    /// Draw into `area` and register any interactions on `cx`.
    fn paint(&self, cx: &mut Cx, area: Area, canvas: &mut Canvas);

    /// Flex weight along a parent's main axis (0 = fixed size).
    fn flex(&self) -> u16 {
        0
    }
}

/// A boxed widget — the type stored as a child in the tree.
pub type Element = Box<dyn Widget>;

// Calling trait methods on `Element` (Box<dyn Widget>) works through deref, so
// no explicit impl is needed.
