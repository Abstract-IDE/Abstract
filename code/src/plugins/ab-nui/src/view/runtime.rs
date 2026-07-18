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
//! Keys are captured by buffer-local maps set once and routed to the focused
//! widget; `<Tab>`/`<S-Tab>` move focus, `<Esc>` hides (non-destructive).

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use crate::error::Result;
use crate::nvim;
use crate::reactive::{EffectHandle, Signal, dispose_effect, effect};
use crate::view::canvas::Canvas;
use crate::view::widget::{Area, Constraints, Cx, Focusable, Key, Widget};
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

        effect(move || {
            epoch.get(); // subscribe: a resize bump re-measures at the new size
            let (w, h) = popup.inner_size().unwrap_or((1, 1));
            let mut canvas = Canvas::new(w, h);
            let mut cx = Cx::new(focused.get());

            let root = build(); // reads signals → this effect re-runs on change
            let area = Area { x: 0, y: 0, w, h };
            root.measure(Constraints::loose(w, h));
            root.paint(&mut cx, area, &mut canvas);
            cx.run_overlays(area, &mut canvas); // dropdowns/tooltips on top

            // Focus correction, silently (notifying would re-enter this
            // running effect):
            // 1. if the previously focused widget had a key and the same index
            //    no longer carries it, follow the key to its new position;
            // 2. clamp the index if the interactive set shrank.
            let n = cx.focusables.len();
            let i = focused.get_untracked();
            if let Some(key) = focused_key.borrow().clone() {
                let at_index = cx.focusables.get(i).and_then(|f| f.key.clone());
                if at_index.as_deref() != Some(key.as_ref())
                    && let Some(pos) =
                        cx.focusables.iter().position(|f| f.key.as_deref() == Some(key.as_ref()))
                {
                    focused.set_silent(pos);
                }
            }
            if n > 0 && focused.get_untracked() >= n {
                focused.set_silent(n - 1);
            }
            *focusables.borrow_mut() = cx.focusables;

            // Every printable key is mapped to our dispatcher, so the buffer
            // can't be edited; no need to toggle it read-only each frame.
            let lines = canvas.to_lines();
            let _ = popup.set_content(&lines);
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

    /// A handler that routes a key to the currently focused widget.
    fn dispatcher(&self) -> impl Fn(Key) + 'static {
        let focusables = self.inner.focusables.clone();
        let focused = self.inner.focused.clone();
        move |key| {
            Self::route(&focusables, &focused, key);
        }
    }

    /// Install focus traversal + the key dispatch table (once).
    fn install_keys(&self) -> Result<()> {
        let popup = &self.inner.popup;

        // Focus traversal: offer Tab/S-Tab to the focused widget first (tables
        // and text areas use them); move focus only when unconsumed.
        for (lhs, key, forward) in [("<Tab>", Key::Tab, true), ("<S-Tab>", Key::BackTab, false)] {
            let f = self.inner.focused.clone();
            let fc = self.inner.focusables.clone();
            let fk = self.inner.focused_key.clone();
            // Weak: a keymap closure must not keep the surface alive (cycle).
            let inner = Rc::downgrade(&self.inner);
            popup.on_key("n", lhs, move || {
                if Self::route(&fc, &f, key) {
                    return;
                }
                let n = fc.borrow().len();
                if n == 0 {
                    return;
                }
                let wrap = inner.upgrade().map(|s| s.focus_wrap.get()).unwrap_or(true);
                let cur = f.get_untracked().min(n - 1);
                let next = match (forward, wrap) {
                    (true, true) => (cur + 1) % n,
                    (true, false) => (cur + 1).min(n - 1),
                    (false, true) => (cur + n - 1) % n,
                    (false, false) => cur.saturating_sub(1),
                };
                // Remember the new widget's key so focus can follow it.
                *fk.borrow_mut() = fc.borrow().get(next).and_then(|f| f.key.clone());
                f.set(next);
            })?;
        }

        // Esc: let the focused widget consume it, otherwise hide the surface.
        {
            let fc = self.inner.focusables.clone();
            let focused = self.inner.focused.clone();
            let p = popup.clone();
            popup.on_key("n", "<Esc>", move || {
                if !Self::route(&fc, &focused, Key::Esc) {
                    p.hide();
                }
            })?;
        }

        // Named keys.
        let named: &[(&str, Key)] = &[
            ("<CR>", Key::Enter),
            ("<BS>", Key::Backspace),
            ("<Del>", Key::Delete),
            ("<Space>", Key::Char(' ')),
            ("<Left>", Key::Left),
            ("<Right>", Key::Right),
            ("<Up>", Key::Up),
            ("<Down>", Key::Down),
            ("<Home>", Key::Home),
            ("<End>", Key::End),
            ("<PageUp>", Key::PageUp),
            ("<PageDown>", Key::PageDown),
        ];
        for (lhs, key) in named {
            let key = *key;
            let dispatch = self.dispatcher();
            popup.on_key("n", lhs, move || dispatch(key))?;
        }

        // Printable ASCII, so text fields and char-driven widgets work.
        for byte in 33u8..=126 {
            let ch = byte as char;
            let lhs = if ch == '<' { "<lt>".to_string() } else { ch.to_string() };
            let key = Key::Char(ch);
            let dispatch = self.dispatcher();
            popup.on_key("n", &lhs, move || dispatch(key))?;
        }

        // Ctrl-letters (skip <C-c>: interrupt stays with Neovim).
        for byte in b'a'..=b'z' {
            let ch = byte as char;
            if ch == 'c' {
                continue;
            }
            let lhs = format!("<C-{ch}>");
            let key = Key::Ctrl(ch);
            let dispatch = self.dispatcher();
            popup.on_key("n", &lhs, move || dispatch(key))?;
        }

        Ok(())
    }
}
