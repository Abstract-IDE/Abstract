//! Layout widgets: stacking, spacing, alignment, and boxed containers.
//! Each is just a `Widget` that arranges children — nothing privileged.

use std::cell::RefCell;
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
///
/// Fixed children (`flex == 0`) keep their extent, in order, as long as space
/// remains — an allocation never exceeds what is left, so the total never
/// exceeds `avail`. Flex children share the rest by weight; the cells lost to
/// integer division go to the largest remainders, so flex children always
/// fill the leftover exactly. Arithmetic is widened, so greedy (`UNBOUNDED`)
/// extents and huge weights cannot overflow.
fn distribute(extents: &[u16], flexes: &[u16], avail: u16) -> Vec<u16> {
    let mut out = vec![0u16; extents.len()];
    let mut remaining = avail as u64;
    for (i, (e, f)) in extents.iter().zip(flexes).enumerate() {
        if *f == 0 {
            let take = (*e as u64).min(remaining);
            out[i] = take as u16;
            remaining -= take;
        }
    }
    let total_flex: u64 = flexes.iter().map(|f| *f as u64).sum();
    if total_flex == 0 {
        return out;
    }
    let leftover = remaining;
    let mut given = 0u64;
    let mut rems: Vec<(u64, usize)> = Vec::new();
    for (i, f) in flexes.iter().enumerate() {
        if *f > 0 {
            let share = leftover * *f as u64;
            out[i] = (share / total_flex) as u16;
            given += share / total_flex;
            rems.push((share % total_flex, i));
        }
    }
    // Largest remainder first; ties go to the earlier child.
    rems.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
    for (_, i) in rems.into_iter().take((leftover - given) as usize) {
        out[i] += 1;
    }
    out
}

/// Sum of extents, saturating (they never exceed the parent's extent once
/// distributed, but stay safe for any input).
fn total(extents: &[u16]) -> u16 {
    extents.iter().fold(0u16, |acc, e| acc.saturating_add(*e))
}

/// Leading offset + inter-child gap for a main-axis alignment.
fn main_offset(main: MainAxis, avail: u16, used: u16, n: usize) -> (u16, u16) {
    match main {
        MainAxis::Start => (0, 0),
        MainAxis::Center => (avail.saturating_sub(used) / 2, 0),
        MainAxis::End => (avail.saturating_sub(used), 0),
        MainAxis::SpaceBetween if n > 1 => (0, (avail.saturating_sub(used) as usize / (n - 1)) as u16),
        MainAxis::SpaceBetween => (avail.saturating_sub(used) / 2, 0),
    }
}

/// Paint `child` into `area`, confined to it: nothing it draws can land on a
/// sibling or the parent's decorations.
pub(crate) fn paint_clipped(child: &dyn Widget, cx: &mut Cx, area: Area, canvas: &mut Canvas) {
    canvas.with_clip(area, |canvas| child.paint(cx, area, canvas));
}

impl Widget for Column {
    fn measure(&self, c: Constraints) -> Size {
        let inner = Constraints::loose(c.max_w, c.max_h);
        let mut w: u16 = 0;
        let mut h: u16 = 0;
        for ch in &self.children {
            let s = ch.measure(inner);
            w = w.max(s.w);
            h = h.saturating_add(s.h);
        }
        c.clamp(Size::new(w, h))
    }

