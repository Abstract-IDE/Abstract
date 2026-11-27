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
        let popup = Popup::new(opts)?;
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
            }),
        })
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
        if let Some(win) = popup.window() {
            let _ = win.set_option("cursorline", false);
        }
        if self.inner.unloaded.replace(false) {
            self.remount()?;
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
        Self::hide_popup(&self.inner.popup);
    }

    fn hide_popup(popup: &Popup) {
        let Some(win) = popup.window().filter(|w| w.is_valid()) else {
            popup.hide(); // nothing shown; just drop any stale handle
            return;
        };
        if !can_close(win) {
            return;
        }
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
                *focusables.borrow_mut() = cx.focusables;
                painted_size.set(size);

                // Diffed against the previous frame: only changed rows reach
                // Neovim, in one call.
                let _ = popup.render_rows(canvas.to_rows());
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
        nvim::buf_autocmd(&["BufWipeout"], buf, Some(group), move || {
            dispose_effect(id);
        })?;
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
            nvim::autocmd(&["VimResized", "WinResized"], None, Some(group), move |args| {
                if !popup.is_open() {
                    return;
                }
                if args.event == "VimResized" {
                    let _ = popup.relayout();
                }
                sync_size(&popup, &painted, &epoch);
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
                Key::Esc => Self::hide_popup(&hide),
                _ => {},
            }
        })
    }
}
