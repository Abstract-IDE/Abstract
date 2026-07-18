//! `Popup` — the foundational widget: a scratch buffer shown in a floating
//! window, with reactive content and Rust keymaps. Every other widget builds
//! on it.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use crate::error::{Error, Result};
use crate::geometry::{Position, Rect, Size};
use crate::nvim::{self, Border, Buffer, FloatConfig, KeymapOpts, Namespace, TitlePos, Window};
use crate::reactive::{EffectHandle, effect};
use crate::text::Line;

/// What kind of window hosts the popup's buffer.
#[derive(Clone, Copy, Debug, Default)]
pub enum Host {
    /// A floating window (the default).
    #[default]
    Float,
    /// A split window (`size`/`position`/`border` options are ignored).
    Split(nvim::SplitConfig),
}

/// Declarative configuration for a [`Popup`].
#[derive(Clone, Debug)]
pub struct PopupOptions {
    /// Float or split (floats honor `size`/`position`/`border`/`zindex`).
    pub host: Host,
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
    /// Keep the buffer alive when the window closes (`bufhidden = "hide"`), so
    /// the popup can be re-opened with its content intact. The buffer is still
    /// deleted on [`Popup::close`]. When `false` (default) the buffer is wiped
    /// as soon as the window closes.
    pub persistent_buffer: bool,
}

