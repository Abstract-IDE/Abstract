//! Frame, embedded windows (BufferInput, CodeView), modal surfaces, icons.

use wp_ui::prelude::*;

use crate::support::*;

fn rows(buf: Buffer) -> Vec<String> {
    lines(buf)
}

// ---- Frame --------------------------------------------------------------------

#[nvim_oxi::test]
fn frame_divider_runs_through_the_body_only() {
    setup();
    let out = render(21, 8, || {
        Frame::new(vec![
            FrameColumn::new(Text::new("H1"), Text::new("b1")).footer(Text::new("F1")),
            FrameColumn::new(Text::new("H2"), Text::new("b2")).footer(Text::new("F2")),
        ])
    });
    // Top border and header row: no divider; it starts at the header rule.
    assert!(!out[0].contains('┬'), "{out:?}");
    assert!(!out[1].trim_end_matches('│').contains('│') || out[1].matches('│').count() == 2, "{out:?}");
    assert!(out[2].contains('┬'), "{out:?}");
    assert_eq!(out[3].matches('│').count(), 3, "body row has the divider: {out:?}");
    assert!(out[5].contains('┴'), "{out:?}");
    assert_eq!(out[6].matches('│').count(), 2, "footer row has no divider: {out:?}");
    assert!(out[1].contains("H1") && out[1].contains("H2") && out[6].contains("F1") && out[6].contains("F2"));
}

#[nvim_oxi::test]
fn frame_draws_the_scrollbar_thumb_on_the_divider() {
    setup();
    let offset = Signal::new(0usize);
    let o = offset.clone();
    let (_s, buf) = surf(21, 10, move || {
        Frame::new(vec![
            FrameColumn::new(Text::new("H"), Text::new("list")),
            FrameColumn::new(Text::new("P"), Text::new("preview")),
        ])
        .scrollbar(100, o.clone())
    });
    let thumb_row = |b: Buffer| rows(b).iter().position(|l| l.contains('┃'));
    assert_eq!(thumb_row(buf), Some(3), "thumb at the top of the body");
    offset.set(95);
    assert_eq!(thumb_row(buf), Some(8), "thumb at the bottom of the body");
    // No extra column: the thumb replaces a divider cell.
    let r = &rows(buf)[8];
    assert_eq!(r.matches('│').count() + r.matches('┃').count(), 3, "{r:?}");
}

// ---- Embedded windows -------------------------------------------------------------

#[nvim_oxi::test]
fn an_embedded_window_follows_its_area_and_the_surface() {
    setup();
    let embedded = EmbeddedWindow::new(EmbedOptions::default()).unwrap();
    let show = Signal::new(true);
    let (e, sh) = (embedded.clone(), show.clone());
    let s = Surface::float(PopupOptions {
        size: Size::cells(20, 6),
        position: Position::At { row: 2.0, col: 3.0 },
        border: Border::None,
        ..Default::default()
    })
    .unwrap()
    .show(move || {
        let body: Element = if sh.get() { Box::new(e.widget()) } else { Box::new(Text::new("none")) };
        col![Text::new("title"), Expanded::new(Padding::left(2, body))]
    })
    .unwrap();

    // Laid over its area: row 1 (after the title), col 2 (after the padding).
    let win = embedded.window().expect("shown with the surface");
    let cfg = win.config().unwrap();
    let origin = s.popup().content_origin().unwrap();
    assert_eq!((cfg.row as i64, cfg.col as i64), (origin.0 + 1, origin.1 + 2));
    assert_eq!((cfg.width, cfg.height), (18, 5));

    // Not painted → hidden; painted again → back.
    show.set(false);
    assert!(embedded.window().is_none());
    show.set(true);
    assert!(embedded.window().is_some());

    // Follows the surface.
    s.hide();
    assert!(embedded.window().is_none());
    s.reopen().unwrap();
    assert!(embedded.window().is_some());
    let buf = embedded.buffer();
    s.close();
    assert!(embedded.window().is_none());
    assert!(!buf.is_valid(), "closed with the surface");
}

