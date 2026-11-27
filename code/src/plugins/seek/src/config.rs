//! Seek's configuration: `require("seek").setup({ .. })`, stored in
//! `vim.g.abstract_seek_opts`. Every field is optional; anything missing or
//! mistyped keeps its default.
//!
//! ```lua
//! require("seek").setup({
//!   layout = { width = 0.8, height = 0.8, preview_ratio = 0.6, min_width_for_preview = 100, preview = true },
//!   modes = { "files", "grep", "buffers", "lines" },  -- enabled, in this order
//!   default_mode = "files",
//!   prefixes = { grep = ">", buffers = "#", lines = "/" },  -- "" disables a prefix
//!   page_size = 200,                                   -- results loaded at a time (more load as you scroll)
//!   files = { hidden = false, follow = false, gitignore = true, exclude = { "*.lock" } },
//!   grep = { regex = false, hidden = false, follow = false, gitignore = true, exclude = {},
//!            max_results = 10000, max_filesize = 1048576, debounce_ms = 80 },
//!   preview = { max_filesize = 1048576, max_lines = 10000, line_numbers = false, media = false },
//!   start_insert = true,
//!   backspace_to_default = true,                       -- <BS> on an empty prompt: back to default_mode
//!   on_reopen = "toggle",                              -- opening it again while open: "toggle" (close) | "keep"
//!   keys = { open = "<CR>", next = { "<Tab>", "<Down>", "<C-j>" }, ... },  -- insert + normal mode
//!   normal_keys = { next = "j", prev = "k", first = "gg", last = "G", close = { "<Esc>", "q" } },
//! })
//! ```

use mlua::{Table, Value};
use wp_ui::{Result, lua};

use crate::entry::Mode;

/// The `vim.g` variable holding the user's options table.
const OPTS_VAR: &str = "abstract_seek_opts";

/// Something a key can do in the seek window.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    Next,
    Prev,
    First,
    Last,
    Open,
    Split,
    VSplit,
    Tab,
    Quickfix,
    Close,
    PreviewDown,
    PreviewUp,
    TogglePreview,
    Maximize,
    Mode(Mode),
}

impl Action {
    /// The name used in the `keys` table.
    pub fn name(self) -> &'static str {
        match self {
            Action::Next => "next",
            Action::Prev => "prev",
            Action::First => "first",
            Action::Last => "last",
            Action::Open => "open",
            Action::Split => "split",
            Action::VSplit => "vsplit",
            Action::Tab => "tab",
            Action::Quickfix => "quickfix",
            Action::Close => "close",
            Action::PreviewDown => "preview_down",
            Action::PreviewUp => "preview_up",
            Action::TogglePreview => "toggle_preview",
            Action::Maximize => "maximize",
            Action::Mode(m) => m.name(),
        }
    }

    const ALL: [Action; 18] = [
        Action::Next,
        Action::Prev,
        Action::First,
        Action::Last,
        Action::Open,
        Action::Split,
        Action::VSplit,
        Action::Tab,
        Action::Quickfix,
        Action::Close,
        Action::PreviewDown,
        Action::PreviewUp,
        Action::TogglePreview,
        Action::Maximize,
        Action::Mode(Mode::Files),
        Action::Mode(Mode::Grep),
        Action::Mode(Mode::Buffers),
        Action::Mode(Mode::Lines),
    ];

    /// Keys bound in both insert and normal mode.
    fn default_keys(self) -> &'static [&'static str] {
        match self {
            Action::Next => &["<Tab>", "<Down>", "<C-j>", "<C-n>"],
            Action::Prev => &["<S-Tab>", "<Up>", "<C-k>"],
            Action::First | Action::Last => &[],
            Action::Open => &["<CR>"],
            Action::Split => &["<C-s>"],
            Action::VSplit => &["<C-v>"],
            Action::Tab => &["<C-t>"],
            Action::Quickfix => &["<C-q>"],
            Action::Close => &["<C-c>"],
            Action::PreviewDown => &["<C-d>"],
            Action::PreviewUp => &["<C-u>"],
            Action::TogglePreview => &["<C-p>"],
            Action::Maximize => &["<C-z>"],
            Action::Mode(Mode::Files) => &["<C-f>"],
            Action::Mode(Mode::Grep) => &["<C-g>"],
            Action::Mode(Mode::Buffers) => &["<C-b>"],
            Action::Mode(Mode::Lines) => &["<C-l>"],
        }
    }

    /// Extra keys bound in normal mode only (where they don't type text).
    fn default_normal_keys(self) -> &'static [&'static str] {
        match self {
            Action::Next => &["j"],
            Action::Prev => &["k"],
            Action::First => &["gg"],
            Action::Last => &["G"],
            // `<Esc>` in insert mode goes to normal mode; in normal mode it closes.
            Action::Close => &["<Esc>", "q"],
            _ => &[],
        }
    }
}

fn key_table(keys: fn(Action) -> &'static [&'static str]) -> Vec<(Action, Vec<String>)> {
    Action::ALL.iter().map(|a| (*a, keys(*a).iter().map(|k| k.to_string()).collect())).collect()
}