    fn paint(&self, cx: &mut Cx, area: Area, canvas: &mut Canvas) {
        let extents: Vec<u16> = self.children.iter().map(|c| c.measure(area.constraints()).h).collect();
        let flexes: Vec<u16> = self.children.iter().map(|c| c.flex()).collect();
        let heights = distribute(&extents, &flexes, area.h);
        let used = total(&heights);
        let (lead, gap) = main_offset(self.main, area.h, used, self.children.len());

        let mut y = area.y.saturating_add(lead);
        for (i, ch) in self.children.iter().enumerate() {
            let min_w = if self.cross == CrossAxis::Stretch { area.w } else { 0 };
            let s = ch.measure(Constraints { min_w, min_h: 0, max_w: area.w, max_h: heights[i] });
            let cw = if self.cross == CrossAxis::Stretch { area.w } else { s.w.min(area.w) };
            let dx = cross_offset(self.cross, area.w, cw);
            let child = Area { x: area.x.saturating_add(dx), y, w: cw, h: heights[i] };
            paint_clipped(ch.as_ref(), cx, child, canvas);
            y = y.saturating_add(heights[i]).saturating_add(gap);
        }
    }
}

impl Widget for Row {
    fn measure(&self, c: Constraints) -> Size {
        let inner = Constraints::loose(c.max_w, c.max_h);
        let mut w: u16 = 0;
        let mut h: u16 = 0;
        for ch in &self.children {
            let s = ch.measure(inner);
            w = w.saturating_add(s.w);
            h = h.max(s.h);
        }
        c.clamp(Size::new(w, h))
    }

