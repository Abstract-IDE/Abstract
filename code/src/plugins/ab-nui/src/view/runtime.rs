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
}

impl SurfaceInner {
    /// Fully tear down: dispose the effect, close the window, wipe the buffer.
    fn free(&self) {
        if let Some(handle) = self.effect.borrow_mut().take() {
            handle.dispose();
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
    /// Create a floating surface from popup options (not shown yet).
    pub fn float(opts: PopupOptions) -> Result<Self> {
        let popup = Popup::new(opts)?;
        // `hide` (not `wipe`) so the buffer/state survive when the window closes;
        // the buffer is wiped explicitly on close/drop.
        popup.buffer().set_option("bufhidden", "hide")?;
        Ok(Self {
            inner: Rc::new(SurfaceInner {
                popup,
                focused: Signal::new(0),
                focusables: Rc::new(RefCell::new(Vec::new())),
                effect: RefCell::new(None),
                mounted: Cell::new(false),
            }),
        })
    }

    /// The underlying popup (for extra styling).
    pub fn popup(&self) -> &Popup {
        &self.inner.popup
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
        }
        Ok(())
    }

    /// Hide the window without destroying anything — state, effect and key maps
    /// all survive, so [`reopen`](Self::reopen) brings it back unchanged.
    pub fn hide(&self) {
        self.inner.popup.close();
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

        effect(move || {
            let (w, h) = popup.inner_size().unwrap_or((1, 1));
            let mut canvas = Canvas::new(w, h);
            let mut cx = Cx::new(focused.get());

            let root = build(); // reads signals → this effect re-runs on change
            let area = Area { x: 0, y: 0, w, h };
            root.measure(Constraints { max_w: w, max_h: h });
            root.paint(&mut cx, area, &mut canvas);

            *focusables.borrow_mut() = cx.focusables;

            // Every printable key is mapped to our dispatcher, so the buffer
            // can't be edited; no need to toggle it read-only each frame.
            let lines = canvas.to_lines();
            let _ = popup.set_content(&lines);
        })
    }

    /// Dispose the effect if the buffer is wiped externally (safety net; normal
    /// teardown goes through `free`).
    fn install_cleanup(&self, handle: EffectHandle) -> Result<()> {
        let id = handle.id();
        let group = nvim::augroup(&format!("AbNuiSurface{id}"))?;
        nvim::buf_autocmd(&["BufWipeout"], self.inner.popup.buffer(), Some(group), move || {
            dispose_effect(id);
        })?;
        Ok(())
    }

    /// A handler that routes a key to the currently focused widget.
    fn dispatcher(&self) -> impl Fn(Key) + 'static {
        let focusables = self.inner.focusables.clone();
        let focused = self.inner.focused.clone();
        move |key| {
            let i = focused.get_untracked();
            let handle = focusables.borrow().get(i).map(|f| f.handle.clone());
            if let Some(h) = handle {
                h(key);
            }
        }
    }

    /// Install focus traversal + the key dispatch table (once).
    fn install_keys(&self) -> Result<()> {
        let popup = &self.inner.popup;

        // Focus traversal.
        {
            let f = self.inner.focused.clone();
            let fc = self.inner.focusables.clone();
            popup.on_key("n", "<Tab>", move || {
                let n = fc.borrow().len();
                if n > 0 {
                    f.update(|s| *s = (*s + 1) % n);
                }
            })?;
        }
        {
            let f = self.inner.focused.clone();
            let fc = self.inner.focusables.clone();
            popup.on_key("n", "<S-Tab>", move || {
                let n = fc.borrow().len();
                if n > 0 {
                    f.update(|s| *s = (*s + n - 1) % n);
                }
            })?;
        }

        // Esc: let the focused widget consume it, otherwise hide the surface.
        {
            let fc = self.inner.focusables.clone();
            let focused = self.inner.focused.clone();
            let p = popup.clone();
            popup.on_key("n", "<Esc>", move || {
                let i = focused.get_untracked();
                let handled = fc.borrow().get(i).map(|f| (f.handle)(Key::Esc)).unwrap_or(false);
                if !handled {
                    p.close();
                }
            })?;
        }

        // Named keys.
        let named: &[(&str, Key)] = &[
            ("<CR>", Key::Enter),
            ("<BS>", Key::Backspace),
            ("<Space>", Key::Char(' ')),
            ("<Left>", Key::Left),
            ("<Right>", Key::Right),
            ("<Up>", Key::Up),
            ("<Down>", Key::Down),
            ("<Home>", Key::Home),
            ("<End>", Key::End),
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

        Ok(())
    }
}