// ---- BufferInput --------------------------------------------------------------------

#[nvim_oxi::test]
fn buffer_input_value_placeholder_and_one_line() {
    setup();
    let input = BufferInput::new().unwrap().placeholder("Search…").start_insert(false);
    let i = input.clone();
    let _s = Surface::float(PopupOptions { size: Size::cells(20, 1), border: Border::None, ..Default::default() })
        .unwrap()
        .show(move || i.widget())
        .unwrap();
    let win = input.window().expect("shown");
    assert_eq!(current_win(), win, "the input has the cursor");
    let buf = input.buffer();
    let placeholder =
        || -> usize { lua_eval(&format!("return #vim.api.nvim_buf_get_extmarks({}, -1, 0, -1, {{}})", buf.id())) };
    assert_eq!(placeholder(), 1, "placeholder while empty");

    // An edit (as Neovim reports it) updates the value and hides the placeholder.
    let value = input.value();
    buf.set_all(["hello"]).unwrap();
    lua_run(&format!("vim.api.nvim_exec_autocmds('TextChanged', {{ buffer = {} }})", buf.id()));
    assert_eq!(value.get_untracked(), "hello");
    assert_eq!(placeholder(), 0);

    // set_text: one line, cursor at the end.
    input.set_text("a\nb").unwrap();
    assert_eq!(input.text(), "a b");
    assert_eq!(lines(buf), v(&["a b"]));
    // Normal mode: the cursor sits on the last character.
    assert_eq!(win.cursor().unwrap(), (1, 2));

    // A multi-line paste is joined (on the next tick).
    buf.set_all(["x", "y"]).unwrap();
    lua_run(&format!("vim.api.nvim_exec_autocmds('TextChanged', {{ buffer = {} }})", buf.id()));
    assert_eq!(value.get_untracked(), "x y");
    assert!(wait(500, move || lines(buf).len() == 1));
}

#[nvim_oxi::test]
fn buffer_input_keys_run_in_the_given_modes() {
    setup();
    let input = BufferInput::new().unwrap().start_insert(false);
    let hits = Signal::new(0);
    let h = hits.clone();
    input.on_key(&["n", "i"], "<C-x>", move || h.update(|n| *n += 1)).unwrap();
    let i = input.clone();
    let _s = Surface::float(PopupOptions { size: Size::cells(20, 1), border: Border::None, ..Default::default() })
        .unwrap()
        .show(move || i.widget())
        .unwrap();
    feed("<C-x>");
    feed("i<C-x><Esc>");
    assert_eq!(hits.get_untracked(), 2);
}

// ---- CodeView -------------------------------------------------------------------------

#[nvim_oxi::test]
fn code_view_shows_highlights_and_scrolls_to_a_line() {
    setup();
    let code = CodeView::new().unwrap();
    let c = code.clone();
    let _s = Surface::float(PopupOptions { size: Size::cells(30, 5), border: Border::None, ..Default::default() })
        .unwrap()
        .show(move || c.widget())
        .unwrap();
    let text: Vec<String> = (1..=50).map(|i| format!("line {i} foo")).collect();
    code.set_content(&text, "lua").unwrap();
    code.highlight(Some(40), &[(8, 11)]).unwrap();
    let buf = code.buffer();
    assert_eq!(code.line_count(), 50);
    let marks = highlights(buf);
    assert!(marks.iter().any(|m| m.0 == 39 && m.1 == 8 && m.2 == 11 && m.3 == "AbNuiCodeMatch"), "{marks:?}");
    let line_hl: bool = lua_eval(&format!(
        "for _, m in ipairs(vim.api.nvim_buf_get_extmarks({}, -1, 0, -1, {{ details = true }})) do if m[4].line_hl_group == 'AbNuiCodeLine' and m[2] == 39 then return true end end return false",
        buf.id()
    ));
    assert!(line_hl);
    // Line 40 is in view.
    let win = code.embedded().window().unwrap();
    let (top, bot): (i64, i64) = {
        let v: Vec<i64> =
            lua_eval(&format!("local i = vim.fn.getwininfo({})[1]; return {{ i.topline, i.botline }}", win.id()));
        (v[0], v[1])
    };
    assert!(top <= 40 && 40 <= bot, "line 40 in view: {top}..{bot}");
    // Read-only and not focusable.
    assert!(!buf.get_option::<bool>("modifiable").unwrap());
    assert!(
        !win.config()
            .map(|_| lua_eval::<bool>(&format!("return vim.api.nvim_win_get_config({}).focusable", win.id())))
            .unwrap()
    );
}

