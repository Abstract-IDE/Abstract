//! One open seek window: its state, the searches feeding it, the widget tree
//! and the actions its keys run.
//!
//! The UI is all ab-nui: a modal `Surface` (the editor behind is dimmed and
//! doesn't react) holding a `Frame` — a `BufferInput` prompt (a real buffer:
//! insert/normal mode, every motion) and the results `List` on the left, a
//! `CodeView` preview (a real buffer: syntax highlighting) on the right, the
//! scrollbar on the divider.

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, Sender, channel};

use wp_ui::geometry::editor_size;
use wp_ui::lua;
use wp_ui::nvim::{self, Buffer, Timer, Window};
use wp_ui::prelude::*;
use wp_ui::reactive::{EffectHandle, unowned};
use wp_ui::text::truncate_line_start;
use wp_ui::theme::groups;

use crate::actions::{self, How};
use crate::config::{Action, Config};
use crate::engine::{self, EmptyOrder, Event, Matcher};
use crate::entry::{Entry, Mode, Target};
use crate::nvim as helpers;
use crate::preview::{self, Preview};

/// How often finished search results are pulled into the UI.
const PUMP_MS: u64 = 16;

#[derive(Clone, Debug, Default, PartialEq)]
struct Status {
    /// How many results there are in all (fuzzy: matched; grep: found so far).
    matched: usize,
    busy: bool,
    truncated: bool,
    error: Option<String>,
}

type Results = Rc<Vec<Rc<Entry>>>;

pub struct Session {
    cfg: Rc<Config>,
    root: PathBuf,
    origin_win: Window,
    origin_buf: Buffer,
    /// The buffer lines mode searches (see `helpers.lines_buffer`).
    lines_buf: Option<Buffer>,
    /// The line to start lines mode on (the cursor's, or the last one seen).
    origin_line: usize,

    mode: Signal<Mode>,
    input: BufferInput,
    results: Signal<Results>,
    status: Signal<Status>,
    list: ListState,
    show_preview: Signal<bool>,
    maximize: Signal<bool>,
    /// Facts about the selected result (footer).
    info: Signal<String>,

    generation: Cell<u64>,
    events: Sender<Event>,
    inbox: Receiver<Event>,
    matchers: RefCell<HashMap<Mode, Matcher>>,
    cancel_walk: Arc<AtomicBool>,
    cancel_grep: RefCell<Arc<AtomicBool>>,
    grep_timer: RefCell<Option<Timer>>,
    grep_re: RefCell<Option<regex::Regex>>,
    /// Every grep match so far; the list shows the first `limit`.
    grep_all: RefCell<Vec<Rc<Entry>>>,
    /// How many results are loaded into the list (grows a page at a time).
    limit: Cell<usize>,
    /// More results were asked for and haven't arrived yet.
    more_pending: Cell<bool>,
    /// Select the last result once everything is loaded (`G`).
    select_last: Cell<bool>,
    /// Select the origin line once the first lines results arrive.
    select_origin_line: Cell<bool>,
    /// The current generation's grep matches have replaced the old list.
    grep_shown: Cell<bool>,
    /// Text put in the prompt by code (`open`'s query, `grep_word`): taken
    /// literally, its first character is never a mode prefix.
    query_from_code: RefCell<Option<String>>,
    /// `<C-q>` is waiting for every result to load.
    quickfix_pending: Cell<bool>,
    filetypes: RefCell<HashMap<String, String>>,

    preview: Preview,
    surface: RefCell<Option<Surface>>,
    pump: RefCell<Option<Timer>>,
    effects: RefCell<Vec<EffectHandle>>,
    closed: Cell<bool>,
}

/// What `open` was asked for.
#[derive(Default)]
pub struct OpenOpts {
    pub mode: Option<Mode>,
    pub query: Option<String>,
    pub cwd: Option<PathBuf>,
}

