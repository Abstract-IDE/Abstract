//! Toast notifications: short-lived floats stacked in the top-right corner,
//! auto-dismissed on a timer (or sticky until dismissed).

use std::cell::{Cell, RefCell};

use crate::error::Result;
use crate::geometry::{Position, Size, editor_size};
use crate::nvim::{Border, Timer};
use crate::text::{Line, Wrap, wrap_line};
use crate::theme::groups;
use crate::view::widgets::text::Text;
use crate::view::runtime::Surface;
use crate::widget::popup::PopupOptions;

/// Notification severity — controls the border/title highlight.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Level {
    #[default]
    Info,
    Success,
    Warn,
    Error,
}

impl Level {
    fn hl(self) -> &'static str {
        match self {
            Level::Info => groups::INFO,
            Level::Success => groups::SUCCESS,
            Level::Warn => groups::WARN,
            Level::Error => groups::ERROR,
        }
    }

    fn default_title(self) -> &'static str {
        match self {
            Level::Info => "Info",
            Level::Success => "Success",
            Level::Warn => "Warning",
            Level::Error => "Error",
        }
    }
}

/// Options for [`notify`].
#[derive(Clone, Debug)]
pub struct NotifyOptions {
    pub title: Option<String>,
    pub level: Level,
    /// Auto-dismiss after this many ms; `None` keeps the toast until dismissed.
    pub timeout_ms: Option<u64>,
    /// Content width in cells.
    pub width: u16,
}

impl Default for NotifyOptions {
    fn default() -> Self {
        Self { title: None, level: Level::Info, timeout_ms: Some(4000), width: 40 }
    }
}

struct Toast {
    id: u64,
    surface: Surface,
    /// Rows the toast occupies including its border.
    rows: u32,
    _timer: Option<Timer>,
}

thread_local! {
    static TOASTS: RefCell<Vec<Toast>> = const { RefCell::new(Vec::new()) };
    static NEXT_ID: Cell<u64> = const { Cell::new(0) };
}

/// A handle to a shown toast.
#[derive(Clone, Copy, Debug)]
pub struct NotifyHandle {
    id: u64,
}

impl NotifyHandle {
    /// Remove the toast now (the stack re-anchors).
    pub fn dismiss(self) {
        dismiss_id(self.id);
    }
}

/// Show a toast. Returns a handle for early dismissal; the toast otherwise
/// lives until its timeout (or forever with `timeout_ms: None`).
pub fn notify(msg: impl Into<String>, opts: NotifyOptions) -> Result<NotifyHandle> {
    let msg = msg.into();
    let width = opts.width.clamp(16, 120);
    let inner_w = width.saturating_sub(2); // horizontal padding

    // Height from the wrapped message (cap so one toast can't fill the screen).
    let rows = wrap_line(&Line::raw(&msg), inner_w, Wrap::Word).len().clamp(1, 10) as u32;
    let total_rows = rows + 2; // border

    let id = NEXT_ID.with(|c| {
        let id = c.get();
        c.set(id + 1);
        id
    });

    let title = opts.title.clone().unwrap_or_else(|| opts.level.default_title().to_string());
    let level = opts.level;
    let surface = Surface::float(PopupOptions {
        size: Size::cells(width as u32, rows),
        position: Position::At { row: 1.0, col: 1.0 }, // restack() places it
        border: Border::Rounded,
        title: Some(format!(" {title} ")),
        focusable: false,
        enter: false,
        zindex: 80,
        ..Default::default()
    })?
    .show(move || {
        Text::new(msg.clone()).wrap(Wrap::Word)
    })?;

    // Level-colored border/title.
    if let Some(win) = surface.popup().window() {
        let hl = level.hl();
        let _ = win.set_option(
            "winhighlight",
            format!("Normal:{n},NormalFloat:{n},FloatBorder:{b},FloatTitle:{b}", n = groups::NORMAL, b = hl),
        );
    }

    let timer = match opts.timeout_ms {
        Some(ms) => Some(Timer::once(ms.max(100), move || dismiss_id(id))?),
        None => None,
    };

    TOASTS.with(|t| t.borrow_mut().push(Toast { id, surface, rows: total_rows, _timer: timer }));
    restack();
    Ok(NotifyHandle { id })
}

/// Dismiss every visible toast.
pub fn dismiss_all() {
    TOASTS.with(|t| t.borrow_mut().clear()); // drop = close + free
}

fn dismiss_id(id: u64) {
    TOASTS.with(|t| t.borrow_mut().retain(|toast| toast.id != id));
    restack();
}

/// Re-anchor the stack in the top-right corner, newest at the top.
fn restack() {
    let Ok((cols, _)) = editor_size() else { return };
    TOASTS.with(|t| {
        let toasts = t.borrow();
        let mut row = 1.0;
        for toast in toasts.iter().rev() {
            let popup = toast.surface.popup();
            if let Ok((w, _)) = popup.inner_size() {
                let col = (cols as f64 - w as f64 - 3.0).max(0.0);
                let _ = popup.set_position(Position::At { row, col });
            }
            row += toast.rows as f64;
        }
    });
}
