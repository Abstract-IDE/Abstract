//! `CodeView`: text shown in a real buffer embedded in a surface, so it gets
//! Neovim's own syntax highlighting (treesitter when a parser exists, regex
//! syntax otherwise), line numbers, and touchpad/wheel scrolling — e.g. a file
//! preview with one line and its matches highlighted.
//!
//! It can also show media — images, video frames, PDF pages — in terminals
//! with the kitty graphics protocol and its unicode placeholders (kitty,
//! Ghostty), with no plugin:
//! [`CodeView::show_media`] (see [`crate::media`]).
//!
//! ```ignore
//! let code = CodeView::new()?;
//! code.set_content(&lines, "rust")?;
//! code.highlight(Some(42), &[(4, 9)])?; // line 42, bytes 4..9 of it
//! // in the tree: code.widget()
//! ```

use std::cell::{Cell, RefCell};
use std::rc::{Rc, Weak};
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::mpsc::channel;

use crate::error::Result;
use crate::lua;
use crate::media::{self, Picture};
use crate::nvim::{Buffer, ExtmarkOpts, Namespace, Timer};
use crate::theme::groups;
use crate::view::embed::{Embed, EmbedOptions, EmbeddedWindow};

struct Inner {
    win: EmbeddedWindow,
    ns: Namespace,
    /// The highlighted line (re-shown when the window reopens).
    line: Cell<Option<usize>>,
    numbers: Cell<bool>,
    /// The media shown instead of text, if any.
    media: RefCell<Option<Media>>,
    /// The image id last asked for: decoders of older ones give up.
    wanted: Arc<AtomicU32>,
}

impl Drop for Inner {
    fn drop(&mut self) {
        // Dropped without `close()`: the terminal still holds the picture.
        if let Some(m) = self.media.get_mut().take()
            && m.sent
        {
            media::delete(m.id);
        }
    }
}

struct Media {
    id: u32,
    /// Decoded (on a background thread); `None` while loading.
    picture: Option<Rc<Picture>>,
    /// Sent to the terminal.
    sent: bool,
    /// Polls for the decoded picture.
    _poll: Option<Timer>,
}

/// A read-only, syntax-highlighted view of some text. Cloning yields another
/// handle to the same view.
#[derive(Clone)]
pub struct CodeView(Rc<Inner>);

impl CodeView {
    pub fn new() -> Result<Self> {
        let win = EmbeddedWindow::new(EmbedOptions { focusable: false, enter: false, zindex: 60 })?;
        win.buffer().set_option("modifiable", false)?;
        let this = Self(Rc::new(Inner {
            win,
            ns: Namespace::create("ab_nui_code_view")?,
            line: Cell::new(None),
            numbers: Cell::new(true),
            media: RefCell::new(None),
            wanted: Arc::new(AtomicU32::new(0)),
        }));
        let weak: Weak<Inner> = Rc::downgrade(&this.0);
        this.0.win.on_open(move |w| {
            let Some(inner) = weak.upgrade() else { return };
            let _ = w.set_option("number", inner.numbers.get());
            for name in ["relativenumber", "wrap", "list", "foldenable", "cursorline"] {
                let _ = w.set_option(name, false);
            }
            let _ = w.set_option("signcolumn", "no");
            let _ = call("view_line", (w.id(), inner.line.get()));
            CodeView(inner).render_media();
        });
        let weak: Weak<Inner> = Rc::downgrade(&this.0);
        this.0.win.on_resize(move |_| {
            if let Some(inner) = weak.upgrade() {
                CodeView(inner).render_media();
            }
        });
        Ok(this)
    }

    /// Show line numbers (default `true`).
    pub fn line_numbers(self, on: bool) -> Self {
        self.set_line_numbers(on);
        self
    }

    pub fn set_line_numbers(&self, on: bool) {
        self.0.numbers.set(on);
        // Media keeps the numbers off (they would push the picture aside);
        // they come back with the text.
        if !self.is_media()
            && let Some(win) = self.0.win.window()
        {
            let _ = win.set_option("number", on);
        }
    }

    /// Whether [`show_media`](Self::show_media) can show `path` (a media
    /// format, a kitty-graphics terminal).
    pub fn supports_media(path: &str) -> bool {
        media::is_media(path) && media::terminal_supported()
    }

    /// Show an image (or a video frame, a PDF page) instead of text, scaled to
    /// fit. Decoding happens in the background ("Loading…" meanwhile).
    /// Returns `false` when it can't (see [`supports_media`](Self::supports_media)).
    pub fn show_media(&self, path: &str) -> Result<bool> {
        if !Self::supports_media(path) {
            return Ok(false);
        }
        self.clear_media();
        self.write_text(&["  Loading…"], "")?;

        let id = media::next_id();
        let (tx, rx) = channel();
        let file = path.to_string();
        self.0.wanted.store(id, Ordering::Relaxed);
        let wanted = self.0.wanted.clone();
        std::thread::spawn(move || {
            // Moving quickly through a list asks for many pictures: decode
            // (maybe run ffmpeg for) only the one still wanted.
            std::thread::sleep(std::time::Duration::from_millis(60));
            if wanted.load(Ordering::Relaxed) == id {
                let _ = tx.send(media::decode(&file));
            }
        });
        let weak = Rc::downgrade(&self.0);
        let poll = Timer::interval(25, move || {
            let Some(inner) = weak.upgrade() else { return };
            if let Ok(result) = rx.try_recv() {
                CodeView(inner).decoded(id, result);
            }
        })?;
        *self.0.media.borrow_mut() = Some(Media { id, picture: None, sent: false, _poll: Some(poll) });
        Ok(true)
    }