impl Session {
    pub fn open(cfg: Config, opts: OpenOpts) -> Result<Rc<Self>> {
        let cfg = Rc::new(cfg);
        let origin_win = Window::current()?;
        let origin_buf = origin_win.buffer()?;
        let lines_buf = helpers::call::<i64>("lines_buffer", origin_buf.id()).ok().filter(|b| *b > 0).map(Buffer);
        let origin_line = match lines_buf {
            Some(b) if b == origin_buf => origin_win.cursor().map(|(r, _)| r.max(1) as usize).unwrap_or(1),
            Some(b) => helpers::call::<usize>("last_line", b.id()).unwrap_or(1),
            None => 1,
        };
        let root = match opts.cwd {
            Some(dir) if dir.is_dir() => dir,
            Some(dir) => return Err(wp_ui::Error::invalid(format!("Seek: no such directory: {}", dir.display()))),
            None => PathBuf::from(lua::call_fn::<String>("getcwd", ())?),
        };
        let mode = opts.mode.filter(|m| cfg.modes.contains(m)).unwrap_or(cfg.default_mode);
        let (events, inbox) = channel();

        let session = Rc::new(Self {
            root,
            origin_win,
            origin_buf,
            lines_buf,
            origin_line,
            mode: Signal::new(mode),
            input: BufferInput::new()?.start_insert(cfg.start_insert),
            results: Signal::new(Rc::new(Vec::new())),
            status: Signal::new(Status::default()),
            list: ListState::new(),
            show_preview: Signal::new(cfg.preview),
            maximize: Signal::new(false),
            info: Signal::new(String::new()),
            generation: Cell::new(0),
            events,
            inbox,
            matchers: RefCell::new(HashMap::new()),
            cancel_walk: Arc::new(AtomicBool::new(false)),
            cancel_grep: RefCell::new(Arc::new(AtomicBool::new(false))),
            grep_timer: RefCell::new(None),
            grep_re: RefCell::new(None),
            grep_all: RefCell::new(Vec::new()),
            limit: Cell::new(cfg.page_size),
            more_pending: Cell::new(false),
            select_last: Cell::new(false),
            select_origin_line: Cell::new(true),
            grep_shown: Cell::new(false),
            query_from_code: RefCell::new(None),
            quickfix_pending: Cell::new(false),
            filetypes: RefCell::new(HashMap::new()),
            preview: Preview::new(cfg.preview_line_numbers)?,
            surface: RefCell::new(None),
            pump: RefCell::new(None),
            effects: RefCell::new(Vec::new()),
            closed: Cell::new(false),
            cfg,
        });
        if let Some(q) = opts.query {
            session.set_query(&q)?;
        }
        session.start()?;
        Ok(session)
    }

    fn start(self: &Rc<Self>) -> Result<()> {
        let weak = Rc::downgrade(self);

        // Pull search results into the UI.
        *self.pump.borrow_mut() = Some(Timer::interval(PUMP_MS, {
            let weak = weak.clone();
            move || {
                if let Some(s) = weak.upgrade() {
                    s.drain();
                }
            }
        })?);

        // Re-search whenever the mode or the query changes.
        let input = unowned(|| {
            let weak = weak.clone();
            effect(move || {
                if let Some(s) = weak.upgrade() {
                    s.on_input();
                }
            })
        });
        // Preview whatever is selected (and load more results near the end).
        let preview = unowned(|| {
            let weak = weak.clone();
            effect(move || {
                if let Some(s) = weak.upgrade() {
                    s.on_selection();
                }
            })
        });
        self.effects.borrow_mut().extend([input, preview]);

        self.bind_keys()?;
        // The prompt closed from outside (`:q` in it): close seek.
        self.input.embedded().on_close(|| {
            let _ = nvim::schedule(crate::close);
        });

        // Modal: the editor behind is dimmed and doesn't react, and the
        // cursor stays in the prompt. The frame itself never takes the cursor.
        let surface = Surface::float(PopupOptions {
            size: Size::ratio(self.cfg.width, self.cfg.height),
            position: Position::Center,
            // The frame draws its own border.
            border: Border::None,
            focusable: false,
            enter: false,
            ..Default::default()
        })?
        .modal(true)
        .show({
            let weak = weak.clone();
            move || match weak.upgrade() {
                Some(s) => s.build(),
                None => Frame::new(Vec::new()),
            }
        })?;
        *self.surface.borrow_mut() = Some(surface);
        Ok(())
    }

    /// Seek's keys on the prompt: in insert and normal mode, plus normal-only ones.
    fn bind_keys(self: &Rc<Self>) -> Result<()> {
        let bind = |modes: &[&str], action: Action, lhs: &str| -> Result<()> {
            let weak = Rc::downgrade(self);
            self.input.on_key(modes, lhs, move || {
                if let Some(s) = weak.upgrade() {
                    s.act(action);
                }
            })
        };
        for (action, keys) in &self.cfg.keys {
            for lhs in keys {
                bind(&["i", "n"], *action, lhs)?;
            }
        }
        for (action, keys) in &self.cfg.normal_keys {
            for lhs in keys {
                bind(&["n"], *action, lhs)?;
            }
        }
        if self.cfg.backspace_to_default {
            for lhs in ["<BS>", "<C-h>"] {
                let weak = Rc::downgrade(self);
                self.input.on_key_or(&["i"], lhs, move || weak.upgrade().is_some_and(|s| s.backspace()))?;
            }
        }
        Ok(())
    }

