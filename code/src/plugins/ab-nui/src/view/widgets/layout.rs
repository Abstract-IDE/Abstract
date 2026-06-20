//! Layout widgets: stacking, spacing, alignment, and boxed containers.
//! Each is just a `Widget` that arranges children — nothing privileged.

use std::rc::Rc;

use crate::view::canvas::Canvas;
use crate::view::widget::{Alignment, Area, Constraints, CrossAxis, Cx, Element, MainAxis, Size, Widget};

// -----------------------------------------------------------------------------
// Column / Row
// -----------------------------------------------------------------------------

/// Vertical stack of children.
pub struct Column {
    children: Vec<Element>,
    main: MainAxis,
    cross: CrossAxis,
}

/// Horizontal stack of children.
pub struct Row {
    children: Vec<Element>,
    main: MainAxis,
    cross: CrossAxis,
}

macro_rules! stack_builders {
    ($t:ty) => {
        impl $t {
            pub fn new(children: Vec<Element>) -> Self {
                Self { children, main: MainAxis::Start, cross: CrossAxis::Start }
            }
            /// Distribution along the layout axis.
            pub fn main(mut self, m: MainAxis) -> Self {
                self.main = m;
                self
            }
            /// Alignment perpendicular to the layout axis.
            pub fn cross(mut self, c: CrossAxis) -> Self {
                self.cross = c;
                self
            }
        }
    };
}
stack_builders!(Column);
stack_builders!(Row);

/// Shared flex-distribution: returns each child's extent along the main axis.
fn distribute(extents: &[u16], flexes: &[u16], avail: u16) -> Vec<u16> {
    let fixed: u16 = extents.iter().zip(flexes).filter(|(_, f)| **f == 0).map(|(e, _)| *e).sum();
    let total_flex: u16 = flexes.iter().sum();
    let leftover = avail.saturating_sub(fixed);
    extents
        .iter()
        .zip(flexes)
        .map(|(e, f)| if *f > 0 && total_flex > 0 { leftover * f / total_flex } else { *e })
        .collect()
}

/// Leading offset + inter-child gap for a main-axis alignment.
fn main_offset(main: MainAxis, avail: u16, used: u16, n: usize) -> (u16, u16) {
    match main {
        MainAxis::Start => (0, 0),
        MainAxis::Center => (avail.saturating_sub(used) / 2, 0),
        MainAxis::End => (avail.saturating_sub(used), 0),
        MainAxis::SpaceBetween if n > 1 => (0, avail.saturating_sub(used) / (n as u16 - 1)),
        MainAxis::SpaceBetween => (avail.saturating_sub(used) / 2, 0),
    }
}

impl Widget for Column {
    fn measure(&self, c: Constraints) -> Size {
        let mut w: u16 = 0;
        let mut h: u16 = 0;
        for ch in &self.children {
            let s = ch.measure(c);
            w = w.max(s.w);
            h = h.saturating_add(s.h);
        }
        Size::new(w.min(c.max_w), h.min(c.max_h))
    }

    fn paint(&self, cx: &mut Cx, area: Area, canvas: &mut Canvas) {
        let extents: Vec<u16> = self.children.iter().map(|c| c.measure(area.constraints()).h).collect();
        let flexes: Vec<u16> = self.children.iter().map(|c| c.flex()).collect();
        let heights = distribute(&extents, &flexes, area.h);
        let used: u16 = heights.iter().sum();
        let (lead, gap) = main_offset(self.main, area.h, used, self.children.len());

        let mut y = area.y + lead;
        for (i, ch) in self.children.iter().enumerate() {
            let s = ch.measure(Constraints { max_w: area.w, max_h: heights[i] });
            let cw = if self.cross == CrossAxis::Stretch { area.w } else { s.w.min(area.w) };
            let dx = cross_offset(self.cross, area.w, cw);
            let child = Area { x: area.x + dx, y, w: cw, h: heights[i] };
            ch.paint(cx, child, canvas);
            y = y.saturating_add(heights[i]).saturating_add(gap);
        }
    }
}