// ---- Modal ------------------------------------------------------------------------------

#[nvim_oxi::test]
fn a_modal_surface_blocks_the_background_and_keeps_the_cursor() {
    setup();
    let background = current_win();
    let input = BufferInput::new().unwrap().start_insert(false);
    let i = input.clone();
    let s = Surface::float(PopupOptions {
        size: Size::cells(20, 3),
        border: Border::None,
        focusable: false,
        enter: false,
        ..Default::default()
    })
    .unwrap()
    .modal(true)
    .show(move || col![Text::new("title"), i.widget()])
    .unwrap();

    // A backdrop below the surface catches the mouse.
    let backdrop: Vec<i64> = lua_eval(
        "local out = {} for _, w in ipairs(vim.api.nvim_list_wins()) do local c = vim.api.nvim_win_get_config(w) \
         if c.relative ~= '' and c.zindex == 49 then out[#out+1] = w; out[#out+1] = c.mouse and 1 or 0; out[#out+1] = c.focusable and 1 or 0 end end return out",
    );
    assert_eq!(backdrop.len(), 3, "one backdrop: {backdrop:?}");
    assert_eq!((backdrop[1], backdrop[2]), (1, 0), "mouse on, not focusable");

    // The cursor is in the input; going to a window behind puts it back.
    let target = input.window().unwrap();
    assert_eq!(current_win(), target);
    background.focus().unwrap();
    assert!(wait(500, move || current_win() == target), "focus returned to the modal");

    // Hidden: no backdrop, no trap.
    s.hide();
    let n: i64 = lua_eval(
        "local n = 0 for _, w in ipairs(vim.api.nvim_list_wins()) do if vim.api.nvim_win_get_config(w).zindex == 49 then n = n + 1 end end return n",
    );
    assert_eq!(n, 0);
    background.focus().unwrap();
    assert!(!wait(150, move || current_win() == target));
}

#[nvim_oxi::test]
fn icons_are_none_without_an_icon_plugin() {
    setup();
    assert_eq!(wp_ui::icons::file("main.rs"), None);
}

#[nvim_oxi::test]
fn on_close_reports_only_closes_from_elsewhere() {
    setup();
    let embedded = EmbeddedWindow::new(EmbedOptions { focusable: true, enter: true, zindex: 60 }).unwrap();
    let closed = Signal::new(0);
    let c = closed.clone();
    embedded.on_close(move || c.update(|n| *n += 1));
    let e = embedded.clone();
    let s = Surface::float(PopupOptions { size: Size::cells(10, 2), border: Border::None, ..Default::default() })
        .unwrap()
        .show(move || e.widget())
        .unwrap();
    // The surface hiding/showing it: not reported.
    s.hide();
    s.reopen().unwrap();
    assert_eq!(closed.get_untracked(), 0);
    // `:close` in it: reported.
    let id = embedded.window().unwrap().id();
    lua_run(&format!("vim.api.nvim_win_close({id}, true)"));
    assert_eq!(closed.get_untracked(), 1);
}