    /// Tear everything down: stop searches, timers and effects, close windows.
    pub fn shutdown(&self) {
        if self.closed.replace(true) {
            return;
        }
        self.cancel_walk.store(true, Ordering::Relaxed);
        self.cancel_grep.borrow().store(true, Ordering::Relaxed);
        self.grep_timer.borrow_mut().take();
        self.pump.borrow_mut().take();
        for e in self.effects.borrow_mut().drain(..) {
            e.dispose();
        }
        self.matchers.borrow_mut().clear();
        // Closing the surface closes its embedded windows (prompt, preview).
        if let Some(surface) = self.surface.borrow_mut().take() {
            surface.close();
        }
        self.input.close();
        self.preview.close();
        if self.origin_win.is_valid() {
            let _ = self.origin_win.focus();
        }
    }

    pub fn focus(&self) -> Result<()> {
        self.input.focus()
    }

    /// Whether the cursor is in seek's prompt.
    pub fn has_focus(&self) -> bool {
        self.input.window().is_some_and(|w| Window::current().is_ok_and(|cur| cur == w))
    }

    pub fn is_visible(&self) -> bool {
        self.surface.borrow().as_ref().is_some_and(|s| s.is_visible())
    }

    pub fn mode(&self) -> Mode {
        self.mode.get_untracked()
    }

    pub fn set_query(&self, query: &str) -> Result<()> {
        *self.query_from_code.borrow_mut() = Some(query.replace(['\n', '\r'], " "));
        self.input.set_text(query)
    }

    /// Whether seek searches `dir`.
    pub fn has_root(&self, dir: &std::path::Path) -> bool {
        let canon = |p: &std::path::Path| p.canonicalize().unwrap_or_else(|_| p.to_path_buf());
        canon(dir) == canon(&self.root)
    }

    pub fn set_mode(&self, mode: Mode) {
        if self.cfg.modes.contains(&mode) {
            self.mode.set(mode);
        }
    }

    // -------------------------------------------------------------------------
    // Searching
    // -------------------------------------------------------------------------

    fn on_input(self: &Rc<Self>) {
        let mode = self.mode.get();
        let q = self.input.value().get();
        let from_code = self.query_from_code.borrow_mut().take().is_some_and(|t| t == q);
        // A mode prefix typed as the first character switches mode.
        if !from_code
            && let Some(first) = q.chars().next()
            && let Some(m) = self.cfg.prefix_mode(first)
            && m != mode
        {
            let rest: String = q.chars().skip(1).collect();
            // Edit the prompt after this callback (we may be inside its
            // TextChanged autocmd).
            let input = self.input.clone();
            let _ = nvim::schedule(move || {
                let _ = input.set_text(&rest);
            });
            self.mode.set(m);
            return;
        }
        untrack(|| {
            self.input.set_placeholder(Some(self.placeholder(mode)));
            self.search(mode, &q);
        });
    }

    fn search(self: &Rc<Self>, mode: Mode, q: &str) {
        let generation = self.generation.get() + 1;
        self.generation.set(generation);
        self.list.selected.set(0);
        self.list.offset.set(0);
        self.limit.set(self.cfg.page_size);
        self.more_pending.set(false);
        self.select_last.set(false);
        self.quickfix_pending.set(false);
        self.grep_shown.set(false);
        // Lines mode opens on the cursor's line: load at least up to it.
        if mode == Mode::Lines && self.select_origin_line.get() && q.is_empty() {
            self.limit.set(self.cfg.page_size + self.origin_line);
        }

        // A new query stops the grep in flight.
        self.cancel_grep.borrow().store(true, Ordering::Relaxed);
        self.grep_timer.borrow_mut().take();

        // Matchers of other modes stop ranking for queries no one sees.
        for (m, matcher) in self.matchers.borrow().iter() {
            if *m != mode {
                matcher.pause();
            }
        }

        if mode != Mode::Grep {
            self.status.update(|s| {
                s.busy = true;
                s.error = None;
                s.truncated = false;
            });
            self.matcher(mode).query(generation, q, self.limit.get());
            return;
        }

        if q.trim().is_empty() {
            *self.grep_re.borrow_mut() = None;
            self.grep_all.borrow_mut().clear();
            self.results.set(Rc::new(Vec::new()));
            self.status.set(Status::default());
            return;
        }
        *self.grep_re.borrow_mut() = engine::build_regex(q, self.cfg.grep.regex).ok().map(|(_, text)| text);
        // The old results stay until the first new match arrives.
        self.grep_all.borrow_mut().clear();
        self.status.update(|s| {
            s.busy = true;
            s.error = None;
            s.truncated = false;
        });
        let cancel = Arc::new(AtomicBool::new(false));
        *self.cancel_grep.borrow_mut() = cancel.clone();
        let (root, query, opts, events) =
            (self.root.clone(), q.to_string(), self.cfg.grep.clone(), self.events.clone());
        let start = move || {
            let (root, query, opts, events, cancel) =
                (root.clone(), query.clone(), opts.clone(), events.clone(), cancel.clone());
            engine::grep(root, query, opts, generation, events, cancel);
        };
        match Timer::once(self.cfg.grep.debounce_ms.max(1), start) {
            Ok(t) => *self.grep_timer.borrow_mut() = Some(t),
            Err(e) => self.status.update(|s| s.error = Some(e.to_string())),
        }
    }