impl Widget for Row {
    fn measure(&self, c: Constraints) -> Size {
        let mut w: u16 = 0;
        let mut h: u16 = 0;
        for ch in &self.children {
            let s = ch.measure(c);
            w = w.saturating_add(s.w);
            h = h.max(s.h);
        }
        Size::new(w.min(c.max_w), h.min(c.max_h))
    }

    fn paint(&self, cx: &mut Cx, area: Area, canvas: &mut Canvas) {
        let extents: Vec<u16> = self.children.iter().map(|c| c.measure(area.constraints()).w).collect();
        let flexes: Vec<u16> = self.children.iter().map(|c| c.flex()).collect();
        let widths = distribute(&extents, &flexes, area.w);
        let used: u16 = widths.iter().sum();
        let (lead, gap) = main_offset(self.main, area.w, used, self.children.len());

        let mut x = area.x + lead;
        for (i, ch) in self.children.iter().enumerate() {
            let s = ch.measure(Constraints { max_w: widths[i], max_h: area.h });
            let chh = if self.cross == CrossAxis::Stretch { area.h } else { s.h.min(area.h) };
            let dy = cross_offset(self.cross, area.h, chh);
            let child = Area { x, y: area.y + dy, w: widths[i], h: chh };
            ch.paint(cx, child, canvas);
            x = x.saturating_add(widths[i]).saturating_add(gap);
        }
    }
}

fn cross_offset(cross: CrossAxis, avail: u16, size: u16) -> u16 {
    match cross {
        CrossAxis::Start | CrossAxis::Stretch => 0,
        CrossAxis::Center => avail.saturating_sub(size) / 2,
        CrossAxis::End => avail.saturating_sub(size),
    }
}

// -----------------------------------------------------------------------------
// Spacer / Expanded / SizedBox
// -----------------------------------------------------------------------------

/// Flexible empty space that pushes siblings apart.
pub struct Spacer {
    weight: u16,
}

impl Spacer {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { weight: 1 }
    }
    pub fn weight(mut self, w: u16) -> Self {
        self.weight = w;
        self
    }
}

impl Widget for Spacer {
    fn measure(&self, _c: Constraints) -> Size {
        Size::new(0, 0)
    }
    fn paint(&self, _cx: &mut Cx, _area: Area, _canvas: &mut Canvas) {}
    fn flex(&self) -> u16 {
        self.weight
    }
}

/// Wraps a child to greedily fill the parent's main axis.
pub struct Expanded {
    child: Element,
    weight: u16,
}

impl Expanded {
    pub fn new(child: impl Widget + 'static) -> Self {
        Self { child: Box::new(child), weight: 1 }
    }
    pub fn weight(mut self, w: u16) -> Self {
        self.weight = w;
        self
    }
}

impl Widget for Expanded {
    fn measure(&self, c: Constraints) -> Size {
        self.child.measure(c)
    }
    fn paint(&self, cx: &mut Cx, area: Area, canvas: &mut Canvas) {
        self.child.paint(cx, area, canvas);
    }
    fn flex(&self) -> u16 {
        self.weight
    }
}

/// A fixed-size box, optionally wrapping a child. Handy as spacing.
pub struct SizedBox {
    w: u16,
    h: u16,
    child: Option<Element>,
}

impl SizedBox {
    pub fn new(w: u16, h: u16) -> Self {
        Self { w, h, child: None }
    }
    /// Horizontal gap.
    pub fn w(w: u16) -> Self {
        Self { w, h: 0, child: None }
    }
    /// Vertical gap.
    pub fn h(h: u16) -> Self {
        Self { w: 0, h, child: None }
    }
    pub fn child(mut self, child: impl Widget + 'static) -> Self {
        self.child = Some(Box::new(child));
        self
    }
}

