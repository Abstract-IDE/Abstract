//! Seek: a search hub (prompt, results, preview) built on ab-nui (`wp_ui`).
//!
//! The whole UI is one ab-nui [`Surface`]: a [`TextField`] prompt, a [`List`]
//! of results and a scrolling preview, laid out by [`frame::Frame`]. The
//! results and preview are placeholder data for now.
//!
//! Keys (all handled by the prompt, which keeps focus):
//! `<Tab>`/`<S-Tab>` move through results, `<C-d>`/`<C-u>` scroll the
//! preview, `<C-p>` toggles the preview, `<C-m>` maximizes it, `<CR>` /
//! `<C-s>` / `<C-v>` open the selected file (edit / split / vsplit),
//! `<Esc>` or `<C-c>` close.

mod frame;

use std::cell::RefCell;
use std::mem::ManuallyDrop;
use std::rc::Rc;

use wp_ui::geometry::editor_size;
use wp_ui::nvim::{self, CommandOpts};
use wp_ui::prelude::*;
use wp_ui::theme::groups;

use crate::frame::Frame;

// ============================================================================
// Configuration
// ============================================================================

pub struct SeekConfig {
    /// Below this many editor columns the preview is hidden.
    pub min_width_for_preview: u32,
    /// Width share of the preview column.
    pub preview_ratio: f64,
    pub width_ratio: f64,
    pub height_ratio: f64,
}

impl Default for SeekConfig {
    fn default() -> Self {
        Self { min_width_for_preview: 100, preview_ratio: 0.6, width_ratio: 0.8, height_ratio: 0.8 }
    }
}

// ============================================================================
// Placeholder data
// ============================================================================

const FILES: &[&str] = &["src/main.rs", "src/ui/seek/internal/layout.rs", "src/ui/theme.rs", "flake.nix"];

const PREVIEW: &[&str] = &[
    "pub fn build_ui(app: &mut App) -> Result<()> {",
    "    let block = Block::default()",
    "        .borders(Borders::ALL)",
    "        .title(\"Search Hub\");",
    "",
    "    let area = f.size();",
    "    f.render_widget(block, area);",
    "",
    "    Ok(())",
    "}",
];

/// How far `<C-d>`/`<C-u>` scroll the preview.
const PREVIEW_STEP: i32 = 10;

const ICON_SEARCH: &str = "\u{f002}";
const ICON_FILE: &str = "\u{f15b}";
const ICON_FOLDER_OPEN: &str = "\u{f07c}";

// ============================================================================
// State
// ============================================================================

thread_local! {
    /// The open seek window. Dropping the surface frees it.
    ///
    /// `ManuallyDrop`: a seek window still open when Neovim exits must not be
    /// freed from a thread-exit destructor — that would call into Lua after
    /// Neovim has torn it down (and crash on quit).
    static SEEK: ManuallyDrop<RefCell<Option<Surface>>> = const { ManuallyDrop::new(RefCell::new(None)) };
}

/// The state one seek window shares across rebuilds.
#[derive(Clone)]
struct Ui {
    config: Rc<SeekConfig>,
    query: FieldState,
    results: ListState,
    preview_offset: Signal<u16>,
    show_preview: Signal<bool>,
    maximize: Signal<bool>,
}

#[derive(Clone, Copy)]
enum OpenMode {
    Edit,
    Split,
    VSplit,
}

impl OpenMode {
    fn command(self) -> &'static str {
        match self {
            OpenMode::Edit => "edit",
            OpenMode::Split => "split",
            OpenMode::VSplit => "vsplit",
        }
    }
}

impl Ui {
    fn new(config: SeekConfig) -> Self {
        Self {
            config: Rc::new(config),
            query: FieldState::new(""),
            results: ListState::new(),
            preview_offset: Signal::new(0),
            show_preview: Signal::new(true),
            maximize: Signal::new(false),
        }
    }

    /// The whole window. Rebuilt whenever a signal it reads changes, and on
    /// editor resize (which can show or hide the preview).
    fn build(&self) -> Frame {
        let (columns, _) = editor_size().unwrap_or((0, 0));
        let has_preview = self.show_preview.get() && columns >= self.config.min_width_for_preview;
        let maximized = has_preview && self.maximize.get();

        let columns = if maximized {
            vec![frame::Column { header: self.prompt(), body: self.preview(), footer: None }]
        } else if has_preview {
            vec![
                frame::Column { header: self.prompt(), body: self.results(), footer: Some(label(" Results")) },
                frame::Column {
                    header: label(&format!(" {ICON_FOLDER_OPEN}  Preview")),
                    body: self.preview(),
                    footer: Some(label(" Info")),
                },
            ]
        } else {
            vec![frame::Column { header: self.prompt(), body: self.results(), footer: Some(label(" Results")) }]
        };
        Frame::new(columns, self.config.preview_ratio)
    }