    /// The matcher for a fuzzy mode, started (with its corpus) on first use.
    fn matcher(&self, mode: Mode) -> Matcher {
        if let Some(m) = self.matchers.borrow().get(&mode) {
            return m.clone();
        }
        let m = match mode {
            Mode::Files => {
                let m = Matcher::spawn(EmptyOrder::Text, self.events.clone());
                engine::walk_files(self.root.clone(), self.cfg.files.clone(), m.clone(), self.cancel_walk.clone());
                m
            },
            Mode::Buffers => {
                let m = Matcher::spawn(EmptyOrder::Corpus, self.events.clone());
                let list: Vec<mlua::Table> = helpers::call("buffers", self.origin_buf.id()).unwrap_or_default();
                m.extend(
                    list.into_iter()
                        .filter_map(|b| {
                            let bufnr = b.get::<i64>("bufnr").ok()?;
                            let modified = b.get::<bool>("modified").unwrap_or(false);
                            let name = b.get::<String>("name").unwrap_or_default();
                            Some(Entry::new(Target::Buffer { bufnr, modified }, name))
                        })
                        .collect(),
                );
                m.complete();
                m
            },
            // Grep searches files directly and never asks for a matcher.
            Mode::Lines | Mode::Grep => {
                let m = Matcher::spawn(EmptyOrder::Corpus, self.events.clone());
                let lines = self.lines_buf.map(|b| b.get_lines(0, -1).unwrap_or_default()).unwrap_or_default();
                m.extend(
                    lines
                        .into_iter()
                        .enumerate()
                        .filter(|(_, l)| !l.trim().is_empty())
                        .map(|(i, l)| {
                            // Indentation is left out of the text (shown and matched).
                            let text = l.trim_start();
                            let indent = l.len() - text.len();
                            Entry::new(Target::Line { lnum: i + 1, indent }, text)
                        })
                        .collect(),
                );
                m.complete();
                m
            },
        };
        self.matchers.borrow_mut().insert(mode, m.clone());
        m
    }

    /// Apply whatever the search threads have sent.
    fn drain(self: &Rc<Self>) {
        let current = self.generation.get();
        while let Ok(event) = self.inbox.try_recv() {
            match event {
                Event::Matched { generation, items, matched, complete } if generation == current => {
                    let items: Vec<Rc<Entry>> = items.into_iter().map(Rc::new).collect();
                    if self.select_origin_line.get()
                        && self.mode.get_untracked() == Mode::Lines
                        && self.input.text().is_empty()
                        && !items.is_empty()
                    {
                        self.select_origin_line.set(false);
                        let i = items.iter().position(|e| e.lnum().is_some_and(|l| l >= self.origin_line));
                        self.list.selected.set(i.unwrap_or(items.len() - 1));
                    }
                    let n = items.len();
                    self.results.set(Rc::new(items));
                    self.status.set(Status { matched, busy: !complete, truncated: false, error: None });
                    self.more_pending.set(false);
                    if self.select_last.get() && n >= matched.min(self.limit.get()) {
                        self.select_last.set(false);
                        self.list.selected.set(n.saturating_sub(1));
                    }
                    if self.quickfix_pending.get() && n >= matched.min(self.limit.get()) {
                        self.quickfix_pending.set(false);
                        self.finish(Finish::Quickfix);
                    }
                },
                Event::Grep { generation, items, done, truncated } if generation == current => {
                    let n = {
                        let mut all = self.grep_all.borrow_mut();
                        all.extend(items.into_iter().map(Rc::new));
                        all.len()
                    };
                    // The list is a window onto the matches: the first matches
                    // replace the old list, then it fills up until it is full
                    // (and an empty search clears it when it ends).
                    if !self.grep_shown.get()
                        || self.results.get_untracked().len() < self.limit.get().min(n)
                        || (done && n == 0)
                    {
                        self.grep_shown.set(n > 0 || done);
                        self.show_grep_page();
                    }
                    self.status.update(|s| {
                        s.matched = n;
                        s.busy = !done;
                        s.truncated |= truncated;
                    });
                },
                Event::Error { generation, message } if generation == current => {
                    self.results.set(Rc::new(Vec::new()));
                    self.status.set(Status { error: Some(message), ..Status::default() });
                },
                _ => {},
            }
        }
    }

    /// Show the first `limit` grep matches.
    fn show_grep_page(&self) {
        let page: Vec<Rc<Entry>> = self.grep_all.borrow().iter().take(self.limit.get()).cloned().collect();
        self.results.set(Rc::new(page));
    }