/// What opening seek again does while it is open (same mode, no query).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reopen {
    /// Close it.
    Toggle,
    /// Leave it open (the cursor goes back to its prompt).
    Keep,
}

/// How to walk the project.
#[derive(Clone, Debug)]
pub struct Walk {
    /// Include hidden (dot) files and directories.
    pub hidden: bool,
    /// Follow symbolic links.
    pub follow: bool,
    /// Honour `.gitignore`, `.ignore` and the global git excludes.
    pub gitignore: bool,
    /// Extra glob patterns to leave out (gitignore syntax, e.g. `"*.lock"`, `"target/"`).
    pub exclude: Vec<String>,
}

impl Default for Walk {
    fn default() -> Self {
        Self { hidden: false, follow: false, gitignore: true, exclude: Vec::new() }
    }
}

#[derive(Clone, Debug)]
pub struct Grep {
    pub walk: Walk,
    /// Treat the query as a regular expression (default: a literal string).
    pub regex: bool,
    /// Stop after this many matches.
    pub max_results: usize,
    /// Skip files bigger than this (bytes).
    pub max_filesize: u64,
    /// Wait this long after the last keystroke before searching.
    pub debounce_ms: u64,
}

impl Default for Grep {
    fn default() -> Self {
        Self { walk: Walk::default(), regex: false, max_results: 10_000, max_filesize: 1 << 20, debounce_ms: 80 }
    }
}

#[derive(Clone, Debug)]
pub struct Config {
    pub width: f64,
    pub height: f64,
    /// Width share of the preview column.
    pub preview_ratio: f64,
    /// Below this many editor columns the preview is hidden.
    pub min_width_for_preview: u32,
    /// Show the preview initially.
    pub preview: bool,
    /// Enabled modes, in display order.
    pub modes: Vec<Mode>,
    pub default_mode: Mode,
    /// Typing one of these as the first character of the query switches mode.
    pub prefixes: Vec<(Mode, char)>,
    /// Results are loaded into the list this many at a time; the next page
    /// loads as the selection nears the end of what is loaded.
    pub page_size: usize,
    pub files: Walk,
    pub grep: Grep,
    pub preview_max_filesize: u64,
    pub preview_max_lines: usize,
    /// Line numbers in the preview.
    pub preview_line_numbers: bool,
    /// Preview images, video frames and PDFs (needs snacks.nvim's image
    /// module and a kitty-graphics terminal: kitty, Ghostty; not inside tmux).
    pub preview_media: bool,
    /// Start in insert mode (otherwise normal mode).
    pub start_insert: bool,
    /// Opening seek again while it is open.
    pub on_reopen: Reopen,
    /// Backspace on an empty prompt returns to the default mode.
    pub backspace_to_default: bool,
    /// Keys bound in insert and normal mode: every lhs for each action.
    pub keys: Vec<(Action, Vec<String>)>,
    /// Keys bound in normal mode only.
    pub normal_keys: Vec<(Action, Vec<String>)>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            width: 0.8,
            height: 0.8,
            preview_ratio: 0.6,
            min_width_for_preview: 100,
            preview: true,
            modes: Mode::ALL.to_vec(),
            default_mode: Mode::Files,
            prefixes: vec![(Mode::Grep, '>'), (Mode::Buffers, '#'), (Mode::Lines, '/')],
            page_size: 200,
            files: Walk::default(),
            grep: Grep::default(),
            preview_max_filesize: 1 << 20,
            preview_max_lines: 10_000,
            preview_line_numbers: false,
            preview_media: false,
            start_insert: true,
            on_reopen: Reopen::Toggle,
            backspace_to_default: true,
            keys: key_table(Action::default_keys),
            normal_keys: key_table(Action::default_normal_keys),
        }
    }
}

impl Config {
    /// Save the user's options table (or `nil`).
    pub fn store(opts: Value) -> Result<()> {
        let g: Table = lua::vim()?.get("g")?;
        g.set(OPTS_VAR, opts)?;
        Ok(())
    }

    /// Load from `vim.g.abstract_seek_opts`.
    pub fn load() -> Result<Self> {
        let g: Table = lua::vim()?.get("g")?;
        let mut c = Self::default();
        if let Value::Table(opts) = g.get::<Value>(OPTS_VAR)? {
            c.apply(&opts);
        }
        Ok(c)
    }

