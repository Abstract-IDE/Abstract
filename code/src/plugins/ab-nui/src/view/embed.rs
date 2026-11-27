//! Embedded windows: real Neovim windows laid over an area of a `Surface`'s
//! widget tree.
//!
//! Some UI is best served by a real buffer — text you edit with every Vim
//! motion ([`BufferInput`](crate::view::BufferInput)), a file shown with its
//! syntax highlighting ([`CodeView`](crate::view::CodeView)). An
//! [`EmbeddedWindow`] is such a window; put its [`Embed`] widget in the tree
//! and the surface keeps the window exactly over the widget's area: placed
//! after each paint, moved on resize, hidden when the widget isn't painted or
//! the surface is hidden, closed with the surface.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use crate::error::Result;
use crate::geometry::{Position, Size as FloatSize};
use crate::nvim::{Border, Buffer, Window};
use crate::view::canvas::Canvas;
use crate::view::widget::{Area, Constraints, Cx, Size, Widget};
use crate::widget::popup::{Popup, PopupOptions};

/// How an embedded window behaves.
#[derive(Clone, Copy, Debug)]
pub struct EmbedOptions {
    /// The cursor can go there (by keys or mouse).
    pub focusable: bool,
    /// Move the cursor there when the window first opens.
    pub enter: bool,
    /// Stacking order; above the host surface's (50 by default).
    pub zindex: u32,
}

impl Default for EmbedOptions {
    fn default() -> Self {
        Self { focusable: false, enter: false, zindex: 60 }
    }
}

/// Editor position and size of an embedded window: `(row, col, width, height)`.
pub type Placement = (i64, i64, u16, u16);

type OpenHook = Rc<dyn Fn(Window)>;

struct Inner {
    popup: Popup,
    focusable: bool,
    placed: Cell<Option<Placement>>,
    on_open: RefCell<Vec<OpenHook>>,
    on_close: RefCell<Vec<Rc<dyn Fn()>>>,
    on_resize: RefCell<Vec<OpenHook>>,
    /// Set while we hide/close the window ourselves (so `on_close` only
    /// reports closes from elsewhere).
    closing: Cell<bool>,
}

/// A real window kept over a widget's area. Cloning yields another handle to
/// the same window.
#[derive(Clone)]
pub struct EmbeddedWindow(Rc<Inner>);

impl EmbeddedWindow {
    /// An embedded window over its own scratch buffer (kept until [`close`](Self::close)).
    pub fn new(opts: EmbedOptions) -> Result<Self> {
        Ok(Self::wrap(Popup::new(popup_options(opts))?, opts))
    }

    /// An embedded window showing a buffer you already have (never deleted by us).
    pub fn with_buffer(buf: Buffer, opts: EmbedOptions) -> Result<Self> {
        Ok(Self::wrap(Popup::with_buffer(buf, popup_options(opts))?, opts))
    }

    fn wrap(popup: Popup, opts: EmbedOptions) -> Self {
        Self(Rc::new(Inner {
            popup,
            focusable: opts.focusable,
            placed: Cell::new(None),
            on_open: RefCell::new(Vec::new()),
            on_close: RefCell::new(Vec::new()),
            on_resize: RefCell::new(Vec::new()),
            closing: Cell::new(false),
        }))
    }

    pub fn buffer(&self) -> Buffer {
        self.0.popup.buffer()
    }

    /// The window, while it is shown.
    pub fn window(&self) -> Option<Window> {
        self.0.popup.window().filter(|w| w.is_valid())
    }

    pub fn is_open(&self) -> bool {
        self.0.popup.is_open()
    }

    pub fn is_focusable(&self) -> bool {
        self.0.focusable
    }

    /// Where the window is, while shown.
    pub fn placement(&self) -> Option<Placement> {
        self.is_open().then(|| self.0.placed.get()).flatten()
    }

    /// Move the cursor into the window (if it is shown).
    pub fn focus(&self) -> Result<()> {
        match self.window() {
            Some(win) => win.focus(),
            None => Ok(()),
        }
    }