    /// Load the next page of results (or all of them).
    fn load_more(&self, all: bool) {
        let loaded = self.results.get_untracked().len();
        let matched = self.status.get_untracked().matched;
        if loaded >= matched || self.more_pending.get() {
            return;
        }
        let limit = if all { matched } else { self.limit.get() + self.cfg.page_size };
        self.limit.set(limit.max(self.limit.get()));
        let mode = self.mode.get_untracked();
        if mode == Mode::Grep {
            self.show_grep_page();
            if self.select_last.replace(false) {
                self.list.selected.set(self.results.get_untracked().len().saturating_sub(1));
            }
            return;
        }
        self.more_pending.set(true);
        // Same generation: the bigger page replaces the list, the selection stays.
        let q = self.input.text();
        self.matcher(mode).query(self.generation.get(), &q, self.limit.get());
    }

    // -------------------------------------------------------------------------
    // Selection: preview, info, paging
    // -------------------------------------------------------------------------

    fn selected(&self) -> Option<Rc<Entry>> {
        let items = self.results.get_untracked();
        items.get(self.list.selected.get_untracked()).cloned()
    }

    fn on_selection(&self) {
        let items = self.results.get();
        let selected = self.list.selected.get();
        let entry = items.get(selected).cloned();
        let show_preview = self.show_preview.get();
        untrack(|| {
            // Nearing the end of what is loaded: load the next page.
            if selected + self.cfg.page_size / 4 >= items.len() {
                self.load_more(false);
            }
            if show_preview {
                let re = self.grep_re.borrow();
                let ctx = preview::Ctx {
                    root: &self.root,
                    lines_buf: self.lines_buf,
                    max_filesize: self.cfg.preview_max_filesize,
                    max_lines: self.cfg.preview_max_lines,
                    grep_re: re.as_ref(),
                    media: self.cfg.preview_media,
                };
                let _ = self.preview.show(entry.as_deref(), &ctx);
            }
            self.info.set(self.describe(entry.as_deref()));
        });
    }

    /// The footer's facts about a result: filetype, size, lines, age…
    fn describe(&self, entry: Option<&Entry>) -> String {
        let Some(entry) = entry else { return String::new() };
        let mut parts: Vec<String> = Vec::new();
        let preview = self.preview.meta();
        match &entry.target {
            Target::File { path } | Target::Match { path, .. } => {
                if let Target::Match { lnum, col, .. } = &entry.target {
                    parts.push(format!("line {lnum}:{}", col + 1));
                }
                let ft = preview.as_ref().map(|(_, ft)| ft.clone()).unwrap_or_else(|| self.filetype(path));
                if self.preview.is_media() {
                    let ext = path.rsplit_once('.').map(|(_, e)| e.to_lowercase()).unwrap_or_default();
                    parts.push(format!("{ext} media").trim().to_string());
                } else if !ft.is_empty() {
                    parts.push(ft);
                }
                if let Ok(meta) = std::fs::metadata(self.root.join(path)) {
                    parts.push(human_size(meta.len()));
                    if let Some((lines, _)) = &preview {
                        parts.push(format!("{lines} lines"));
                    }
                    if let Ok(t) = meta.modified() {
                        parts.push(format!("modified {}", ago(t)));
                    }
                }
            },
            Target::Buffer { bufnr, modified } => {
                if let Some((lines, ft)) = &preview {
                    if !ft.is_empty() {
                        parts.push(ft.clone());
                    }
                    parts.push(format!("{lines} lines"));
                }
                parts.push(if *modified { "unsaved changes".into() } else { "saved".into() });
                parts.push(format!("buffer {bufnr}"));
            },
            Target::Line { lnum, .. } => {
                let total = self.lines_buf.and_then(|b| b.line_count().ok()).unwrap_or(0);
                parts.push(format!("line {lnum} of {total}"));
                if let Some((_, ft)) = preview.filter(|(_, ft)| !ft.is_empty()) {
                    parts.push(ft);
                }
            },
        }
        parts.join(" · ")
    }

    fn filetype(&self, path: &str) -> String {
        if let Some(ft) = self.filetypes.borrow().get(path) {
            return ft.clone();
        }
        let ft: String = helpers::call("filetype", path).unwrap_or_default();
        self.filetypes.borrow_mut().insert(path.to_string(), ft.clone());
        ft
    }

    // -------------------------------------------------------------------------
    // Actions
    // -------------------------------------------------------------------------

