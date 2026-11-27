//! Regression tests for the core components (`Popup`, `Menu`, `Input`), the
//! `vim.ui` adapters and the Neovim bridge.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use wp_ui::nvim::{self, CommandOpts, FloatConfig, Relative};
use wp_ui::prelude::*;

use crate::support::*;

/// What `Menu::open` / `Input::open` answered: `Some(answer)` once called.
type Answer<T> = Rc<RefCell<Vec<Option<T>>>>;

fn answers<T>() -> Answer<T> {
    Rc::new(RefCell::new(Vec::new()))
}

fn menu(items: &[&str], got: &Answer<String>) -> Popup {
    let items = items.iter().map(|s| MenuItem::new(*s, s.to_string())).collect();
    let menu = Menu::new(items, MenuOptions { size: Size::cells(20, 5), ..Default::default() }).unwrap();
    let popup = menu.popup().clone();
    let g = got.clone();
    menu.open(move |v| g.borrow_mut().push(v)).unwrap();
    popup
}

fn input(initial: &str, got: &Answer<String>) -> Popup {
    let input = Input::new(InputOptions { initial: initial.into(), ..Default::default() }).unwrap();
    let popup = input.popup().clone();
    let g = got.clone();
    input.open(move |v| g.borrow_mut().push(v)).unwrap();
    popup
}

fn autocmd_count(event: &str) -> i64 {
    lua_eval(&format!("return #vim.api.nvim_get_autocmds({{ event = '{event}' }})"))
}

/// The `:augroup` listing (every defined group name).
fn augroups() -> String {
    lua_eval("return vim.api.nvim_exec2('augroup', { output = true }).output")
}

/// `_G.calls`/`_G.item` recorder for `vim.ui.select`.
const SELECT_RECORDER: &str = r#"
  _G.calls, _G.item, _G.idx = 0, "unset", "unset"
  _G.rec = function(item, idx) _G.calls = _G.calls + 1; _G.item = item; _G.idx = idx end
"#;

/// Typed text appends to an Input that was just opened: `startinsert!` from a
/// script only takes effect once Neovim gets back to waiting for keys.
fn append_prefix() -> &'static str {
    let mode: String = lua_eval("return vim.api.nvim_get_mode().mode");
    if mode.starts_with('i') { "" } else { "A" }
}

fn calls() -> i64 {
    lua_eval("return _G.calls")
}

// ---- 1. the window closed or left without <CR>/<Esc> ----------------------------

#[nvim_oxi::test]
fn menu_closed_externally_answers_nil_and_tears_down() {
    setup();
    wp_ui::vim_ui::register_select().unwrap();
    lua_run(SELECT_RECORDER);
    lua_run("vim.ui.select({ 'a', 'b' }, {}, _G.rec)");
    let win = floats()[0];
    let buf = win.buffer().unwrap();
    // Another plugin closes it.
    lua_run(&format!("vim.api.nvim_win_close({}, true)", win.id()));
    assert!(wait(500, || lua_eval::<i64>("return _G.calls") > 0), "on_choice never called");
    assert_eq!(calls(), 1);
    assert!(lua_eval::<Option<String>>("return _G.item").is_none());
    // Nothing left behind: no relayout autocmd, no group, no buffer.
    assert_eq!(autocmd_count("VimResized"), 0);
    assert!(!augroups().contains("AbNuiPopup"));
    assert!(!buf.is_valid());
    feed("<Esc>q<CR>");
    assert_eq!(calls(), 1);
}

#[nvim_oxi::test]
fn menu_closed_with_q_command_or_left_answers_nil_once() {
    setup();
    let got = answers();
    let popup = menu(&["a", "b"], &got);
    ex("close");
    assert!(wait(500, {
        let g = got.clone();
        move || !g.borrow().is_empty()
    }));
    assert_eq!(*got.borrow(), vec![None]);
    assert!(!popup.is_open());

    // Focus leaving the menu (<C-w>p, a click elsewhere) is a cancel too.
    let got = answers();
    let popup = menu(&["a", "b"], &got);
    ex("wincmd p");
    assert!(wait(500, {
        let g = got.clone();
        move || !g.borrow().is_empty()
    }));
    assert_eq!(*got.borrow(), vec![None]);
    assert!(!popup.is_open());
    assert!(floats().is_empty());
    assert_eq!(autocmd_count("VimResized"), 0);
}