impl Default for PopupOptions {
    fn default() -> Self {
        Self {
            host: Host::Float,
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
            persistent_buffer: false,
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
    /// Reactive effects owned by this popup, disposed on [`Popup::close`].
    effects: Rc<RefCell<Vec<EffectHandle>>>,
    /// The per-instance autocmd group, deleted on [`Popup::close`].
    group: Rc<Cell<Option<i64>>>,
    /// Whether the `VimResized` relayout autocmd is installed.
    resize_installed: Rc<Cell<bool>>,
    /// Shared (all clones see updates — e.g. [`Popup::set_position`]).
    opts: Rc<RefCell<PopupOptions>>,
}

impl Popup {
    /// Create a popup (buffer + namespace) without opening the window yet.
    pub fn new(opts: PopupOptions) -> Result<Self> {
        let buf = Buffer::scratch()?;
        buf.set_option("bufhidden", if opts.persistent_buffer { "hide" } else { "wipe" })?;
        let ns = Namespace::create("ab_nui_popup")?;
        Ok(Self {
            buf,
            ns,
            win: Rc::new(RefCell::new(None)),
            effects: Rc::new(RefCell::new(Vec::new())),
            group: Rc::new(Cell::new(None)),
            resize_installed: Rc::new(Cell::new(false)),
            opts: Rc::new(RefCell::new(opts)),
        })
    }

    /// The popup's per-instance autocmd group (created on first use).
    fn augroup(&self) -> Result<i64> {
        if let Some(g) = self.group.get() {
            return Ok(g);
        }
        let g = nvim::unique_augroup("AbNuiPopup")?;
        self.group.set(Some(g));
        Ok(g)
    }

    /// The managed buffer.
    pub fn buffer(&self) -> Buffer {
        self.buf
    }

    /// The current window, if open.
    pub fn window(&self) -> Option<Window> {
        *self.win.borrow()
    }

    /// The content area size in cells `(width, height)`. Floats resolve from
    /// options; splits report the live window size (options once open).
    pub fn inner_size(&self) -> Result<(u16, u16)> {
        let opts = self.opts.borrow();
        if let Host::Split(cfg) = opts.host {
            if let Some(win) = self.window()
                && win.is_valid()
            {
                return Ok((win.width()? as u16, win.height()? as u16));
            }
            // Not open yet: a rough estimate; the real size arrives on open.
            let (cols, lines) = crate::geometry::editor_size()?;
            let size = cfg.size.unwrap_or(0).max(1) as u16;
            return Ok(match cfg.dir {
                nvim::SplitDir::Left | nvim::SplitDir::Right => (size.min(cols as u16), lines as u16),
                nvim::SplitDir::Above | nvim::SplitDir::Below => (cols as u16, size.min(lines as u16)),
            });
        }
        let rect = Rect::resolve(opts.size, opts.position)?;
        Ok((rect.width as u16, rect.height as u16))
    }

    /// Move the popup (all clones see the change; the open window is
    /// repositioned immediately).
    pub fn set_position(&self, position: Position) -> Result<()> {
        self.opts.borrow_mut().position = position;
        self.relayout()
    }

    /// Resize the popup (the open window is resized immediately).
    pub fn set_size(&self, size: Size) -> Result<()> {
        self.opts.borrow_mut().size = size;
        self.relayout()
    }

    /// Whether the popup is currently open.
    pub fn is_open(&self) -> bool {
        self.window().map(Window::is_valid).unwrap_or(false)
    }

    fn float_config(&self) -> Result<FloatConfig> {
        let opts = self.opts.borrow();
        let rect = Rect::resolve(opts.size, opts.position)?;
        Ok(FloatConfig {
            rect,
            relative: nvim::Relative::Editor,
            anchor: nvim::Anchor::NW,
            bufpos: None,
            border: opts.border,
            title: opts.title.clone(),
            title_pos: opts.title_pos,
            focusable: opts.focusable,
            zindex: opts.zindex,
            enter: opts.enter,
            minimal: opts.minimal,
        })
    }

    /// Open the window (no-op if already open).
    pub fn open(&self) -> Result<()> {
        if self.is_open() {
            return Ok(());
        }
        let host = self.opts.borrow().host;
        let win = match host {
            Host::Float => Window::open_float(self.buf, &self.float_config()?)?,
            Host::Split(cfg) => Window::open_split(self.buf, &cfg)?,
        };
        win.apply_default_highlight()?;
        win.set_option("cursorline", true)?;
        win.set_option("wrap", false)?;
        *self.win.borrow_mut() = Some(win);

        // Keep `is_open()` truthful if the user closes the window externally
        // (`:q`, `:close`): clear the stored handle when THIS window goes away.
        {
            let group = self.augroup()?;
            let slot = self.win.clone();
            let id = win.id();
            nvim::autocmd_once(&["WinClosed"], Some(&id.to_string()), Some(group), move |_| {
                let mut w = slot.borrow_mut();
                if w.map(Window::id) == Some(id) {
                    *w = None;
                }
            })?;
        }

        // Reposition on editor resize (installed once per popup instance).
        if !self.resize_installed.replace(true) {
            let group = self.augroup()?;
            let this = self.clone();
            nvim::autocmd(&["VimResized"], None, Some(group), move |_| {
                let _ = this.relayout();
            })?;
        }

        if self.opts.borrow().close_on_leave {
            self.install_close_on_leave()?;
        }
        Ok(())
    }

    /// Recompute geometry and reposition an open window (e.g. after a resize).
    /// Splits are laid out by Neovim, so this is float-only.
    pub fn relayout(&self) -> Result<()> {
        if matches!(self.opts.borrow().host, Host::Split(_)) {
            return Ok(());
        }
        if let Some(win) = self.window()
            && win.is_valid()
        {
            let config = self.float_config()?;
            win.set_config(&config)?;
        }
        Ok(())
    }

    /// Close the window WITHOUT tearing anything down: effects, keymaps and
    /// (with `persistent_buffer`) the buffer all survive, so [`Popup::open`]
    /// brings the popup back intact. With the default wiped buffer this is
    /// equivalent to a close.
    pub fn hide(&self) {
        if let Some(win) = self.win.borrow_mut().take()
            && win.is_valid()
        {
            let _ = win.close();
        }
    }

    /// Fully tear the popup down: dispose every owned reactive effect, delete
    /// the per-instance autocmd group, and close the window. Idempotent.
    pub fn close(&self) {
        for handle in self.effects.borrow_mut().drain(..) {
            handle.dispose();
        }
        if let Some(g) = self.group.take() {
            let _ = nvim::del_augroup(g);
        }
        self.resize_installed.set(false);
        self.hide();
    }

    /// A reusable closer suitable for capturing into keymap callbacks.
    /// Performs the same full teardown as [`Popup::close`].
    pub fn closer(&self) -> impl Fn() + 'static {
        let this = self.clone();
        move || this.close()
    }

    /// Set static content once.
    pub fn set_content(&self, lines: &[Line]) -> Result<()> {
        nvim::render(self.buf, self.ns, lines)
    }

    /// Bind content to reactive state: `f` is re-run (and the buffer re-rendered)
    /// whenever any signal it reads changes. This is the main way to drive a
    /// popup from a [`crate::reactive::Signal`].
    ///
    /// The effect is owned by the popup and disposed on [`Popup::close`]; the
    /// handle is also returned for callers that manage it themselves.
    pub fn render_reactive(&self, f: impl Fn() -> Vec<Line> + 'static) -> EffectHandle {
        let buf = self.buf;
        let ns = self.ns;
        let handle = effect(move || {
            let lines = f();
            let _ = nvim::render(buf, ns, &lines);
        });
        self.effects.borrow_mut().push(handle);
        handle
    }

    /// Register an extra reactive effect to be disposed with this popup (for
    /// composite widgets that keep their own effects alongside the render one).
    pub fn own_effect(&self, handle: EffectHandle) {
        self.effects.borrow_mut().push(handle);
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
        let group = self.augroup()?;
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