    fn paint(&self, cx: &mut Cx, area: Area, canvas: &mut Canvas) {
        let extents: Vec<u16> = self.children.iter().map(|c| c.measure(area.constraints()).w).collect();
        let flexes: Vec<u16> = self.children.iter().map(|c| c.flex()).collect();
        let widths = distribute(&extents, &flexes, area.w);
        let used = total(&widths);
        let (lead, gap) = main_offset(self.main, area.w, used, self.children.len());

        let mut x = area.x.saturating_add(lead);
        for (i, ch) in self.children.iter().enumerate() {
            let min_h = if self.cross == CrossAxis::Stretch { area.h } else { 0 };
            let s = ch.measure(Constraints { min_w: 0, min_h, max_w: widths[i], max_h: area.h });
            let chh = if self.cross == CrossAxis::Stretch { area.h } else { s.h.min(area.h) };
            let dy = cross_offset(self.cross, area.h, chh);
            let child = Area { x, y: area.y.saturating_add(dy), w: widths[i], h: chh };
            paint_clipped(ch.as_ref(), cx, child, canvas);
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
    fn measure(&self, c: Constraints) -> Size {
        c.clamp(Size::new(0, 0))
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
        c.clamp(self.child.measure(c))
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
        c.clamp(Size::new(self.w, self.h))
    }
    fn paint(&self, cx: &mut Cx, area: Area, canvas: &mut Canvas) {
        if let Some(child) = &self.child {
            // The child gets the box's size, not whatever area the parent offers.
            let sized = Area { w: self.w.min(area.w), h: self.h.min(area.h), ..area };
            paint_clipped(child.as_ref(), cx, sized, canvas);
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
        c.clamp(Size::new(c.max_w, c.max_h))
    }
    fn paint(&self, cx: &mut Cx, area: Area, canvas: &mut Canvas) {
        paint_aligned(self.child.as_ref(), Alignment::Center, cx, area, canvas);
    }
}

/// Paint `child` at its measured size, placed by `alignment` within `area`.
fn paint_aligned(child: &dyn Widget, alignment: Alignment, cx: &mut Cx, area: Area, canvas: &mut Canvas) {
    let s = child.measure(area.constraints());
    let (w, h) = (s.w.min(area.w), s.h.min(area.h));
    let (dx, dy) = alignment.offset(area.w, area.h, w, h);
    let placed = Area { x: area.x.saturating_add(dx), y: area.y.saturating_add(dy), w, h };
    paint_clipped(child, cx, placed, canvas);
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
        c.clamp(Size::new(c.max_w, c.max_h))
    }
    fn paint(&self, cx: &mut Cx, area: Area, canvas: &mut Canvas) {
        paint_aligned(self.child.as_ref(), self.alignment, cx, area, canvas);
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

impl Padding {
    fn insets(&self) -> (u16, u16) {
        (self.l.saturating_add(self.r), self.t.saturating_add(self.b))
    }
}

impl Widget for Padding {
    fn measure(&self, c: Constraints) -> Size {
        let (dw, dh) = self.insets();
        let inner = Constraints::loose(c.max_w.saturating_sub(dw), c.max_h.saturating_sub(dh));
        let s = self.child.measure(inner);
        c.clamp(Size::new(s.w.saturating_add(dw), s.h.saturating_add(dh)))
    }
    fn paint(&self, cx: &mut Cx, area: Area, canvas: &mut Canvas) {
        let (dw, dh) = self.insets();
        let inner = Area {
            x: area.x.saturating_add(self.l.min(area.w)),
            y: area.y.saturating_add(self.t.min(area.h)),
            w: area.w.saturating_sub(dw),
            h: area.h.saturating_sub(dh),
        };
        paint_clipped(self.child.as_ref(), cx, inner, canvas);
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
        let inner = Constraints::loose(c.max_w, c.max_h);
        let mut w: u16 = 0;
        let mut h: u16 = 0;
        for ch in &self.children {
            let s = ch.measure(inner);
            w = w.max(s.w);
            h = h.max(s.h);
        }
        c.clamp(Size::new(w, h))
    }

    fn paint(&self, cx: &mut Cx, area: Area, canvas: &mut Canvas) {
        for ch in &self.children {
            paint_aligned(ch.as_ref(), self.align, cx, area, canvas);
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
        let inner = area.shrink(self.x.min(area.w), self.y.min(area.h), 0, 0);
        let s = self.child.measure(inner.constraints());
        let child = Area { x: inner.x, y: inner.y, w: s.w.min(inner.w), h: s.h.min(inner.h) };
        paint_clipped(self.child.as_ref(), cx, child, canvas);
    }
}

// -----------------------------------------------------------------------------
// Fraction — percentage-of-parent sizing
// -----------------------------------------------------------------------------

/// Sizes its child as a fraction of the parent's constraints (e.g. `0.5` = half
/// the available width/height). An axis without a fraction takes the child's
/// own measured size.
///
/// Painted into an area that a parent sized from this widget's own `measure`
/// (a `Row`/`Column` slot), the child fills it; painted into a larger area
/// the parent did not size for it (e.g. directly inside a `Container`), the
/// fraction is taken of that area and the child is painted top-left in it.
pub struct Fraction {
    fw: Option<f32>,
    fh: Option<f32>,
    child: Element,
    /// Sizes `measure` returned this frame: an area of one of these sizes was
    /// allotted by a parent that already applied the fraction.
    measured: RefCell<Vec<Size>>,
}

impl Fraction {
    pub fn new(fw: f32, fh: f32, child: impl Widget + 'static) -> Self {
        Self { fw: Some(fw), fh: Some(fh), child: Box::new(child), measured: RefCell::default() }
    }
    /// Fraction of the available width; height from the child.
    pub fn w(fw: f32, child: impl Widget + 'static) -> Self {
        Self { fw: Some(fw), fh: None, child: Box::new(child), measured: RefCell::default() }
    }
    /// Fraction of the available height; width from the child.
    pub fn h(fh: f32, child: impl Widget + 'static) -> Self {
        Self { fw: None, fh: None, child: Box::new(child), measured: RefCell::default() }.set_fh(fh)
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
        let s = c.clamp(self.resolve(c));
        let mut measured = self.measured.borrow_mut();
        if !measured.contains(&s) && measured.len() < 16 {
            measured.push(s);
        }
        s
    }
    fn paint(&self, cx: &mut Cx, area: Area, canvas: &mut Canvas) {
        let size = Size::new(area.w, area.h);
        let sized = if self.measured.borrow().contains(&size) {
            area
        } else {
            let s = area.constraints().clamp(self.resolve(area.constraints()));
            Area { w: s.w, h: s.h, ..area }
        };
        paint_clipped(self.child.as_ref(), cx, sized, canvas);
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
        c.clamp(match self.dir {
            DividerDir::Horizontal => Size::new(c.max_w, 1),
            DividerDir::Vertical => Size::new(1, c.max_h),
        })
    }
    fn paint(&self, _cx: &mut Cx, area: Area, canvas: &mut Canvas) {
        match self.dir {
            DividerDir::Horizontal => {
                for x in area.x..area.x.saturating_add(area.w) {
                    canvas.set(x, area.y, self.ch, self.hl.clone());
                }
            },
            DividerDir::Vertical => {
                for y in area.y..area.y.saturating_add(area.h) {
                    canvas.set(area.x, y, self.ch, self.hl.clone());
                }
            },
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
            BoxStyle::Single => Some(BorderChars { tl: '┌', tr: '┐', bl: '└', br: '┘', h: '─', v: '│' }),
            BoxStyle::Rounded => Some(BorderChars { tl: '╭', tr: '╮', bl: '╰', br: '╯', h: '─', v: '│' }),
            BoxStyle::Double => Some(BorderChars { tl: '╔', tr: '╗', bl: '╚', br: '╝', h: '═', v: '║' }),
            BoxStyle::Thick => Some(BorderChars { tl: '┏', tr: '┓', bl: '┗', br: '┛', h: '━', v: '┃' }),
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

    /// Total horizontal and vertical space taken by border + padding.
    fn extra(&self) -> (u16, u16) {
        let b = self.border_inset();
        let (l, t, r, bo) = self.pad;
        (
            (b as u32 * 2 + l as u32 + r as u32).min(u16::MAX as u32) as u16,
            (b as u32 * 2 + t as u32 + bo as u32).min(u16::MAX as u32) as u16,
        )
    }

    fn inner(&self, area: Area) -> Area {
        let b = self.border_inset();
        let (l, t, _, _) = self.pad;
        let (extra_w, extra_h) = self.extra();
        Area {
            x: area.x.saturating_add(b.saturating_add(l).min(area.w)),
            y: area.y.saturating_add(b.saturating_add(t).min(area.h)),
            w: area.w.saturating_sub(extra_w),
            h: area.h.saturating_sub(extra_h),
        }
    }

    /// Paint `label` on border row `y`, aligned within the area's top/bottom edge.
    fn edge_label(&self, canvas: &mut Canvas, area: Area, y: u16, label: &str, pos: EdgeAlign) {
        let label = format!(" {} ", label.trim());
        let lw = crate::text::display_width(&label);
        if lw.saturating_add(2) > area.w {
            return;
        }
        let x = match pos {
            EdgeAlign::Left => area.x + 1,
            EdgeAlign::Center => area.x + (area.w - lw) / 2,
            EdgeAlign::Right => area.x + area.w - lw - 1,
        };
        canvas.put_str_clipped(x, y, lw, &label, self.title_hl.clone());
    }
}

impl Widget for Container {
    fn measure(&self, c: Constraints) -> Size {
        let (extra_w, extra_h) = self.extra();
        let inner = Constraints::loose(c.max_w.saturating_sub(extra_w), c.max_h.saturating_sub(extra_h));
        let cs = self.child.as_ref().map(|c| c.measure(inner)).unwrap_or_default();
        c.clamp(Size::new(cs.w.saturating_add(extra_w), cs.h.saturating_add(extra_h)))
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
            paint_clipped(child.as_ref(), cx, self.inner(area), canvas);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::view::widgets::input::AreaState;
    use crate::view::widgets::{Text, TextArea};

    fn paint(w: &dyn Widget, cw: u16, ch: u16) -> Vec<String> {
        let mut canvas = Canvas::new(cw, ch);
        let mut cx = Cx::new(usize::MAX);
        w.paint(&mut cx, Area { x: 0, y: 0, w: cw, h: ch }, &mut canvas);
        canvas.to_lines().iter().map(|l| l.text().trim_end().to_string()).collect()
    }

    #[test]
    fn distribute_never_overflows() {
        let big = Constraints::UNBOUNDED;
        assert_eq!(distribute(&[big, big, big], &[0, 0, 0], 10), vec![10, 0, 0]);
        assert_eq!(distribute(&[0, 0], &[1000, 1000], 100), vec![50, 50]);
        assert_eq!(distribute(&[0], &[u16::MAX], u16::MAX), vec![u16::MAX]);
        assert_eq!(distribute(&[big, 3], &[0, 0], u16::MAX), vec![big, 3]);
    }

    #[test]
    fn distribute_clamps_fixed_children_to_remaining_space() {
        assert_eq!(distribute(&[2, 10], &[0, 0], 8), vec![2, 6]);
        assert_eq!(distribute(&[1, 1, 1], &[0, 0, 0], 2), vec![1, 1, 0]);
        // Flex children get nothing once fixed ones used it all.
        assert_eq!(distribute(&[5, 0, 5], &[0, 1, 0], 7), vec![5, 0, 2]);
    }

    #[test]
    fn distribute_hands_out_rounding_remainder() {
        assert_eq!(distribute(&[0, 0, 0], &[1, 1, 1], 10), vec![4, 3, 3]);
        let d = distribute(&[1, 0, 1, 0, 1], &[0, 1, 0, 3, 0], 12);
        assert_eq!(d, vec![1, 2, 1, 7, 1]);
        assert_eq!(d.iter().sum::<u16>(), 12);
    }

    #[test]
    fn huge_spacer_weight_does_not_panic() {
        let col =
            Column::new(vec![Box::new(Text::new("a")), Box::new(Spacer::new().weight(1000)), Box::new(Text::new("b"))]);
        let rows = paint(&col, 3, 100);
        assert_eq!(rows[0], "a");
        assert_eq!(rows[99], "b");
    }

    #[test]
    fn greedy_children_in_column_do_not_overflow() {
        let st = AreaState::new("");
        let col = Column::new(vec![
            Box::new(TextArea::new(&st)),
            Box::new(TextArea::new(&st)),
            Box::new(Divider::vertical()),
        ]);
        let s = col.measure(Constraints::loose(10, Constraints::UNBOUNDED));
        assert_eq!(s.h, Constraints::UNBOUNDED);
        let row = Row::new(vec![Box::new(TextArea::new(&st)), Box::new(Center::new(Text::new("x")))]);
        assert_eq!(row.measure(Constraints::loose(Constraints::UNBOUNDED, 5)).w, Constraints::UNBOUNDED);
        paint(&col, 10, 50);
    }

    #[test]
    fn padding_measure_respects_constraints() {
        let p = Padding::all(2, Text::new("x"));
        let s = p.measure(Constraints::loose(3, 3));
        assert_eq!((s.w, s.h), (3, 3));
        assert_eq!(p.measure(Constraints::loose(10, 10)), Size::new(5, 5));
    }

    #[test]
    fn container_clips_child_to_inner_area() {
        let c = Container::new(Text::new("hello world")).border(BoxStyle::Single);
        assert_eq!(paint(&c, 8, 3), vec!["┌──────┐", "│hello │", "└──────┘"]);
    }

    #[test]
    fn fraction_and_sized_box_paint_their_own_size() {
        let c = Container::new(Fraction::new(0.5, 1.0, Divider::new())).border(BoxStyle::Single);
        assert_eq!(paint(&c, 12, 3)[1], "│─────     │");
        let c = Container::new(SizedBox::new(3, 1).child(Divider::new())).border(BoxStyle::Single);
        assert_eq!(paint(&c, 8, 3)[1], "│───   │");
        // Inside a Row the slot was sized from the fraction already.
        let r = Row::new(vec![Box::new(Fraction::w(0.5, Divider::new())), Box::new(Text::new("x"))]);
        assert_eq!(paint(&r, 20, 1)[0], "──────────x");
    }
}
