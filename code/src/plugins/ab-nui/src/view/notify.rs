//! Toast notifications: short-lived floats stacked in the top-right corner,
//! auto-dismissed on a timer (or sticky until dismissed).
//!
//! The stack re-anchors on every notify/dismiss and on editor resize. Toasts
//! that don't fit above the command line are kept hidden (their timers keep
//! running) and appear as soon as there is room; the newest toast is always
//! shown.

use std::cell::{Cell, RefCell};

use crate::error::Result;
use crate::geometry::{Position, Size, editor_size};
use crate::lua;
use crate::nvim::{self, Border, Timer};
use crate::text::{Line, Wrap, wrap_line};
use crate::theme::groups;
use crate::view::runtime::Surface;
use crate::view::widgets::text::Text;
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
    level: Level,
    /// Rows the toast occupies including its border.
    rows: u32,
    _timer: Option<Timer>,
}

thread_local! {
    // `ManuallyDrop`: toasts still showing when Neovim exits must not try to
    // close their windows from a thread-exit destructor.
    static TOASTS: std::mem::ManuallyDrop<RefCell<Vec<Toast>>> =
        const { std::mem::ManuallyDrop::new(RefCell::new(Vec::new())) };
    static NEXT_ID: Cell<u64> = const { Cell::new(0) };
    /// Whether the (single, shared) `VimResized` restack hook is installed.
    static RESIZE_HOOK: Cell<bool> = const { Cell::new(false) };
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

    // Height from the message wrapped exactly as the `Text` below wraps it (at
    // the full width), capped so one toast can't fill the screen.
    let rows = wrap_line(&Line::raw(&msg), width, Wrap::Word).len().clamp(1, 10) as u32;
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
    .show(move || Text::new(msg.clone()).wrap(Wrap::Word))?;

    style(&surface, level);

    let timer = match opts.timeout_ms {
        Some(ms) => Some(Timer::once(ms.max(100), move || dismiss_id(id))?),
        None => None,
    };

    install_resize_hook();
    TOASTS.with(|t| t.borrow_mut().push(Toast { id, surface, level, rows: total_rows, _timer: timer }));
    restack();
    Ok(NotifyHandle { id })
}

/// Level-colored border/title on the toast's current window (re-applied
/// whenever a hidden toast gets a new window).
fn style(surface: &Surface, level: Level) {
    if let Some(win) = surface.popup().window() {
        let hl = level.hl();
        let _ = win.set_option(
            "winhighlight",
            format!("Normal:{n},NormalFloat:{n},FloatBorder:{b},FloatTitle:{b}", n = groups::NORMAL, b = hl),
        );
    }
}

/// Re-anchor the stack whenever the editor is resized (one hook for all
/// toasts, installed with the first one).
fn install_resize_hook() {
    if RESIZE_HOOK.with(|h| h.replace(true)) {
        return;
    }
    if nvim::autocmd(&["VimResized"], None, None, |_| restack()).is_err() {
        RESIZE_HOOK.with(|h| h.set(false));
    }
}

/// Dismiss every visible toast.
pub fn dismiss_all() {
    // Take the toasts out before dropping them: closing a toast runs
    // `WinClosed`/`BufWipeout` autocmds synchronously, and those may call
    // `notify` (which needs the list).
    let gone = TOASTS.with(|t| std::mem::take(&mut *t.borrow_mut()));
    drop(gone); // drop = close + free
}

fn dismiss_id(id: u64) {
    let gone: Vec<Toast> = TOASTS.with(|t| {
        let mut toasts = t.borrow_mut();
        let (gone, keep) = std::mem::take(&mut *toasts).into_iter().partition(|toast| toast.id == id);
        *toasts = keep;
        gone
    });
    if gone.is_empty() {
        return;
    }
    drop(gone); // outside the borrow, as in `dismiss_all`
    restack();
}

/// Re-anchor the stack in the top-right corner, newest at the top. Toasts
/// that would reach past the bottom (the command line) are hidden until
/// there is room; the newest one is always shown.
fn restack() {
    let Ok((cols, lines)) = editor_size() else { return };
    let cmdheight: i64 = lua::get_option("cmdheight").unwrap_or(1);
    let bottom = (lines as i64 - cmdheight.max(0)) as f64;
    // Snapshot, then release the borrow: moving/hiding/opening windows runs
    // autocmds that may re-enter `notify`/`dismiss`.
    let stack: Vec<(Surface, Level, u32)> = TOASTS
        .with(|t| t.borrow().iter().rev().map(|toast| (toast.surface.clone(), toast.level, toast.rows)).collect());
    let mut row = 1.0;
    let mut full = false;
    for (i, (surface, level, rows)) in stack.iter().enumerate() {
        full = full || (i > 0 && row + *rows as f64 > bottom);
        if full {
            surface.hide();
            continue;
        }
        let popup = surface.popup();
        if let Ok((w, _)) = popup.inner_size() {
            let col = (cols as f64 - w as f64 - 3.0).max(0.0);
            let _ = popup.set_position(Position::At { row, col });
        }
        if !surface.is_visible() && surface.reopen().is_ok() {
            style(surface, *level);
        }
        row += *rows as f64;
    }
}
