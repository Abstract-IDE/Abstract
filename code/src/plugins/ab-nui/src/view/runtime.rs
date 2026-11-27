//! `Surface` — mounts a widget tree into a floating [`Popup`] and runs it.
//!
//! A `Surface` is an owned handle that *owns the UI's lifetime*:
//!
//! * Keep it → the UI (and the state its tree captures) stays alive, even while
//!   [`hidden`](Surface::hide). Re-[`show`](Surface::reopen) restores it intact.
//! * Drop it (or call [`close`](Surface::close)) → the window closes, the buffer
//!   is wiped, and the render effect is disposed. Nothing leaks permanently.
//!
//! The tree is rebuilt reactively: the `build` closure runs inside an
//! [`effect`], so any `Signal`/`Memo` it reads triggers a repaint on change.
//! A repaint only sends Neovim the rows that actually changed, and several
//! state changes made by one key press or timer tick produce a single repaint.
//! Keys are captured by buffer-local maps and routed to the focused widget;
//! `<Tab>`/`<S-Tab>` move focus, `<Esc>` hides (non-destructive).
//!
//! To close a surface from inside its own tree (a "Close" button), use
//! [`Surface::closer`] — capturing the `Surface` itself in its build closure
//! would form a reference cycle that keeps it alive forever.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use crate::error::{Error, Result};
use crate::lua;
use crate::nvim::{self, Window};
use crate::reactive::{EffectHandle, Signal, dispose_effect, effect};
use crate::view::canvas::Canvas;
use crate::view::embed::Embeds;
use crate::view::widget::{Area, Cx, Focusable, Key, Widget};
use crate::widget::popup::{Popup, PopupOptions};

/// A live UI surface: a floating window driven by a widget tree. Cloning yields
/// another handle to the *same* surface; the surface is freed when the last
/// handle is dropped.
#[derive(Clone)]
pub struct Surface {
    inner: Rc<SurfaceInner>,
}

/// An extra surface-level key map (see [`Surface::on_key`]), kept so it can be
/// reinstalled when Neovim unloads the buffer.
type ExtraKey = (String, Rc<dyn Fn()>);

struct SurfaceInner {
    popup: Popup,
    focused: Signal<usize>,
    focusables: Rc<RefCell<Vec<Focusable>>>,
    effect: RefCell<Option<EffectHandle>>,
    mounted: Cell<bool>,
    /// Set by [`Surface::close`] / drop: the surface can no longer be shown.
    closed: Cell<bool>,
    /// Set when Neovim unloads the buffer (`:bdelete`): its key maps, options
    /// and text are gone, so the next reopen has to restore them.
    unloaded: Rc<Cell<bool>>,
    /// Bumped on resize/reopen so the render effect re-measures the tree.
    epoch: Signal<u64>,
    /// The text-area size the last frame was painted at, so resize events
    /// that don't change it can be ignored.
    painted_size: Rc<Cell<Option<(u16, u16)>>>,
    /// The surface's autocmd group (cleanup + resize), deleted on free.
    group: Cell<Option<i64>>,
    /// The focus key of the focused widget (when it has one), so focus can
    /// re-attach by identity after the tree changes shape. Refreshed after
    /// every paint. (`Rc` so the render effect can hold it without holding
    /// the whole surface.)
    focused_key: Rc<RefCell<Option<Rc<str>>>>,
    /// Whether Tab at the last widget wraps to the first (default true).
    focus_wrap: Cell<bool>,
    /// Maps added with [`Surface::on_key`].
    extra_keys: RefCell<Vec<ExtraKey>>,
    /// Embedded windows and modality (shared with the render effect and autocmds).
    extras: Rc<Extras>,
}

/// What a surface shows besides its own window: embedded windows laid over
/// its tree, and — when modal — a backdrop over the rest of the editor.
struct Extras {
    embeds: RefCell<Embeds>,
    /// The last frame's mouse-wheel regions (root-canvas coordinates).
    scrolls: RefCell<Vec<(Area, crate::view::widget::ScrollHandler)>>,
    /// Buffers the wheel keys are bound on.
    wheel_bufs: RefCell<Vec<i64>>,
    modal: Cell<bool>,
    backdrop: RefCell<Option<Popup>>,
    /// The surface's stacking order; the backdrop sits just below it.
    zindex: u32,
}