impl Widget for SizedBox {
    fn measure(&self, c: Constraints) -> Size {
        Size::new(self.w.min(c.max_w), self.h.min(c.max_h))
    }
    fn paint(&self, cx: &mut Cx, area: Area, canvas: &mut Canvas) {
        if let Some(child) = &self.child {
            child.paint(cx, area, canvas);
        }
    }
}

// -----------------------------------------------------------------------------
// Center / Align / Padding
// -----------------------------------------------------------------------------

/// Centers its child within the available area (greedy: fills the parent).
pub struct Center {
    child: Element,
}

impl Center {
    pub fn new(child: impl Widget + 'static) -> Self {
        Self { child: Box::new(child) }
    }
}

impl Widget for Center {
    fn measure(&self, c: Constraints) -> Size {
        Size::new(c.max_w, c.max_h)
    }
    fn paint(&self, cx: &mut Cx, area: Area, canvas: &mut Canvas) {
        let s = self.child.measure(area.constraints());
        let (dx, dy) = Alignment::Center.offset(area.w, area.h, s.w, s.h);
        let child = Area { x: area.x + dx, y: area.y + dy, w: s.w, h: s.h };
        self.child.paint(cx, child, canvas);
    }
}

/// Aligns its child within the available area (greedy: fills the parent).
pub struct Align {
    alignment: Alignment,
    child: Element,
}

impl Align {
    pub fn new(alignment: Alignment, child: impl Widget + 'static) -> Self {
        Self { alignment, child: Box::new(child) }
    }
}

impl Widget for Align {
    fn measure(&self, c: Constraints) -> Size {
        Size::new(c.max_w, c.max_h)
    }
    fn paint(&self, cx: &mut Cx, area: Area, canvas: &mut Canvas) {
        let s = self.child.measure(area.constraints());
        let (dx, dy) = self.alignment.offset(area.w, area.h, s.w, s.h);
        let child = Area { x: area.x + dx, y: area.y + dy, w: s.w, h: s.h };
        self.child.paint(cx, child, canvas);
    }
}

/// Insets a child by per-side padding.
pub struct Padding {
    l: u16,
    t: u16,
    r: u16,
    b: u16,
    child: Element,
}

impl Padding {
    pub fn all(n: u16, child: impl Widget + 'static) -> Self {
        Self { l: n, t: n, r: n, b: n, child: Box::new(child) }
    }
    pub fn symmetric(h: u16, v: u16, child: impl Widget + 'static) -> Self {
        Self { l: h, t: v, r: h, b: v, child: Box::new(child) }
    }
}

impl Widget for Padding {
    fn measure(&self, c: Constraints) -> Size {
        let inner = Constraints {
            max_w: c.max_w.saturating_sub(self.l + self.r),
            max_h: c.max_h.saturating_sub(self.t + self.b),
        };
        let s = self.child.measure(inner);
        Size::new(s.w + self.l + self.r, s.h + self.t + self.b)
    }
    fn paint(&self, cx: &mut Cx, area: Area, canvas: &mut Canvas) {
        self.child.paint(cx, area.shrink(self.l, self.t, self.r, self.b), canvas);
    }
}

// -----------------------------------------------------------------------------
// Divider
// -----------------------------------------------------------------------------

/// A horizontal rule that spans the available width.
pub struct Divider {
    hl: Option<Rc<str>>,
}

impl Divider {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { hl: Some(Rc::from(crate::theme::groups::BORDER)) }
    }
}

impl Widget for Divider {
    fn measure(&self, c: Constraints) -> Size {
        Size::new(c.max_w, 1)
    }
    fn paint(&self, _cx: &mut Cx, area: Area, canvas: &mut Canvas) {
        let line: String = "─".repeat(area.w as usize);
        canvas.put_str(area.x, area.y, &line, self.hl.clone());
    }
}

// -----------------------------------------------------------------------------
// Container — a bordered, titled box around a child
// -----------------------------------------------------------------------------

