//! `Input` — a single-line text prompt. `<CR>` submits, `<Esc>` cancels, and an
//! optional [`Input::on_change`] callback fires on every edit. The current text
//! is exposed as a reactive [`Signal`].

use std::cell::Cell;
use std::rc::Rc;

use crate::error::Result;
use crate::geometry::{Position, Size};
use crate::nvim::{self, Border, TitlePos};
use crate::reactive::Signal;
use crate::widget::popup::{Popup, PopupOptions};

/// Configuration for an [`Input`].
pub struct InputOptions {
    pub title: Option<String>,
    pub width: Size,
    pub position: Position,
    pub border: Border,
    /// Pre-filled text. Line breaks become spaces (the input is one line).
    pub initial: String,
}

impl Default for InputOptions {
    fn default() -> Self {
        Self {
            title: Some(" Input ".to_string()),
            width: Size::ratio(0.5, 0.0), // height forced to 1 below
            position: Position::Center,
            border: Border::Rounded,
            initial: String::new(),
        }
    }
}

/// A callback invoked with the current text on every edit.
type ChangeHandler = Rc<dyn Fn(&str)>;

/// A single-line reactive text input.
pub struct Input {
    popup: Popup,
    value: Signal<String>,
    on_change: Option<ChangeHandler>,
}

impl Input {
    /// Build an input. Does not open it yet.
    pub fn new(opts: InputOptions) -> Result<Self> {
        // Force a one-row popup regardless of the requested height.
        let size = Size { width: opts.width.width, height: crate::geometry::Dim::Cells(1) };
        let popup = Popup::new(PopupOptions {
            title: opts.title,
            title_pos: TitlePos::Center,
            size,
            position: opts.position,
            border: opts.border,
            enter: true,
            // Focus leaving the input dismisses it (see `Input::open`).
            close_on_leave: true,
            ..Default::default()
        })?;
        let initial = opts.initial.replace("\r\n", " ").replace(['\n', '\r'], " ");
        Ok(Self { popup, value: Signal::new(initial), on_change: None })
    }

    /// The reactive current text.
    pub fn value(&self) -> Signal<String> {
        self.value.clone()
    }

    /// The underlying popup.
    pub fn popup(&self) -> &Popup {
        &self.popup
    }

    /// Call `f` with the current text on every edit (builder style).
    pub fn on_change(mut self, f: impl Fn(&str) + 'static) -> Self {
        self.on_change = Some(Rc::new(f));
        self
    }

    /// Open the input and start insert mode. `on_done` is called exactly
    /// once: with the text on `<CR>`, or with `None` when the input is
    /// dismissed — `<Esc>`, the cursor leaving the input window, or the window
    /// being closed by anything else (`:q`, `<C-w>c`, another plugin). The
    /// input is fully closed (and insert mode left) before `on_done` runs. (If
    /// `open` itself fails, `on_done` is never called.)
    pub fn open(self, on_done: impl Fn(Option<String>) + 'static) -> Result<()> {
        let popup = self.popup.clone();
        let result = self.open_inner(on_done);
        if result.is_err() {
            popup.close();
        }
        result
    }

    fn open_inner(self, on_done: impl Fn(Option<String>) + 'static) -> Result<()> {
        let buf = self.popup.buffer();

        // Seed initial text without locking the buffer (inputs are editable).
        buf.set_all([self.value.get_untracked()])?;

        self.popup.open()?;
        if let Some(win) = self.popup.window() {
            let _ = win.set_option("cursorline", false);
        }

        // Keep the signal in sync with edits (in the popup's own group, so it
        // goes away with the popup).
        let group = self.popup.augroup()?;
        let value = self.value.clone();
        let on_change = self.on_change.clone();
        nvim::buf_autocmd(&["TextChangedI", "TextChanged"], buf, Some(group), move || {
            if let Ok(line) = buf.first_line() {
                if let Some(f) = &on_change {
                    f(&line);
                }
                value.set(line);
            }
        })?;

        // Exactly once: whichever way the input ends first wins.
        let fired = Rc::new(Cell::new(false));
        let on_done = Rc::new(on_done);
        let finish: Rc<dyn Fn(Option<String>)> = {
            let popup = self.popup.clone();
            Rc::new(move |text: Option<String>| {
                if fired.replace(true) {
                    return;
                }
                // Leave insert mode, but only if it is the input's (when the
                // window was closed from elsewhere, the mode belongs to
                // whatever window the user is in now).
                let here = popup.window().is_some_and(|w| nvim::Window::current().ok() == Some(w));
                let stopped = here && nvim::mode().is_ok_and(|m| m.starts_with('i') || m.starts_with('R'));
                if stopped {
                    let _ = crate::lua::cmd("stopinsert");
                }
                popup.close();
                if stopped {
                    // `stopinsert` only takes effect once the key handler
                    // returns: answer after that, or a prompt the callback
                    // opens (rename, then confirm) would lose its insert mode.
                    let on_done = on_done.clone();
                    let _ = nvim::schedule(move || on_done(text));
                } else {
                    on_done(text);
                }
            })
        };

        // Submit.
        for mode in ["i", "n"] {
            let finish = finish.clone();
            let value = self.value.clone();
            self.popup.on_key(mode, "<CR>", move || {
                finish(Some(buf.first_line().unwrap_or_else(|_| value.get_untracked())));
            })?;
        }

        // Cancel.
        for mode in ["i", "n"] {
            let finish = finish.clone();
            self.popup.on_key(mode, "<Esc>", move || finish(None))?;
        }

        // Closed or left some other way: that is a cancel too.
        self.popup.set_on_dismiss(move || finish(None));

        // Already inserting (opened from insert mode)? `startinsert!` then
        // just moves the cursor to the end of the line, which is all we need.
        crate::lua::cmd("startinsert!")?;
        Ok(())
    }
}
