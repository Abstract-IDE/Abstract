//! Window handle (floats and splits), configuration, and operations.

use mlua::{FromLua, IntoLua, Table};

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

/// What a float's `row`/`col` are measured against.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Relative {
    /// The whole editor grid.
    #[default]
    Editor,
    /// Another window (its top-left corner) — lets a float anchor to a float.
    Win(Window),
    /// The cursor in the current window.
    Cursor,
}

/// Which corner of the float sits at `(row, col)`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Anchor {
    #[default]
    NW,
    NE,
    SW,
    SE,
}

impl Anchor {
    fn as_str(self) -> &'static str {
        match self {
            Anchor::NW => "NW",
            Anchor::NE => "NE",
            Anchor::SW => "SW",
            Anchor::SE => "SE",
        }
    }
}

/// Everything needed to open a floating window over `rect`.
#[derive(Clone, Debug)]
pub struct FloatConfig {
    pub rect: Rect,
    pub relative: Relative,
    pub anchor: Anchor,
    /// Place the float relative to this (0-based) text position in the window's
    /// buffer. Only valid with [`Relative::Win`].
    pub bufpos: Option<(i64, i64)>,
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
            relative: Relative::default(),
            anchor: Anchor::default(),
            bufpos: None,
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
        match self.relative {
            Relative::Editor => t.set("relative", "editor")?,
            Relative::Cursor => t.set("relative", "cursor")?,
            Relative::Win(win) => {
                t.set("relative", "win")?;
                t.set("win", win.id())?;
                if let Some((row, col)) = self.bufpos {
                    let pos = lua::table()?;
                    pos.set(1, row)?;
                    pos.set(2, col)?;
                    t.set("bufpos", pos)?;
                }
            }
        }
        t.set("anchor", self.anchor.as_str())?;
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

/// Which side of the reference window a split opens on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SplitDir {
    Left,
    Right,
    Above,
    Below,
}

impl SplitDir {
    fn as_str(self) -> &'static str {
        match self {
            SplitDir::Left => "left",
            SplitDir::Right => "right",
            SplitDir::Above => "above",
            SplitDir::Below => "below",
        }
    }
}

/// Everything needed to open a split window (`nvim_open_win` with `split`;
/// requires Neovim ≥ 0.10).
#[derive(Clone, Copy, Debug)]
pub struct SplitConfig {
    pub dir: SplitDir,
    /// Split relative to this window (current window if `None`).
    pub win: Option<Window>,
    /// Width (for left/right) or height (for above/below) in cells.
    pub size: Option<u32>,
    /// Whether the cursor enters the window on open.
    pub enter: bool,
    /// Use `style = "minimal"` (no numbers/signcolumn/etc).
    pub minimal: bool,
}

impl SplitConfig {
    pub fn new(dir: SplitDir) -> Self {
        Self { dir, win: None, size: None, enter: true, minimal: true }
    }

    fn to_table(self) -> Result<Table> {
        let t = lua::table()?;
        t.set("split", self.dir.as_str())?;
        if let Some(win) = self.win {
            t.set("win", win.id())?;
        }
        if let Some(size) = self.size {
            match self.dir {
                SplitDir::Left | SplitDir::Right => t.set("width", size as i64)?,
                SplitDir::Above | SplitDir::Below => t.set("height", size as i64)?,
            }
        }
        if self.minimal {
            t.set("style", "minimal")?;
        }
        Ok(t)
    }
}

/// A window's live configuration, read back via `nvim_win_get_config`.
#[derive(Clone, Debug, Default)]
pub struct WinConfigInfo {
    /// `"editor"`, `"win"`, `"cursor"`, or `""` for non-floating windows.
    pub relative: String,
    pub anchor: String,
    pub row: f64,
    pub col: f64,
    pub width: i64,
    pub height: i64,
    pub zindex: Option<i64>,
    /// The reference window for `relative = "win"`.
    pub win: Option<i64>,
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

    /// Open a split window showing `buf` with the given config.
    pub fn open_split(buf: Buffer, config: &SplitConfig) -> Result<Self> {
        let cfg = config.to_table()?;
        let id: i64 = lua::call_api("nvim_open_win", (buf.id(), config.enter, cfg))?;
        Ok(Window(id))
    }

    /// The raw window id.
    pub fn id(self) -> i64 {
        self.0
    }

    /// Read back the window's live configuration.
    pub fn config(self) -> Result<WinConfigInfo> {
        let t: Table = lua::call_api("nvim_win_get_config", (self.0,))?;
        Ok(WinConfigInfo {
            relative: t.get("relative").unwrap_or_default(),
            anchor: t.get("anchor").unwrap_or_default(),
            row: t.get("row").unwrap_or(0.0),
            col: t.get("col").unwrap_or(0.0),
            width: t.get("width").unwrap_or(0),
            height: t.get("height").unwrap_or(0),
            zindex: t.get("zindex").ok(),
            win: t.get("win").ok(),
        })
    }

    /// The window's width in cells.
    pub fn width(self) -> Result<i64> {
        lua::call_api("nvim_win_get_width", (self.0,))
    }

    /// The window's height in cells.
    pub fn height(self) -> Result<i64> {
        lua::call_api("nvim_win_get_height", (self.0,))
    }

    /// The buffer displayed in this window.
    pub fn buffer(self) -> Result<Buffer> {
        let id: i64 = lua::call_api("nvim_win_get_buf", (self.0,))?;
        Ok(Buffer(id))
    }

    /// Read a window-local option.
    pub fn get_option<R: FromLua>(self, name: &str) -> Result<R> {
        let scope = lua::table()?;
        scope.set("win", self.0)?;
        lua::get_option_scoped(name, scope)
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