#[nvim_oxi::test]
fn the_wheel_scrolls_the_widget_under_it() {
    setup();
    let list = ListState::new();
    let offset = Signal::new(0u16);
    let (l, o) = (list.clone(), offset.clone());
    let items: Vec<String> = (0..50).map(|i| format!("item {i}")).collect();
    let (s, _buf) = surf(40, 5, move || {
        let lines: Vec<Element> = (0..30).map(|i| Box::new(Text::new(format!("row {i}"))) as Element).collect();
        row![Expanded::new(List::new(items.clone(), &l)), Expanded::new(ScrollView::new(o.clone(), Column::new(lines))),]
    });
    // Over the list (left half): moves the selection, clamped.
    assert!(s.scroll_at(1, 2, 3));
    assert_eq!(list.selected.get_untracked(), 3);
    s.scroll_at(1, 2, -10);
    assert_eq!(list.selected.get_untracked(), 0);
    // Over the scroll view (right half): moves its offset.
    assert!(s.scroll_at(1, 30, 4));
    assert_eq!(offset.get_untracked(), 4);
    assert_eq!(list.selected.get_untracked(), 0, "the list didn't move");
    // Nothing scrollable there.
    let (s2, _) = surf(10, 2, || Text::new("static"));
    assert!(!s2.scroll_at(0, 0, 3));
}

#[nvim_oxi::test]
fn code_view_line_numbers_are_optional() {
    setup();
    let code = CodeView::new().unwrap().line_numbers(false);
    let c = code.clone();
    let _s = Surface::float(PopupOptions { size: Size::cells(20, 3), border: Border::None, ..Default::default() })
        .unwrap()
        .show(move || c.widget())
        .unwrap();
    let win = code.embedded().window().unwrap();
    assert!(!win.get_option::<bool>("number").unwrap());
    code.set_line_numbers(true);
    assert!(win.get_option::<bool>("number").unwrap());
}

#[nvim_oxi::test]
fn a_styled_span_keeps_its_colour_on_the_selected_row() {
    setup();
    lua_run(
        "vim.api.nvim_set_hl(0, 'TestMatch', { fg = '#ff0000', bold = true }); vim.api.nvim_set_hl(0, 'AbNuiSelection', { fg = '#000000', bg = '#00ffff' })",
    );
    let state = ListState::new();
    let (_s, buf) = surf(20, 2, move || {
        List::with_render(vec!["one", "two"], &state, |item, _| {
            Line::from_spans([Span::hl("x", "TestMatch"), Span::raw(*item)])
        })
        .scrollbar(false)
    });
    // Selected row: the styled "x" is red on the selection's cyan, bold.
    let group = hl_at(buf, 0, 4).expect("highlighted");
    assert_ne!(group, "AbNuiSelection", "kept its own colour");
    let hl: Vec<i64> = lua_eval(&format!(
        "local h = vim.api.nvim_get_hl(0, {{ name = '{group}', link = false }}) return {{ h.fg or -1, h.bg or -1, h.bold and 1 or 0 }}"
    ));
    assert_eq!(hl, vec![0xff0000, 0x00ffff, 1]);
    // Plain text on the selected row takes the selection's own colours.
    assert_eq!(hl_at(buf, 0, 6).as_deref(), Some("AbNuiSelection"));
}

#[nvim_oxi::test]
fn the_cursor_never_stays_in_a_non_focusable_part() {
    setup();
    let input = BufferInput::new().unwrap().start_insert(false);
    let code = CodeView::new().unwrap();
    let (i, c) = (input.clone(), code.clone());
    let s = Surface::float(PopupOptions {
        size: Size::cells(30, 6),
        border: Border::None,
        focusable: false,
        enter: false,
        ..Default::default()
    })
    .unwrap()
    .show(move || col![i.widget(), Expanded::new(c.widget())])
    .unwrap();
    let target = input.window().unwrap();
    // What a mouse click does: enter the frame, or the code view.
    for win in [s.popup().window().unwrap(), code.embedded().window().unwrap()] {
        lua_run(&format!("vim.api.nvim_set_current_win({})", win.id()));
        assert!(wait(500, move || current_win() == target), "back in the input from {win:?}");
    }
}