    /// The search prompt. It keeps focus, so every seek key is bound here.
    fn prompt(&self) -> Element {
        let field = Keyed::new(TextField::new(&self.query))
            .focus_key("prompt")
            .on_key(Key::Tab, self.act(|ui| ui.step_results(1)))
            .on_key(Key::BackTab, self.act(|ui| ui.step_results(-1)))
            .on_key(Key::Ctrl('d'), self.act(|ui| ui.scroll_preview(PREVIEW_STEP)))
            .on_key(Key::Ctrl('u'), self.act(|ui| ui.scroll_preview(-PREVIEW_STEP)))
            .on_key(Key::Ctrl('p'), self.act(|ui| ui.show_preview.update(|v| *v = !*v)))
            .on_key(Key::Ctrl('m'), self.act(|ui| ui.maximize.update(|v| *v = !*v)))
            .on_key(Key::Enter, self.act(|ui| ui.open_selected(OpenMode::Edit)))
            .on_key(Key::Ctrl('s'), self.act(|ui| ui.open_selected(OpenMode::Split)))
            .on_key(Key::Ctrl('v'), self.act(|ui| ui.open_selected(OpenMode::VSplit)))
            .on_key(Key::Esc, close_later);
        Box::new(row![Text::new(format!(" {ICON_SEARCH}  ")).fg(groups::ACCENT), Expanded::new(field)])
    }

    fn results(&self) -> Element {
        let list = List::with_render(FILES.to_vec(), &self.results, |path, _| {
            Line::from_spans([Span::hl(format!("{ICON_FILE} "), groups::ACCENT), Span::raw(*path)])
        });
        Box::new(col![Text::new("FILES").fg(groups::HEADER), Expanded::new(list)])
    }

    fn preview(&self) -> Element {
        let width = PREVIEW.len().to_string().len().max(3);
        let rows: Vec<Element> = PREVIEW
            .iter()
            .enumerate()
            .map(|(i, code)| {
                let line = Line::from_spans([Span::hl(format!("{:>width$} ", i + 1), "LineNr"), Span::raw(*code)]);
                Box::new(Text::from_line(line)) as Element
            })
            .collect();
        Box::new(ScrollView::new(self.preview_offset.clone(), Column::new(rows)).scrollbar(false))
    }

    /// A key handler running `f` on this window's state.
    fn act(&self, f: impl Fn(&Ui) + 'static) -> impl Fn() + 'static {
        let ui = self.clone();
        move || f(&ui)
    }

    /// Move the result selection, wrapping at both ends.
    fn step_results(&self, step: isize) {
        let n = FILES.len() as isize;
        self.results.selected.update(|sel| *sel = (*sel as isize + step).rem_euclid(n) as usize);
    }

    fn scroll_preview(&self, by: i32) {
        let max = PREVIEW.len().saturating_sub(1) as i32;
        self.preview_offset.update(|o| *o = (*o as i32 + by).clamp(0, max) as u16);
    }

    /// Close seek and open the selected file. Deferred to the next tick so the
    /// window is not freed from inside its own key handler.
    fn open_selected(&self, mode: OpenMode) {
        let path = FILES.get(self.results.selected.get_untracked()).copied();
        let _ = nvim::schedule(move || {
            close();
            if let Some(path) = path {
                report(open_file(path, mode));
            }
        });
    }
}

/// A one-line frame label (header/footer).
fn label(text: &str) -> Element {
    Box::new(Text::new(text).fg(groups::TITLE).ellipsis())
}

fn open_file(path: &str, mode: OpenMode) -> Result<()> {
    let escaped: String = wp_ui::lua::call_fn("fnameescape", (path,))?;
    wp_ui::lua::cmd(&format!("{} {escaped}", mode.command()))
}

/// Show an error where there is no caller to return it to.
fn report(res: Result<()>) {
    if let Err(e) = res {
        let opts = NotifyOptions { title: Some("Seek".into()), level: Level::Error, ..Default::default() };
        if notify(e.to_string(), opts).is_err() {
            eprintln!("Seek: {e}");
        }
    }
}

// ============================================================================
// Public API
// ============================================================================

/// Register `:AbstractSeek`. ab-nui must already be initialised (`wp_ui::init`).
pub fn setup() {
    report(nvim::user_command(
        "AbstractSeek",
        &CommandOpts { nargs: Some("0"), desc: Some("Open the Seek search hub".into()), ..Default::default() },
        |_| report(open()),
    ));
}

/// Open seek, or focus it if it is already open.
pub fn open() -> Result<()> {
    let existing = SEEK.with(|s| s.borrow().clone());
    if let Some(surface) = existing {
        if let Some(win) = surface.popup().window().filter(|_| surface.is_visible()) {
            return win.focus();
        }
        // Its window was closed some other way (`:q`); start over.
        close();
    }

    let config = SeekConfig::default();
    let size = wp_ui::geometry::Size::ratio(config.width_ratio, config.height_ratio);
    let ui = Ui::new(config);
    let surface = Surface::float(PopupOptions {
        size,
        position: Position::Center,
        // The frame draws its own border.
        border: Border::None,
        ..Default::default()
    })?
    .initial_focus(0);
    // `<C-c>` is left out of the surface's key table (it stays Neovim's interrupt).
    surface.on_key("<C-c>", close_later)?;
    let surface = surface.show(move || ui.build())?;

    SEEK.with(|s| *s.borrow_mut() = Some(surface));
    Ok(())
}

/// Close seek (frees the window and its state).
pub fn close() {
    if let Some(surface) = SEEK.with(|s| s.borrow_mut().take()) {
        surface.close();
    }
}

/// [`close`] on the next tick — for key handlers of the seek window itself.
fn close_later() {
    let _ = nvim::schedule(close);
}
