//! The search engine. Everything here runs on background threads and never
//! touches Neovim; results go back to the UI as [`Event`]s over a channel the
//! UI drains on a timer.
//!
//! - [`Matcher`]: a thread that owns a corpus (files, buffers or lines) and
//!   answers fuzzy queries with the top results, scoring in parallel.
//! - [`walk_files`]: walks the project (ripgrep's walker: `.gitignore`-aware)
//!   and streams paths into a matcher.
//! - [`grep`]: searches file contents for one query, streaming matches.
//!
//! Every query carries a generation number; the UI drops events from
//! generations it has moved past.

use std::cmp::Ordering;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering as AtomicOrdering};
use std::sync::mpsc::{Receiver, RecvTimeoutError, Sender, channel};
use std::thread;
use std::time::{Duration, Instant};

use ignore::overrides::OverrideBuilder;
use ignore::{WalkBuilder, WalkState};
use rayon::prelude::*;

use crate::config::{Grep, Walk};
use crate::entry::{Entry, Target};
use crate::fuzzy::{Pattern, Scratch};

/// What the background threads report.
pub enum Event {
    /// Fresh top results for a fuzzy query (replaces the previous list):
    /// `items` are the best `limit` of the `matched` entries.
    Matched { generation: u64, items: Vec<Entry>, matched: usize, complete: bool },
    /// More grep matches (appended), and whether the search is over.
    Grep { generation: u64, items: Vec<Entry>, done: bool, truncated: bool },
    /// The query couldn't run (e.g. an invalid regex).
    Error { generation: u64, message: String },
}

/// How results are ordered when the query is empty.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EmptyOrder {
    /// As the corpus was given (buffers by recency, lines in order).
    Corpus,
    /// Alphabetically (files).
    Text,
}

enum Cmd {
    Extend(Vec<Entry>),
    Complete,
    Query { generation: u64, query: String, limit: usize },
    Pause,
}

/// Recompute at most this often while the corpus is still streaming in.
const THROTTLE: Duration = Duration::from_millis(40);

/// A fuzzy matcher thread. Dropping every handle stops it.
#[derive(Clone)]
pub struct Matcher {
    tx: Sender<Cmd>,
}

impl Matcher {
    pub fn spawn(order: EmptyOrder, events: Sender<Event>) -> Self {
        let (tx, rx) = channel();
        let _ = thread::Builder::new().name("seek-matcher".into()).spawn(move || run_matcher(rx, order, events));
        Self { tx }
    }

    /// Add entries to the corpus.
    pub fn extend(&self, entries: Vec<Entry>) {
        let _ = self.tx.send(Cmd::Extend(entries));
    }

    /// The corpus is complete (no more `extend`s).
    pub fn complete(&self) {
        let _ = self.tx.send(Cmd::Complete);
    }

    /// Stop ranking (another mode is searching); the corpus keeps growing.
    pub fn pause(&self) {
        let _ = self.tx.send(Cmd::Pause);
    }

    /// Rank the corpus for `query` and send the best `limit` results (sent
    /// again as the corpus grows). Asking again with a bigger `limit` loads
    /// more of the same ranking.
    pub fn query(&self, generation: u64, query: &str, limit: usize) {
        let _ = self.tx.send(Cmd::Query { generation, query: query.to_string(), limit });
    }
}

fn run_matcher(rx: Receiver<Cmd>, order: EmptyOrder, events: Sender<Event>) {
    let mut corpus: Vec<Entry> = Vec::new();
    let mut complete = false;
    let mut query: Option<(u64, String, usize)> = None;
    let mut dirty = false;
    let mut last = Instant::now() - THROTTLE;

    loop {
        // Wait for work; while results are pending, only until the throttle expires.
        let first = if dirty {
            match rx.recv_timeout(THROTTLE.saturating_sub(last.elapsed())) {
                Ok(cmd) => Some(cmd),
                Err(RecvTimeoutError::Timeout) => None,
                Err(RecvTimeoutError::Disconnected) => return,
            }
        } else {
            match rx.recv() {
                Ok(cmd) => Some(cmd),
                Err(_) => return,
            }
        };

        // Apply everything that is queued, keeping only the newest query.
        let mut urgent = false;
        for cmd in first.into_iter().chain(std::iter::from_fn(|| rx.try_recv().ok())) {
            match cmd {
                Cmd::Extend(more) => {
                    corpus.extend(more);
                    dirty = true;
                },
                Cmd::Complete => {
                    complete = true;
                    dirty = true;
                    urgent = true;
                },
                Cmd::Query { generation, query: q, limit } => {
                    query = Some((generation, q, limit));
                    dirty = true;
                    urgent = true;
                },
                Cmd::Pause => {
                    query = None;
                    dirty = false;
                },
            }
        }

        let Some((generation, q, limit)) = &query else {
            // Nothing to rank for: sleep until a command comes (not spin on
            // the throttle timeout while the corpus streams in).
            dirty = false;
            continue;
        };
        if dirty && (urgent || last.elapsed() >= THROTTLE) {
            let (items, matched) = top(&corpus, q, order, *limit);
            let event = Event::Matched { generation: *generation, items, matched, complete };
            if events.send(event).is_err() {
                return;
            }
            dirty = false;
            last = Instant::now();
        }
    }
}