#[nvim_oxi::test]
fn input_closed_externally_or_left_answers_nil_once() {
    setup();
    wp_ui::vim_ui::register_input().unwrap();
    lua_run("_G.calls = 0; vim.ui.input({ default = 'x' }, function(v) _G.calls = _G.calls + 1; _G.v = v end)");
    let win = floats()[0];
    lua_run(&format!("vim.api.nvim_win_close({}, true)", win.id()));
    assert!(wait(500, || lua_eval::<i64>("return _G.calls") > 0), "on_confirm never called");
    assert_eq!(calls(), 1);
    assert!(lua_eval::<Option<String>>("return _G.v").is_none());
    assert_eq!(autocmd_count("VimResized"), 0);
    assert_eq!(autocmd_count("TextChangedI"), 0);

    let got = answers();
    let popup = input("y", &got);
    ex("stopinsert");
    ex("wincmd p");
    assert!(wait(500, {
        let g = got.clone();
        move || !g.borrow().is_empty()
    }));
    assert_eq!(*got.borrow(), vec![None]);
    assert!(!popup.is_open());
}

// ---- 2. <CR> on an empty list ---------------------------------------------------

#[nvim_oxi::test]
fn vim_ui_select_empty_list_enter_answers_nil() {
    setup();
    wp_ui::vim_ui::register_select().unwrap();
    lua_run(SELECT_RECORDER);
    lua_run("vim.ui.select({}, { prompt = 'Nothing' }, _G.rec)");
    feed("<CR>");
    assert_eq!(calls(), 1);
    assert!(lua_eval::<Option<String>>("return _G.item").is_none());
    assert!(floats().is_empty());
}

// ---- 3. the cursor is the selection ---------------------------------------------

#[nvim_oxi::test]
fn menu_selection_follows_any_cursor_motion() {
    setup();
    let items = ["one", "two", "three", "four", "five"];

    let got = answers();
    let _ = menu(&items, &got);
    feed("G<CR>");
    assert_eq!(*got.borrow(), vec![Some("five".to_string())]);

    let got = answers();
    let _ = menu(&items, &got);
    feed("Ggg<CR>");
    assert_eq!(*got.borrow(), vec![Some("one".to_string())]);

    let got = answers();
    let _ = menu(&items, &got);
    feed("3j<CR>");
    assert_eq!(*got.borrow(), vec![Some("four".to_string())]);

    let got = answers();
    let _ = menu(&items, &got);
    feed("/thr<CR><CR>");
    assert_eq!(*got.borrow(), vec![Some("three".to_string())]);

    // The marker follows too (CursorMoved).
    let got = answers();
    let popup = menu(&items, &got);
    popup.set_cursor(4, 0).unwrap();
    lua_run("vim.api.nvim_exec_autocmds('CursorMoved', { buffer = 0 })");
    assert_eq!(lines(popup.buffer())[3], "▌ four");
    feed("<CR>");
    assert_eq!(*got.borrow(), vec![Some("four".to_string())]);
}

// ---- 4. opened from insert mode -------------------------------------------------

#[nvim_oxi::test]
fn vim_ui_select_and_input_work_from_insert_mode() {
    setup();
    wp_ui::vim_ui::register_all().unwrap();
    lua_run(SELECT_RECORDER);
    lua_run(
        r#"
        vim.keymap.set('i', '<F2>', function() vim.ui.select({ 'a', 'b', 'c' }, {}, _G.rec) end)
        vim.keymap.set('i', '<F3>', function()
          vim.ui.input({ default = 'x' }, function(v) _G.v = v end)
        end)
        "#,
    );
    feed("i<F2>j<CR>");
    assert_eq!(calls(), 1);
    assert_eq!(lua_eval::<String>("return _G.item"), "b");
    assert!(floats().is_empty());
    // The main buffer was not typed into.
    assert_eq!(lines(current_win().buffer().unwrap()), v(&[""]));

    feed("i<F3>yz<CR>");
    assert_eq!(lua_eval::<String>("return _G.v"), "xyz");
    assert!(floats().is_empty());
    assert_eq!(lines(current_win().buffer().unwrap()), v(&[""]));
}

// ---- 5. line breaks in content --------------------------------------------------

#[nvim_oxi::test]
fn line_breaks_in_content_do_not_break_the_frame() {
    setup();
    let p = Popup::new(PopupOptions { size: Size::cells(20, 3), ..Default::default() }).unwrap();
    p.open().unwrap();
    p.set_content(&[Line::raw("a\nb"), Line::from_spans([Span::hl("c\r\nd", "ErrorMsg")])]).unwrap();
    assert_eq!(lines(p.buffer()), v(&["a↵b", "c↵d"]));
    assert_eq!(hl_at(p.buffer(), 1, 0).as_deref(), Some("ErrorMsg"));
    p.close();

    wp_ui::vim_ui::register_all().unwrap();
    lua_run(SELECT_RECORDER);
    lua_run("vim.ui.select({ 'x\\ny', 'z' }, { prompt = 'Code\\nactions:' }, _G.rec)");
    let win = floats()[0];
    assert_eq!(lines(win.buffer().unwrap()), v(&["▌ x↵y", "  z"]));
    let title: String = lua_eval(&format!("return vim.api.nvim_win_get_config({}).title[1][1]", win.id()));
    assert_eq!(title, " Code↵actions ");
    feed("<CR>");
    assert_eq!(lua_eval::<String>("return _G.item"), "x\ny");

    lua_run("vim.ui.input({ default = 'a\\nb' }, function(v) _G.v = v end)");
    assert_eq!(lines(floats()[0].buffer().unwrap()), v(&["a b"]));
    feed("<CR>");
    assert_eq!(lua_eval::<String>("return _G.v"), "a b");
}

