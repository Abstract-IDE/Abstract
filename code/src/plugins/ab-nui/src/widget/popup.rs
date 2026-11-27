//! `Popup` — the foundational widget: a scratch buffer shown in a floating
//! window, with reactive content and Rust keymaps. Every other widget builds
//! on it.
//!
//! ## Lifetime
//!
//! * [`Popup::open`] shows the window; [`Popup::hide`] closes the window but
//!   keeps everything else (buffer, content, effects, keymaps), so `open`
//!   brings it back intact. Closing the window some other way (`:q`,
//!   `<C-w>c`, another plugin) is a hide too.
//! * [`Popup::close`] is the final teardown: effects disposed, autocmds
//!   deleted, window closed and the popup's own buffer deleted (a buffer
//!   handed in with [`Popup::with_buffer`] is never deleted). A closed popup
//!   cannot be opened again.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use crate::error::{Error, Result};
use crate::geometry::{Position, Rect, Size};
use crate::nvim::{self, Border, Buffer, FloatConfig, KeymapOpts, Namespace, Renderer, Row, TitlePos, Window};
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
    /// [`Popup::close`] automatically when the cursor leaves the popup's
    /// window (`WinLeave`).
    pub close_on_leave: bool,
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
        }
    }
}

/// See [`Popup::set_on_dismiss`].
type DismissHook = Rc<RefCell<Option<Rc<dyn Fn()>>>>;

/// A floating popup window over a managed scratch buffer.
///
/// Cloning a `Popup` yields another handle to the *same* window/buffer (the
/// window handle is shared), so it is cheap to pass into callbacks.
#[derive(Clone)]
pub struct Popup {
    buf: Buffer,
    ns: Namespace,
    /// Paints into `buf`, remembering the last frame so repaints only send
    /// what changed.
    renderer: Rc<Renderer>,
    win: Rc<RefCell<Option<Window>>>,
    /// Reactive effects owned by this popup, disposed on [`Popup::close`].
    effects: Rc<RefCell<Vec<EffectHandle>>>,
    /// The per-instance autocmd group, deleted on [`Popup::close`].
    group: Rc<Cell<Option<i64>>>,
    /// Whether the per-instance autocmds (`VimResized` relayout and, with
    /// `close_on_leave`, `WinLeave`) are installed. Once per group lifetime,
    /// however often the popup is re-opened.
    hooks_installed: Rc<Cell<bool>>,
    /// What to do, instead of the default, when the window is closed or (with
    /// `close_on_leave`) left by anything but this popup. Run on the next
    /// tick, outside the autocmd. See [`Popup::set_on_dismiss`].
    on_dismiss: DismissHook,
    /// Shared (all clones see updates — e.g. [`Popup::set_position`]).
    opts: Rc<RefCell<PopupOptions>>,
    /// `false` for a popup made by [`Popup::with_buffer`]: the buffer belongs
    /// to the caller, so the popup leaves its options alone.
    owns_buffer: bool,
}

impl Popup {
    /// Create a popup (buffer + namespace) without opening the window yet.
    /// The buffer lives until [`Popup::close`] (`bufhidden = "hide"`).
    pub fn new(opts: PopupOptions) -> Result<Self> {
        let buf = Buffer::scratch()?;
        buf.set_option("bufhidden", "hide")?;
        Self::build(buf, opts, true)
    }

    /// Create a popup that shows a buffer the caller already has (a terminal,
    /// a file, another plugin's buffer) instead of a scratch buffer of its own.
    ///
    /// The popup only manages the window: it does not touch the buffer's
    /// options (`bufhidden` included) or the window's `cursorline`/`wrap`, and
    /// closing the popup never deletes the buffer. Don't paint into it with
    /// [`Popup::set_content`] unless that is really what you want.
    pub fn with_buffer(buf: Buffer, opts: PopupOptions) -> Result<Self> {
        if !buf.is_valid() {
            return Err(Error::invalid(format!("buffer {} is not valid", buf.id())));
        }
        Self::build(buf, opts, false)
    }

    fn build(buf: Buffer, opts: PopupOptions, owns_buffer: bool) -> Result<Self> {
        let ns = Namespace::create("ab_nui_popup")?;
        Ok(Self {
            buf,
            ns,
            renderer: Rc::new(Renderer::new(buf, ns)),
            win: Rc::new(RefCell::new(None)),
            effects: Rc::new(RefCell::new(Vec::new())),
            group: Rc::new(Cell::new(None)),
            hooks_installed: Rc::new(Cell::new(false)),
            on_dismiss: Rc::new(RefCell::new(None)),
            opts: Rc::new(RefCell::new(opts)),
            owns_buffer,
        })
    }