impl Extras {
    /// Lay the embedded windows over their areas (host text area at `origin`).
    fn place_embeds(&self, origin: (i64, i64)) {
        let placements = self.embeds.borrow().placements(origin);
        // Embedded windows stack on top of the surface, whatever its zindex.
        for (win, _) in &placements {
            win.raise_to(self.zindex + 1);
        }
        crate::view::embed::apply(placements);
    }

    /// Scroll the innermost scrollable widget at (row, col) of the last
    /// frame by `rows`. Returns whether one was there.
    fn scroll_at(&self, row: u16, col: u16, rows: i32) -> bool {
        let handler = self
            .scrolls
            .borrow()
            .iter()
            .filter(|(a, _)| a.contains(col, row))
            .min_by_key(|(a, _)| a.w as u32 * a.h as u32)
            .map(|(_, h)| h.clone());
        match handler {
            Some(h) => {
                h(rows);
                true
            },
            None => false,
        }
    }

    /// Cover the editor so clicks and scrolls outside the surface do nothing.
    fn show_backdrop(&self) {
        if !self.modal.get() || self.backdrop.borrow().as_ref().is_some_and(|b| b.is_open()) {
            return;
        }
        let made = match self.backdrop.borrow().clone() {
            Some(b) => Ok(b),
            None => Popup::new(PopupOptions {
                size: crate::geometry::Size::ratio(1.0, 1.0),
                position: crate::geometry::Position::At { row: 0.0, col: 0.0 },
                border: crate::nvim::Border::None,
                focusable: false,
                enter: false,
                zindex: self.zindex.saturating_sub(1).max(1),
                // Not focusable, but catching the mouse: clicks/scrolls stop here.
                mouse: Some(true),
                ..Default::default()
            }),
        };
        let Ok(backdrop) = made else { return };
        if backdrop.open().is_ok()
            && let Some(win) = backdrop.window()
        {
            let hl = format!("Normal:{b},NormalFloat:{b}", b = crate::theme::groups::BACKDROP);
            let _ = win.set_option("winhighlight", hl);
            // Dim what is behind (needs true colour; otherwise just block it).
            if lua::get_option::<bool>("termguicolors").unwrap_or(false) {
                let _ = win.set_option("winblend", 45);
            }
        }
        *self.backdrop.borrow_mut() = Some(backdrop);
    }

    fn hide(&self) {
        for win in self.embeds.borrow().known() {
            win.hide();
        }
        if let Some(b) = self.backdrop.borrow().as_ref() {
            b.hide();
        }
    }

    fn close(&self) {
        let known = self.embeds.borrow_mut().take_known();
        for win in known {
            win.close();
        }
        if let Some(b) = self.backdrop.borrow_mut().take() {
            b.close();
        }
        // The wheel maps may be on buffers that outlive the surface (one
        // given to an embedded window): take them off.
        let bufs: Vec<i64> = self.wheel_bufs.borrow_mut().drain(..).collect();
        for buf in bufs.into_iter().map(crate::nvim::Buffer).filter(|b| b.is_valid()) {
            for lhs in WHEEL_KEYS.iter().map(|(lhs, ..)| lhs) {
                for mode in ["n", "i"] {
                    let _ = lua::call_api::<()>("nvim_buf_del_keymap", (buf.id(), mode, *lhs));
                }
            }
        }
    }

    /// Where the cursor belongs: the first focusable embedded window, else
    /// the surface's own window when it is focusable.
    fn focus_target(&self, popup: &Popup) -> Option<Window> {
        self.embeds
            .borrow()
            .focus_target()
            .or_else(|| popup.takes_input().then(|| popup.window()).flatten())
    }
}

/// Where focus should be, given the interactive widgets a paint produced:
///
/// 1. if the focused widget had a key and the same index no longer carries
///    it, follow the key to its new position;
/// 2. clamp the index if the interactive set shrank.
fn corrected_focus(focusables: &[Focusable], index: usize, key: Option<&str>) -> usize {
    let mut index = index;
    if let Some(key) = key {
        let at_index = focusables.get(index).and_then(|f| f.key.as_deref());
        if at_index != Some(key)
            && let Some(pos) = focusables.iter().position(|f| f.key.as_deref() == Some(key))
        {
            index = pos;
        }
    }
    if !focusables.is_empty() && index >= focusables.len() {
        index = focusables.len() - 1;
    }
    index
}