/// The best `max` entries for `query`, with highlight positions, and how many matched.
pub fn top(corpus: &[Entry], query: &str, order: EmptyOrder, max: usize) -> (Vec<Entry>, usize) {
    let pattern = Pattern::parse(query);
    if pattern.is_empty() {
        let mut idx: Vec<usize> = (0..corpus.len()).collect();
        if order == EmptyOrder::Text {
            let by_text = |a: &usize, b: &usize| corpus[*a].text.cmp(&corpus[*b].text);
            if idx.len() > max {
                idx.select_nth_unstable_by(max, by_text);
                idx.truncate(max);
            }
            idx.sort_unstable_by(by_text);
        }
        idx.truncate(max);
        return (idx.into_iter().map(|i| corpus[i].clone()).collect(), corpus.len());
    }

    let mut scored: Vec<(i64, usize)> = corpus
        .par_iter()
        .enumerate()
        .map_init(Scratch::default, |scratch, (i, e)| pattern.score(&e.text, scratch).map(|s| (s, i)))
        .flatten()
        .collect();
    let matched = scored.len();
    let rank = |a: &(i64, usize), b: &(i64, usize)| -> Ordering {
        b.0.cmp(&a.0).then(corpus[a.1].text.len().cmp(&corpus[b.1].text.len())).then(a.1.cmp(&b.1))
    };
    if scored.len() > max {
        scored.select_nth_unstable_by(max, rank);
        scored.truncate(max);
    }
    scored.sort_unstable_by(rank);
    let items = scored
        .into_iter()
        .map(|(_, i)| {
            let mut e = corpus[i].clone();
            e.hl = pattern.positions(&e.text);
            e
        })
        .collect();
    (items, matched)
}

// -----------------------------------------------------------------------------
// Walking
// -----------------------------------------------------------------------------

fn walker(root: &Path, opts: &Walk) -> WalkBuilder {
    let mut b = WalkBuilder::new(root);
    b.hidden(!opts.hidden)
        .follow_links(opts.follow)
        .git_ignore(opts.gitignore)
        .git_global(opts.gitignore)
        .git_exclude(opts.gitignore)
        .ignore(opts.gitignore)
        .parents(opts.gitignore)
        .require_git(false)
        // Never descend into the git database, even with `hidden`.
        .filter_entry(|e| e.file_name() != ".git");
    if !opts.exclude.is_empty() {
        let mut ov = OverrideBuilder::new(root);
        for glob in &opts.exclude {
            // An override glob whitelists; `!glob` excludes.
            let _ = ov.add(&format!("!{glob}"));
        }
        if let Ok(ov) = ov.build() {
            b.overrides(ov);
        }
    }
    b
}

/// `path` relative to `root`, with `/` separators.
fn relative(root: &Path, path: &Path) -> String {
    let rel = path.strip_prefix(root).unwrap_or(path);
    let s = rel.to_string_lossy();
    if std::path::MAIN_SEPARATOR == '/' { s.into_owned() } else { s.replace(std::path::MAIN_SEPARATOR, "/") }
}

/// Collects entries on one walker thread and hands them on in batches.
struct Batch<F: FnMut(Vec<Entry>)> {
    items: Vec<Entry>,
    flush: F,
}

impl<F: FnMut(Vec<Entry>)> Batch<F> {
    fn push(&mut self, e: Entry, limit: usize) {
        self.items.push(e);
        if self.items.len() >= limit {
            (self.flush)(std::mem::take(&mut self.items));
        }
    }
}

impl<F: FnMut(Vec<Entry>)> Drop for Batch<F> {
    fn drop(&mut self) {
        if !self.items.is_empty() {
            (self.flush)(std::mem::take(&mut self.items));
        }
    }
}

