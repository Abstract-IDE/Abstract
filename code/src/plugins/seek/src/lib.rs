#![allow(dead_code)] // Silences unused struct fields like `prompt_buf`
                     //
use std::cell::RefCell;

use nvim_oxi::{
    self as oxi, Function, api,
    api::{
        Buffer, Window,
        opts::{OptionOpts, SetKeymapOpts},
        types::{Mode, WindowBorder, WindowConfig, WindowRelativeTo, WindowStyle},
    },
};

use wl_utils::panic::SafeFunctionExt;


// ============================================================================
// Configuration
// ============================================================================

pub struct SeekConfig {
    pub min_width_for_preview: u32,
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
// State Management
// ============================================================================

struct SeekState {
    bg_buf: Buffer,
    bg_win: Window,
    prompt_buf: Buffer,
    results_buf: Buffer,
    preview_buf: Buffer,
    prompt_win: Window,
    results_win: Window,
    preview_win: Option<Window>,
    show_preview: bool,
    maximize_preview: bool,
}

thread_local! {
    static STATE: RefCell<Option<SeekState>> = const { RefCell::new(None) };
}

// ============================================================================
// Helpers
// ============================================================================

fn pad(s: &str, width: usize) -> String {
    let chars_count = s.chars().count();
    if chars_count >= width {
        s.chars().take(width).collect()
    } else {
        format!("{}{}", s, " ".repeat(width - chars_count))
    }
}

/// Safely applies window-local options using the modern Neovim API
fn set_win_opt(win: &Window, name: &str, value: impl oxi::conversion::ToObject) {
    let opts = OptionOpts::builder().win(win.clone()).build();
    let _ = api::set_option_value(name, value, &opts);
}

// ============================================================================
// Core UI Logic & Dynamic Layout Engine
// ============================================================================

pub fn open() -> oxi::Result<()> {
    if STATE.with(|s| s.borrow().is_some()) {
        return Ok(());
    }

    // 1. Initialize Buffers
    let bg_buf = api::create_buf(false, true)?;
    let prompt_buf = api::create_buf(false, true)?;
    let mut results_buf = api::create_buf(false, true)?;
    let mut preview_buf = api::create_buf(false, true)?;

    results_buf.set_lines(
        ..,
        false,
        vec!["FILES", "   src/main.rs", "   src/ui/seek/internal/layout.rs", "   src/ui/theme.rs", "   flake.nix"],
    )?;

    preview_buf.set_lines(
        ..,
        false,
        vec![
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
        ],
    )?;

    // 2. Open dummy invisible windows initially so we can pass them to `update_layout`
    let dummy_config = WindowConfig::builder()
        .relative(WindowRelativeTo::Editor)
        .width(1)
        .height(1)
        .row(-100.0)
        .col(-100.0)
        .build();

    let bg_win = api::open_win(&bg_buf, false, &dummy_config)?;
    let prompt_win = api::open_win(&prompt_buf, true, &dummy_config)?; // Enter prompt
    let results_win = api::open_win(&results_buf, false, &dummy_config)?;

    // 3. Set Base Options
    set_win_opt(&bg_win, "winhl", "Normal:FloatBorder");
    set_win_opt(&results_win, "cursorline", true);

    STATE.with(|s| {
        *s.borrow_mut() = Some(SeekState {
            bg_buf,
            bg_win,
            prompt_buf: prompt_buf.clone(),
            results_buf,
            preview_buf,
            prompt_win,
            results_win,
            preview_win: None,
            show_preview: true,
            maximize_preview: false,
        });
    });

    // 4. Calculate and apply the exact layout geometry
    STATE.with(|s| {
        if let Some(state) = s.borrow_mut().as_mut() {
            let _ = update_layout(state);
        }
    });

    setup_keymaps(&prompt_buf)?;
    api::command("startinsert")?;

    Ok(())
}

fn update_layout(state: &mut SeekState) -> oxi::Result<()> {
    let config = SeekConfig::default();
    let columns: u32 = api::get_option_value("columns", &Default::default())?;
    let lines: u32 = api::get_option_value("lines", &Default::default())?;

    let total_width = ((columns as f64 * config.width_ratio) as usize).max(40);
    let total_height = ((lines as f64 * config.height_ratio) as usize).max(15);
    let row = ((lines as usize - total_height) / 2) as f64;
    let col = ((columns as usize - total_width) / 2) as f64;

    // Dynamically disable preview if screen is too small, regardless of toggle state
    let has_preview = state.show_preview && columns >= config.min_width_for_preview;
    let is_max = state.maximize_preview && has_preview;

    // Calculate internal widths
    let (left_width, right_width) = if is_max {
        (total_width.saturating_sub(2), total_width.saturating_sub(2))
    } else if has_preview {
        let rw = ((total_width.saturating_sub(3)) as f64 * config.preview_ratio) as usize;
        let lw = total_width.saturating_sub(3).saturating_sub(rw);
        (lw, rw)
    } else {
        (total_width.saturating_sub(2), 0)
    };

    let prefix = "   ";
    let prefix_len = prefix.chars().count();
    let prompt_input_width = left_width.saturating_sub(prefix_len).max(1);
    let mut bg_lines = Vec::with_capacity(total_height);

    // 1. Render Background Canvas
    if is_max {
        bg_lines.push(format!("╭{}╮", "─".repeat(left_width)));
        bg_lines.push(format!("│{}{}│", prefix, " ".repeat(prompt_input_width)));
        bg_lines.push(format!("├{}┤", "─".repeat(left_width)));
        for _ in 3..(total_height - 1) {
            bg_lines.push(format!("│{}│", " ".repeat(left_width)));
        }
        bg_lines.push(format!("╰{}╯", "─".repeat(left_width)));
    } else if has_preview {
        bg_lines.push(format!("╭{}┬{}╮", "─".repeat(left_width), "─".repeat(right_width)));
        bg_lines.push(format!("│{}{}│{}│", prefix, " ".repeat(prompt_input_width), pad("   Preview", right_width)));
        bg_lines.push(format!("├{}┼{}┤", "─".repeat(left_width), "─".repeat(right_width)));
        for _ in 3..(total_height - 3) {
            bg_lines.push(format!("│{}│{}│", " ".repeat(left_width), " ".repeat(right_width)));
        }
        bg_lines.push(format!("├{}┼{}┤", "─".repeat(left_width), "─".repeat(right_width)));
        bg_lines.push(format!("│{}│{}│", pad(" Results", left_width), pad(" Info", right_width)));
        bg_lines.push(format!("╰{}┴{}╯", "─".repeat(left_width), "─".repeat(right_width)));
    } else {
        bg_lines.push(format!("╭{}╮", "─".repeat(left_width)));
        bg_lines.push(format!("│{}{}│", prefix, " ".repeat(prompt_input_width)));
        bg_lines.push(format!("├{}┤", "─".repeat(left_width)));
        for _ in 3..(total_height - 3) {
            bg_lines.push(format!("│{}│", " ".repeat(left_width)));
        }
        bg_lines.push(format!("├{}┤", "─".repeat(left_width)));
        bg_lines.push(format!("│{}│", pad(" Results", left_width)));
        bg_lines.push(format!("╰{}╯", "─".repeat(left_width)));
    }
    state.bg_buf.set_lines(.., false, bg_lines)?;

    // 2. Reposition Windows
    state.bg_win.set_config(
        &WindowConfig::builder()
            .relative(WindowRelativeTo::Editor)
            .width(total_width as u32)
            .height(total_height as u32)
            .row(row)
            .col(col)
            .style(WindowStyle::Minimal)
            .border(WindowBorder::None)
            .focusable(false) // Cannot be clicked natively!
            .zindex(40)
            .build(),
    )?;

    state.prompt_win.set_config(
        &WindowConfig::builder()
            .relative(WindowRelativeTo::Editor)
            .width(prompt_input_width as u32)
            .height(1)
            .row(row + 1.0)
            .col(col + 1.0 + prefix_len as f64)
            .style(WindowStyle::Minimal)
            .border(WindowBorder::None)
            .zindex(50)
            .build(),
    )?;

    state.results_win.set_config(&if is_max {
        // Hide results window out of bounds when maximized
        WindowConfig::builder()
            .relative(WindowRelativeTo::Editor)
            .width(1)
            .height(1)
            .row(-100.0)
            .col(-100.0)
            .build()
    } else {
        WindowConfig::builder()
            .relative(WindowRelativeTo::Editor)
            .width(left_width as u32)
            .height((total_height - 6) as u32)
            .row(row + 3.0)
            .col(col + 1.0)
            .style(WindowStyle::Minimal)
            .border(WindowBorder::None)
            .focusable(true)
            .zindex(50)
            .build() // Can be clicked natively!
    })?;

    if has_preview {
        let pw_config = WindowConfig::builder()
            .relative(WindowRelativeTo::Editor)
            .width(right_width as u32)
            .height((if is_max { total_height - 4 } else { total_height - 6 }) as u32)
            .row(row + 3.0)
            .col(if is_max { col + 1.0 } else { col + left_width as f64 + 2.0 })
            .style(WindowStyle::Minimal)
            .border(WindowBorder::None)
            .focusable(true)
            .zindex(50)
            .build(); // Can be clicked natively!

        if let Some(pw) = state.preview_win.as_mut() {
            pw.set_config(&pw_config)?;
        } else {
            let pw = api::open_win(&state.preview_buf, false, &pw_config)?;
            set_win_opt(&pw, "cursorline", true);
            set_win_opt(&pw, "number", true); // Shows line numbers in preview
            state.preview_win = Some(pw);
        }
    } else if let Some(pw) = state.preview_win.take() {
        let _ = pw.close(true);
    }

    Ok(())
}

// ============================================================================
// Keymaps & Actions
// ============================================================================

fn setup_keymaps(prompt_buf: &Buffer) -> oxi::Result<()> {
    let map = |mode: Mode, key: &str, action: fn()| -> oxi::Result<()> {
        let mut buf = prompt_buf.clone();
        buf.set_keymap(
            mode,
            key,
            "",
            &SetKeymapOpts::builder()
                .callback(Function::from_safe_fn
                    (move |()| {
                    action();
                    Ok::<(), oxi::Error>(())
                }))
                .noremap(true)
                .silent(true)
                .nowait(true)
                .build(),
        )?;
        Ok(())
    };

    // Scrolling & Toggles
    map(Mode::Insert, "<Tab>", || scroll_results(true))?;
    map(Mode::Insert, "<S-Tab>", || scroll_results(false))?;
    map(Mode::Insert, "<C-d>", || scroll_preview(true))?;
    map(Mode::Insert, "<C-u>", || scroll_preview(false))?;
    map(Mode::Insert, "<C-p>", toggle_preview)?;
    map(Mode::Insert, "<C-m>", toggle_maximize)?;

    // Opening Files
    map(Mode::Insert, "<CR>", || action_open(OpenMode::Edit))?;
    map(Mode::Insert, "<C-s>", || action_open(OpenMode::Split))?;
    map(Mode::Insert, "<C-v>", || action_open(OpenMode::VSplit))?;

    // Exiting
    map(Mode::Insert, "<C-c>", action_close)?;
    map(Mode::Normal, "<Esc><Esc>", action_close)?;
    map(Mode::Normal, "<CR>", || action_open(OpenMode::Edit))?;

    Ok(())
}

fn action_close() {
    STATE.with(|s| {
        if let Some(state) = s.borrow_mut().take() {
            let _ = state.prompt_win.close(true);
            let _ = state.results_win.close(true);
            let _ = state.bg_win.close(true);
            if let Some(pw) = state.preview_win {
                let _ = pw.close(true);
            }
            let _ = api::command("stopinsert");
        }
    });
}

fn toggle_preview() {
    STATE.with(|s| {
        if let Some(state) = s.borrow_mut().as_mut() {
            state.show_preview = !state.show_preview;
            let _ = update_layout(state);
        }
    });
}

fn toggle_maximize() {
    STATE.with(|s| {
        if let Some(state) = s.borrow_mut().as_mut() {
            state.maximize_preview = !state.maximize_preview;
            let _ = update_layout(state);
        }
    });
}

fn scroll_results(down: bool) {
    STATE.with(|s| {
        if let Some(state) = s.borrow_mut().as_mut()
            && let Ok(mut cursor) = state.results_win.get_cursor()
        {
            let max = state.results_buf.line_count().unwrap_or(1);
            cursor.0 = if down {
                if cursor.0 < max { cursor.0 + 1 } else { 1 }
            } else if cursor.0 > 1 {
                cursor.0 - 1
            } else {
                max
            };
            let _ = state.results_win.set_cursor(cursor.0, cursor.1);
        }
    });
}

fn scroll_preview(down: bool) {
    STATE.with(|s| {
        if let Some(state) = s.borrow_mut().as_mut()
            && let Some(pw) = state.preview_win.as_mut()
            && let Ok(mut cursor) = pw.get_cursor()
        {
            let max = state.preview_buf.line_count().unwrap_or(1);
            cursor.0 = if down { (cursor.0 + 10).min(max) } else { cursor.0.saturating_sub(10).max(1) };
            let _ = pw.set_cursor(cursor.0, cursor.1);
        }
    });
}

enum OpenMode {
    Edit,
    Split,
    VSplit,
}

fn action_open(mode: OpenMode) {
    let mut target = String::new();
    STATE.with(|s| {
        if let Some(state) = s.borrow().as_ref()
            && let Ok(cursor) = state.results_win.get_cursor()
            && let Ok(lines) = state.results_buf.get_lines(cursor.0 - 1..cursor.0, false)
            && let Some(file) = lines.into_iter().next()
        {
            let text = file.to_string_lossy();
            target = text.chars().skip(4).collect::<String>().trim().to_string(); // Strip icon spacing
        }
    });

    action_close();

    if !target.is_empty() {
        let cmd = match mode {
            OpenMode::Edit => format!("edit {}", target),
            OpenMode::Split => format!("split {}", target),
            OpenMode::VSplit => format!("vsplit {}", target),
        };
        // Fails safely if the string happens to be a "FILES" header rather than an actual file
        let _ = api::command(&cmd);
    }
}