// ---- 6. nil / missing opts ------------------------------------------------------

#[nvim_oxi::test]
fn vim_ui_accepts_nil_or_missing_opts() {
    setup();
    wp_ui::vim_ui::register_all().unwrap();
    lua_run(SELECT_RECORDER);
    lua_run("vim.ui.select({ 'a', 'b' }, nil, _G.rec)");
    feed("j<CR>");
    assert_eq!(lua_eval::<String>("return _G.item"), "b");

    lua_run("vim.ui.input(nil, function(v) _G.v1 = v end)");
    feed(&format!("{}hi<CR>", append_prefix()));
    assert_eq!(lua_eval::<String>("return _G.v1"), "hi");

    lua_run("vim.ui.input(function(v) _G.v2 = v end)");
    feed(&format!("{}yo<CR>", append_prefix()));
    assert_eq!(lua_eval::<String>("return _G.v2"), "yo");
}

// ---- 7. geometry: content size, border and command line -------------------------

#[nvim_oxi::test]
fn popup_size_is_the_content_size_and_fits_the_usable_area() {
    setup();
    resize(80, 24); // cmdheight = 1, no tabline: 23 usable rows
    let p = Popup::new(PopupOptions { size: Size::ratio(1.0, 1.0), ..Default::default() }).unwrap();
    assert_eq!(p.inner_size().unwrap(), (78, 21));
    p.open().unwrap();
    let win = p.window().unwrap();
    assert_eq!((win.width().unwrap(), win.height().unwrap()), (78, 21));
    assert_eq!(p.inner_size().unwrap(), (78, 21));
    p.close();

    // Centred as a whole box (content 20x6 + border) in 80x23.
    let p = Popup::new(PopupOptions { size: Size::cells(20, 6), ..Default::default() }).unwrap();
    p.open().unwrap();
    let c = p.window().unwrap().config().unwrap();
    assert_eq!((c.row, c.col, c.width, c.height), (7.0, 29.0, 20, 6));
    p.close();

    // A shown tabline is not usable either.
    ex("set showtabline=2");
    let p =
        Popup::new(PopupOptions { size: Size::ratio(1.0, 1.0), border: Border::None, ..Default::default() }).unwrap();
    p.open().unwrap();
    let c = p.window().unwrap().config().unwrap();
    assert_eq!((c.row, c.height), (1.0, 22));
    assert_eq!(p.inner_size().unwrap(), (80, 22));
}

// ---- 8. errors from the user's callbacks ----------------------------------------

#[nvim_oxi::test]
fn vim_ui_callback_errors_are_reported() {
    setup();
    wp_ui::vim_ui::register_all().unwrap();
    lua_run(
        r#"
        _G.notes = {}
        vim.notify = function(msg, level) table.insert(_G.notes, { msg = msg, level = level }) end
        vim.ui.select({ 'a' }, {}, function() error('select boom') end)
        "#,
    );
    feed("<CR>");
    lua_run("vim.ui.input({}, function() error('input boom') end)");
    feed("<CR>");
    assert_eq!(lua_eval::<i64>("return #_G.notes"), 2);
    assert!(lua_eval::<String>("return _G.notes[1].msg").contains("select boom"));
    assert!(lua_eval::<String>("return _G.notes[2].msg").contains("input boom"));
    assert_eq!(lua_eval::<i64>("return _G.notes[1].level"), lua_eval::<i64>("return vim.log.levels.ERROR"));
}

// ---- 9 / A. the buffer lives until close() --------------------------------------

#[nvim_oxi::test]
fn popup_hide_round_trips_and_close_deletes_the_buffer() {
    setup();
    let p = Popup::new(PopupOptions { size: Size::cells(10, 2), ..Default::default() }).unwrap();
    p.open().unwrap();
    p.set_content(&[Line::raw("kept")]).unwrap();
    p.hide();
    assert!(p.buffer().is_valid());
    p.open().unwrap();
    assert_eq!(lines(p.buffer()), v(&["kept"]));

    // Closed with :q: still just hidden.
    ex("close");
    assert!(p.buffer().is_valid());

    p.close();
    assert!(!p.buffer().is_valid());
    assert!(p.open().is_err(), "a closed popup stays closed");
}

// ---- 10 / E. clearing a title ---------------------------------------------------

