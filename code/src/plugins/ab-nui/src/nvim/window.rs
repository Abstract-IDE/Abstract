//! Floating window handle, configuration, and operations.

use mlua::{IntoLua, Table};

use crate::error::Result;
use crate::geometry::Rect;
use crate::lua;
use crate::nvim::buffer::Buffer;

/// Border styles for a floating window.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Border {
    None,
    Single,
    Double,
    #[default]
    Rounded,
    Solid,
    Shadow,
}

impl Border {
    fn as_str(self) -> &'static str {
        match self {
            Border::None => "none",
            Border::Single => "single",
            Border::Double => "double",
            Border::Rounded => "rounded",
            Border::Solid => "solid",
            Border::Shadow => "shadow",
        }
    }
}

/// Where the title sits on the top border.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TitlePos {
    Left,
    #[default]
    Center,
    Right,
}

impl TitlePos {
    fn as_str(self) -> &'static str {
        match self {
            TitlePos::Left => "left",
            TitlePos::Center => "center",
            TitlePos::Right => "right",
        }
    }
}

/// Everything needed to open a floating window over `rect`.
#[derive(Clone, Debug)]
pub struct FloatConfig {
    pub rect: Rect,
    pub border: Border,
    pub title: Option<String>,
    pub title_pos: TitlePos,
    pub focusable: bool,
    pub zindex: u32,
    /// Whether the cursor enters the window on open.
    pub enter: bool,
    /// Use `style = "minimal"` (no numbers/signcolumn/etc).
    pub minimal: bool,
}

impl FloatConfig {
    pub fn new(rect: Rect) -> Self {
        Self {
            rect,
            border: Border::default(),
            title: None,
            title_pos: TitlePos::default(),
            focusable: true,
            zindex: 50,
            enter: true,
            minimal: true,
        }
    }

    /// Build the `nvim_open_win` config table.
    fn to_table(&self) -> Result<Table> {
        let t = lua::table()?;
        t.set("relative", "editor")?;
        t.set("width", self.rect.width as i64)?;
        t.set("height", self.rect.height as i64)?;
        t.set("row", self.rect.row)?;
        t.set("col", self.rect.col)?;
        t.set("focusable", self.focusable)?;
        t.set("zindex", self.zindex as i64)?;
        t.set("border", self.border.as_str())?;
        if self.minimal {
            t.set("style", "minimal")?;
        }
        if let Some(title) = &self.title {
            t.set("title", title.clone())?;
            t.set("title_pos", self.title_pos.as_str())?;
        }
        Ok(t)
    }
}

/// A Neovim window, identified by its id.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Window(pub i64);

impl Window {
    /// Open a floating window showing `buf` with the given config.
    pub fn open_float(buf: Buffer, config: &FloatConfig) -> Result<Self> {
        let cfg = config.to_table()?;
        let id: i64 = lua::call_api("nvim_open_win", (buf.id(), config.enter, cfg))?;
        Ok(Window(id))
    }

    /// The raw window id.
    pub fn id(self) -> i64 {
        self.0
    }

    /// Whether the window still exists.
    pub fn is_valid(self) -> bool {
        lua::call_api::<bool>("nvim_win_is_valid", (self.0,)).unwrap_or(false)
    }

    /// Reposition/reconfigure an open float.
    pub fn set_config(self, config: &FloatConfig) -> Result<()> {
        let cfg = config.to_table()?;
        lua::call_api::<()>("nvim_win_set_config", (self.0, cfg))
    }

    /// Set a window-local option (e.g. `"winhighlight"`, `"cursorline"`).
    pub fn set_option(self, name: &str, value: impl IntoLua) -> Result<()> {
        let scope = lua::table()?;
        scope.set("win", self.0)?;
        lua::set_option_scoped(name, value, scope)
    }

    /// Map this window's `Normal`/`FloatBorder`/`FloatTitle` to ab-nui groups.
    pub fn apply_default_highlight(self) -> Result<()> {
        use crate::theme::groups;
        let value = format!(
            "Normal:{n},NormalFloat:{n},FloatBorder:{b},FloatTitle:{t}",
            n = groups::NORMAL,
            b = groups::BORDER,
            t = groups::TITLE,
        );
        self.set_option("winhighlight", value)
    }

    /// Move the cursor to `(row, col)` (1-based row, 0-based col).
    pub fn set_cursor(self, row: i64, col: i64) -> Result<()> {
        let pos = lua::lua()?.create_table()?;
        pos.set(1, row)?;
        pos.set(2, col)?;
        lua::call_api::<()>("nvim_win_set_cursor", (self.0, pos))
    }

    /// Current cursor as `(row, col)`.
    pub fn cursor(self) -> Result<(i64, i64)> {
        let pos: Vec<i64> = lua::call_api("nvim_win_get_cursor", (self.0,))?;
        Ok((pos.first().copied().unwrap_or(1), pos.get(1).copied().unwrap_or(0)))
    }

    /// Make this the current window.
    pub fn focus(self) -> Result<()> {
        lua::call_api::<()>("nvim_set_current_win", (self.0,))
    }

    /// Close the window (`force = true`).
    pub fn close(self) -> Result<()> {
        lua::call_api::<()>("nvim_win_close", (self.0, true))
    }
}