/// Whether Neovim will let `win` close: the last non-floating window of the
/// last tab page cannot (E444/E5601).
fn can_close(win: Window) -> bool {
    if win.config().map(|c| !c.relative.is_empty()).unwrap_or(true) {
        return true; // floats (or a window we can't inspect) — just try
    }
    let Ok(tab) = lua::call_api::<i64>("nvim_win_get_tabpage", (win.id(),)) else { return true };
    let wins: Vec<i64> = lua::call_api("nvim_tabpage_list_wins", (tab,)).unwrap_or_default();
    let normal = wins
        .into_iter()
        .filter(|&w| Window(w).config().map(|c| c.relative.is_empty()).unwrap_or(false))
        .count();
    if normal > 1 {
        return true;
    }
    let tabs: Vec<i64> = lua::call_api("nvim_list_tabpages", ()).unwrap_or_default();
    tabs.len() > 1
}

impl SurfaceInner {
    /// Fully tear down: dispose the effect, delete the autocmd group, close the
    /// window, wipe the buffer. Idempotent.
    fn free(&self) {
        if self.closed.replace(true) {
            return;
        }
        if let Some(handle) = self.effect.borrow_mut().take() {
            handle.dispose();
        }
        if let Some(group) = self.group.take() {
            let _ = nvim::del_augroup(group);
        }
        self.extras.close();
        self.popup.close(); // closes the window and deletes the buffer
        self.extra_keys.borrow_mut().clear();
    }

    fn ensure_open(&self) -> Result<()> {
        if self.closed.get() {
            return Err(Error::invalid("surface is closed"));
        }
        Ok(())
    }

    /// Re-measure the tree if the surface's text area no longer has the size
    /// the last frame was painted at.
    fn sync_size(&self) {
        sync_size(&self.popup, &self.painted_size, &self.epoch);
    }
}

/// Bump `epoch` (→ repaint) when `popup`'s text area differs from the size the
/// last frame was painted at.
fn sync_size(popup: &Popup, painted: &Cell<Option<(u16, u16)>>, epoch: &Signal<u64>) {
    if popup.inner_size().ok() != painted.get() {
        epoch.update(|n| *n += 1);
    }
}

impl Drop for SurfaceInner {
    fn drop(&mut self) {
        self.free();
    }
}

impl Surface {
    /// Create a surface in a split window (not shown yet). Same lifecycle as
    /// [`Surface::float`]; Neovim manages the geometry.
    pub fn split(cfg: crate::nvim::SplitConfig) -> Result<Self> {
        Self::float(PopupOptions { host: crate::widget::popup::Host::Split(cfg), ..Default::default() })
    }

    /// Create a floating surface from popup options (not shown yet).
    pub fn float(opts: PopupOptions) -> Result<Self> {
        // The popup keeps its buffer (and so the rendered state) while hidden
        // and deletes it on close, which `free` calls.
        let zindex = opts.zindex;
        // Wheel events over the surface must reach it (it routes them to the
        // widget under the pointer), even when it can't be focused.
        let popup = Popup::new(PopupOptions { mouse: Some(true), ..opts })?;
        Ok(Self {
            inner: Rc::new(SurfaceInner {
                popup,
                focused: Signal::new(0),
                focusables: Rc::new(RefCell::new(Vec::new())),
                effect: RefCell::new(None),
                mounted: Cell::new(false),
                closed: Cell::new(false),
                unloaded: Rc::new(Cell::new(false)),
                epoch: Signal::new(0),
                painted_size: Rc::new(Cell::new(None)),
                group: Cell::new(None),
                focused_key: Rc::new(RefCell::new(None)),
                focus_wrap: Cell::new(true),
                extra_keys: RefCell::new(Vec::new()),
                extras: Rc::new(Extras {
                    embeds: RefCell::new(Embeds::default()),
                    scrolls: RefCell::new(Vec::new()),
                    wheel_bufs: RefCell::new(Vec::new()),
                    modal: Cell::new(false),
                    backdrop: RefCell::new(None),
                    zindex,
                }),
            }),
        })
    }

    /// Make the surface modal: while it is shown, a backdrop covers the rest
    /// of the editor (dimming it; clicks and scrolls there do nothing) and the
    /// cursor can't leave it — if it lands in a window behind, it is put back
    /// (floats stacked above the surface, like a menu it opens, are fine).
    pub fn modal(self, modal: bool) -> Self {
        self.inner.extras.modal.set(modal);
        self
    }

