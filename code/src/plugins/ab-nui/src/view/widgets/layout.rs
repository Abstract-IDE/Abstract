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
            let min_w = if self.cross == CrossAxis::Stretch { area.w } else { 0 };
            let s = ch.measure(Constraints { min_w, min_h: 0, max_w: area.w, max_h: heights[i] });
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
            let min_h = if self.cross == CrossAxis::Stretch { area.h } else { 0 };
            let s = ch.measure(Constraints { min_w: 0, min_h, max_w: widths[i], max_h: area.h });
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
    /// Independent per-side insets (left, top, right, bottom).
    pub fn only(l: u16, t: u16, r: u16, b: u16, child: impl Widget + 'static) -> Self {
        Self { l, t, r, b, child: Box::new(child) }
    }
    pub fn left(n: u16, child: impl Widget + 'static) -> Self {
        Self::only(n, 0, 0, 0, child)
    }
    pub fn top(n: u16, child: impl Widget + 'static) -> Self {
        Self::only(0, n, 0, 0, child)
    }
    pub fn right(n: u16, child: impl Widget + 'static) -> Self {
        Self::only(0, 0, n, 0, child)
    }
    pub fn bottom(n: u16, child: impl Widget + 'static) -> Self {
        Self::only(0, 0, 0, n, child)
    }
}

impl Widget for Padding {
    fn measure(&self, c: Constraints) -> Size {
        let inner = Constraints::loose(
            c.max_w.saturating_sub(self.l + self.r),
            c.max_h.saturating_sub(self.t + self.b),
        );
        let s = self.child.measure(inner);
        Size::new(s.w + self.l + self.r, s.h + self.t + self.b)
    }
    fn paint(&self, cx: &mut Cx, area: Area, canvas: &mut Canvas) {
        self.child.paint(cx, area.shrink(self.l, self.t, self.r, self.b), canvas);
    }
}

// -----------------------------------------------------------------------------
// Stack / Positioned — z-layered overlay within one area
// -----------------------------------------------------------------------------

/// Layers children on top of each other; later children paint over earlier
/// ones (paint order = z-order). Children are placed by `align` unless wrapped
/// in [`Positioned`].
pub struct Stack {
    children: Vec<Element>,
    align: Alignment,
}

impl Stack {
    pub fn new(children: Vec<Element>) -> Self {
        Self { children, align: Alignment::TopLeft }
    }
    pub fn align(mut self, align: Alignment) -> Self {
        self.align = align;
        self
    }
}

impl Widget for Stack {
    fn measure(&self, c: Constraints) -> Size {
        let mut w: u16 = 0;
        let mut h: u16 = 0;
        for ch in &self.children {
            let s = ch.measure(c);
            w = w.max(s.w);
            h = h.max(s.h);
        }
        c.clamp(Size::new(w, h))
    }

    fn paint(&self, cx: &mut Cx, area: Area, canvas: &mut Canvas) {
        for ch in &self.children {
            let s = ch.measure(area.constraints());
            let (dx, dy) = self.align.offset(area.w, area.h, s.w, s.h);
            let child = Area { x: area.x + dx, y: area.y + dy, w: s.w.min(area.w), h: s.h.min(area.h) };
            ch.paint(cx, child, canvas);
        }
    }
}

/// Offsets its child within a [`Stack`] (or any parent area) by `(x, y)`.
pub struct Positioned {
    x: u16,
    y: u16,
    child: Element,
}

impl Positioned {
    pub fn new(x: u16, y: u16, child: impl Widget + 'static) -> Self {
        Self { x, y, child: Box::new(child) }
    }
}

impl Widget for Positioned {
    fn measure(&self, c: Constraints) -> Size {
        let s = self.child.measure(c);
        c.clamp(Size::new(s.w.saturating_add(self.x), s.h.saturating_add(self.y)))
    }
    fn paint(&self, cx: &mut Cx, area: Area, canvas: &mut Canvas) {
        let inner = area.shrink(self.x, self.y, 0, 0);
        let s = self.child.measure(inner.constraints());
        let child = Area { x: inner.x, y: inner.y, w: s.w.min(inner.w), h: s.h.min(inner.h) };
        self.child.paint(cx, child, canvas);
    }
}

// -----------------------------------------------------------------------------
// Fraction — percentage-of-parent sizing
// -----------------------------------------------------------------------------

/// Sizes its child as a fraction of the parent's constraints (e.g. `0.5` = half
/// the available width/height). An axis without a fraction takes the child's
/// own measured size.
pub struct Fraction {
    fw: Option<f32>,
    fh: Option<f32>,
    child: Element,
}

