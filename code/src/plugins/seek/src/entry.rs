//! Modes and result entries.

/// What seek is searching.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Mode {
    /// Project files (fuzzy).
    Files,
    /// File contents (literal or regex).
    Grep,
    /// Open buffers (fuzzy).
    Buffers,
    /// Lines of the buffer seek was opened from (fuzzy).
    Lines,
}

impl Mode {
    pub const ALL: [Mode; 4] = [Mode::Files, Mode::Grep, Mode::Buffers, Mode::Lines];

    /// The name used in config, commands and the Lua API.
    pub fn name(self) -> &'static str {
        match self {
            Mode::Files => "files",
            Mode::Grep => "grep",
            Mode::Buffers => "buffers",
            Mode::Lines => "lines",
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            Mode::Files => "Files",
            Mode::Grep => "Grep",
            Mode::Buffers => "Buffers",
            Mode::Lines => "Lines",
        }
    }

    pub fn icon(self) -> &'static str {
        match self {
            Mode::Files => "\u{f15b}",
            Mode::Grep => "\u{f002}",
            Mode::Buffers => "\u{f0c5}",
            Mode::Lines => "\u{f0ca}",
        }
    }

    pub fn parse(s: &str) -> Option<Mode> {
        Mode::ALL.into_iter().find(|m| m.name().eq_ignore_ascii_case(s.trim()))
    }
}

/// Where an entry leads.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Target {
    /// A file, by path relative to the search root.
    File { path: String },
    /// A grep match: path relative to the root, 1-based line, 0-based byte column.
    Match { path: String, lnum: usize, col: usize },
    /// An open buffer.
    Buffer { bufnr: i64, modified: bool },
    /// A line (1-based) of the buffer lines mode searches; `indent` bytes of
    /// leading whitespace were left out of the entry's text.
    Line { lnum: usize, indent: usize },
}

/// One result.
#[derive(Clone, Debug)]
pub struct Entry {
    pub target: Target,
    /// The text that is matched and shown (a relative path, a line, a buffer name).
    pub text: String,
    /// Char indices into `text` to highlight.
    pub hl: Vec<usize>,
}

impl Entry {
    pub fn new(target: Target, text: impl Into<String>) -> Self {
        Self { target, text: text.into(), hl: Vec::new() }
    }

    /// The 1-based line this entry points at, if any.
    pub fn lnum(&self) -> Option<usize> {
        match self.target {
            Target::Match { lnum, .. } | Target::Line { lnum, .. } => Some(lnum),
            _ => None,
        }
    }
}