    /// Scroll whatever scrollable widget is at (`row`, `col`) of the surface's
    /// text area by `rows` (> 0: down) — what a mouse-wheel step over it
    /// does. Returns whether there was one.
    pub fn scroll_at(&self, row: u16, col: u16, rows: i32) -> bool {
        self.inner.extras.scroll_at(row, col, rows)
    }

    /// Move the cursor into the surface: its first focusable embedded
    /// window, else its own window.
    pub fn focus(&self) -> Result<()> {
        match self.inner.extras.focus_target(&self.inner.popup) {
            Some(win) => win.focus(),
            None => Ok(()),
        }
    }

    /// The underlying popup (for extra styling).
    pub fn popup(&self) -> &Popup {
        &self.inner.popup
    }

    /// Which widget (by registration order) starts focused. Call before
    /// [`Surface::show`].
    pub fn initial_focus(self, index: usize) -> Self {
        self.inner.focused.set_silent(index);
        self
    }

    /// Whether Tab past the last widget wraps to the first (default `true`;
    /// `false` stops at the ends).
    pub fn focus_wrap(self, wrap: bool) -> Self {
        self.inner.focus_wrap.set(wrap);
        self
    }

    /// Bind an extra surface-level normal-mode key (checked by Neovim's maps,
    /// independent of widget focus).
    pub fn on_key(&self, lhs: &str, f: impl Fn() + 'static) -> Result<()> {
        self.inner.ensure_open()?;
        let f: Rc<dyn Fn()> = Rc::new(f);
        self.install_extra_key(lhs, f.clone())?;
        self.inner.extra_keys.borrow_mut().push((lhs.to_string(), f));
        Ok(())
    }

    fn install_extra_key(&self, lhs: &str, f: Rc<dyn Fn()>) -> Result<()> {
        self.inner.popup.on_key("n", lhs, move || f())
    }