impl Fraction {
    pub fn new(fw: f32, fh: f32, child: impl Widget + 'static) -> Self {
        Self { fw: Some(fw), fh: Some(fh), child: Box::new(child) }
    }
    /// Fraction of the available width; height from the child.
    pub fn w(fw: f32, child: impl Widget + 'static) -> Self {
        Self { fw: Some(fw), fh: None, child: Box::new(child) }
    }
    /// Fraction of the available height; width from the child.
    pub fn h(fh: f32, child: impl Widget + 'static) -> Self {
        Self { fw: None, fh: None, child: Box::new(child) }.set_fh(fh)
    }

    fn set_fh(mut self, fh: f32) -> Self {
        self.fh = Some(fh);
        self
    }

    fn resolve(&self, c: Constraints) -> Size {
        let frac = |f: f32, max: u16| ((max as f32) * f.clamp(0.0, 1.0)).round() as u16;
        let w = self.fw.map(|f| frac(f, c.max_w));
        let h = self.fh.map(|f| frac(f, c.max_h));
        let inner = Constraints::loose(w.unwrap_or(c.max_w), h.unwrap_or(c.max_h));
        let s = self.child.measure(inner);
        Size::new(w.unwrap_or(s.w), h.unwrap_or(s.h))
    }
}

impl Widget for Fraction {
    fn measure(&self, c: Constraints) -> Size {
        c.clamp(self.resolve(c))
    }
    fn paint(&self, cx: &mut Cx, area: Area, canvas: &mut Canvas) {
        self.child.paint(cx, area, canvas);
    }
}

// -----------------------------------------------------------------------------
// Divider
// -----------------------------------------------------------------------------

/// Which way a [`Divider`] runs.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum DividerDir {
    #[default]
    Horizontal,
    Vertical,
}

/// A rule spanning the available width (or height when vertical).
pub struct Divider {
    dir: DividerDir,
    ch: char,
    hl: Option<Rc<str>>,
}

impl Divider {
    /// A horizontal rule (`─`) across the available width.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { dir: DividerDir::Horizontal, ch: '─', hl: Some(Rc::from(crate::theme::groups::BORDER)) }
    }
    /// A vertical rule (`│`) down the available height.
    pub fn vertical() -> Self {
        Self { dir: DividerDir::Vertical, ch: '│', hl: Some(Rc::from(crate::theme::groups::BORDER)) }
    }
    /// Use a custom rule character.
    pub fn ch(mut self, ch: char) -> Self {
        self.ch = ch;
        self
    }
    /// Paint with the given highlight group.
    pub fn fg(mut self, group: &str) -> Self {
        self.hl = Some(Rc::from(group));
        self
    }
}

impl Widget for Divider {
    fn measure(&self, c: Constraints) -> Size {
        match self.dir {
            DividerDir::Horizontal => Size::new(c.max_w, 1),
            DividerDir::Vertical => Size::new(1, c.max_h),
        }
    }
    fn paint(&self, _cx: &mut Cx, area: Area, canvas: &mut Canvas) {
        match self.dir {
            DividerDir::Horizontal => {
                for x in area.x..area.x.saturating_add(area.w) {
                    canvas.set(x, area.y, self.ch, self.hl.clone());
                }
            }
            DividerDir::Vertical => {
                for y in area.y..area.y.saturating_add(area.h) {
                    canvas.set(area.x, y, self.ch, self.hl.clone());
                }
            }
        }
    }
}

// -----------------------------------------------------------------------------
// Container — a bordered, titled box around a child
// -----------------------------------------------------------------------------

/// The six characters a box border is drawn with.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BorderChars {
    pub tl: char,
    pub tr: char,
    pub bl: char,
    pub br: char,
    pub h: char,
    pub v: char,
}

/// Box-drawing style for a [`Container`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BoxStyle {
    None,
    Single,
    #[default]
    Rounded,
    Double,
    Thick,
    Custom(BorderChars),
}

impl BoxStyle {
    fn chars(self) -> Option<BorderChars> {
        match self {
            BoxStyle::None => None,
            BoxStyle::Single => {
                Some(BorderChars { tl: '┌', tr: '┐', bl: '└', br: '┘', h: '─', v: '│' })
            }
            BoxStyle::Rounded => {
                Some(BorderChars { tl: '╭', tr: '╮', bl: '╰', br: '╯', h: '─', v: '│' })
            }
            BoxStyle::Double => {
                Some(BorderChars { tl: '╔', tr: '╗', bl: '╚', br: '╝', h: '═', v: '║' })
            }
            BoxStyle::Thick => {
                Some(BorderChars { tl: '┏', tr: '┓', bl: '┗', br: '┛', h: '━', v: '┃' })
            }
            BoxStyle::Custom(chars) => Some(chars),
        }
    }
}

/// Where an edge label (title/footer) sits on its border row.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum EdgeAlign {
    Left,
    #[default]
    Center,
    Right,
}

/// A box with an optional border, title, footer, and inner padding.
pub struct Container {
    child: Option<Element>,
    style: BoxStyle,
    title: Option<String>,
    title_pos: EdgeAlign,
    footer: Option<String>,
    footer_pos: EdgeAlign,
    pad: (u16, u16, u16, u16), // l, t, r, b
    border_hl: Option<Rc<str>>,
    title_hl: Option<Rc<str>>,
}

