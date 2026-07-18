//! Styled text — the content model rendered into buffers.
//!
//! A [`Span`] is a run of text with an optional highlight group. A [`Line`] is a
//! sequence of spans. Widgets produce `Vec<Line>`; [`crate::nvim::render`] turns
//! that into buffer lines plus extmark highlights.

use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

/// Display width of one char in terminal cells (0, 1, or 2).
pub fn char_width(ch: char) -> u16 {
    UnicodeWidthChar::width(ch).unwrap_or(0) as u16
}

/// Display width of a string in terminal cells (unicode-aware).
pub fn display_width(s: &str) -> u16 {
    UnicodeWidthStr::width(s).min(u16::MAX as usize) as u16
}

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

    /// Display width in terminal cells.
    pub fn width(&self) -> u16 {
        display_width(&self.text)
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

    /// Display width in terminal cells.
    pub fn width(&self) -> u16 {
        self.spans.iter().map(Span::width).sum()
    }
}

/// How [`wrap_line`] breaks a line that exceeds the width.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Wrap {
    /// No wrapping — the line is returned as-is (callers clip when painting).
    #[default]
    None,
    /// Break at word boundaries (falling back to char breaks for long words).
    Word,
    /// Break at any character.
    Char,
}

/// Wrap a styled line to `width` display cells, preserving span highlights
/// (a span that straddles a break is split). `width == 0` yields the input.
pub fn wrap_line(line: &Line, width: u16, wrap: Wrap) -> Vec<Line> {
    if width == 0 || wrap == Wrap::None || line.width() <= width {
        return vec![line.clone()];
    }

    // Flatten to (char, hl-index) units, remembering each span's group.
    let groups: Vec<Option<String>> = line.spans.iter().map(|s| s.hl.clone()).collect();
    let chars: Vec<(char, usize)> = line
        .spans
        .iter()
        .enumerate()
        .flat_map(|(i, s)| s.text.chars().map(move |c| (c, i)))
        .collect();

    // Compute break points as ranges of the chars vec.
    let mut rows: Vec<(usize, usize)> = Vec::new(); // [start, end)
    let mut start = 0;
    let mut col = 0u16;
    let mut last_space: Option<usize> = None;
    let mut i = 0;
    while i < chars.len() {
        let (ch, _) = chars[i];
        let w = char_width(ch);
        if col + w > width && i > start {
            let brk = match wrap {
                // If the overflowing char is itself a space, break right here;
                // otherwise pull the last word onto the next row.
                Wrap::Word if ch == ' ' => i,
                Wrap::Word => last_space
                    .filter(|&s| s > start)
                    .map(|s| s + 1) // break after the space
                    .unwrap_or(i),
                _ => i,
            };
            rows.push((start, brk));
            start = brk;
            // Skip leading spaces on the new row for word wrap.
            if wrap == Wrap::Word {
                while start < chars.len() && chars[start].0 == ' ' {
                    start += 1;
                }
            }
            i = start;
            col = 0;
            last_space = None;
            continue;
        }
        if ch == ' ' {
            last_space = Some(i);
        }
        col += w;
        i += 1;
    }
    if start < chars.len() {
        rows.push((start, chars.len()));
    }
    if rows.is_empty() {
        rows.push((0, 0));
    }

    // Rebuild styled lines from the ranges.
    rows.iter()
        .map(|&(s, e)| {
            let mut out = Line::empty();
            let mut run = String::new();
            let mut run_group: Option<usize> = None;
            for &(ch, g) in &chars[s..e] {
                match run_group {
                    Some(cur) if cur == g => run.push(ch),
                    Some(cur) => {
                        out.spans.push(Span { text: std::mem::take(&mut run), hl: groups[cur].clone() });
                        run.push(ch);
                        run_group = Some(g);
                    }
                    None => {
                        run.push(ch);
                        run_group = Some(g);
                    }
                }
            }
            if let Some(cur) = run_group {
                out.spans.push(Span { text: run, hl: groups[cur].clone() });
            }
            out
        })
        .collect()
}

/// Truncate a styled line to `width` display cells, appending `…` when
/// anything was cut (the ellipsis fits within `width`).
pub fn truncate_line(line: &Line, width: u16) -> Line {
    if line.width() <= width {
        return line.clone();
    }
    let budget = width.saturating_sub(1); // room for the ellipsis
    let mut out = Line::empty();
    let mut col = 0u16;
    'outer: for span in &line.spans {
        let mut kept = String::new();
        for ch in span.text.chars() {
            let w = char_width(ch);
            if col + w > budget {
                if !kept.is_empty() {
                    out.spans.push(Span { text: kept, hl: span.hl.clone() });
                }
                out.spans.push(Span { text: "…".into(), hl: span.hl.clone() });
                break 'outer;
            }
            col += w;
            kept.push(ch);
        }
        out.spans.push(Span { text: kept, hl: span.hl.clone() });
    }
    out
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn width_is_display_cells() {
        assert_eq!(display_width("abc"), 3);
        assert_eq!(display_width("日本"), 4);
        assert_eq!(Line::raw("a日").width(), 3);
    }

    #[test]
    fn word_wrap_breaks_on_spaces() {
        let line = Line::raw("hello brave new world");
        let rows = wrap_line(&line, 11, Wrap::Word);
        let texts: Vec<String> = rows.iter().map(Line::text).collect();
        assert_eq!(texts, vec!["hello brave", "new world"]);
    }

    #[test]
    fn word_wrap_falls_back_to_char_for_long_words() {
        let line = Line::raw("abcdefghij");
        let rows = wrap_line(&line, 4, Wrap::Word);
        let texts: Vec<String> = rows.iter().map(Line::text).collect();
        assert_eq!(texts, vec!["abcd", "efgh", "ij"]);
    }

    #[test]
    fn wrap_preserves_span_styles() {
        let line = Line::from_spans([Span::hl("red", "Error"), Span::raw("blue")]);
        let rows = wrap_line(&line, 5, Wrap::Char);
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].spans[0].hl.as_deref(), Some("Error"));
        assert_eq!(rows[0].spans[0].text, "red");
        assert_eq!(rows[0].spans[1].text, "bl");
        assert_eq!(rows[1].text(), "ue");
    }

    #[test]
    fn wide_chars_wrap_by_cells() {
        let line = Line::raw("日本語");
        let rows = wrap_line(&line, 4, Wrap::Char);
        let texts: Vec<String> = rows.iter().map(Line::text).collect();
        assert_eq!(texts, vec!["日本", "語"]);
    }

    #[test]
    fn truncate_appends_ellipsis() {
        let line = Line::raw("hello world");
        let t = truncate_line(&line, 7);
        assert_eq!(t.text(), "hello …");
        assert!(t.width() <= 7);
        // No-op when it fits.
        assert_eq!(truncate_line(&line, 20).text(), "hello world");
    }
}
