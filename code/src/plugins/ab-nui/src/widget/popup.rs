//! `Popup` — the foundational widget: a scratch buffer shown in a floating
//! window, with reactive content and Rust keymaps. Every other widget builds
//! on it.

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::{Error, Result};
use crate::geometry::{Position, Rect, Size};
use crate::nvim::{self, Border, Buffer, FloatConfig, KeymapOpts, Namespace, TitlePos, Window};
use crate::reactive::effect;
use crate::text::Line;

/// Declarative configuration for a [`Popup`].
#[derive(Clone, Debug)]
pub struct PopupOptions {
    pub size: Size,
    pub position: Position,
    pub border: Border,
    pub title: Option<String>,
    pub title_pos: TitlePos,
    pub focusable: bool,
    pub enter: bool,
    pub zindex: u32,
    pub minimal: bool,
    /// Close automatically when the window loses focus (`WinLeave`).
    pub close_on_leave: bool,
}

impl Default for PopupOptions {
    fn default() -> Self {
        Self {
            size: Size::ratio(0.5, 0.4),
            position: Position::Center,
            border: Border::Rounded,
            title: None,
            title_pos: TitlePos::Center,
            focusable: true,
            enter: true,
            zindex: 50,
            minimal: true,
            close_on_leave: false,
        }
    }
}

/// A floating popup window over a managed scratch buffer.
///
/// Cloning a `Popup` yields another handle to the *same* window/buffer (the
/// window handle is shared), so it is cheap to pass into callbacks.
#[derive(Clone)]
pub struct Popup {
    buf: Buffer,
    ns: Namespace,
    win: Rc<RefCell<Option<Window>>>,
    opts: PopupOptions,
}

impl Popup {
    /// Create a popup (buffer + namespace) without opening the window yet.
    pub fn new(opts: PopupOptions) -> Result<Self> {
        let buf = Buffer::scratch()?;
        buf.set_option("bufhidden", "wipe")?;
        let ns = Namespace::create("ab_nui_popup")?;
        Ok(Self { buf, ns, win: Rc::new(RefCell::new(None)), opts })
    }

    /// The managed buffer.
    pub fn buffer(&self) -> Buffer {
        self.buf
    }

    /// The current window, if open.
    pub fn window(&self) -> Option<Window> {
        *self.win.borrow()
    }

    /// The content area size in cells `(width, height)`, resolved from options.
    pub fn inner_size(&self) -> Result<(u16, u16)> {
        let rect = Rect::resolve(self.opts.size, self.opts.position)?;
        Ok((rect.width as u16, rect.height as u16))
    }

    /// Whether the popup is currently open.
    pub fn is_open(&self) -> bool {
        self.window().map(Window::is_valid).unwrap_or(false)
    }

    fn float_config(&self) -> Result<FloatConfig> {
        let rect = Rect::resolve(self.opts.size, self.opts.position)?;
        Ok(FloatConfig {
            rect,
            border: self.opts.border,
            title: self.opts.title.clone(),
            title_pos: self.opts.title_pos,
            focusable: self.opts.focusable,
            zindex: self.opts.zindex,
            enter: self.opts.enter,
            minimal: self.opts.minimal,
        })
    }

    /// Open the window (no-op if already open).
    pub fn open(&self) -> Result<()> {
        if self.is_open() {
            return Ok(());
        }
        let config = self.float_config()?;
        let win = Window::open_float(self.buf, &config)?;
        win.apply_default_highlight()?;
        win.set_option("cursorline", true)?;
        win.set_option("wrap", false)?;
        *self.win.borrow_mut() = Some(win);

        if self.opts.close_on_leave {
            self.install_close_on_leave()?;
        }
        Ok(())
    }

    /// Recompute geometry and reposition an open window (e.g. after a resize).
    pub fn relayout(&self) -> Result<()> {
        if let Some(win) = self.window()
            && win.is_valid()
        {
            let config = self.float_config()?;
            win.set_config(&config)?;
        }
        Ok(())
    }

    /// Close the window. The buffer is wiped automatically (`bufhidden`).
    pub fn close(&self) {
        if let Some(win) = self.win.borrow_mut().take()
            && win.is_valid()
        {
            let _ = win.close();
        }
    }

    /// A reusable closer suitable for capturing into keymap callbacks.
    pub fn closer(&self) -> impl Fn() + 'static {
        let win = self.win.clone();
        move || {
            if let Some(w) = win.borrow_mut().take()
                && w.is_valid()
            {
                let _ = w.close();
            }
        }
    }

    /// Set static content once.
    pub fn set_content(&self, lines: &[Line]) -> Result<()> {
        nvim::render(self.buf, self.ns, lines)
    }

    /// Bind content to reactive state: `f` is re-run (and the buffer re-rendered)
    /// whenever any signal it reads changes. This is the main way to drive a
    /// popup from a [`crate::reactive::Signal`].
    pub fn render_reactive(&self, f: impl Fn() -> Vec<Line> + 'static) {
        let buf = self.buf;
        let ns = self.ns;
        effect(move || {
            let lines = f();
            let _ = nvim::render(buf, ns, &lines);
        });
    }

    /// Bind a buffer-local keymap to a Rust callback.
    pub fn on_key(&self, mode: &str, lhs: &str, callback: impl Fn() + 'static) -> Result<()> {
        nvim::buf_keymap(self.buf, mode, lhs, KeymapOpts::default(), callback)
    }

    /// Convenience: bind `lhs` (in normal mode) to close the popup.
    pub fn on_close_key(&self, lhs: &str) -> Result<()> {
        self.on_key("n", lhs, self.closer())
    }

    fn install_close_on_leave(&self) -> Result<()> {
        let group = nvim::augroup("AbNuiPopupClose")?;
        nvim::buf_autocmd(&["WinLeave"], self.buf, Some(group), self.closer())?;
        Ok(())
    }

    /// Move the cursor within the popup window, if open.
    pub fn set_cursor(&self, row: i64, col: i64) -> Result<()> {
        match self.window() {
            Some(win) if win.is_valid() => win.set_cursor(row, col),
            _ => Err(Error::invalid("popup is not open")),
        }
    }
}