impl Container {
    pub fn new(child: impl Widget + 'static) -> Self {
        Self { child: Some(Box::new(child)), ..Self::empty() }
    }
    /// An empty bordered box.
    pub fn empty() -> Self {
        Self {
            child: None,
            style: BoxStyle::Rounded,
            title: None,
            title_pos: EdgeAlign::Center,
            footer: None,
            footer_pos: EdgeAlign::Center,
            pad: (0, 0, 0, 0),
            border_hl: Some(Rc::from(crate::theme::groups::BORDER)),
            title_hl: Some(Rc::from(crate::theme::groups::TITLE)),
        }
    }
    pub fn border(mut self, style: BoxStyle) -> Self {
        self.style = style;
        self
    }
    /// Paint the border with the given highlight group.
    pub fn border_hl(mut self, group: &str) -> Self {
        self.border_hl = Some(Rc::from(group));
        self
    }
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
    /// Paint the title/footer with the given highlight group.
    pub fn title_hl(mut self, group: &str) -> Self {
        self.title_hl = Some(Rc::from(group));
        self
    }
    pub fn title_pos(mut self, pos: EdgeAlign) -> Self {
        self.title_pos = pos;
        self
    }
    /// A label on the bottom border.
    pub fn footer(mut self, footer: impl Into<String>) -> Self {
        self.footer = Some(footer.into());
        self
    }
    pub fn footer_pos(mut self, pos: EdgeAlign) -> Self {
        self.footer_pos = pos;
        self
    }
    pub fn padding(mut self, pad: u16) -> Self {
        self.pad = (pad, pad, pad, pad);
        self
    }
    /// Independent per-side inner padding (left, top, right, bottom).
    pub fn padding_only(mut self, l: u16, t: u16, r: u16, b: u16) -> Self {
        self.pad = (l, t, r, b);
        self
    }

    fn border_inset(&self) -> u16 {
        if self.style == BoxStyle::None { 0 } else { 1 }
    }

    fn inner(&self, area: Area) -> Area {
        let b = self.border_inset();
        let (l, t, r, bo) = self.pad;
        area.shrink(b + l, b + t, b + r, b + bo)
    }

    /// Paint `label` on border row `y`, aligned within the area's top/bottom edge.
    fn edge_label(&self, canvas: &mut Canvas, area: Area, y: u16, label: &str, pos: EdgeAlign) {
        let label = format!(" {} ", label.trim());
        let lw = crate::text::display_width(&label);
        if lw + 2 > area.w {
            return;
        }
        let x = match pos {
            EdgeAlign::Left => area.x + 1,
            EdgeAlign::Center => area.x + (area.w - lw) / 2,
            EdgeAlign::Right => area.x + area.w - lw - 1,
        };
        canvas.put_str(x, y, &label, self.title_hl.clone());
    }
}

impl Widget for Container {
    fn measure(&self, c: Constraints) -> Size {
        let b = self.border_inset();
        let (l, t, r, bo) = self.pad;
        let extra_w = b * 2 + l + r;
        let extra_h = b * 2 + t + bo;
        let inner = Constraints::loose(c.max_w.saturating_sub(extra_w), c.max_h.saturating_sub(extra_h));
        let cs = self.child.as_ref().map(|c| c.measure(inner)).unwrap_or_default();
        c.clamp(Size::new((cs.w + extra_w).min(c.max_w), (cs.h + extra_h).min(c.max_h)))
    }

    fn paint(&self, cx: &mut Cx, area: Area, canvas: &mut Canvas) {
        if let Some(chars) = self.style.chars()
            && area.w >= 2
            && area.h >= 2
        {
            let x0 = area.x;
            let y0 = area.y;
            let x1 = area.x + area.w - 1;
            let y1 = area.y + area.h - 1;
            for x in x0..=x1 {
                canvas.set(x, y0, chars.h, self.border_hl.clone());
                canvas.set(x, y1, chars.h, self.border_hl.clone());
            }
            for y in y0..=y1 {
                canvas.set(x0, y, chars.v, self.border_hl.clone());
                canvas.set(x1, y, chars.v, self.border_hl.clone());
            }
            canvas.set(x0, y0, chars.tl, self.border_hl.clone());
            canvas.set(x1, y0, chars.tr, self.border_hl.clone());
            canvas.set(x0, y1, chars.bl, self.border_hl.clone());
            canvas.set(x1, y1, chars.br, self.border_hl.clone());

            if let Some(title) = &self.title {
                self.edge_label(canvas, area, y0, title, self.title_pos);
            }
            if let Some(footer) = &self.footer {
                self.edge_label(canvas, area, y1, footer, self.footer_pos);
            }
        }

        if let Some(child) = &self.child {
            child.paint(cx, self.inner(area), canvas);
        }
    }
}