    /// A closure that closes this surface (like [`Surface::close`]) if it is
    /// still alive, and does nothing otherwise. It holds only a weak reference,
    /// so it is the way to close a surface from inside its own tree — a
    /// "Close" button, an `on_key` handler — without keeping it alive:
    ///
    /// ```ignore
    /// let surface = Surface::float(opts)?;
    /// let close = Rc::new(surface.closer());
    /// let surface = surface.show(move || {
    ///     let close = close.clone();
    ///     Button::new("Close").on_press(move || close())
    /// })?;
    /// ```
    pub fn closer(&self) -> impl Fn() + 'static {
        let inner = Rc::downgrade(&self.inner);
        move || {
            if let Some(inner) = inner.upgrade() {
                inner.free();
            }
        }
    }

    /// Whether the window is currently open.
    pub fn is_visible(&self) -> bool {
        self.inner.popup.is_open()
    }

    /// Mount the tree and open the window. Returns the owning handle — keep it
    /// alive for as long as you want the UI to exist.
    ///
    /// A surface shows one tree for its whole life: calling `show` on a
    /// surface that already shows one is an error (use [`reopen`](Self::reopen)
    /// to bring a hidden surface back, or drive changes through signals).
    #[must_use = "the Surface owns the UI; drop it and the window closes and frees"]
    pub fn show<W: Widget + 'static>(self, build: impl Fn() -> W + 'static) -> Result<Self> {
        self.inner.ensure_open()?;
        if self.inner.mounted.get() {
            return Err(Error::invalid("surface is already showing a tree (use reopen() to show it again)"));
        }
        self.mount(build)?;
        self.reopen()?;
        Ok(self)
    }

    /// Reopen the window after [`hide`](Self::hide) (state is intact). No-op if
    /// already visible; an error once the surface is closed.
    pub fn reopen(&self) -> Result<()> {
        self.inner.ensure_open()?;
        let popup = &self.inner.popup;
        if popup.is_open() {
            return Ok(());
        }
        popup.open()?;
        self.inner.extras.show_backdrop();
        if let Some(win) = popup.window() {
            let _ = win.set_option("cursorline", false);
        }
        if self.inner.unloaded.get() {
            self.remount()?;
            self.inner.unloaded.set(false);
        }
        // Re-measure at the real window size (splits only know it now; the
        // editor may have been resized while hidden).
        self.inner.epoch.update(|n| *n += 1);
        Ok(())
    }

    /// Neovim unloaded the buffer (`:bdelete`), which cleared its options, key
    /// maps and text: restore them and repaint from scratch.
    fn remount(&self) -> Result<()> {
        let buf = self.inner.popup.buffer();
        buf.set_option("buftype", "nofile")?;
        buf.set_option("bufhidden", "hide")?;
        buf.set_option("swapfile", false)?;
        if self.inner.mounted.get() {
            self.install_keys()?;
            // Unloading dropped the wheel maps too.
            self.inner.extras.wheel_bufs.borrow_mut().retain(|b| *b != buf.id());
            if self.inner.popup.takes_input() {
                install_wheel(buf, &self.inner.popup, &self.inner.extras)?;
            }
        }
        let extra: Vec<ExtraKey> = self.inner.extra_keys.borrow().clone();
        for (lhs, f) in extra {
            self.install_extra_key(&lhs, f)?;
        }
        self.inner.popup.invalidate();
        Ok(())
    }

    /// Hide the window without destroying anything — state, effect and key maps
    /// all survive, so [`reopen`](Self::reopen) brings it back unchanged.
    ///
    /// The last window of the editor cannot close; hiding a surface shown
    /// there is a no-op (it stays visible).
    pub fn hide(&self) {
        Self::hide_popup(&self.inner.popup, &self.inner.extras);
    }

    fn hide_popup(popup: &Popup, extras: &Extras) {
        let Some(win) = popup.window().filter(|w| w.is_valid()) else {
            popup.hide(); // nothing shown; just drop any stale handle
            extras.hide();
            return;
        };
        if !can_close(win) {
            return;
        }
        extras.hide();
        popup.hide();
        if win.is_valid() && !popup.is_open() {
            // Closing failed anyway, but the popup forgot the window: make
            // the window stop showing the surface so "hidden" is true and a
            // reopen doesn't leave two windows on the same buffer.
            if let Ok(empty) = lua::call_api::<i64>("nvim_create_buf", (true, false)) {
                let _ = lua::call_api::<()>("nvim_win_set_buf", (win.id(), empty));
            }
        }
    }

    /// Toggle visibility (hide if shown, reopen if hidden). An error once the
    /// surface is closed.
    pub fn toggle(&self) -> Result<()> {
        self.inner.ensure_open()?;
        if self.is_visible() {
            self.hide();
            Ok(())
        } else {
            self.reopen()
        }
    }

    /// Permanently free the surface (window, buffer, effect). Equivalent to
    /// dropping the last handle. Idempotent.
    pub fn close(&self) {
        self.inner.free();
    }

    /// Re-measure the tree if its size changed (e.g. after a
    /// [`Layout`](crate::view::Layout) resized the popup). Hidden surfaces
    /// re-measure on reopen.
    pub(crate) fn sync_size(&self) {
        if self.is_visible() {
            self.inner.sync_size();
        }
    }

    /// Set up the render effect + key routing (once per surface).
    fn mount<W: Widget + 'static>(&self, build: impl Fn() -> W + 'static) -> Result<()> {
        self.inner.mounted.set(true);
        if self.inner.popup.takes_input() {
            install_wheel(self.inner.popup.buffer(), &self.inner.popup, &self.inner.extras)?;
        }
        let handle = self.start_render(build);
        *self.inner.effect.borrow_mut() = Some(handle);
        self.install_keys()?;
        self.install_cleanup(handle)?;
        Ok(())
    }

    /// The reactive paint loop.
    fn start_render<W: Widget + 'static>(&self, build: impl Fn() -> W + 'static) -> EffectHandle {
        let popup = self.inner.popup.clone();
        let focused = self.inner.focused.clone();
        let focusables = self.inner.focusables.clone();
        let epoch = self.inner.epoch.clone();
        let focused_key = self.inner.focused_key.clone();
        let painted_size = self.inner.painted_size.clone();
        let extras = self.inner.extras.clone();

        // One canvas for the surface's whole life: each frame resets it
        // rather than allocating a new grid.
        let mut canvas = Canvas::new(0, 0);

        // Unowned: the surface handle decides when rendering ends, not whichever
        // effect happened to be running when the surface was shown.
        crate::reactive::unowned(move || {
            effect(move || {
                epoch.get(); // subscribe: a resize bump re-measures at the new size
                let size = popup.inner_size().ok();
                let (w, h) = size.unwrap_or((1, 1));
                let root = build(); // reads signals → this effect re-runs on change
                let area = Area { x: 0, y: 0, w, h };

                // Paint, then check that the focus index still points at the
                // widget it should. If the tree changed shape it may not — then
                // paint once more with the corrected index, so the frame that
                // reaches the screen highlights the widget that receives the keys.
                let mut focus = focused.get();
                let mut cx;
                let mut pass = 0;
                loop {
                    canvas.reset(w, h);
                    cx = Cx::new(focus);
                    root.paint(&mut cx, area, &mut canvas);
                    cx.run_overlays(area, &mut canvas); // dropdowns/tooltips on top

                    let corrected = corrected_focus(&cx.focusables, focus, focused_key.borrow().as_deref());
                    pass += 1;
                    if corrected == focus || pass == 2 {
                        break;
                    }
                    focus = corrected;
                }
                // Silently: notifying would only re-run this effect to paint the
                // frame it has just painted.
                focused.set_silent(focus);
                // Track the focused widget's identity from now on (initial focus,
                // clamping and key-following included); `None` when it has no
                // key, so a vanished keyed widget can't pull focus back later.
                *focused_key.borrow_mut() = cx.focusables.get(focus).and_then(|f| f.key.clone());
                let embeds = cx.take_embeds();
                *extras.scrolls.borrow_mut() = cx.take_scrolls();
                *focusables.borrow_mut() = cx.focusables;
                painted_size.set(size);

                // Diffed against the previous frame: only changed rows reach
                // Neovim, in one call.
                let _ = popup.render_rows(canvas.to_rows());

                // Lay embedded windows over their areas (hide the ones not
                // painted this frame). Needs the window: on the first show
                // this runs again once it is open.
                let gone = extras.embeds.borrow_mut().set_frame(embeds);
                for win in gone {
                    win.close();
                }
                // The wheel works wherever the cursor can be: also in
                // focusable embedded windows (e.g. an input).
                let input_bufs = extras.embeds.borrow().focusable_buffers();
                for buf in input_bufs {
                    let _ = install_wheel(buf, &popup, &extras);
                }
                if let Some(origin) = popup.content_origin() {
                    extras.place_embeds(origin);
                }
            })
        })
    }

    /// Install the surface's autocmds: dispose the effect if the buffer is
    /// wiped externally (safety net; normal teardown goes through `free`),
    /// note when it is unloaded, and relayout + re-measure on resize.
    fn install_cleanup(&self, handle: EffectHandle) -> Result<()> {
        let id = handle.id();
        let group = nvim::unique_augroup("AbNuiSurface")?;
        self.inner.group.set(Some(group));
        let buf = self.inner.popup.buffer();
        {
            // Wiped from outside: nothing can show again, so the backdrop
            // and embedded windows go too.
            let extras = self.inner.extras.clone();
            nvim::buf_autocmd(&["BufWipeout"], buf, Some(group), move || {
                dispose_effect(id);
                let extras = extras.clone();
                let _ = nvim::schedule(move || extras.close());
            })?;
        }
        {
            // The surface's window closed from outside (`:q`, `<C-w>c`,
            // `:only`…): it is hidden now, so hide what goes with it — the
            // backdrop would otherwise keep blocking the editor.
            let popup = self.inner.popup.clone();
            let extras = self.inner.extras.clone();
            nvim::autocmd(&["WinClosed"], None, Some(group), move |_| {
                let (popup, extras) = (popup.clone(), extras.clone());
                let _ = nvim::schedule(move || {
                    if !popup.is_open() {
                        extras.hide();
                    }
                });
            })?;
        }
        {
            let unloaded = self.inner.unloaded.clone();
            nvim::buf_autocmd(&["BufUnload"], buf, Some(group), move || unloaded.set(true))?;
        }
        {
            // Only a visible surface reacts, and it re-measures only when its
            // own text area changed size (a split being dragged elsewhere
            // doesn't touch a float). Hidden surfaces re-measure on reopen.
            let popup = self.inner.popup.clone();
            let epoch = self.inner.epoch.clone();
            let painted = self.inner.painted_size.clone();
            let extras = self.inner.extras.clone();
            nvim::autocmd(&["VimResized", "WinResized"], None, Some(group), move |args| {
                if !popup.is_open() {
                    return;
                }
                if args.event == "VimResized" {
                    let _ = popup.relayout();
                    if let Some(b) = extras.backdrop.borrow().as_ref() {
                        let _ = b.relayout();
                    }
                    // The surface may have moved without changing size.
                    if let Some(origin) = popup.content_origin() {
                        extras.place_embeds(origin);
                    }
                }
                sync_size(&popup, &painted, &epoch);
            })?;
        }
        {
            // Where the cursor may be. A mouse click can still put it in one
            // of our non-focusable windows (the frame, a code view): move it
            // on. And when modal, it can't stay in a window behind us.
            let popup = self.inner.popup.clone();
            let extras = self.inner.extras.clone();
            nvim::autocmd(&["WinEnter"], None, Some(group), move |_| {
                if !popup.is_open() {
                    return;
                }
                let Ok(cur) = Window::current() else { return };
                let own_window = popup.window() == Some(cur);
                let own_embed = extras.embeds.borrow().contains(cur);
                if (own_window && popup.takes_input()) || extras.embeds.borrow().contains_focusable(cur) {
                    return;
                }
                if !own_window && !own_embed {
                    if !extras.modal.get() {
                        return;
                    }
                    // Floats stacked on top (a menu or prompt the surface
                    // opened) and the command-line window are fine.
                    let above = cur
                        .config()
                        .is_ok_and(|c| !c.relative.is_empty() && c.zindex.unwrap_or(50) >= extras.zindex as i64);
                    let cmdwin = lua::call_fn::<String>("getcmdwintype", ()).is_ok_and(|t| !t.is_empty());
                    if above || cmdwin {
                        return;
                    }
                }
                let (popup, extras) = (popup.clone(), extras.clone());
                let _ = nvim::schedule(move || {
                    if !popup.is_open() {
                        return;
                    }
                    match extras.focus_target(&popup) {
                        Some(win) => {
                            let _ = win.focus();
                        },
                        // Nowhere in the surface takes the cursor: give it back.
                        None => {
                            let ours = Window::current()
                                .is_ok_and(|c| popup.window() == Some(c) || extras.embeds.borrow().contains(c));
                            if ours {
                                let _ = lua::cmd("wincmd p");
                            }
                        },
                    }
                });
            })?;
        }
        {
            // A surface paints exactly what fits its window, so its view must
            // never move: undo any scroll (mouse wheel, touchpad, `zl`…)
            // instead of showing a shifted, half-painted UI.
            let popup = self.inner.popup.clone();
            nvim::autocmd(&["WinScrolled"], None, Some(group), move |args| {
                if let Some(win) = popup.window()
                    && args.matched == win.id().to_string()
                {
                    let _ = win.reset_view();
                }
            })?;
        }
        Ok(())
    }

    /// Route a key to the currently focused widget (index clamped defensively).
    /// Returns whether the widget consumed it.
    fn route(focusables: &RefCell<Vec<Focusable>>, focused: &Signal<usize>, key: Key) -> bool {
        let handle = {
            let fc = focusables.borrow();
            if fc.is_empty() {
                return false;
            }
            let i = focused.get_untracked().min(fc.len() - 1);
            fc[i].handle.clone()
        };
        handle(key)
    }

    /// Install the key table: every key a widget can receive is bound to one
    /// dispatcher, which routes it to the focused widget and falls back to
    /// focus traversal (`<Tab>`/`<S-Tab>`) or hiding (`<Esc>`). Runs on mount
    /// and again if Neovim unloads the buffer (which clears its maps).
    fn install_keys(&self) -> Result<()> {
        let popup = &self.inner.popup;
        // Nothing can be typed into a window the cursor never enters (a toast).
        if !popup.takes_input() {
            return Ok(());
        }

        let focusables = self.inner.focusables.clone();
        let focused = self.inner.focused.clone();
        let focused_key = self.inner.focused_key.clone();
        let hide = popup.clone();
        let hide_extras = self.inner.extras.clone();
        // Weak: a keymap closure must not keep the surface alive (cycle).
        let inner = Rc::downgrade(&self.inner);

        nvim::buf_keymap_table(popup.buffer(), "n", Key::bindings(), move |code| {
            let Some(key) = Key::from_code(code) else { return };
            // The focused widget gets first refusal on every key (tables and
            // text areas use Tab; dropdowns use Esc).
            if Self::route(&focusables, &focused, key) {
                return;
            }
            match key {
                Key::Tab | Key::BackTab => {
                    let n = focusables.borrow().len();
                    if n == 0 {
                        return;
                    }
                    let wrap = inner.upgrade().map(|s| s.focus_wrap.get()).unwrap_or(true);
                    let cur = focused.get_untracked().min(n - 1);
                    let next = match (key == Key::Tab, wrap) {
                        (true, true) => (cur + 1) % n,
                        (true, false) => (cur + 1).min(n - 1),
                        (false, true) => (cur + n - 1) % n,
                        (false, false) => cur.saturating_sub(1),
                    };
                    // Point the key at the new widget before the repaint, so
                    // focus correction doesn't pull it back to the old one.
                    *focused_key.borrow_mut() = focusables.borrow().get(next).and_then(|f| f.key.clone());
                    focused.set(next);
                },
                Key::Esc => Self::hide_popup(&hide, &hide_extras),
                _ => {},
            }
        })
    }
}