    pub fn is_media(&self) -> bool {
        self.0.media.borrow().is_some()
    }

    fn decoded(&self, id: u32, result: std::result::Result<Picture, String>) {
        {
            let mut media = self.0.media.borrow_mut();
            let Some(m) = media.as_mut().filter(|m| m.id == id) else { return };
            m._poll = None;
            match result {
                Ok(picture) => m.picture = Some(Rc::new(picture)),
                Err(e) => {
                    drop(media);
                    let _ = self.write_text(&[format!("  Can't show this file: {e}")], "");
                    return;
                },
            }
        }
        self.render_media();
    }

    /// Lay the picture out for the window's current size (sending it to the
    /// terminal the first time).
    fn render_media(&self) {
        let (id, picture, sent) = match self.0.media.borrow().as_ref() {
            Some(Media { id, picture: Some(p), sent, .. }) => (*id, p.clone(), *sent),
            _ => return,
        };
        let (Some(win), Some((_, _, w, h))) = (self.0.win.window(), self.0.win.placement()) else { return };
        if !sent {
            if media::transmit(id, &picture).is_err() {
                return;
            }
            if let Some(m) = self.0.media.borrow_mut().as_mut() {
                m.sent = true;
            }
        }
        let _ = win.set_option("number", false);
        let (cols, rows) = media::fit(picture.width, picture.height, w, h, media::cell_size());
        let _ = media::place(id, cols, rows);
        let pad = " ".repeat(((w - cols) / 2) as usize);
        let lines: Vec<String> =
            media::placeholder_lines(cols, rows).into_iter().map(|l| format!("{pad}{l}")).collect();
        let _ = self.write_text(&lines, "");
        if let Ok(group) = media::highlight(id) {
            let buf = self.buffer();
            for (row, line) in lines.iter().enumerate() {
                let opts = ExtmarkOpts {
                    end_col: Some(line.len() as i64),
                    hl_group: Some(group.clone()),
                    ..Default::default()
                };
                let _ = self.0.ns.set_extmark(buf, row as i64, pad.len() as i64, &opts);
            }
        }
    }

    /// Stop showing media (the terminal forgets the picture).
    fn clear_media(&self) {
        self.0.wanted.store(0, Ordering::Relaxed);
        if let Some(m) = self.0.media.borrow_mut().take()
            && m.sent
        {
            media::delete(m.id);
        }
        if let Some(win) = self.0.win.window() {
            let _ = win.set_option("number", self.0.numbers.get());
        }
    }

    /// Text into the buffer, leaving any media state alone.
    fn write_text<S: AsRef<str>>(&self, lines: &[S], filetype: &str) -> Result<()> {
        let buf = self.buffer();
        buf.set_all(lines.iter().map(|l| l.as_ref().replace(['\n', '\r'], " ")))?;
        self.0.ns.clear(buf)?;
        self.0.line.set(None);
        call("set_syntax", (buf.id(), filetype))
    }

    /// Replace the text and highlight it as `filetype` (`""`: no highlighting).
    pub fn set_content<S: AsRef<str>>(&self, lines: &[S], filetype: &str) -> Result<()> {
        self.clear_media();
        self.write_text(lines, filetype)
    }

    /// Highlight `line` (1-based; `None` for none) and byte `ranges` on it,
    /// and bring it into view (centred).
    pub fn highlight(&self, line: Option<usize>, ranges: &[(usize, usize)]) -> Result<()> {
        if self.is_media() {
            return Ok(());
        }
        let buf = self.buffer();
        self.0.ns.clear(buf)?;
        let count = buf.line_count()? as usize;
        let line = line.filter(|l| (1..=count).contains(l));
        if let Some(l) = line {
            let row = l as i64 - 1;
            let opts =
                ExtmarkOpts { line_hl_group: Some(groups::CODE_LINE.into()), priority: Some(10), ..Default::default() };
            self.0.ns.set_extmark(buf, row, 0, &opts)?;
            for &(start, end) in ranges {
                let opts = ExtmarkOpts {
                    end_col: Some(end as i64),
                    hl_group: Some(groups::CODE_MATCH.into()),
                    priority: Some(200),
                    ..Default::default()
                };
                let _ = self.0.ns.set_extmark(buf, row, start as i64, &opts);
            }
        }
        self.0.line.set(line);
        if let Some(win) = self.0.win.window() {
            call("view_line", (win.id(), line))?;
        }
        Ok(())
    }

    /// Scroll by `lines` (negative: up).
    pub fn scroll_by(&self, lines: i64) -> Result<()> {
        match self.0.win.window() {
            Some(win) => call("scroll_by", (win.id(), lines)),
            None => Ok(()),
        }
    }

    /// Rows visible (0 while hidden).
    pub fn height(&self) -> u16 {
        self.0.win.placement().map(|p| p.3).unwrap_or(0)
    }

    pub fn line_count(&self) -> usize {
        self.buffer().line_count().unwrap_or(0).max(0) as usize
    }

    /// The widget that reserves the view's place in a surface's tree.
    pub fn widget(&self) -> Embed {
        self.0.win.widget()
    }

    pub fn embedded(&self) -> &EmbeddedWindow {
        &self.0.win
    }

    pub fn buffer(&self) -> Buffer {
        self.0.win.buffer()
    }

    pub fn close(&self) {
        self.clear_media();
        self.0.win.close();
    }
}

fn call(name: &'static str, args: impl mlua::IntoLuaMulti) -> Result<()> {
    lua::driver_fn(name)?.call::<()>(args)?;
    Ok(())
}