    fn act(self: &Rc<Self>, action: Action) {
        let n = self.results.get_untracked().len();
        let step = |by: isize| {
            if n > 0 {
                self.list.selected.update(|s| *s = (*s as isize + by).rem_euclid(n as isize) as usize);
            }
        };
        match action {
            Action::Next => step(1),
            Action::Prev => step(-1),
            Action::First => self.list.selected.set(0),
            Action::Last => {
                if n < self.status.get_untracked().matched {
                    // Load up to the real last result, then select it.
                    self.select_last.set(true);
                    self.load_more(true);
                } else {
                    self.list.selected.set(n.saturating_sub(1));
                }
            },
            Action::Open => self.finish(Finish::Open(How::Edit)),
            Action::Split => self.finish(Finish::Open(How::Split)),
            Action::VSplit => self.finish(Finish::Open(How::VSplit)),
            Action::Tab => self.finish(Finish::Open(How::Tab)),
            Action::Quickfix => {
                // Every result goes to the list, not just the loaded ones.
                let mode = self.mode.get_untracked();
                if mode != Mode::Grep && n < self.status.get_untracked().matched {
                    self.quickfix_pending.set(true);
                    self.load_more(true);
                } else {
                    self.finish(Finish::Quickfix);
                }
            },
            Action::Close => self.finish(Finish::Nothing),
            Action::PreviewDown => {
                let _ = self.preview.scroll_half_page(true);
            },
            Action::PreviewUp => {
                let _ = self.preview.scroll_half_page(false);
            },
            Action::TogglePreview => self.show_preview.update(|v| *v = !*v),
            Action::Maximize => {
                self.show_preview.set(true);
                self.maximize.update(|v| *v = !*v);
            },
            Action::Mode(m) => self.set_mode(m),
        }
    }

    /// Close seek, then do `what` — on the next tick, outside the key handler
    /// of the window being closed.
    fn finish(self: &Rc<Self>, what: Finish) {
        let selected = self.selected();
        let all = match self.mode.get_untracked() {
            Mode::Grep => Rc::new(self.grep_all.borrow().clone()),
            _ => self.results.get_untracked(),
        };
        let (root, lines_buf) = (self.root.clone(), self.lines_buf);
        let title = format!("Seek {}: {}", self.mode.get_untracked().title(), self.input.text());
        let _ = nvim::schedule(move || {
            crate::close();
            let res = match what {
                Finish::Nothing => Ok(()),
                Finish::Open(how) => match selected {
                    Some(e) => actions::open(&e, how, &root, lines_buf),
                    None => Ok(()),
                },
                Finish::Quickfix => actions::quickfix(&all, &title, &root, lines_buf),
            };
            crate::report(res);
        });
    }

    // -------------------------------------------------------------------------
    // The widget tree
    // -------------------------------------------------------------------------

    fn build(self: &Rc<Self>) -> Frame {
        let (columns, _) = editor_size().unwrap_or((0, 0));
        let has_preview = self.show_preview.get() && columns >= self.cfg.min_width_for_preview;
        let maximized = has_preview && self.maximize.get();

        if maximized {
            return Frame::new(vec![FrameColumn::new(self.prompt_row(), self.preview_body())]);
        }
        let results = FrameColumn::new(self.prompt_row(), self.results_view()).footer(self.counts());
        let frame = if has_preview {
            Frame::new(vec![results, FrameColumn::new(self.preview_title(), self.preview_body()).footer(self.info())])
                .split(self.cfg.preview_ratio)
        } else {
            Frame::new(vec![results])
        };
        // The scrollbar sits on the divider; it spans all the results, not
        // just the loaded page.
        let total = self.status.get().matched.max(self.results.get().len());
        frame.scrollbar(total, self.list.offset.clone())
    }

    /// A `❯`, then the prompt (its placeholder names the mode).
    /// The mode's icon, then the prompt.
    fn prompt_row(&self) -> Element {
        let icon = Text::from_line(Line::from_spans([Span::hl(format!(" {} ", self.mode.get().icon()), "SeekMode")]));
        Box::new(row![icon, Expanded::new(self.input.widget())])
    }

    fn results_view(self: &Rc<Self>) -> Element {
        let items = self.results.get();
        let status = self.status.get();
        if items.is_empty() {
            let msg = if let Some(e) = &status.error {
                return Box::new(Text::new(format!(" {e}")).fg(groups::ERROR).wrap(Wrap::Word));
            } else if status.busy {
                " Searching…"
            } else if self.mode.get() == Mode::Grep && self.input.value().get().trim().is_empty() {
                " Type to search file contents"
            } else if self.mode.get() == Mode::Lines && self.lines_buf.is_none() {
                " No file buffer to search lines in"
            } else {
                " No results"
            };
            return Box::new(Text::new(msg).fg(groups::MUTED));
        }
        Box::new(
            List::with_render(items.as_ref().clone(), &self.list, |e, cx| row(e, cx))
                .scrollbar(false)
                .gutter(false),
        )
    }