/// The wheel keys and the (rows, columns) one step scrolls.
const WHEEL_KEYS: &[(&str, i32, i32)] = &[
    ("<ScrollWheelDown>", 1, 0),
    ("<ScrollWheelUp>", -1, 0),
    ("<ScrollWheelRight>", 0, 1),
    ("<ScrollWheelLeft>", 0, -1),
];

/// Bind the wheel keys on `buf` (once) to [`on_wheel`].
fn install_wheel(buf: crate::nvim::Buffer, popup: &Popup, extras: &Rc<Extras>) -> Result<()> {
    if extras.wheel_bufs.borrow().contains(&buf.id()) {
        return Ok(());
    }
    extras.wheel_bufs.borrow_mut().push(buf.id());
    for &(lhs, rows, cols) in WHEEL_KEYS {
        for mode in ["n", "i"] {
            let (popup, extras) = (popup.clone(), Rc::downgrade(extras));
            nvim::buf_keymap(buf, mode, lhs, nvim::KeymapOpts::default(), move || {
                if let Some(extras) = extras.upgrade() {
                    on_wheel(&popup, &extras, lhs, rows, cols);
                }
            })?;
        }
    }
    Ok(())
}

/// Lines (vertical) and columns (horizontal) one wheel step scrolls, from
/// 'mousescroll' (default `ver:3,hor:6`).
fn wheel_steps() -> (i32, i32) {
    let (mut ver, mut hor) = (3, 6);
    if let Ok(opt) = lua::get_option::<String>("mousescroll") {
        for part in opt.split(',') {
            match part.split_once(':') {
                Some(("ver", n)) => ver = n.parse().unwrap_or(ver),
                Some(("hor", n)) => hor = n.parse().unwrap_or(hor),
                _ => {},
            }
        }
    }
    (ver, hor)
}

