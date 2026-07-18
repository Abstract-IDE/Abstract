//! `Input` — a single-line text prompt. `<CR>` submits, `<Esc>` cancels, and an
//! optional [`Input::on_change`] callback fires on every edit. The current text
//! is exposed as a reactive [`Signal`].

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

/// A callback invoked with the current text on every edit.
type ChangeHandler = std::rc::Rc<dyn Fn(&str)>;

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
            ..Default::default()
        })?;
        Ok(Self { popup, value: Signal::new(opts.initial), on_change: None })
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
        self.on_change = Some(std::rc::Rc::new(f));
        self
    }

    /// Open the input and start insert mode. `on_submit` fires with the final
    /// text on `<CR>` (the input closes first).
    pub fn open(self, on_submit: impl Fn(String) + 'static) -> Result<()> {
        self.open_with(on_submit, || {})
    }

    /// Like [`Input::open`], but `on_cancel` fires when the user dismisses the
    /// input (`<Esc>`) instead of submitting. Exactly one of the two callbacks
    /// runs, at most once.
    pub fn open_with(self, on_submit: impl Fn(String) + 'static, on_cancel: impl Fn() + 'static) -> Result<()> {
        let buf = self.popup.buffer();

        // Seed initial text without locking the buffer (inputs are editable).
        let initial = self.value.get_untracked();
        buf.set_all([initial])?;

        self.popup.open()?;
        if let Some(win) = self.popup.window() {
            let _ = win.set_option("cursorline", false);
        }

        // Keep the signal in sync with edits (per-instance group, so several
        // inputs can be alive at once).
        let group = nvim::unique_augroup("AbNuiInput")?;
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

        // Submit/cancel are mutually exclusive and fire at most once.
        let fired = std::rc::Rc::new(std::cell::Cell::new(false));

        // Submit.
        let submit = {
            let value = self.value.clone();
            let popup = self.popup.clone();
            let fired = fired.clone();
            std::rc::Rc::new(move || {
                if fired.replace(true) {
                    return;
                }
                let text = buf.first_line().unwrap_or_else(|_| value.get_untracked());
                let _ = crate::lua::cmd("stopinsert");
                popup.close();
                on_submit(text);
            })
        };
        for mode in ["i", "n"] {
            let s = submit.clone();
            self.popup.on_key(mode, "<CR>", move || s())?;
        }

        // Cancel.
        let on_cancel = std::rc::Rc::new(on_cancel);
        for mode in ["i", "n"] {
            let popup = self.popup.clone();
            let on_cancel = on_cancel.clone();
            let fired = fired.clone();
            self.popup.on_key(mode, "<Esc>", move || {
                if fired.replace(true) {
                    return;
                }
                let _ = crate::lua::cmd("stopinsert");
                popup.close();
                on_cancel();
            })?;
        }

        crate::lua::cmd("startinsert!")?;
        Ok(())
    }
}