#[nvim_oxi::test]
fn a_list_without_gutter_starts_at_the_edge() {
    setup();
    let state = ListState::new();
    let (_s, buf) = surf(12, 2, move || List::new(vec!["one", "two"], &state).scrollbar(false).gutter(false));
    assert_eq!(trimmed(buf), v(&["one", "two"]));
    // The selection is still shown: a full-width bar.
    assert_eq!(hl_at(buf, 0, 0).as_deref(), Some("AbNuiSelection"));
    assert_eq!(hl_at(buf, 0, 11).as_deref(), Some("AbNuiSelection"));
}

#[nvim_oxi::test]
fn a_match_keeps_its_colour_on_a_background_only_selection() {
    setup();
    lua_run(
        "vim.api.nvim_set_hl(0, 'AbNuiSelection', { bg = '#242424' }); vim.api.nvim_set_hl(0, 'AbNuiMatch', { fg = '#e0c0c0' })",
    );
    let state = ListState::new();
    let (_s, buf) = surf(20, 2, move || {
        List::with_render(vec!["one", "two"], &state, |item, _| {
            Line::from_spans([Span::hl("m", wp_ui::theme::groups::MATCH), Span::raw(*item)])
        })
        .scrollbar(false)
        .gutter(false)
    });
    let group = hl_at(buf, 0, 0).expect("highlighted");
    let colours: Vec<i64> = lua_eval(&format!(
        "local h = vim.api.nvim_get_hl(0, {{ name = '{group}', link = false }}) return {{ h.fg or -1, h.bg or -1 }}"
    ));
    assert_eq!(colours, vec![0xe0c0c0, 0x242424]);
}

#[nvim_oxi::test]
fn a_match_on_the_selected_row_keeps_the_background() {
    setup();
    lua_run("vim.api.nvim_set_hl(0, 'AbNuiSelection', { fg = '#111111', bg = '#00ffff' })");
    let state = ListState::new();
    let (_s, buf) = surf(20, 2, move || {
        List::with_render(vec!["one", "two"], &state, |item, _| {
            Line::from_spans([Span::hl("m", wp_ui::theme::groups::MATCH), Span::raw(*item)])
        })
        .scrollbar(false)
        .gutter(false)
    });
    let colours = |group: String| -> Vec<i64> {
        lua_eval(&format!(
            "local h = vim.api.nvim_get_hl(0, {{ name = '{group}', link = false }}) return {{ h.fg or -1, h.bg or -1 }}"
        ))
    };
    // Selected row: the selection's background stays; the text takes its
    // complementary colour.
    let sel_match = hl_at(buf, 0, 0).expect("highlighted");
    assert_eq!(colours(sel_match), vec![0xff0000, 0x00ffff]);
    // Other rows: the plain match highlight.
    assert_eq!(hl_at(buf, 1, 0).as_deref(), Some("AbNuiMatch"));
}

#[nvim_oxi::test]
fn text_can_cut_from_the_left() {
    setup();
    assert_eq!(render(12, 1, || Text::new("src/very/long/main.rs").ellipsis_start()), v(&["…ong/main.rs"]));
    assert_eq!(render(12, 1, || Text::new("src/very/long/main.rs").ellipsis()), v(&["src/very/lo…"]));
}

