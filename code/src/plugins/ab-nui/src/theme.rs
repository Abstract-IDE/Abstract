//! Highlight group helpers and the library's default groups.

use mlua::Table;

use crate::error::Result;
use crate::lua;

/// The highlight groups ab-nui's built-in widgets use. They link to standard
/// Neovim groups by default so they look right in any colorscheme; override
/// them with [`set_hl`] to restyle.
pub mod groups {
    pub const NORMAL: &str = "AbNuiNormal";
    pub const BORDER: &str = "AbNuiBorder";
    pub const TITLE: &str = "AbNuiTitle";
    pub const SELECTION: &str = "AbNuiSelection";
    pub const PROMPT: &str = "AbNuiPrompt";
    pub const MUTED: &str = "AbNuiMuted";
    pub const FIELD: &str = "AbNuiField";
    pub const FOCUS: &str = "AbNuiFocus";
    pub const SCROLLBAR: &str = "AbNuiScrollbar";
    pub const SCROLLBAR_THUMB: &str = "AbNuiScrollbarThumb";
    pub const HEADER: &str = "AbNuiHeader";
    pub const ACCENT: &str = "AbNuiAccent";
    pub const SUCCESS: &str = "AbNuiSuccess";
    pub const WARN: &str = "AbNuiWarn";
    pub const ERROR: &str = "AbNuiError";
    pub const INFO: &str = "AbNuiInfo";
    pub const TAB_ACTIVE: &str = "AbNuiTabActive";
    pub const TAB_INACTIVE: &str = "AbNuiTabInactive";
    pub const CURSOR: &str = "AbNuiCursor";
}

/// Define (or override) a highlight group via `nvim_set_hl(0, name, opts)`.
///
/// `opts` is a table of the usual `nvim_set_hl` keys, e.g. `{ link = "..." }`
/// or `{ fg = "#ffffff", bold = true }`.
pub fn set_hl(name: &str, opts: Table) -> Result<()> {
    lua::call_api::<()>("nvim_set_hl", (0i64, name.to_string(), opts))
}

/// Link one highlight group to another.
pub fn link(name: &str, to: &str) -> Result<()> {
    let opts = lua::table()?;
    opts.set("link", to.to_string())?;
    opts.set("default", true)?;
    set_hl(name, opts)
}

/// Install the default highlight groups. Safe to call repeatedly; the links are
/// registered with `default = true`, so user overrides win.
pub fn setup() -> Result<()> {
    link(groups::NORMAL, "NormalFloat")?;
    link(groups::BORDER, "FloatBorder")?;
    link(groups::TITLE, "FloatTitle")?;
    link(groups::SELECTION, "PmenuSel")?;
    link(groups::PROMPT, "Question")?;
    link(groups::MUTED, "Comment")?;
    link(groups::FIELD, "Pmenu")?;
    link(groups::FOCUS, "Visual")?;
    link(groups::SCROLLBAR, "PmenuSbar")?;
    link(groups::SCROLLBAR_THUMB, "PmenuThumb")?;
    link(groups::HEADER, "Title")?;
    link(groups::ACCENT, "Function")?;
    link(groups::SUCCESS, "DiagnosticOk")?;
    link(groups::WARN, "DiagnosticWarn")?;
    link(groups::ERROR, "DiagnosticError")?;
    link(groups::INFO, "DiagnosticInfo")?;
    link(groups::TAB_ACTIVE, "TabLineSel")?;
    link(groups::TAB_INACTIVE, "TabLine")?;
    link(groups::CURSOR, "Cursor")?;
    Ok(())
}