/// Walk `root` on a background thread, streaming every file into `matcher`.
pub fn walk_files(root: PathBuf, opts: Walk, matcher: Matcher, cancel: Arc<AtomicBool>) {
    let _ = thread::Builder::new().name("seek-files".into()).spawn(move || {
        walker(&root, &opts).build_parallel().run(|| {
            let matcher = matcher.clone();
            let cancel = cancel.clone();
            let root = root.clone();
            let mut batch = Batch { items: Vec::new(), flush: move |items| matcher.extend(items) };
            Box::new(move |entry| {
                if cancel.load(AtomicOrdering::Relaxed) {
                    return WalkState::Quit;
                }
                if let Ok(entry) = entry
                    && entry.file_type().is_some_and(|t| {
                        // A link counts when it leads to a file (not a
                        // directory, not nowhere).
                        t.is_file() || t.is_symlink() && std::fs::metadata(entry.path()).is_ok_and(|m| m.is_file())
                    })
                {
                    let path = relative(&root, entry.path());
                    batch.push(Entry::new(Target::File { path: path.clone() }, path), 512);
                }
                WalkState::Continue
            })
        });
        if !cancel.load(AtomicOrdering::Relaxed) {
            matcher.complete();
        }
    });
}

// -----------------------------------------------------------------------------
// Grep
// -----------------------------------------------------------------------------

/// The longest line shown for a match (the line is still searched in full).
const MAX_LINE_CHARS: usize = 400;

/// A regex for `query`: literal unless `regex`, smart-case.
pub fn build_regex(query: &str, regex: bool) -> Result<(regex::bytes::Regex, regex::Regex), String> {
    let pattern = if regex { query.to_string() } else { regex::escape(query) };
    let ignore_case = !query.chars().any(|c| c.is_uppercase());
    let bytes = regex::bytes::RegexBuilder::new(&pattern)
        .case_insensitive(ignore_case)
        .build()
        .map_err(|e| e.to_string())?;
    let text = regex::RegexBuilder::new(&pattern)
        .case_insensitive(ignore_case)
        .build()
        .map_err(|e| e.to_string())?;
    Ok((bytes, text))
}

/// Search the contents of every file under `root` for `query` on background
/// threads; stops at `opts.max_results` or when `cancel` is set.
pub fn grep(root: PathBuf, query: String, opts: Grep, generation: u64, events: Sender<Event>, cancel: Arc<AtomicBool>) {
    let _ = thread::Builder::new().name("seek-grep".into()).spawn(move || {
        let (bytes_re, text_re) = match build_regex(&query, opts.regex) {
            Ok(r) => r,
            Err(message) => {
                let _ = events.send(Event::Error { generation, message });
                return;
            },
        };
        let found = Arc::new(AtomicUsize::new(0));
        let truncated = Arc::new(AtomicBool::new(false));
        walker(&root, &opts.walk).build_parallel().run(|| {
            let (events, cancel, found, truncated) = (events.clone(), cancel.clone(), found.clone(), truncated.clone());
            let (bytes_re, text_re, root) = (bytes_re.clone(), text_re.clone(), root.clone());
            let (max, max_size) = (opts.max_results, opts.max_filesize);
            let mut batch = Batch {
                items: Vec::new(),
                flush: {
                    let events = events.clone();
                    move |items| {
                        let _ = events.send(Event::Grep { generation, items, done: false, truncated: false });
                    }
                },
            };
            Box::new(move |entry| {
                if cancel.load(AtomicOrdering::Relaxed) || truncated.load(AtomicOrdering::Relaxed) {
                    return WalkState::Quit;
                }
                let Ok(entry) = entry else { return WalkState::Continue };
                if !entry.file_type().is_some_and(|t| t.is_file()) {
                    return WalkState::Continue;
                }
                if entry.metadata().is_ok_and(|m| m.len() > max_size) {
                    return WalkState::Continue;
                }
                let Ok(data) = std::fs::read(entry.path()) else { return WalkState::Continue };
                if is_binary(&data) {
                    return WalkState::Continue;
                }
                let path = relative(&root, entry.path());
                for (i, line) in data.split(|b| *b == b'\n').enumerate() {
                    let line = line.strip_suffix(b"\r").unwrap_or(line);
                    let Some(m) = bytes_re.find(line) else { continue };
                    if found.fetch_add(1, AtomicOrdering::Relaxed) >= max {
                        truncated.store(true, AtomicOrdering::Relaxed);
                        return WalkState::Quit;
                    }
                    batch.push(match_entry(&path, i + 1, m.start(), line, &text_re), 64);
                }
                WalkState::Continue
            })
        });
        if !cancel.load(AtomicOrdering::Relaxed) {
            let truncated = truncated.load(AtomicOrdering::Relaxed);
            let _ = events.send(Event::Grep { generation, items: Vec::new(), done: true, truncated });
        }
    });
}

/// NUL in the first 8 KiB: treat as binary (what ripgrep and git do).
pub fn is_binary(data: &[u8]) -> bool {
    data[..data.len().min(8192)].contains(&0)
}

