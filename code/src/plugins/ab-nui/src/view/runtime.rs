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
//! [`effect`], so any `Signal`/`Store` it reads triggers a repaint on change.
//! A repaint only sends Neovim the rows that actually changed, and several
//! state changes made by one key press or timer tick produce a single repaint.
//! Keys are captured by buffer-local maps set once and routed to the focused
//! widget; `<Tab>`/`<S-Tab>` move focus, `<Esc>` hides (non-destructive).

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use crate::error::Result;
use crate::nvim;
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

struct SurfaceInner {
    popup: Popup,
    focused: Signal<usize>,
    focusables: Rc<RefCell<Vec<Focusable>>>,
    effect: RefCell<Option<EffectHandle>>,
    mounted: Cell<bool>,
    /// Bumped on editor resize so the render effect re-measures the tree.
    epoch: Signal<u64>,
    /// The surface's autocmd group (cleanup + resize), deleted on free.
    group: Cell<Option<i64>>,
    /// The focus key of the focused widget (when it has one), so focus can
    /// re-attach by identity after the tree changes shape. (`Rc` so the render
    /// effect can hold it without holding the whole surface.)
    focused_key: Rc<RefCell<Option<Rc<str>>>>,
    /// Whether Tab at the last widget wraps to the first (default true).
    focus_wrap: Cell<bool>,
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

impl SurfaceInner {
    /// Fully tear down: dispose the effect, delete the autocmd group, close the
    /// window, wipe the buffer.
    fn free(&self) {
        if let Some(handle) = self.effect.borrow_mut().take() {
            handle.dispose();
        }
        if let Some(group) = self.group.take() {
            let _ = nvim::del_augroup(group);
        }
        self.popup.close();
        let _ = self.popup.buffer().delete();
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
        // The buffer/state must survive when the window closes (hide/reopen);
        // it is deleted explicitly on close/drop.
        let popup = Popup::new(PopupOptions { persistent_buffer: true, ..opts })?;
        Ok(Self {
            inner: Rc::new(SurfaceInner {
                popup,
                focused: Signal::new(0),
                focusables: Rc::new(RefCell::new(Vec::new())),
                effect: RefCell::new(None),
                mounted: Cell::new(false),
                epoch: Signal::new(0),
                group: Cell::new(None),
                focused_key: Rc::new(RefCell::new(None)),
                focus_wrap: Cell::new(true),
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
        self.inner.popup.on_key("n", lhs, f)
    }

    /// Whether the window is currently open.
    pub fn is_visible(&self) -> bool {
        self.inner.popup.is_open()
    }

    /// Mount the tree (once) and open the window. Returns the owning handle —
    /// keep it alive for as long as you want the UI to exist.
    #[must_use = "the Surface owns the UI; drop it and the window closes and frees"]
    pub fn show<W: Widget + 'static>(self, build: impl Fn() -> W + 'static) -> Result<Self> {
        self.mount(build)?;
        self.reopen()?;
        Ok(self)
    }

    /// Reopen the window after [`hide`](Self::hide) (state is intact). No-op if
    /// already visible.
    pub fn reopen(&self) -> Result<()> {
        if !self.inner.popup.is_open() {
            self.inner.popup.open()?;
            if let Some(win) = self.inner.popup.window() {
                let _ = win.set_option("cursorline", false);
            }
            // Re-measure at the real window size (splits only know it now).
            self.inner.epoch.update(|n| *n += 1);
        }
        Ok(())
    }

    /// Hide the window without destroying anything — state, effect and key maps
    /// all survive, so [`reopen`](Self::reopen) brings it back unchanged.
    pub fn hide(&self) {
        self.inner.popup.hide();
    }

    /// Toggle visibility (hide if shown, reopen if hidden).
    pub fn toggle(&self) -> Result<()> {
        if self.is_visible() {
            self.hide();
            Ok(())
        } else {
            self.reopen()
        }
    }

    /// Permanently free the surface (window, buffer, effect). Equivalent to
    /// dropping the last handle.
    pub fn close(&self) {
        self.inner.free();
    }

    /// Set up the render effect + key routing exactly once.
    fn mount<W: Widget + 'static>(&self, build: impl Fn() -> W + 'static) -> Result<()> {
        if self.inner.mounted.replace(true) {
            return Ok(());
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

        // One canvas for the surface's whole life: each frame resets it
        // rather than allocating a new grid.
        let mut canvas = Canvas::new(0, 0);

        effect(move || {
            epoch.get(); // subscribe: a resize bump re-measures at the new size
            let (w, h) = popup.inner_size().unwrap_or((1, 1));
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
            *focusables.borrow_mut() = cx.focusables;

            // Diffed against the previous frame: only changed rows reach
            // Neovim, in one call.
            let _ = popup.render_rows(canvas.to_rows());
        })
    }

    /// Install the surface's autocmds: dispose the effect if the buffer is
    /// wiped externally (safety net; normal teardown goes through `free`), and
    /// relayout + re-measure on editor resize.
    fn install_cleanup(&self, handle: EffectHandle) -> Result<()> {
        let id = handle.id();
        let group = nvim::unique_augroup("AbNuiSurface")?;
        self.inner.group.set(Some(group));
        nvim::buf_autocmd(&["BufWipeout"], self.inner.popup.buffer(), Some(group), move || {
            dispose_effect(id);
        })?;
        {
            let popup = self.inner.popup.clone();
            let epoch = self.inner.epoch.clone();
            nvim::autocmd(&["VimResized", "WinResized"], None, Some(group), move |_| {
                let _ = popup.relayout();
                epoch.update(|n| *n += 1);
            })?;
        }
        Ok(())
    }

    /// Route a key to the currently focused widget (index clamped defensively).
    /// Returns whether the widget consumed it.
    fn route(
        focusables: &RefCell<Vec<Focusable>>,
        focused: &Signal<usize>,
        key: Key,
    ) -> bool {
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

    /// Install the key table (once): every key a widget can receive is bound
    /// to one dispatcher, which routes it to the focused widget and falls back
    /// to focus traversal (`<Tab>`/`<S-Tab>`) or hiding (`<Esc>`).
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
                    // Remember the new widget's key so focus can follow it.
                    *focused_key.borrow_mut() = focusables.borrow().get(next).and_then(|f| f.key.clone());
                    focused.set(next);
                }
                Key::Esc => hide.hide(),
                _ => {}
            }
        })
    }
}