/// A wheel step: scroll whatever is under the pointer — a widget of this
/// surface (the innermost scroll region), one of its embedded windows
/// (natively) — or, outside the surface, do what Neovim would (nothing at
/// all when the surface is modal).
fn on_wheel(popup: &Popup, extras: &Extras, key: &str, rows: i32, cols: i32) {
    let Ok(pos) = lua::call_fn::<mlua::Table>("getmousepos", ()) else { return };
    let winid: i64 = pos.get("winid").unwrap_or(0);
    let (ver, hor) = wheel_steps();
    if popup.window().map(Window::id) == Some(winid) {
        if rows == 0 {
            return;
        }
        let inset = popup.border_inset();
        let y = pos.get::<i64>("winrow").unwrap_or(0) - 1 - inset;
        let x = pos.get::<i64>("wincol").unwrap_or(0) - 1 - inset;
        if x < 0 || y < 0 {
            return;
        }
        extras.scroll_at(y as u16, x as u16, rows * ver);
    } else if extras.embeds.borrow().contains(Window(winid)) {
        if let Ok(f) = lua::driver_fn("scroll_win") {
            let _ = f.call::<()>((winid, rows * ver, cols * hor));
        }
    } else if !extras.modal.get()
        && let Ok(keys) = lua::call_api::<String>("nvim_replace_termcodes", (key, true, false, true))
    {
        // Not ours: let Neovim scroll whatever is there.
        let _ = lua::call_api::<()>("nvim_feedkeys", (keys, "n", false));
    }
}