    fn apply(&mut self, t: &Table) {
        if let Some(l) = table(t, "layout") {
            set_f64(&l, "width", &mut self.width);
            set_f64(&l, "height", &mut self.height);
            set_f64(&l, "preview_ratio", &mut self.preview_ratio);
            if let Some(n) = int(&l, "min_width_for_preview") {
                self.min_width_for_preview = n.max(0) as u32;
            }
            set_bool(&l, "preview", &mut self.preview);
        }
        self.width = self.width.clamp(0.1, 1.0);
        self.height = self.height.clamp(0.1, 1.0);
        self.preview_ratio = self.preview_ratio.clamp(0.1, 0.9);

        if let Some(list) = strings(t, "modes") {
            let modes: Vec<Mode> = list.iter().filter_map(|s| Mode::parse(s)).collect();
            if !modes.is_empty() {
                self.modes = modes;
            }
        }
        if let Some(m) = string(t, "default_mode").and_then(|s| Mode::parse(&s)) {
            self.default_mode = m;
        }
        if !self.modes.contains(&self.default_mode) {
            self.default_mode = self.modes[0];
        }
        if let Some(p) = table(t, "prefixes") {
            for mode in Mode::ALL {
                if let Some(s) = string(&p, mode.name()) {
                    self.prefixes.retain(|(m, _)| *m != mode);
                    if let Some(ch) = s.chars().next() {
                        self.prefixes.push((mode, ch));
                    }
                }
            }
        }
        // `max_results` is the old name.
        if let Some(n) = int(t, "page_size").or_else(|| int(t, "max_results")) {
            self.page_size = n.clamp(10, 100_000) as usize;
        }
        if let Some(f) = table(t, "files") {
            apply_walk(&f, &mut self.files);
        }
        if let Some(g) = table(t, "grep") {
            apply_walk(&g, &mut self.grep.walk);
            set_bool(&g, "regex", &mut self.grep.regex);
            if let Some(n) = int(&g, "max_results") {
                self.grep.max_results = n.clamp(1, 1_000_000) as usize;
            }
            if let Some(n) = int(&g, "max_filesize") {
                self.grep.max_filesize = n.max(0) as u64;
            }
            if let Some(n) = int(&g, "debounce_ms") {
                self.grep.debounce_ms = n.clamp(0, 5000) as u64;
            }
        }
        if let Some(p) = table(t, "preview") {
            if let Some(n) = int(&p, "max_filesize") {
                self.preview_max_filesize = n.max(0) as u64;
            }
            if let Some(n) = int(&p, "max_lines") {
                self.preview_max_lines = n.max(1) as usize;
            }
            set_bool(&p, "line_numbers", &mut self.preview_line_numbers);
            set_bool(&p, "media", &mut self.preview_media);
        }
        set_bool(t, "start_insert", &mut self.start_insert);
        set_bool(t, "backspace_to_default", &mut self.backspace_to_default);
        match string(t, "on_reopen").as_deref() {
            Some("toggle") => self.on_reopen = Reopen::Toggle,
            Some("keep") => self.on_reopen = Reopen::Keep,
            _ => {},
        }
        if let Some(k) = table(t, "keys") {
            apply_keys(&k, &mut self.keys);
        }
        if let Some(k) = table(t, "normal_keys") {
            apply_keys(&k, &mut self.normal_keys);
        }
    }

    /// The mode a query prefix selects.
    pub fn prefix_mode(&self, ch: char) -> Option<Mode> {
        self.prefixes.iter().find(|(m, c)| *c == ch && self.modes.contains(m)).map(|(m, _)| *m)
    }
}

fn apply_keys(t: &Table, keys: &mut [(Action, Vec<String>)]) {
    for (action, lhs) in keys {
        match t.get::<Value>(action.name()) {
            // An empty key can't be mapped: `""` unbinds, like `false`.
            Ok(Value::String(s)) => *lhs = Some(s.to_string_lossy()).into_iter().filter(|k| !k.is_empty()).collect(),
            Ok(Value::Table(list)) => {
                *lhs = list.sequence_values::<String>().filter_map(|v| v.ok()).filter(|k| !k.is_empty()).collect();
            },
            // `false` unbinds the action.
            Ok(Value::Boolean(false)) => lhs.clear(),
            _ => {},
        }
    }
}

fn apply_walk(t: &Table, w: &mut Walk) {
    set_bool(t, "hidden", &mut w.hidden);
    set_bool(t, "follow", &mut w.follow);
    set_bool(t, "gitignore", &mut w.gitignore);
    if let Some(list) = strings(t, "exclude") {
        w.exclude = list;
    }
}

fn table(t: &Table, key: &str) -> Option<Table> {
    match t.get::<Value>(key) {
        Ok(Value::Table(t)) => Some(t),
        _ => None,
    }
}

fn string(t: &Table, key: &str) -> Option<String> {
    t.get::<Option<String>>(key).ok().flatten()
}

fn strings(t: &Table, key: &str) -> Option<Vec<String>> {
    table(t, key).map(|l| l.sequence_values::<String>().filter_map(|v| v.ok()).collect())
}

fn int(t: &Table, key: &str) -> Option<i64> {
    t.get::<Option<i64>>(key).ok().flatten()
}

fn set_f64(t: &Table, key: &str, out: &mut f64) {
    if let Ok(Some(v)) = t.get::<Option<f64>>(key) {
        *out = v;
    }
}

fn set_bool(t: &Table, key: &str, out: &mut bool) {
    if let Ok(Some(v)) = t.get::<Option<bool>>(key) {
        *out = v;
    }
}