    /// `selected/loaded/total`.
    fn counts(&self) -> Element {
        let s = self.status.get();
        let loaded = self.results.get().len();
        let selected = if loaded == 0 { 0 } else { self.list.selected.get().min(loaded - 1) + 1 };
        let mut line = Line::empty();
        match &s.error {
            Some(_) => line.spans.push(Span::hl(" error", groups::ERROR)),
            None => {
                let more = if s.truncated { "+" } else { "" };
                line.spans.push(Span::hl(format!(" {selected}/{loaded}/{}{more}", s.matched), "SeekCount"));
                if s.busy {
                    line.spans.push(Span::hl(" …", "SeekCount"));
                }
            },
        }
        Box::new(Text::from_line(line).ellipsis())
    }

    /// The preview, a cell in from the divider so the text doesn't touch it.
    fn preview_body(&self) -> Padding {
        Padding::left(1, self.preview.widget())
    }

    fn preview_title(&self) -> Element {
        let items = self.results.get();
        let title = match items.get(self.list.selected.get()).map(|e| e.as_ref()) {
            Some(Entry { target: Target::File { path } | Target::Match { path, .. }, .. }) => path.clone(),
            Some(Entry { target: Target::Buffer { .. }, text, .. }) => text.clone(),
            Some(Entry { target: Target::Line { .. }, .. }) => self.origin_name(),
            None => String::new(),
        };
        // Styled like the results: a dim directory, the plain file name, and
        // cut from the left when long so the name stays visible.
        let mut icon = Line::from_spans([Span::raw(" ")]);
        if !title.is_empty() {
            push_icon(&mut icon, &title);
        }
        let mut path = Line::empty();
        push_path(&mut path, &title);
        Box::new(row![Text::from_line(icon), Expanded::new(Text::from_line(path).ellipsis_start())])
    }

    /// The lines-mode buffer's name, relative to the cwd (or `~`).
    fn origin_name(&self) -> String {
        let name = self.lines_buf.and_then(|b| b.name().ok()).unwrap_or_default();
        lua::call_fn::<String>("fnamemodify", (name, ":~:.")).unwrap_or_default()
    }

    fn info(&self) -> Element {
        Box::new(Text::new(format!(" {}", self.info.get())).fg("SeekCount").ellipsis())
    }
}

enum Finish {
    Nothing,
    Open(How),
    Quickfix,
}

/// One result row, fitted to `cx.width`: long paths lose characters on the
/// left so the file name stays visible. On the selected row only the matches
/// (and icons) keep their colour; dim parts take the selection's.
fn row(e: &Entry, cx: &ItemCx) -> Line {
    let dim = |group: &'static str| (!cx.selected).then_some(group);
    // One space of padding, like the prompt (` ❯`) and the count below.
    let mut line = Line::from_spans([Span::raw(" ")]);
    match &e.target {
        Target::File { path } => {
            push_icon(&mut line, path);
            let base = path.rfind('/').map(|i| i + 1).unwrap_or(0);
            let dir_chars = path[..base].chars().count();
            let mut body = Line::empty();
            push_highlighted(&mut body, &e.text, &e.hl, |i| if i < dir_chars { dim("SeekDir") } else { None });
            let room = cx.width.saturating_sub(line.width());
            line.spans.extend(truncate_line_start(&body, room).spans);
        },
        Target::Match { path, lnum, col } => {
            // The path gets at most two fifths of the row; the line text the rest.
            let place = Line::from_spans([
                Span { text: path.clone(), hl: dim("SeekDir").map(str::to_string) },
                Span { text: format!(":{lnum}:{} ", col + 1), hl: dim("SeekLnum").map(str::to_string) },
            ]);
            line.spans.extend(truncate_line_start(&place, (cx.width * 2 / 5).max(12)).spans);
            push_highlighted(&mut line, &e.text, &e.hl, |_| None);
        },
        Target::Buffer { bufnr, modified } => {
            push_icon(&mut line, &e.text);
            let mut suffix = Line::empty();
            if *modified {
                suffix.spans.push(Span { text: " [+]".into(), hl: dim("SeekModified").map(str::to_string) });
            }
            suffix.spans.push(Span { text: format!("  #{bufnr}"), hl: dim("SeekCount").map(str::to_string) });
            let mut name = Line::empty();
            push_highlighted(&mut name, &e.text, &e.hl, |_| None);
            let room = cx.width.saturating_sub(line.width() + suffix.width());
            line.spans.extend(truncate_line_start(&name, room).spans);
            line.spans.extend(suffix.spans);
        },
        Target::Line { lnum, .. } => {
            line.spans.push(Span { text: format!("{lnum:>5} "), hl: dim("SeekLnum").map(str::to_string) });
            push_highlighted(&mut line, &e.text, &e.hl, |_| None);
        },
    }
    line
}

/// A path with its directory dimmed, as in the results.
fn push_path(line: &mut Line, path: &str) {
    let base = path.rfind('/').map(|i| i + 1).unwrap_or(0);
    if base > 0 {
        line.spans.push(Span::hl(&path[..base], "SeekDir"));
    }
    line.spans.push(Span::raw(&path[base..]));
}

