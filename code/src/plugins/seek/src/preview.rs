//! The preview: what a result shows — a file, a buffer, a line — loaded into
//! an ab-nui `CodeView` (a real buffer, so it gets syntax highlighting).

use std::cell::{Cell, RefCell};
use std::path::{Path, PathBuf};

use wp_ui::Result;
use wp_ui::nvim::Buffer;
use wp_ui::view::{CodeView, Embed};

use crate::engine::is_binary;
use crate::entry::{Entry, Target};
use crate::nvim;

/// What the view currently holds.
#[derive(Clone, Debug, PartialEq, Eq)]
enum Source {
    File(PathBuf),
    /// An image/video/PDF shown as media.
    Media(PathBuf),
    Buffer(i64),
    Empty,
}

pub struct Preview {
    view: CodeView,
    source: RefCell<Source>,
    /// Line count and filetype of real text content (not a "Binary file" note).
    meta: RefCell<Option<(usize, String)>>,
    /// The content was cut at this many lines.
    cut_at: Cell<Option<usize>>,
}

/// Everything needed to show an entry.
pub struct Ctx<'a> {
    pub root: &'a Path,
    /// The buffer lines mode searches (none: no file buffer around).
    pub lines_buf: Option<Buffer>,
    pub max_filesize: u64,
    pub max_lines: usize,
    /// Highlights matches on the target line (grep).
    pub grep_re: Option<&'a regex::Regex>,
    /// Show images/videos/PDFs as media (when the terminal can).
    pub media: bool,
}

impl Preview {
    pub fn new(line_numbers: bool) -> Result<Self> {
        Ok(Self {
            view: CodeView::new()?.line_numbers(line_numbers),
            source: RefCell::new(Source::Empty),
            meta: RefCell::new(None),
            cut_at: Cell::new(None),
        })
    }

    /// The widget that reserves the preview's place in the tree.
    pub fn widget(&self) -> Embed {
        self.view.widget()
    }

    /// Show `entry` (or nothing).
    pub fn show(&self, entry: Option<&Entry>, ctx: &Ctx) -> Result<()> {
        let Some(entry) = entry else {
            if *self.source.borrow() != Source::Empty {
                self.view.set_content::<&str>(&[], "")?;
                *self.source.borrow_mut() = Source::Empty;
                *self.meta.borrow_mut() = None;
            }
            return self.view.highlight(None, &[]);
        };

        let source = match &entry.target {
            Target::File { path } if ctx.media && CodeView::supports_media(&ctx.root.join(path).to_string_lossy()) => {
                Source::Media(ctx.root.join(path))
            },
            Target::File { path } | Target::Match { path, .. } => Source::File(ctx.root.join(path)),
            Target::Buffer { bufnr, .. } => Source::Buffer(*bufnr),
            Target::Line { .. } => match ctx.lines_buf {
                Some(b) => Source::Buffer(b.id()),
                None => Source::Empty,
            },
        };
        if let Source::Media(path) = &source {
            if *self.source.borrow() != source {
                *self.meta.borrow_mut() = None;
                self.view.show_media(&path.to_string_lossy())?;
                *self.source.borrow_mut() = source;
            }
            return Ok(());
        }
        let line = match &entry.target {
            Target::Match { lnum, .. } | Target::Line { lnum, .. } => Some(*lnum),
            Target::Buffer { bufnr, .. } => Some(nvim::call::<usize>("last_line", *bufnr)?),
            Target::File { .. } => None,
        };
        // Load at most `max_lines`, but always reach the target line.
        let target = line.unwrap_or(0);
        let beyond_cut = self.cut_at.get().is_some_and(|cut| target > cut);
        if *self.source.borrow() != source || beyond_cut {
            let max = ctx.max_lines.max(target + ctx.max_lines / 2);
            let (lines, ft, text) = load(&source, ctx, max)?;
            self.cut_at.set((text && lines.len() >= max).then_some(max));
            *self.meta.borrow_mut() = text.then(|| (lines.len(), ft.clone()));
            self.view.set_content(&lines, &ft)?;
            *self.source.borrow_mut() = source;
        }

        let ranges = match &entry.target {
            Target::Match { lnum, .. } => match_ranges(self.view.buffer(), *lnum, ctx.grep_re),
            // Lines entries leave out the indentation: shift past it.
            Target::Line { indent, .. } => {
                char_ranges(&entry.text, &entry.hl).into_iter().map(|(s, e)| (s + indent, e + indent)).collect()
            },
            _ => Vec::new(),
        };
        self.view.highlight(line, &ranges)
    }