    /// Run `f` with the window every time it opens (set window options,
    /// restore the view…).
    pub fn on_open(&self, f: impl Fn(Window) + 'static) {
        self.0.on_open.borrow_mut().push(Rc::new(f));
    }

    /// Run `f` with the window when its size changes while it is shown.
    pub fn on_resize(&self, f: impl Fn(Window) + 'static) {
        self.0.on_resize.borrow_mut().push(Rc::new(f));
    }

    /// Run `f` when the window is closed by something other than its surface
    /// (`:q` in it, another plugin closing it…).
    pub fn on_close(&self, f: impl Fn() + 'static) {
        self.0.on_close.borrow_mut().push(Rc::new(f));
    }

    /// The widget that reserves this window's place in the tree (it takes
    /// all the space it is given; see [`Embed::height`]).
    pub fn widget(&self) -> Embed {
        Embed { win: self.clone(), height: None }
    }

    /// Stack above the host (at `z` or higher).
    pub(crate) fn raise_to(&self, z: u32) {
        self.0.popup.raise_to(z);
    }

    /// Whether `win` is this embedded window.
    pub fn is(&self, win: Window) -> bool {
        self.0.popup.window() == Some(win)
    }

    /// Show the window at `placement`, or hide it.
    pub(crate) fn place(&self, placement: Option<Placement>) -> Result<()> {
        let Some(p @ (row, col, w, h)) = placement else {
            self.hide();
            return Ok(());
        };
        let popup = &self.0.popup;
        if self.0.placed.get() != Some(p) || !popup.is_open() {
            let resized = popup.is_open() && self.0.placed.get().is_some_and(|old| (old.2, old.3) != (w, h));
            popup.set_position(Position::At { row: row as f64, col: col as f64 })?;
            popup.set_size(FloatSize::cells(w.max(1) as u32, h.max(1) as u32))?;
            self.0.placed.set(Some(p));
            if resized && let Some(win) = popup.window() {
                let hooks: Vec<_> = self.0.on_resize.borrow().clone();
                for hook in hooks {
                    hook(win);
                }
            }
        }
        if !popup.is_open() {
            popup.open()?;
            if let Some(win) = popup.window() {
                let weak = Rc::downgrade(&self.0);
                crate::nvim::autocmd_once(&["WinClosed"], Some(&win.id().to_string()), None, move |_| {
                    if let Some(inner) = weak.upgrade()
                        && !inner.closing.get()
                    {
                        let hooks: Vec<_> = inner.on_close.borrow().clone();
                        for hook in hooks {
                            hook();
                        }
                    }
                })?;
                let _ = win.set_option("cursorline", false);
                let hooks: Vec<_> = self.0.on_open.borrow().clone();
                for hook in hooks {
                    hook(win);
                }
            }
        }
        Ok(())
    }

    /// Hide the window (its buffer and content stay).
    pub fn hide(&self) {
        leave_insert_mode(self.window());
        self.0.closing.set(true);
        self.0.popup.hide();
        self.0.closing.set(false);
    }

    /// Close the window and free its buffer (unless it was given one).
    pub fn close(&self) {
        leave_insert_mode(self.window());
        self.0.closing.set(true);
        self.0.popup.close();
        self.0.closing.set(false);
    }

    fn ptr(&self) -> *const Inner {
        Rc::as_ptr(&self.0)
    }
}

fn popup_options(opts: EmbedOptions) -> PopupOptions {
    PopupOptions {
        size: FloatSize::cells(1, 1),
        position: Position::At { row: 0.0, col: 0.0 },
        border: Border::None,
        focusable: opts.focusable,
        enter: opts.enter,
        zindex: opts.zindex,
        // Wheel and clicks reach every embedded window, focusable or not.
        mouse: Some(true),
        ..Default::default()
    }
}

/// Closing a window from insert mode would carry insert mode into whichever
/// window gets the cursor next.
fn leave_insert_mode(win: Option<Window>) {
    if let Some(win) = win
        && Window::current().is_ok_and(|cur| cur == win)
        && crate::nvim::mode().is_ok_and(|m| m.starts_with('i') || m.starts_with('R'))
    {
        let _ = crate::lua::cmd("stopinsert");
    }
}

