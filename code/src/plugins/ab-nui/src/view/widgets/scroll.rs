//! Scrolling: a generic viewport ([`ScrollView`]) that makes any child
//! scrollable, and the shared scrollbar drawing used by every scrolling widget.

use std::rc::Rc;

use crate::reactive::Signal;
use crate::theme::groups;
use crate::view::canvas::Canvas;
use crate::view::widget::{Area, Constraints, Cx, Key, Size, Widget};

/// Draw a vertical scrollbar in `area` (a 1-cell-wide column): a track with a
/// thumb sized/positioned for `offset` in a `total`-row content viewed through
/// `viewport` rows. No-op when everything fits.
pub(crate) fn draw_scrollbar(canvas: &mut Canvas, area: Area, total: usize, viewport: usize, offset: usize) {
    if total <= viewport || area.h == 0 {
        return;
    }
    let track: Rc<str> = Rc::from(groups::SCROLLBAR);
    let thumb_hl: Rc<str> = Rc::from(groups::SCROLLBAR_THUMB);

    let h = area.h as usize;
    let thumb_len = ((viewport * h).div_ceil(total)).clamp(1, h);
    let max_offset = total - viewport;
    let thumb_top = if max_offset == 0 { 0 } else { (offset.min(max_offset) * (h - thumb_len)).div_ceil(max_offset) };

    for row in 0..h {
        let (ch, hl) = if row >= thumb_top && row < thumb_top + thumb_len {
            ('█', thumb_hl.clone())
        } else {
            ('│', track.clone())
        };
        canvas.set(area.x, area.y + row as u16, ch, Some(hl));
    }
}

/// A standalone scrollbar widget (1 cell wide), for custom layouts.
pub struct Scrollbar {
    total: usize,
    viewport: usize,
    offset: usize,
}

impl Scrollbar {
    pub fn new(total: usize, viewport: usize, offset: usize) -> Self {
        Self { total, viewport, offset }
    }
}

impl Widget for Scrollbar {
    fn measure(&self, c: Constraints) -> Size {
        c.clamp(Size::new(1, c.max_h))
    }
    fn paint(&self, _cx: &mut Cx, area: Area, canvas: &mut Canvas) {
        draw_scrollbar(canvas, area, self.total, self.viewport, self.offset);
    }
}

/// The tallest offscreen canvas a [`ScrollView`] paints its content on;
/// content measured taller is cut off at this many rows.
pub const MAX_SCROLL_ROWS: u16 = 4096;

/// A vertical viewport over any child: the child is painted at its measured
/// height onto an offscreen canvas and the visible window is blitted through.
/// A greedy child (one that measures the whole unbounded height, e.g. a
/// `TextArea`) gets the viewport's height instead.
///
/// Focused, it consumes `Up`/`Down`/`PageUp`/`PageDown`/`<C-d>`/`<C-u>`/
/// `Home`/`End`. When focus is on an interactive widget inside the content,
/// the view scrolls to keep that widget visible; the hit areas of the
/// content's widgets are clipped to the viewport.
pub struct ScrollView {
    child: Element,
    offset: Signal<u16>,
    show_scrollbar: bool,
}

use crate::view::widget::Element;

impl ScrollView {
    /// `offset` is the first visible content row — keep it in your state so the
    /// scroll position survives rebuilds.
    pub fn new(offset: Signal<u16>, child: impl Widget + 'static) -> Self {
        Self { child: Box::new(child), offset, show_scrollbar: true }
    }
    pub fn scrollbar(mut self, show: bool) -> Self {
        self.show_scrollbar = show;
        self
    }

    fn bar_w(&self) -> u16 {
        if self.show_scrollbar { 1 } else { 0 }
    }

    /// Rows of the offscreen content canvas for a viewport `view_h` tall.
    fn content_rows(&self, view_w: u16, view_h: u16) -> u16 {
        let measured = self.content_size(view_w).h;
        let rows = if measured >= Constraints::UNBOUNDED { view_h } else { measured.min(MAX_SCROLL_ROWS) };
        rows.max(1)
    }