#[nvim_oxi::test]
fn code_view_shows_media_as_placeholder_cells() {
    setup();
    // A kitty-graphics terminal with true colour.
    unsafe { std::env::set_var("KITTY_WINDOW_ID", "1") };
    lua_run("vim.o.termguicolors = true");
    let dir = std::env::temp_dir().join(format!("ab-nui-media-view-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let png = dir.join("pic.png");
    image::RgbImage::new(180, 90).save(&png).unwrap();
    let path = png.to_string_lossy().to_string();

    let code = CodeView::new().unwrap();
    let c = code.clone();
    let _s = Surface::float(PopupOptions { size: Size::cells(40, 10), border: Border::None, ..Default::default() })
        .unwrap()
        .show(move || c.widget())
        .unwrap();
    assert!(CodeView::supports_media(&path));
    assert!(!CodeView::supports_media("main.rs"));
    assert!(code.show_media(&path).unwrap());
    let buf = code.buffer();
    assert!(wait(3000, move || lines(buf).iter().any(|l| l.contains('\u{10EEEE}'))), "placeholders appear");

    // The cells carry the image id as their colour; numbers are off for media.
    let marks = highlights(buf);
    let group = &marks.first().expect("coloured").3;
    assert!(group.starts_with("AbNuiMedia"), "{marks:?}");
    let id: i64 = group.trim_start_matches("AbNuiMedia").parse().unwrap();
    let fg: i64 = lua_eval(&format!("return vim.api.nvim_get_hl(0, {{ name = '{group}' }}).fg"));
    assert_eq!(fg, id);
    let win = code.embedded().window().unwrap();
    assert!(!win.get_option::<bool>("number").unwrap());
    assert!(code.is_media());

    // Text replaces it; numbers come back.
    code.set_content(&["fn main() {}"], "rust").unwrap();
    assert!(!code.is_media());
    assert_eq!(lines(buf), v(&["fn main() {}"]));
    assert!(win.get_option::<bool>("number").unwrap());
    let _ = std::fs::remove_dir_all(&dir);
}

#[nvim_oxi::test]
fn a_key_handler_can_let_the_key_through() {
    setup();
    let input = BufferInput::new().unwrap().start_insert(false);
    let take = Signal::new(false);
    let hits = Signal::new(0);
    let (t, h) = (take.clone(), hits.clone());
    input
        .on_key_or(&["i"], "<BS>", move || {
            h.update(|n| *n += 1);
            t.get_untracked()
        })
        .unwrap();
    let i = input.clone();
    let _s = Surface::float(PopupOptions { size: Size::cells(20, 1), border: Border::None, ..Default::default() })
        .unwrap()
        .show(move || i.widget())
        .unwrap();
    input.set_text("ab").unwrap();
    // Declined: an ordinary backspace.
    feed("A<BS><Esc>");
    assert_eq!(input.text(), "a");
    // Taken: nothing is deleted.
    take.set(true);
    feed("A<BS><Esc>");
    assert_eq!(input.text(), "a");
    assert_eq!(hits.get_untracked(), 2);
}

#[nvim_oxi::test]
fn clusters_are_as_wide_as_neovim_draws_them() {
    setup();
    for sample in
        ["👍🏽", "❤️", "e\u{301}", "🇺🇸", "👨\u{200d}👩\u{200d}👧", "\u{1112}\u{1161}\u{11AB}", "☺️", "x\u{FE0F}"]
    {
        let nvim: i64 = lua_eval(&format!("return vim.fn.strdisplaywidth({sample:?})"));
        assert_eq!(wp_ui::text::display_width(sample) as i64, nvim, "{sample:?}");
    }
    // Painted: what follows the clusters lands where Neovim draws it (the
    // `|` comes after 9 cells on screen, as on the canvas).
    let out = render(12, 1, || row![Text::new("a👍🏽❤️é🇺🇸b"), Text::new("|")]);
    let col: i64 =
        lua_eval(&format!("local s = {:?} return vim.fn.strdisplaywidth(s:sub(1, s:find('|', 1, true) - 1))", out[0]));
    assert_eq!(col, 9, "{out:?}");
    assert!(out[0].starts_with("a👍🏽❤️é🇺🇸b|"), "{out:?}");
}