    /// Whether media (an image…) is shown rather than text.
    pub fn is_media(&self) -> bool {
        matches!(*self.source.borrow(), Source::Media(_))
    }

    /// Line count and filetype of what is shown (when it is text).
    pub fn meta(&self) -> Option<(usize, String)> {
        self.meta.borrow().clone()
    }

    /// Scroll by half a page (negative: up).
    pub fn scroll_half_page(&self, down: bool) -> Result<()> {
        let half = (self.view.height() as i64 / 2).max(1);
        self.view.scroll_by(if down { half } else { -half })
    }

    pub fn close(&self) {
        self.view.close();
    }
}

/// Lines and filetype for a source, and whether that is the real text (not a
/// note such as "Binary file").
fn load(source: &Source, ctx: &Ctx, max_lines: usize) -> Result<(Vec<String>, String, bool)> {
    let note = |msg: String| Ok((vec![msg], String::new(), false));
    match source {
        Source::Empty | Source::Media(_) => Ok((Vec::new(), String::new(), false)),
        Source::Buffer(b) => {
            let (lines, ft): (Vec<String>, String) = nvim::call("buffer_lines", (*b, max_lines as i64))?;
            Ok((lines, ft, true))
        },
        Source::File(path) => {
            let display = path.to_string_lossy().to_string();
            // An open buffer shows its current (possibly unsaved) content.
            let loaded: i64 = nvim::call("loaded_buffer", display.as_str())?;
            if loaded > 0 {
                let (lines, ft): (Vec<String>, String) = nvim::call("buffer_lines", (loaded, max_lines as i64))?;
                return Ok((lines, ft, true));
            }
            let size = std::fs::metadata(path).map(|m| m.len()).unwrap_or(0);
            if size > ctx.max_filesize {
                return note(format!("  File too large to preview ({} KiB)", size / 1024));
            }
            let data = match std::fs::read(path) {
                Ok(d) => d,
                Err(e) => return note(format!("  Can't read file: {e}")),
            };
            if is_binary(&data) {
                return note("  Binary file".to_string());
            }
            let text = String::from_utf8_lossy(&data);
            let lines = text.lines().take(max_lines).map(|l| l.trim_end_matches('\r').to_string()).collect();
            let ft: String = nvim::call("filetype", display.as_str())?;
            Ok((lines, ft, true))
        },
    }
}

/// Byte ranges of `re`'s matches on line `lnum` of the preview buffer.
fn match_ranges(buf: Buffer, lnum: usize, re: Option<&regex::Regex>) -> Vec<(usize, usize)> {
    let Some(re) = re else { return Vec::new() };
    let Ok(lines) = buf.get_lines(lnum as i64 - 1, lnum as i64) else { return Vec::new() };
    let Some(line) = lines.first() else { return Vec::new() };
    re.find_iter(line).map(|m| (m.start(), m.end())).collect()
}

/// Merge highlighted char indices of `text` into byte ranges.
fn char_ranges(text: &str, hl: &[usize]) -> Vec<(usize, usize)> {
    let starts: Vec<(usize, usize)> = text.char_indices().map(|(b, c)| (b, b + c.len_utf8())).collect();
    let mut out: Vec<(usize, usize)> = Vec::new();
    for &i in hl {
        let Some(&(s, e)) = starts.get(i) else { continue };
        match out.last_mut() {
            Some(last) if last.1 == s => last.1 = e,
            _ => out.push((s, e)),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::char_ranges;

    #[test]
    fn char_indices_become_merged_byte_ranges() {
        assert_eq!(char_ranges("abcdef", &[1, 2, 4]), vec![(1, 3), (4, 5)]);
        assert_eq!(char_ranges("aéb", &[1, 2]), vec![(1, 4)]);
    }
}
