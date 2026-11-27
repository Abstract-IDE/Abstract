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
    /// The dimmed backdrop behind a modal surface.
    pub const BACKDROP: &str = "AbNuiBackdrop";
    /// The highlighted line in a code view.
    pub const CODE_LINE: &str = "AbNuiCodeLine";
    /// Highlighted ranges (matches) in a code view.
    pub const CODE_MATCH: &str = "AbNuiCodeMatch";
    /// Placeholder text in an empty input.
    pub const PLACEHOLDER: &str = "AbNuiPlaceholder";
    /// Matched characters (e.g. of a search query) in list rows. On a
    /// selected row they are drawn inverted against the selection.
    pub const MATCH: &str = "AbNuiMatch";
}

/// How a derived group is made from the colorscheme's colours.
#[derive(Clone)]
enum Derived {
    Mix {
        fg_from: String,
        bg_from: String,
    },
    Inverse {
        group: String,
    },
    /// A fixed text colour (e.g. a kitty image id).
    Fg(i64),
}

thread_local! {
    /// Every derived group, by name: defined on first use and defined again
    /// after `:colorscheme` (which clears all non-default groups).
    static DERIVED: std::cell::RefCell<std::collections::HashMap<String, Derived>> =
        std::cell::RefCell::new(Default::default());
}

fn derived(name: String, how: Derived) -> String {
    if DERIVED.with(|d| d.borrow().contains_key(&name)) {
        return name;
    }
    if define(&name, &how).is_ok() {
        DERIVED.with(|d| d.borrow_mut().insert(name.clone(), how));
    }
    name
}

/// A highlight group with `fg_from`'s text colour and style on `bg_from`'s
/// background — e.g. a match highlight that stays visible on a selected row.
/// Created on first use and kept in sync with colorscheme changes.
pub fn mix(fg_from: &str, bg_from: &str) -> String {
    let name = format!("AbNuiMix_{}_{}", sanitize(fg_from), sanitize(bg_from));
    derived(name, Derived::Mix { fg_from: fg_from.into(), bg_from: bg_from.into() })
}

/// `group`'s background with its text in the complementary colour (bold) —
/// e.g. a match standing out on a selected row without breaking the row's
/// background. Kept in sync with colorscheme changes.
pub fn inverse(group: &str) -> String {
    let name = format!("AbNuiInverse_{}", sanitize(group));
    derived(name, Derived::Inverse { group: group.into() })
}

/// A group named `name` whose text colour is `rgb`, kept across colorscheme
/// changes until [`forget`].
pub fn fixed_fg(name: &str, rgb: i64) -> Result<()> {
    let how = Derived::Fg(rgb);
    define(name, &how)?;
    DERIVED.with(|d| d.borrow_mut().insert(name.to_string(), how));
    Ok(())
}

/// Stop keeping a group made by [`fixed_fg`] (it is cleared).
pub fn forget(name: &str) {
    if DERIVED.with(|d| d.borrow_mut().remove(name)).is_some() {
        let _ = lua::table().and_then(|t| set_hl(name, t));
    }
}

/// The colours a group really shows: links followed, `reverse` applied, and
/// what it leaves unset taken from the float background ([`groups::NORMAL`],
/// then `Normal`).
struct Colors {
    fg: Option<i64>,
    bg: Option<i64>,
    ctermfg: Option<i64>,
    ctermbg: Option<i64>,
    attrs: Table,
}

fn get_hl(group: &str) -> Result<Table> {
    let opts = lua::table()?;
    opts.set("name", group)?;
    opts.set("link", false)?;
    lua::call_api("nvim_get_hl", (0, opts))
}

fn colors(group: &str) -> Result<Colors> {
    let raw = get_hl(group)?;
    let (float, normal) = (get_hl(groups::NORMAL)?, get_hl("Normal")?);
    let base =
        |key: &str| -> Result<Option<i64>> { Ok(float.get::<Option<i64>>(key)?.or(normal.get::<Option<i64>>(key)?)) };
    let mut fg = match raw.get::<Option<i64>>("fg")? {
        Some(c) => Some(c),
        None => base("fg")?,
    };
    let mut bg = match raw.get::<Option<i64>>("bg")? {
        Some(c) => Some(c),
        None => base("bg")?,
    };
    let (mut ctermfg, mut ctermbg) = (raw.get::<Option<i64>>("ctermfg")?, raw.get::<Option<i64>>("ctermbg")?);
    if raw.get::<Option<bool>>("reverse")?.unwrap_or(false) {
        std::mem::swap(&mut fg, &mut bg);
        std::mem::swap(&mut ctermfg, &mut ctermbg);
    }
    Ok(Colors { fg, bg, ctermfg, ctermbg, attrs: raw })
}

fn define(name: &str, how: &Derived) -> Result<()> {
    let out = lua::table()?;
    match how {
        Derived::Mix { fg_from, bg_from } => {
            let (text, under) = (colors(fg_from)?, colors(bg_from)?);
            let own = get_hl(fg_from)?;
            for key in ["bold", "italic", "underline", "undercurl", "strikethrough", "sp"] {
                out.set(key, text.attrs.get::<mlua::Value>(key)?)?;
            }
            // Without its own colour, the text keeps the background group's.
            let has_fg =
                own.get::<Option<i64>>("fg")?.is_some() || own.get::<Option<bool>>("reverse")?.unwrap_or(false);
            out.set("fg", if has_fg { text.fg } else { under.fg })?;
            out.set("ctermfg", text.ctermfg.or(under.ctermfg))?;
            out.set("bg", under.bg)?;
            out.set("ctermbg", under.ctermbg)?;
        },
        Derived::Inverse { group } => {
            let c = colors(group)?;
            if let Some(bg) = c.bg {
                out.set("bg", bg)?;
                out.set("fg", 0xFF_FFFF - bg)?;
            }
            // Terminal colours: keep the group's, the text just turns bold.
            out.set("ctermbg", c.ctermbg)?;
            out.set("ctermfg", c.ctermfg)?;
            out.set("bold", true)?;
        },
        Derived::Fg(rgb) => out.set("fg", *rgb)?,
    }
    set_hl(name, out)
}

fn sanitize(group: &str) -> String {
    group.chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '_' }).collect()
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
    link(groups::CODE_LINE, "CursorLine")?;
    link(groups::CODE_MATCH, "Search")?;
    link(groups::PLACEHOLDER, "Comment")?;
    link(groups::MATCH, "Special")?;
    let backdrop = lua::table()?;
    backdrop.set("bg", "#000000")?;
    backdrop.set("ctermbg", 0)?;
    backdrop.set("default", true)?;
    set_hl(groups::BACKDROP, backdrop)?;
    // A colorscheme clears every group that isn't a default link: define
    // the derived ones again from the new colours (same names, so whatever
    // shows them updates).
    crate::nvim::autocmd(&["ColorScheme"], None, Some(crate::nvim::augroup("AbNuiTheme")?), |_| {
        let all: Vec<(String, Derived)> =
            DERIVED.with(|d| d.borrow().iter().map(|(n, h)| (n.clone(), h.clone())).collect());
        for (name, how) in all {
            let _ = define(&name, &how);
        }
    })?;
    Ok(())
}