    /// The popup's per-instance autocmd group (created on first use, deleted
    /// on [`Popup::close`]) — for components that hang their own autocmds on
    /// the popup's lifetime.
    pub(crate) fn augroup(&self) -> Result<i64> {
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

    /// The content area size in cells `(width, height)`: the live window's
    /// size once open, otherwise what the options resolve to (the border
    /// excluded).
    pub fn inner_size(&self) -> Result<(u16, u16)> {
        if let Some(win) = self.window()
            && win.is_valid()
        {
            return Ok((win.width()? as u16, win.height()? as u16));
        }
        let opts = self.opts.borrow();
        if let Host::Split(cfg) = opts.host {
            // Not open yet: a rough estimate; the real size arrives on open.
            let (cols, lines) = crate::geometry::editor_size()?;
            let size = cfg.size.unwrap_or(0).max(1) as u16;
            return Ok(match cfg.dir {
                nvim::SplitDir::Left | nvim::SplitDir::Right => (size.min(cols as u16), lines as u16),
                nvim::SplitDir::Above | nvim::SplitDir::Below => (cols as u16, size.min(lines as u16)),
            });
        }
        let rect = Rect::resolve_framed(opts.size, opts.position, opts.border.frame())?;
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

    /// Change the title on the top border (the open window is updated
    /// immediately). Floats only, and only shown when the popup has a border.
    /// An empty title clears it, like [`Popup::clear_title`].
    pub fn set_title(&self, title: impl Into<String>) -> Result<()> {
        let title = title.into();
        self.opts.borrow_mut().title = (!title.is_empty()).then_some(title);
        self.relayout()
    }

    /// Remove the title from the top border.
    pub fn clear_title(&self) -> Result<()> {
        self.opts.borrow_mut().title = None;
        self.relayout()
    }

    /// Whether the popup is currently open.
    pub fn is_open(&self) -> bool {
        self.window().map(Window::is_valid).unwrap_or(false)
    }

    fn float_config(&self) -> Result<FloatConfig> {
        let opts = self.opts.borrow();
        let rect = Rect::resolve_framed(opts.size, opts.position, opts.border.frame())?;
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

    /// Open the window (no-op if already open). Fails once the popup has been
    /// [closed](Popup::close).
    pub fn open(&self) -> Result<()> {
        if self.is_open() {
            return Ok(());
        }
        if !self.buf.is_valid() {
            return Err(Error::invalid("the popup was closed (its buffer is gone)"));
        }
        let host = self.opts.borrow().host;
        let win = match host {
            Host::Float => Window::open_float(self.buf, &self.float_config()?)?,
            Host::Split(cfg) => Window::open_split(self.buf, &cfg)?,
        };
        win.apply_default_highlight()?;
        if self.owns_buffer {
            win.set_option("cursorline", true)?;
            win.set_option("wrap", false)?;
        }
        *self.win.borrow_mut() = Some(win);

        // Keep `is_open()` truthful if the window is closed externally (`:q`,
        // `<C-w>c`, another plugin): clear the stored handle when THIS window
        // goes away. (`hide`/`close` take the handle out first, so for them
        // this is a no-op.)
        {
            let group = self.augroup()?;
            let slot = self.win.clone();
            let on_dismiss = self.on_dismiss.clone();
            let id = win.id();
            nvim::autocmd_once(&["WinClosed"], Some(&id.to_string()), Some(group), move |_| {
                let external = {
                    let mut w = slot.borrow_mut();
                    let external = w.map(Window::id) == Some(id);
                    if external {
                        *w = None;
                    }
                    external
                };
                if external {
                    dismiss_later(&on_dismiss);
                }
            })?;
        }

        if !self.hooks_installed.replace(true) {
            let group = self.augroup()?;
            // Reposition on editor resize.
            let this = self.clone();
            nvim::autocmd(&["VimResized"], None, Some(group), move |_| {
                let _ = this.relayout();
            })?;
            if self.opts.borrow().close_on_leave {
                self.install_close_on_leave(group)?;
            }
        }
        Ok(())
    }

    /// Replace what happens when the window is closed (`:q`, `<C-w>c`,
    /// `nvim_win_close` from another plugin) or, with `close_on_leave`, left —
    /// anything but [`Popup::hide`]/[`Popup::close`]. `f` runs on the next
    /// tick (outside the autocmd) and typically ends with [`Popup::close`].
    /// Without it, an external close is a hide and leaving closes.
    ///
    /// Used by components that must answer exactly once (Menu, Input).
    /// Cleared by [`Popup::close`].
    pub(crate) fn set_on_dismiss(&self, f: impl Fn() + 'static) {
        *self.on_dismiss.borrow_mut() = Some(Rc::new(f));
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

    /// Close the window WITHOUT tearing anything down: the buffer and its
    /// content, effects and keymaps all survive, so [`Popup::open`] brings the
    /// popup back intact.
    pub fn hide(&self) {
        // Take the handle out before closing: closing fires this popup's own
        // `WinClosed` autocmd, which borrows `self.win` again.
        let win = self.win.borrow_mut().take();
        if let Some(win) = win
            && win.is_valid()
        {
            let _ = win.close();
        }
    }

    /// Fully tear the popup down: dispose every owned reactive effect, delete
    /// the per-instance autocmd group, close the window and delete the popup's
    /// own buffer (never one given to [`Popup::with_buffer`]). Idempotent; the
    /// popup cannot be re-opened afterwards.
    pub fn close(&self) {
        let effects: Vec<EffectHandle> = self.effects.borrow_mut().drain(..).collect();
        for handle in effects {
            handle.dispose();
        }
        // Drop the dismiss hook: it usually holds a clone of this popup.
        let dismiss = self.on_dismiss.borrow_mut().take();
        drop(dismiss);
        // The group goes first, so closing the window below does not look
        // like an external close.
        if let Some(g) = self.group.take() {
            let _ = nvim::del_augroup(g);
        }
        self.hooks_installed.set(false);
        self.hide();
        if self.owns_buffer && self.buf.is_valid() {
            let _ = self.buf.delete();
        }
    }

    /// A reusable closer suitable for capturing into keymap callbacks.
    /// Performs the same full teardown as [`Popup::close`].
    pub fn closer(&self) -> impl Fn() + 'static {
        let this = self.clone();
        move || this.close()
    }

    /// The highlight namespace the popup paints with.
    pub fn namespace(&self) -> Namespace {
        self.ns
    }

    /// Whether a user can end up with the cursor in this popup's window (it
    /// is focusable, or opens with the cursor in it).
    pub(crate) fn takes_input(&self) -> bool {
        let opts = self.opts.borrow();
        opts.focusable || opts.enter
    }

    /// Replace the popup's content. Only the rows that differ from what the
    /// popup last painted are sent to Neovim.
    pub fn set_content(&self, lines: &[Line]) -> Result<()> {
        self.render_rows(lines.iter().map(Row::from_line).collect())
    }

    /// Replace the popup's content with pre-flattened rows (what a
    /// [`Canvas`](crate::view::canvas::Canvas) produces).
    pub fn render_rows(&self, rows: Vec<Row>) -> Result<()> {
        self.renderer.render(rows)
    }

    /// Tell the popup its buffer was written to by other means (directly
    /// through [`Popup::buffer`], say), so the next paint starts from scratch
    /// instead of diffing against content that is no longer there.
    pub fn invalidate(&self) {
        self.renderer.invalidate();
    }

    /// Bind content to reactive state: `f` is re-run (and the buffer re-rendered)
    /// whenever any signal it reads changes. This is the main way to drive a
    /// popup from a [`crate::reactive::Signal`].
    ///
    /// The effect is owned by the popup and disposed on [`Popup::close`]; the
    /// handle is also returned for callers that manage it themselves.
    pub fn render_reactive(&self, f: impl Fn() -> Vec<Line> + 'static) -> EffectHandle {
        let renderer = self.renderer.clone();
        // Unowned: the popup's handle decides when this ends, not whichever
        // effect happened to be running when the popup was built.
        let handle = crate::reactive::unowned(|| {
            effect(move || {
                let lines = f();
                let _ = renderer.render(lines.iter().map(Row::from_line).collect());
            })
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

    fn install_close_on_leave(&self, group: i64) -> Result<()> {
        let this = self.clone();
        nvim::buf_autocmd(&["WinLeave"], self.buf, Some(group), move || {
            // Only when leaving THIS popup's window: the buffer may be shown in
            // other windows too (`with_buffer`).
            let Some(win) = this.window() else { return };
            if Window::current().ok() != Some(win) {
                return;
            }
            if this.on_dismiss.borrow().is_some() {
                dismiss_later(&this.on_dismiss);
            } else {
                this.close();
            }
        })?;
        Ok(())
    }

    /// The cursor's (0-based) row in the popup window, if open.
    pub(crate) fn cursor_row(&self) -> Option<usize> {
        let win = self.window().filter(|w| w.is_valid())?;
        win.cursor().ok().map(|(row, _)| (row.max(1) - 1) as usize)
    }

    /// Move the cursor within the popup window, if open.
    pub fn set_cursor(&self, row: i64, col: i64) -> Result<()> {
        match self.window() {
            Some(win) if win.is_valid() => win.set_cursor(row, col),
            _ => Err(Error::invalid("popup is not open")),
        }
    }
}

/// Run the dismiss hook (if any) on the next tick.
fn dismiss_later(hook: &DismissHook) {
    let hook = hook.borrow().clone();
    if let Some(f) = hook {
        let _ = nvim::schedule(move || f());
    }
}
