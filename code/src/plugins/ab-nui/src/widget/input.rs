//! `Input` — a single-line text prompt. `<CR>` submits, `<Esc>` cancels, and an
//! optional `on_change` fires on every edit. The current text is exposed as a
//! reactive [`Signal`].

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
    /// Pre-filled text.
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

/// A single-line reactive text input.
pub struct Input {
    popup: Popup,
    value: Signal<String>,
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
            ..Default::default()
        })?;
        Ok(Self { popup, value: Signal::new(opts.initial) })
    }

    /// The reactive current text.
    pub fn value(&self) -> Signal<String> {
        self.value.clone()
    }

    /// The underlying popup.
    pub fn popup(&self) -> &Popup {
        &self.popup
    }

    /// Open the input and start insert mode. `on_submit` fires with the final
    /// text on `<CR>` (the input closes first).
    pub fn open(self, on_submit: impl Fn(String) + 'static) -> Result<()> {
        let buf = self.popup.buffer();

        // Seed initial text without locking the buffer (inputs are editable).
        let initial = self.value.get_untracked();
        buf.set_all([initial])?;

        self.popup.open()?;
        if let Some(win) = self.popup.window() {
            let _ = win.set_option("cursorline", false);
        }

        // Keep the signal in sync with edits.
        let group = nvim::augroup("AbNuiInput")?;
        let value = self.value.clone();
        nvim::buf_autocmd(&["TextChangedI", "TextChanged"], buf, Some(group), move || {
            if let Ok(line) = buf.first_line() {
                value.set(line);
            }
        })?;

        // Submit.
        let value = self.value.clone();
        let close = self.popup.closer();
        let submit = move || {
            let text = buf.first_line().unwrap_or_else(|_| value.get_untracked());
            let _ = crate::lua::cmd("stopinsert");
            close();
            on_submit(text);
        };
        let submit = std::rc::Rc::new(submit);
        {
            let s = submit.clone();
            self.popup.on_key("i", "<CR>", move || s())?;
        }
        {
            let s = submit.clone();
            self.popup.on_key("n", "<CR>", move || s())?;
        }

        // Cancel.
        let cancel = self.popup.closer();
        self.popup.on_key("i", "<Esc>", move || {
            let _ = crate::lua::cmd("stopinsert");
            cancel();
        })?;
        self.popup.on_key("n", "<Esc>", self.popup.closer())?;

        crate::lua::cmd("startinsert!")?;
        Ok(())
    }
}
