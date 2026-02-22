//! Core logic to determine the indentation style from text content.
//!
//! The `analyzer` module processes the content of a file (as bytes) to determine which indentation style is used.
//! It inspects each line's leading whitespace and classifies the file's indent style as tabs, spaces, mixed, none, or binary (the latter is typically flagged during scanning).
//!
//! The analysis logic handles different line endings (LF, CRLF, CR) and operates on raw bytes, making it robust even if the input is not valid UTF-8.

use rayon::prelude::*;

use super::types::IndentStyle;

/// Analyzer for indentation patterns.
pub struct IndentAnalyzer;

impl IndentAnalyzer {
    pub fn new() -> Self {
        Self
    }

    /// Analyze the given content bytes and determine the indentation style.
    ///
    /// This method assumes the input is not binary (for binary content, detection should be handled in the scanning phase).
    /// It will interpret the bytes, handling UTF-8 or other encodings gracefully by focusing only on ASCII whitespace and line breaks.
    pub fn analyze(&self, content: &[u8]) -> IndentStyle {
        if content.is_empty() {
            return IndentStyle::None;
        }
        // Split the content into lines (handles LF, CRLF, and CR line endings).
        let lines = Self::split_lines(content);

        // Counters for lines that use spaces, tabs, or mixed indent.
        let (space_count, tab_count, mixed_count) = lines
            .par_iter()
            .map(|line| Self::classify_line_indent(line))
            .reduce(|| (0, 0, 0), |(s1, t1, m1), (s2, t2, m2)| (s1 + s2, t1 + t2, m1 + m2));

        // Determine the overall indentation style.
        match (mixed_count > 0, space_count > 0, tab_count > 0) {
            (true, _, _) | (_, true, true) => IndentStyle::Mixed,
            (_, true, _) => IndentStyle::Space(Self::infer_space_indent_width(&lines)),
            (_, _, true) => IndentStyle::Tab,
            _ => IndentStyle::None,
        }
    }

    /// Infer the typical number of spaces per indent level by examining space-indented lines.
    /// Returns a common indent size (like 2 or 4) if it can be determined, or 1 if inconsistent or only single-space indents.
    fn infer_space_indent_width(lines: &[Vec<u8>]) -> usize {
        // Collect indent lengths (leading spaces count) for each line that is indented with spaces.
        let mut indent_lengths: Vec<usize> = lines
            .iter()
            .filter_map(|line| {
                if line.is_empty() || line[0] != b' ' {
                    None
                } else {
                    let count = line.iter().take_while(|&&b| b == b' ').count();
                    if count > 0 { Some(count) } else { None }
                }
            })
            .collect();
        if indent_lengths.is_empty() {
            return 1;
        }
        indent_lengths.sort_unstable();
        let min = indent_lengths[0];
        if min == 0 {
            return 1;
        }
        // If all indent lengths are multiples of the smallest, assume that as indent unit.
        if indent_lengths.iter().all(|&len| len % min == 0) {
            return min;
        }
        // Otherwise, find the greatest common divisor of all indent lengths as a guess.
        let mut g = indent_lengths[0];
        for &len in &indent_lengths[1..] {
            g = gcd(g, len);
            if g <= 1 {
                break;
            }
        }

        g.max(1)
    }

    /// Classify a single line's leading indent characters.
    /// Returns a tuple `(spaces, tabs, mixed)` counts, where each is 0 or 1 for this line.
    fn classify_line_indent(line: &[u8]) -> (usize, usize, usize) {
        if line.is_empty() {
            return (0, 0, 0);
        }
        let mut space = 0;
        let mut tab = 0;
        let mut mixed = 0;
        let mut i = 0;
        // Count leading spaces or tabs
        while i < line.len() {
            match line[i] {
                b' ' => {
                    if tab > 0 {
                        mixed = 1;
                        break; // already saw a tab, now a space -> mixed indent
                    }
                    space = 1;
                },
                b'\t' => {
                    if space > 0 {
                        mixed = 1;
                        break;
                    }
                    tab = 1;
                },
                _ => break, // first non-whitespace character reached
            }
            i += 1;
        }
        // Note: if a line is only whitespace (indent and nothing else), we treat those indent characters as valid evidence.
        if mixed == 1 {
            (0, 0, 1)
        } else if tab == 1 {
            (0, 1, 0)
        } else if space == 1 {
            (1, 0, 0)
        } else {
            (0, 0, 0)
        }
    }

    /// Split content into lines, handling different line terminators (LF, CRLF, CR).
    /// Returns a vector of lines, each as a `Vec<u8>` of its bytes (excluding newline characters).
    fn split_lines(content: &[u8]) -> Vec<Vec<u8>> {
        let mut lines = Vec::new();
        let len = content.len();
        if len == 0 {
            return lines;
        }
        let mut start = 0;
        let mut i = 0;
        while i < len {
            match content[i] {
                b'\n' => {
                    // LF found – mark end of line
                    let end = if i > 0 && content[i - 1] == b'\r' {
                        i - 1 // exclude the preceding CR from the line
                    } else {
                        i
                    };
                    lines.push(content[start..end].to_vec());
                    start = i + 1;
                },
                b'\r' => {
                    if i == len - 1 {
                        // CR at EOF (no LF after), treat as line break
                        lines.push(content[start..i].to_vec());
                        start = i + 1;
                    }
                    // If CR is followed by LF, the LF branch above will handle pushing the line.
                },
                _ => { /* normal character, continue */ },
            }
            i += 1;
        }
        // Push the last line if the file doesn't end with a newline
        if start < len {
            lines.push(content[start..len].to_vec());
        }

        lines
    }
}

impl Default for IndentAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { a } else { gcd(b, a % b) }
}