/// A grep result: the line (leading whitespace trimmed, long lines cut) with
/// its matches highlighted.
fn match_entry(path: &str, lnum: usize, col: usize, line: &[u8], re: &regex::Regex) -> Entry {
    let text = String::from_utf8_lossy(line);
    let trimmed = text.trim_start();
    let shown: String = trimmed.chars().take(MAX_LINE_CHARS).collect();
    let mut hl = Vec::new();
    for m in re.find_iter(&shown) {
        let start = shown[..m.start()].chars().count();
        let len = m.as_str().chars().count();
        hl.extend(start..start + len);
    }
    let mut e = Entry::new(Target::Match { path: path.to_string(), lnum, col }, shown);
    e.hl = hl;
    e
}

#[cfg(test)]
mod tests {
    use super::*;

    fn files(paths: &[&str]) -> Vec<Entry> {
        paths.iter().map(|p| Entry::new(Target::File { path: p.to_string() }, *p)).collect()
    }

    #[test]
    fn top_ranks_and_counts() {
        let corpus = files(&["src/domain.rs", "src/main.rs", "README.md"]);
        let (items, matched) = top(&corpus, "main", EmptyOrder::Text, 10);
        assert_eq!(matched, 2);
        assert_eq!(items[0].text, "src/main.rs");
        assert_eq!(items[0].hl, vec![4, 5, 6, 7]);
    }

    #[test]
    fn empty_query_orders_and_limits() {
        let corpus = files(&["b", "c", "a"]);
        let (items, matched) = top(&corpus, "", EmptyOrder::Text, 2);
        assert_eq!(matched, 3);
        assert_eq!(items.iter().map(|e| e.text.as_str()).collect::<Vec<_>>(), vec!["a", "b"]);
        let (items, _) = top(&corpus, "", EmptyOrder::Corpus, 10);
        assert_eq!(items.iter().map(|e| e.text.as_str()).collect::<Vec<_>>(), vec!["b", "c", "a"]);
    }

    #[test]
    fn regex_is_literal_and_smart_case_by_default() {
        let (re, _) = build_regex("a.b", false).unwrap();
        assert!(re.is_match(b"xa.by"));
        assert!(!re.is_match(b"axb"));
        let (re, _) = build_regex("Foo", false).unwrap();
        assert!(!re.is_match(b"foo"));
        let (re, _) = build_regex("foo", false).unwrap();
        assert!(re.is_match(b"FOO"));
        assert!(build_regex("(", true).is_err());
    }

    #[test]
    fn match_entries_trim_and_highlight() {
        let (_, re) = build_regex("foo", false).unwrap();
        let e = match_entry("a.rs", 3, 4, b"    let foo = 1;", &re);
        assert_eq!(e.text, "let foo = 1;");
        assert_eq!(e.hl, vec![4, 5, 6]);
        assert_eq!(e.target, Target::Match { path: "a.rs".into(), lnum: 3, col: 4 });
    }

    #[test]
    fn walking_and_grepping_a_tree() {
        let dir = std::env::temp_dir().join(format!("seek-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("src")).unwrap();
        std::fs::create_dir_all(dir.join("ignored")).unwrap();
        std::fs::write(dir.join(".gitignore"), "ignored/\n").unwrap();
        std::fs::write(dir.join("src/main.rs"), "fn main() {\n    println!(\"hi\");\n}\n").unwrap();
        std::fs::write(dir.join("ignored/x.rs"), "fn main() {}\n").unwrap();
        std::fs::write(dir.join("bin.dat"), b"main\0\0").unwrap();

        // Files: the ignored directory and hidden files are skipped.
        let (tx, rx) = channel();
        let m = Matcher::spawn(EmptyOrder::Text, tx);
        walk_files(dir.clone(), Walk::default(), m.clone(), Arc::new(AtomicBool::new(false)));
        m.query(1, "", 100);
        let mut files = Vec::new();
        while let Ok(Event::Matched { items, complete, .. }) = rx.recv_timeout(Duration::from_secs(5)) {
            files = items.into_iter().map(|e| e.text).collect::<Vec<_>>();
            if complete {
                break;
            }
        }
        assert_eq!(files, vec!["bin.dat", "src/main.rs"]);

        // Grep: binary and ignored files are skipped.
        let (tx, rx) = channel();
        grep(dir.clone(), "main".into(), Grep::default(), 7, tx, Arc::new(AtomicBool::new(false)));
        let mut found = Vec::new();
        loop {
            match rx.recv_timeout(Duration::from_secs(5)).expect("grep finishes") {
                Event::Grep { generation, items, done, .. } => {
                    assert_eq!(generation, 7);
                    found.extend(items);
                    if done {
                        break;
                    }
                },
                _ => panic!("unexpected event"),
            }
        }
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].target, Target::Match { path: "src/main.rs".into(), lnum: 1, col: 3 });
        let _ = std::fs::remove_dir_all(&dir);
    }
}
