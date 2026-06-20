//! Styled text — the content model rendered into buffers.
//!
//! A [`Span`] is a run of text with an optional highlight group. A [`Line`] is a
//! sequence of spans. Widgets produce `Vec<Line>`; [`crate::nvim::render`] turns
//! that into buffer lines plus extmark highlights.

/// A run of text optionally painted with a highlight group.
#[derive(Clone, Debug, Default)]
pub struct Span {
    pub text: String,
    pub hl: Option<String>,
}

impl Span {
    /// Unstyled text.
    pub fn raw(text: impl Into<String>) -> Self {
        Self { text: text.into(), hl: None }
    }

    /// Text painted with the given highlight group.
    pub fn hl(text: impl Into<String>, group: impl Into<String>) -> Self {
        Self { text: text.into(), hl: Some(group.into()) }
    }
}

/// A single rendered line: a list of spans.
#[derive(Clone, Debug, Default)]
pub struct Line {
    pub spans: Vec<Span>,
}

impl Line {
    /// An empty line.
    pub fn empty() -> Self {
        Self { spans: Vec::new() }
    }

    /// A line of unstyled text.
    pub fn raw(text: impl Into<String>) -> Self {
        Self { spans: vec![Span::raw(text)] }
    }

    /// A line built from spans.
    pub fn from_spans(spans: impl IntoIterator<Item = Span>) -> Self {
        Self { spans: spans.into_iter().collect() }
    }

    /// Append a span (builder style).
    pub fn push(mut self, span: Span) -> Self {
        self.spans.push(span);
        self
    }

    /// The plain concatenated text of the line.
    pub fn text(&self) -> String {
        let mut s = String::new();
        for span in &self.spans {
            s.push_str(&span.text);
        }
        s
    }
}

impl From<&str> for Line {
    fn from(s: &str) -> Self {
        Line::raw(s)
    }
}

impl From<String> for Line {
    fn from(s: String) -> Self {
        Line::raw(s)
    }
}
