//! `BufferInput`: a one-line text input backed by a real buffer, embedded in
//! a surface. Unlike [`TextField`](crate::view::TextField) (drawn on the
//! canvas, keys routed by the surface), it is edited with Vim itself — insert
//! and normal mode, every motion and operator, undo, the user's own mappings —
//! and the real cursor is in it.
//!
//! ```ignore
//! let input = BufferInput::new()?.placeholder("Search…");
//! let query = input.value();                      // Signal<String>, follows the text
//! input.on_key(&["i", "n"], "<CR>", move || ..)?; // buffer-local keys
//! Surface::float(opts)?.modal(true).show(move || row![Text::new("❯ "), Expanded::new(input.widget())])?;
//! ```

use std::cell::{Cell, RefCell};
use std::rc::{Rc, Weak};

use crate::error::Result;
use crate::lua;
use crate::nvim::{self, Buffer, ExtmarkOpts, KeymapOpts, Namespace, VirtTextPos, Window};
use crate::reactive::Signal;
use crate::text::{Line, Span};
use crate::theme::groups;
use crate::view::embed::{Embed, EmbedOptions, EmbeddedWindow};

struct Inner {
    win: EmbeddedWindow,
    ns: Namespace,
    value: Signal<String>,
    placeholder: RefCell<Option<String>>,
    start_insert: Cell<bool>,
}

/// A one-line, Vim-edited input. Cloning yields another handle to the same input.
#[derive(Clone)]
pub struct BufferInput(Rc<Inner>);

impl BufferInput {
    pub fn new() -> Result<Self> {
        let win = EmbeddedWindow::new(EmbedOptions { focusable: true, enter: true, zindex: 60 })?;
        let buf = win.buffer();
        // No completion menus while typing (blink.cmp and others honour this).
        lua::call_api::<()>("nvim_buf_set_var", (buf.id(), "completion", false))?;
        buf.set_option("textwidth", 0)?;
        buf.set_option("undolevels", 200)?;

        let this = Self(Rc::new(Inner {
            win,
            ns: Namespace::create("ab_nui_buffer_input")?,
            value: Signal::new(String::new()),
            placeholder: RefCell::new(None),
            start_insert: Cell::new(true),
        }));

        let weak = Rc::downgrade(&this.0);
        nvim::buf_autocmd(&["TextChanged", "TextChangedI", "TextChangedP"], buf, None, move || {
            if let Some(inner) = weak.upgrade() {
                BufferInput(inner).sync();
            }
        })?;

        let weak: Weak<Inner> = Rc::downgrade(&this.0);
        this.0.win.on_open(move |w| {
            let Some(inner) = weak.upgrade() else { return };
            for (name, value) in [("number", false), ("relativenumber", false), ("list", false), ("wrap", false)] {
                let _ = w.set_option(name, value);
            }
            let _ = w.set_option("signcolumn", "no");
            let _ = w.set_option("sidescrolloff", 4);
            let end = inner.value.get_untracked().len() as i64;
            let _ = w.set_cursor(1, end);
            if inner.start_insert.get() && Window::current().is_ok_and(|cur| cur == w) {
                let _ = lua::cmd("startinsert!");
            }
        });
        this.refresh_placeholder();
        Ok(this)
    }

    /// Dimmed text shown while the input is empty.
    pub fn placeholder(self, text: impl Into<String>) -> Self {
        self.set_placeholder(Some(text.into()));
        self
    }

    /// Enter insert mode when the input opens (default `true`).
    pub fn start_insert(self, on: bool) -> Self {
        self.0.start_insert.set(on);
        self
    }

    /// The text, as a signal that follows every edit.
    pub fn value(&self) -> Signal<String> {
        self.0.value.clone()
    }

    /// The text right now (read from the buffer, so it is current even
    /// before the edit has been reported to [`value`](Self::value)).
    pub fn text(&self) -> String {
        self.buffer().get_lines(0, -1).map(|l| l.join(" ")).unwrap_or_else(|_| self.0.value.get_untracked())
    }

    /// Replace the text (the cursor goes to its end).
    pub fn set_text(&self, text: &str) -> Result<()> {
        let one_line = text.replace(['\n', '\r'], " ");
        self.buffer().set_all([one_line.as_str()])?;
        if let Some(win) = self.0.win.window() {
            win.set_cursor(1, one_line.len() as i64)?;
        }
        self.0.value.set_if_changed(one_line);
        self.refresh_placeholder();
        Ok(())
    }

    pub fn set_placeholder(&self, text: Option<String>) {
        *self.0.placeholder.borrow_mut() = text;
        self.refresh_placeholder();
    }

    /// Bind `lhs` in each of `modes` (e.g. `&["i", "n"]`) for this input.
    pub fn on_key(&self, modes: &[&str], lhs: &str, f: impl Fn() + 'static) -> Result<()> {
        let f = Rc::new(f);
        for mode in modes {
            let f = f.clone();
            nvim::buf_keymap(self.buffer(), mode, lhs, KeymapOpts::default(), move || f())?;
        }
        Ok(())
    }

    /// Like [`on_key`](Self::on_key), but `f` may decline the key — return
    /// `false` and it does what it normally does. `f` runs under Neovim's text
    /// lock: decide, and schedule any change.
    pub fn on_key_or(&self, modes: &[&str], lhs: &str, f: impl Fn() -> bool + 'static) -> Result<()> {
        let f = Rc::new(f);
        for mode in modes {
            let f = f.clone();
            nvim::buf_keymap_or(self.buffer(), mode, lhs, KeymapOpts::default(), move || f())?;
        }
        Ok(())
    }

    /// The widget that reserves the input's place in a surface's tree (one row).
    pub fn widget(&self) -> Embed {
        self.0.win.widget().height(1)
    }

    pub fn embedded(&self) -> &EmbeddedWindow {
        &self.0.win
    }

    pub fn buffer(&self) -> Buffer {
        self.0.win.buffer()
    }

    pub fn window(&self) -> Option<Window> {
        self.0.win.window()
    }

    pub fn focus(&self) -> Result<()> {
        self.0.win.focus()
    }

    pub fn close(&self) {
        self.0.win.close();
    }

    /// The buffer changed: keep it on one line and update the value.
    fn sync(&self) {
        let lines = self.buffer().get_lines(0, -1).unwrap_or_default();
        let text = lines.join(" ");
        if lines.len() > 1 {
            // A paste with line breaks: join it (after this autocmd).
            let weak = Rc::downgrade(&self.0);
            let joined = text.clone();
            let _ = nvim::schedule(move || {
                if let Some(inner) = weak.upgrade() {
                    let _ = BufferInput(inner).set_text(&joined);
                }
            });
        }
        self.0.value.set_if_changed(text);
        self.refresh_placeholder();
    }

    fn refresh_placeholder(&self) {
        let buf = self.buffer();
        let _ = self.0.ns.clear(buf);
        let placeholder = self.0.placeholder.borrow().clone();
        if let Some(text) = placeholder.filter(|_| self.0.value.get_untracked().is_empty()) {
            let opts = ExtmarkOpts {
                virt_text: Some(Line::from_spans([Span::hl(text, groups::PLACEHOLDER)])),
                virt_text_pos: Some(VirtTextPos::Overlay),
                ..Default::default()
            };
            let _ = self.0.ns.set_extmark(buf, 0, 0, &opts);
        }
    }
}