fn push_icon(line: &mut Line, name: &str) {
    if let Some((icon, hl)) = wp_ui::icons::file(name) {
        line.spans.push(Span::hl(format!("{icon} "), &hl));
    }
}

fn placeholder(mode: Mode) -> &'static str {
    match mode {
        Mode::Files => "Search files…",
        Mode::Grep => "Search in files…",
        Mode::Buffers => "Search buffers…",
        Mode::Lines => "Search lines…",
    }
}

impl Session {
    /// The empty prompt's text: the mode, and how to change it — the prefixes
    /// from the default mode, the way back from the others.
    fn placeholder(&self, mode: Mode) -> String {
        let mut text = placeholder(mode).to_string();
        let default = self.cfg.default_mode;
        if mode == default {
            let hints: Vec<String> = self
                .cfg
                .prefixes
                .iter()
                .filter(|(m, _)| *m != mode && self.cfg.modes.contains(m))
                .map(|(m, c)| format!("{c} {}", m.name()))
                .collect();
            if !hints.is_empty() {
                text.push_str(&format!("   {}", hints.join(" · ")));
            }
        } else if self.cfg.backspace_to_default {
            text.push_str(&format!("   ⌫ {}", default.name()));
        }
        text
    }

    /// Backspace in insert mode: on an empty prompt, back to the default mode
    /// (returns `true`: handled); otherwise a plain backspace (`false`).
    fn backspace(self: &Rc<Self>) -> bool {
        let default = self.cfg.default_mode;
        if !self.input.text().is_empty() || self.mode.get_untracked() == default {
            return false;
        }
        // Under the mapping's text lock: switch after it.
        let weak = Rc::downgrade(self);
        let _ = nvim::schedule(move || {
            if let Some(s) = weak.upgrade() {
                s.mode.set(default);
            }
        });
        true
    }
}

fn human_size(bytes: u64) -> String {
    match bytes {
        b if b < 1024 => format!("{b} B"),
        b if b < 1024 * 1024 => format!("{:.1} KiB", b as f64 / 1024.0),
        b => format!("{:.1} MiB", b as f64 / (1024.0 * 1024.0)),
    }
}

/// "3m ago", "2d ago"…
fn ago(t: std::time::SystemTime) -> String {
    let secs = std::time::SystemTime::now().duration_since(t).map(|d| d.as_secs()).unwrap_or(0);
    match secs {
        s if s < 60 => "just now".into(),
        s if s < 3600 => format!("{}m ago", s / 60),
        s if s < 86_400 => format!("{}h ago", s / 3600),
        s if s < 30 * 86_400 => format!("{}d ago", s / 86_400),
        s if s < 365 * 86_400 => format!("{}mo ago", s / (30 * 86_400)),
        s => format!("{}y ago", s / (365 * 86_400)),
    }
}

/// Append `text` with the chars in `hl` highlighted as matches; `base` gives
/// the highlight of every other char (by char index).
fn push_highlighted(line: &mut Line, text: &str, hl: &[usize], base: impl Fn(usize) -> Option<&'static str>) {
    let mut run = String::new();
    let mut run_hl: Option<&str> = None;
    let mut hits = hl.iter().peekable();
    for (i, c) in text.chars().enumerate() {
        let is_hit = hits.peek() == Some(&&i);
        if is_hit {
            hits.next();
        }
        let group = if is_hit { Some(groups::MATCH) } else { base(i) };
        if group != run_hl && !run.is_empty() {
            line.spans.push(Span { text: std::mem::take(&mut run), hl: run_hl.map(str::to_string) });
        }
        run_hl = group;
        run.push(c);
    }
    if !run.is_empty() {
        line.spans.push(Span { text: run, hl: run_hl.map(str::to_string) });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sizes_and_ages() {
        assert_eq!(human_size(512), "512 B");
        assert_eq!(human_size(12_595), "12.3 KiB");
        assert_eq!(human_size(3 * 1024 * 1024), "3.0 MiB");
        let now = std::time::SystemTime::now();
        assert_eq!(ago(now), "just now");
        assert_eq!(ago(now - std::time::Duration::from_secs(3 * 86_400)), "3d ago");
    }

    #[test]
    fn highlighted_runs() {
        let mut line = Line::empty();
        push_highlighted(&mut line, "src/main.rs", &[4, 5], |i| (i < 4).then_some("SeekDir"));
        let spans: Vec<(String, Option<String>)> = line.spans.into_iter().map(|s| (s.text, s.hl)).collect();
        assert_eq!(
            spans,
            vec![
                ("src/".into(), Some("SeekDir".into())),
                ("ma".into(), Some("AbNuiMatch".into())),
                ("in.rs".into(), None),
            ]
        );
    }
}