/// Box-drawing style for a [`Container`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BoxStyle {
    None,
    Single,
    #[default]
    Rounded,
}

impl BoxStyle {
    /// `(top_left, top_right, bottom_left, bottom_right, horizontal, vertical)`.
    fn chars(self) -> Option<(char, char, char, char, char, char)> {
        match self {
            BoxStyle::None => None,
            BoxStyle::Single => Some(('┌', '┐', '└', '┘', '─', '│')),
            BoxStyle::Rounded => Some(('╭', '╮', '╰', '╯', '─', '│')),
        }
    }
}

/// A box with an optional border, title, and inner padding.
pub struct Container {
    child: Option<Element>,
    style: BoxStyle,
    title: Option<String>,
    pad: u16,
    border_hl: Option<Rc<str>>,
    title_hl: Option<Rc<str>>,
}

impl Container {
    pub fn new(child: impl Widget + 'static) -> Self {
        Self {
            child: Some(Box::new(child)),
            style: BoxStyle::Rounded,
            title: None,
            pad: 0,
            border_hl: Some(Rc::from(crate::theme::groups::BORDER)),
            title_hl: Some(Rc::from(crate::theme::groups::TITLE)),
        }
    }
    /// An empty bordered box.
    pub fn empty() -> Self {
        Self {
            child: None,
            style: BoxStyle::Rounded,
            title: None,
            pad: 0,
            border_hl: Some(Rc::from(crate::theme::groups::BORDER)),
            title_hl: Some(Rc::from(crate::theme::groups::TITLE)),
        }
    }
    pub fn border(mut self, style: BoxStyle) -> Self {
        self.style = style;
        self
    }
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
    pub fn padding(mut self, pad: u16) -> Self {
        self.pad = pad;
        self
    }

    fn border_inset(&self) -> u16 {
        if self.style == BoxStyle::None { 0 } else { 1 }
    }

    fn inner(&self, area: Area) -> Area {
        let b = self.border_inset();
        area.shrink(b + self.pad, b + self.pad, b + self.pad, b + self.pad)
    }
}

impl Widget for Container {
    fn measure(&self, c: Constraints) -> Size {
        let extra = (self.border_inset() + self.pad) * 2;
        let inner = Constraints { max_w: c.max_w.saturating_sub(extra), max_h: c.max_h.saturating_sub(extra) };
        let cs = self.child.as_ref().map(|c| c.measure(inner)).unwrap_or_default();
        Size::new((cs.w + extra).min(c.max_w), (cs.h + extra).min(c.max_h))
    }

    fn paint(&self, cx: &mut Cx, area: Area, canvas: &mut Canvas) {
        if let Some((tl, tr, bl, br, h, v)) = self.style.chars()
            && area.w >= 2
            && area.h >= 2
        {
            let x0 = area.x;
            let y0 = area.y;
            let x1 = area.x + area.w - 1;
            let y1 = area.y + area.h - 1;
            for x in x0..=x1 {
                canvas.set(x, y0, h, self.border_hl.clone());
                canvas.set(x, y1, h, self.border_hl.clone());
            }
            for y in y0..=y1 {
                canvas.set(x0, y, v, self.border_hl.clone());
                canvas.set(x1, y, v, self.border_hl.clone());
            }
            canvas.set(x0, y0, tl, self.border_hl.clone());
            canvas.set(x1, y0, tr, self.border_hl.clone());
            canvas.set(x0, y1, bl, self.border_hl.clone());
            canvas.set(x1, y1, br, self.border_hl.clone());

            if let Some(title) = &self.title {
                let label = format!(" {} ", title.trim());
                let tw = label.chars().count() as u16;
                if tw + 2 <= area.w {
                    let tx = x0 + (area.w.saturating_sub(tw)) / 2;
                    canvas.put_str(tx, y0, &label, self.title_hl.clone());
                }
            }
        }

        if let Some(child) = &self.child {
            child.paint(cx, self.inner(area), canvas);
        }
    }
}