#[nvim_oxi::test]
fn popup_title_can_be_cleared() {
    setup();
    let p =
        Popup::new(PopupOptions { size: Size::cells(10, 2), title: Some(" t ".into()), ..Default::default() }).unwrap();
    p.open().unwrap();
    let id = p.window().unwrap().id();
    let has_title = move || lua_eval::<bool>(&format!("return vim.api.nvim_win_get_config({id}).title ~= nil"));
    assert!(has_title());
    p.clear_title().unwrap();
    assert!(!has_title());
    p.set_title(" u ").unwrap();
    assert!(has_title());
    p.set_title("").unwrap();
    assert!(!has_title());

    // A borderless popup with a title opens fine (there is just no title).
    let q = Popup::new(PopupOptions {
        size: Size::cells(10, 2),
        border: Border::None,
        title: Some(" t ".into()),
        ..Default::default()
    })
    .unwrap();
    q.open().unwrap();
    q.set_title(" u ").unwrap();
}

// ---- 11. cursor-relative floats -------------------------------------------------

#[nvim_oxi::test]
fn cursor_relative_float_keeps_its_place_on_set_config() {
    setup();
    let main = current_win();
    lua_run("vim.api.nvim_buf_set_lines(0, 0, -1, false, { 'aaaaaaaaaa', 'bbbbbbbbbb', 'cccccccccc' })");
    main.set_cursor(2, 4).unwrap();
    let mut cfg = FloatConfig::new(Rect { row: 1.0, col: 0.0, width: 10, height: 2 });
    cfg.relative = Relative::Cursor;
    cfg.border = Border::None;
    cfg.enter = false;
    let win = Window::open_float(Buffer::scratch().unwrap(), &cfg).unwrap();
    let pos = || -> Vec<i64> { wp_ui::lua::call_api("nvim_win_get_position", (win.id(),)).unwrap() };
    let before = pos();

    main.set_cursor(1, 0).unwrap();
    cfg.rect.width = 12;
    win.set_config(&cfg).unwrap();
    assert_eq!(pos(), before);
    assert_eq!(win.width().unwrap(), 12);

    // Focused, the "cursor" would be the float's own.
    win.focus().unwrap();
    cfg.rect.height = 3;
    win.set_config(&cfg).unwrap();
    assert_eq!(pos(), before);
    assert_eq!(win.height().unwrap(), 3);
}

// ---- lower severity -------------------------------------------------------------

#[nvim_oxi::test]
fn finished_inputs_and_menus_leave_no_autocmd_groups() {
    setup();
    let got = answers();
    let _ = input("a", &got);
    feed("<CR>");
    let _ = menu(&["a"], &got);
    feed("<CR>");
    assert_eq!(*got.borrow(), vec![Some("a".to_string()), Some("a".to_string())]);
    let groups = augroups();
    assert!(!groups.contains("AbNuiInput") && !groups.contains("AbNuiPopup"), "{groups}");
    assert_eq!(autocmd_count("TextChangedI"), 0);
    assert_eq!(autocmd_count("CursorMoved"), 0);
}

#[nvim_oxi::test]
fn close_on_leave_installs_once_and_only_watches_its_own_window() {
    setup();
    let p = Popup::new(PopupOptions { size: Size::cells(10, 2), close_on_leave: true, ..Default::default() }).unwrap();
    for _ in 0..3 {
        p.open().unwrap();
        p.hide();
    }
    p.open().unwrap();
    let n: i64 =
        lua_eval(&format!("return #vim.api.nvim_get_autocmds({{ event = 'WinLeave', buffer = {} }})", p.buffer().id()));
    assert_eq!(n, 1);
    ex("wincmd p");
    assert!(!p.is_open());

    // `with_buffer`: leaving ANOTHER window that shows the same buffer is not
    // leaving the popup.
    let buf = Buffer::scratch().unwrap();
    buf.set_option("bufhidden", "hide").unwrap();
    lua_run(&format!("vim.api.nvim_win_set_buf(0, {})", buf.id()));
    let shared = Popup::with_buffer(
        buf,
        PopupOptions { size: Size::cells(10, 2), close_on_leave: true, enter: false, ..Default::default() },
    )
    .unwrap();
    shared.open().unwrap();
    ex("split");
    ex("wincmd p");
    assert!(shared.is_open());
    shared.window().unwrap().focus().unwrap();
    ex("wincmd p");
    assert!(!shared.is_open());
    assert!(buf.is_valid());
}

#[nvim_oxi::test]
fn user_command_with_completion_takes_arguments() {
    setup();
    let got = Rc::new(Cell::new(0usize));
    let g = got.clone();
    nvim::user_command("AbNuiCompl", &CommandOpts { complete: Some("file".into()), ..Default::default() }, move |a| {
        g.set(a.fargs.len())
    })
    .unwrap();
    ex("AbNuiCompl a b");
    assert_eq!(got.get(), 2);
}