/// Reserves an area of the tree for an [`EmbeddedWindow`]. Paints nothing;
/// fills the space it is given (or a fixed number of rows).
pub struct Embed {
    win: EmbeddedWindow,
    height: Option<u16>,
}

impl Embed {
    /// Take exactly `rows` rows (e.g. 1 for an input) instead of all there is.
    pub fn height(mut self, rows: u16) -> Self {
        self.height = Some(rows);
        self
    }
}

impl Widget for Embed {
    fn measure(&self, c: Constraints) -> Size {
        let h = self.height.map_or(c.max_h, |rows| rows.min(c.max_h));
        c.clamp(Size::new(c.max_w, h))
    }

    fn paint(&self, cx: &mut Cx, area: Area, canvas: &mut Canvas) {
        let h = self.height.map_or(area.h, |rows| rows.min(area.h));
        cx.embed(canvas.screen_map(), Area { h, ..area }, self.win.clone());
    }
}

/// The embedded windows of one surface: every window it has shown, and where
/// the last frame put them.
#[derive(Default)]
pub(crate) struct Embeds {
    known: Vec<EmbeddedWindow>,
    frame: Vec<(EmbeddedWindow, Area)>,
}

impl Embeds {
    /// Record a painted frame's embeds (areas already on the root canvas).
    /// Returns the windows to close: gone from the frame, and no one else
    /// holds them (built afresh each render), so they can't come back.
    #[must_use]
    pub fn set_frame(&mut self, frame: Vec<(EmbeddedWindow, Area)>) -> Vec<EmbeddedWindow> {
        for (win, _) in &frame {
            if !self.known.iter().any(|k| k.ptr() == win.ptr()) {
                self.known.push(win.clone());
            }
        }
        self.frame = frame;
        let (keep, gone): (Vec<_>, Vec<_>) = std::mem::take(&mut self.known)
            .into_iter()
            .partition(|w| Rc::strong_count(&w.0) > 1 || self.frame.iter().any(|(f, _)| f.ptr() == w.ptr()));
        self.known = keep;
        gone
    }

    /// Where every window goes, `origin` being the editor cell of the host's
    /// text area (`None`: not in the last frame — hide it). Apply with
    /// [`apply`] once no borrow of `self` is held: placing runs user hooks.
    pub fn placements(&self, origin: (i64, i64)) -> Vec<(EmbeddedWindow, Option<Placement>)> {
        self.known
            .iter()
            .map(|win| {
                let area = self.frame.iter().find(|(w, _)| w.ptr() == win.ptr()).map(|(_, a)| *a);
                let placement = area
                    .filter(|a| a.w > 0 && a.h > 0)
                    .map(|a| (origin.0 + a.y as i64, origin.1 + a.x as i64, a.w, a.h));
                (win.clone(), placement)
            })
            .collect()
    }

    pub fn known(&self) -> Vec<EmbeddedWindow> {
        self.known.clone()
    }

    pub fn take_known(&mut self) -> Vec<EmbeddedWindow> {
        self.frame.clear();
        std::mem::take(&mut self.known)
    }

    /// The window the cursor should be in: the first focusable one shown.
    pub fn focus_target(&self) -> Option<Window> {
        self.known.iter().filter(|w| w.is_focusable()).find_map(|w| w.window())
    }

    /// Buffers of the focusable embedded windows (where the cursor can be).
    pub fn focusable_buffers(&self) -> Vec<Buffer> {
        self.known.iter().filter(|w| w.is_focusable()).map(|w| w.buffer()).collect()
    }

    /// Whether `win` is one of the focusable embedded windows.
    pub fn contains_focusable(&self, win: Window) -> bool {
        self.known.iter().any(|w| w.is_focusable() && w.is(win))
    }

    pub fn contains(&self, win: Window) -> bool {
        self.known.iter().any(|w| w.is(win))
    }
}

/// Place (or hide) each window.
pub(crate) fn apply(placements: Vec<(EmbeddedWindow, Option<Placement>)>) {
    for (win, placement) in placements {
        let _ = win.place(placement);
    }
}