    /// The child's full height when given unbounded vertical room.
    fn content_size(&self, max_w: u16) -> Size {
        self.child.measure(Constraints::loose(max_w, Constraints::UNBOUNDED))
    }
}

impl Widget for ScrollView {
    fn measure(&self, c: Constraints) -> Size {
        let content = self.content_size(c.max_w.saturating_sub(self.bar_w()));
        c.clamp(Size::new(content.w.saturating_add(self.bar_w()), content.h))
    }

    fn paint(&self, cx: &mut Cx, area: Area, canvas: &mut Canvas) {
        let view_w = area.w.saturating_sub(self.bar_w());
        let content_h = self.content_rows(view_w, area.h);

        // A tracked read: scrolling (or any outside write to the offset) must
        // repaint the surface. The clamp is written back silently, since this
        // frame already shows the clamped value.
        let max_offset = content_h.saturating_sub(area.h);
        let mut offset = self.offset.get().min(max_offset);

        // Paint the child on an offscreen canvas...
        let (mut off, placement) = canvas.offscreen(view_w, content_h);
        let before = cx.focusables.len();
        self.child.paint(cx, Area { x: 0, y: 0, w: view_w, h: content_h }, &mut off);
        let after = cx.focusables.len();

        // ...scroll the focused widget (if it is ours) into view...
        if (before..after).contains(&cx.focused) {
            let f = cx.focusables[cx.focused].area;
            let (top, bottom) = (f.y as u32, f.y as u32 + f.h.max(1) as u32);
            if top < offset as u32 {
                offset = f.y;
            } else if bottom > offset as u32 + area.h as u32 {
                offset = (bottom - area.h as u32).min(top) as u16;
            }
            offset = offset.min(max_offset);
        }
        self.offset.set_silent(offset);
        let view = Area { x: area.x, y: area.y, w: view_w, h: area.h };
        placement.set(area.x as i32, area.y as i32 - offset as i32, view);

        // ...remap the child's focusable areas into viewport coordinates,
        // clipped to the viewport (a widget scrolled out keeps its focus id
        // but has an empty area)...
        let (vis_top, vis_bottom) = (offset as u32, offset as u32 + area.h as u32);
        for f in &mut cx.focusables[before..after] {
            let a = f.area;
            let top = (a.y as u32).clamp(vis_top, vis_bottom);
            let bottom = (a.y as u32 + a.h as u32).clamp(vis_top, vis_bottom);
            let x = a.x.min(view_w);
            f.area = Area {
                x: area.x.saturating_add(x),
                y: area.y.saturating_add((top - vis_top) as u16),
                w: a.w.min(view_w - x),
                h: (bottom - top) as u16,
            };
        }

        // ...and blit the visible window through.
        canvas.blit(&off, Area { x: 0, y: offset, w: view_w, h: area.h }, area.x, area.y);

        if self.show_scrollbar {
            draw_scrollbar(
                canvas,
                Area { x: area.x + view_w, y: area.y, w: 1, h: area.h },
                content_h as usize,
                area.h as usize,
                offset as usize,
            );
        }

        // Scroll keys (when this viewport is the focused widget).
        let sig = self.offset.clone();
        let page = area.h.max(1);
        cx.register(
            area,
            Rc::new(move |k| {
                let step: i32 = match k {
                    Key::Down | Key::Char('j') => 1,
                    Key::Up | Key::Char('k') => -1,
                    Key::PageDown | Key::Ctrl('f') => page as i32,
                    Key::PageUp | Key::Ctrl('b') => -(page as i32),
                    Key::Ctrl('d') => (page / 2).max(1) as i32,
                    Key::Ctrl('u') => -((page / 2).max(1) as i32),
                    Key::Home | Key::Char('g') => {
                        sig.set(0);
                        return true;
                    },
                    Key::End | Key::Char('G') => {
                        sig.set(max_offset);
                        return true;
                    },
                    _ => return false,
                };
                sig.update(|o| {
                    let next = (*o as i32 + step).clamp(0, max_offset as i32);
                    *o = next as u16;
                });
                true
            }),
        );
    }
}
