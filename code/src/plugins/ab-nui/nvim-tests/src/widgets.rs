//! Core components (`Popup`, `Menu`, `Input`), `vim.ui` adapters, toasts,
//! timers and spinners.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use wp_ui::nvim;
use wp_ui::prelude::*;

use crate::support::*;

type Log = Rc<RefCell<Vec<String>>>;

fn log() -> Log {
    Rc::new(RefCell::new(Vec::new()))
}

fn popup_at(w: u32, h: u32, row: f64, col: f64) -> Popup {
    let p =
        Popup::new(PopupOptions { size: Size::cells(w, h), position: Position::At { row, col }, ..Default::default() })
            .unwrap();
    p.open().unwrap();
    p
}

// ---- Popup ----------------------------------------------------------------------

#[nvim_oxi::test]
fn popup_set_position_and_size_change_window_config() {
    setup();
    let p = popup_at(20, 5, 3.0, 7.0);
    let win = p.window().unwrap();
    let c = win.config().unwrap();
    assert_eq!((c.relative.as_str(), c.width, c.height, c.row, c.col), ("editor", 20, 5, 3.0, 7.0));
    p.set_position(Position::At { row: 1.0, col: 2.0 }).unwrap();
    let c = win.config().unwrap();
    assert_eq!((c.row, c.col, c.width, c.height), (1.0, 2.0, 20, 5));
    p.set_size(Size::cells(30, 6)).unwrap();
    let c = win.config().unwrap();
    assert_eq!((c.row, c.col, c.width, c.height), (1.0, 2.0, 30, 6));
    assert_eq!(p.inner_size().unwrap(), (30, 6));
}

#[nvim_oxi::test]
fn popup_center_position_and_resize_reanchoring() {
    setup();
    let p = Popup::new(PopupOptions { size: Size::cells(20, 6), ..Default::default() }).unwrap();
    p.open().unwrap();
    let win = p.window().unwrap();
    // The whole box (content + 2-cell border) is centred in the usable area
    // (the editor minus the command line).
    let area = wp_ui::geometry::usable_area().unwrap();
    let c = win.config().unwrap();
    assert_eq!(c.col, ((area.width - 22) / 2) as f64);
    assert_eq!(c.row, ((area.height - 8) / 2) as f64);
    resize(100, 40);
    let c = win.config().unwrap();
    assert_eq!(c.col, 39.0);
    assert_eq!(c.row, 15.0);
}

#[nvim_oxi::test]
fn popup_ratio_size_resolves_against_editor() {
    setup();
    let p = Popup::new(PopupOptions { size: Size::ratio(0.5, 0.25), ..Default::default() }).unwrap();
    p.open().unwrap();
    let area = wp_ui::geometry::usable_area().unwrap();
    let c = p.window().unwrap().config().unwrap();
    assert_eq!(c.width, (area.width as f64 * 0.5).round() as i64);
    assert_eq!(c.height, (area.height as f64 * 0.25).round() as i64);
}

#[nvim_oxi::test]
fn popup_close_on_leave() {
    setup();
    let stay = popup_at(10, 2, 1.0, 1.0);
    ex("wincmd p"); // leave the popup for the main window
    assert!(stay.is_open());
    stay.close();

    let p = Popup::new(PopupOptions {
        size: Size::cells(10, 2),
        position: Position::At { row: 1.0, col: 1.0 },
        close_on_leave: true,
        ..Default::default()
    })
    .unwrap();
    p.open().unwrap();
    assert!(p.is_open());
    ex("wincmd p");
    assert!(!p.is_open());
    assert_eq!(floats().len(), 0);
}

#[nvim_oxi::test]
fn popup_on_key_and_on_close_key() {
    setup();
    let base = win_count();
    let p = popup_at(10, 2, 1.0, 1.0);
    let l = log();
    let l2 = l.clone();
    p.on_key("n", "x", move || l2.borrow_mut().push("x".into())).unwrap();
    p.on_close_key("q").unwrap();
    feed("xx");
    assert_eq!(*l.borrow(), v(&["x", "x"]));
    assert_eq!(win_count(), base + 1);
    feed("q");
    assert!(!p.is_open());
    assert_eq!(win_count(), base);
    assert!(!p.buffer().is_valid(), "non-persistent popup buffers are wiped on close");
}

#[nvim_oxi::test]
fn popup_closed_externally_reports_not_open() {
    setup();
    let p = popup_at(10, 2, 1.0, 1.0);
    ex("close");
    assert!(!p.is_open());
}

#[nvim_oxi::test]
fn popup_hide_keeps_effects_and_close_disposes_them() {
    setup();
    let n = Signal::new(0);
    let n2 = n.clone();
    let p = Popup::new(PopupOptions { size: Size::cells(10, 1), ..Default::default() }).unwrap();
    p.open().unwrap();
    p.render_reactive(move || vec![Line::raw(format!("n={}", n2.get()))]);
    p.hide();
    assert!(!p.is_open());
    n.set(4);
    assert_eq!(lines(p.buffer()), v(&["n=4"]));
    p.close();
    n.set(5);
    assert!(!p.buffer().is_valid(), "close deletes the popup's buffer");
}

#[nvim_oxi::test]
fn popup_set_cursor_requires_open_window() {
    setup();
    let p = popup_at(10, 3, 1.0, 1.0);
    p.set_content(&[Line::raw("a"), Line::raw("b"), Line::raw("c")]).unwrap();
    p.set_cursor(2, 0).unwrap();
    assert_eq!(p.window().unwrap().cursor().unwrap(), (2, 0));
    p.hide();
    assert!(p.set_cursor(1, 0).is_err());
}

// ---- Menu -----------------------------------------------------------------------

fn open_menu(chosen: &Log, cancelled: &Rc<Cell<u32>>) -> Popup {
    let menu = Menu::new(
        vec![MenuItem::new("one", 1), MenuItem::new("two", 2), MenuItem::new("three", 3)],
        MenuOptions { size: Size::cells(20, 3), ..Default::default() },
    )
    .unwrap();
    let popup = menu.popup().clone();
    let (c, k) = (chosen.clone(), cancelled.clone());
    menu.open(move |v: Option<i32>| match v {
        Some(v) => c.borrow_mut().push(v.to_string()),
        None => k.set(k.get() + 1),
    })
    .unwrap();
    popup
}

#[nvim_oxi::test]
fn menu_renders_selection_marker_and_moves() {
    setup();
    let (chosen, cancelled) = (log(), Rc::new(Cell::new(0)));
    let popup = open_menu(&chosen, &cancelled);
    let buf = popup.buffer();
    assert_eq!(lines(buf), v(&["▌ one", "  two", "  three"]));
    assert_eq!(hl_at(buf, 0, 0).as_deref(), Some("AbNuiSelection"));
    feed("j");
    assert_eq!(
        lines(buf),
        v(&["▌ one", "  two", "  three"])
            .iter()
            .enumerate()
            .map(|(i, s)| {
                let label = s.trim_start_matches(['▌', ' ']);
                if i == 1 { format!("▌ {label}") } else { format!("  {label}") }
            })
            .collect::<Vec<_>>()
    );
    assert_eq!(hl_at(buf, 0, 0), None);
    assert_eq!(hl_at(buf, 1, 0).as_deref(), Some("AbNuiSelection"));
    // The window cursor follows the selection.
    assert_eq!(popup.window().unwrap().cursor().unwrap().0, 2);
}

#[nvim_oxi::test]
fn menu_navigation_wraps_with_jk_and_arrows() {
    setup();
    let (chosen, cancelled) = (log(), Rc::new(Cell::new(0)));
    let popup = open_menu(&chosen, &cancelled);
    feed("k<CR>"); // wraps to the last
    assert_eq!(*chosen.borrow(), v(&["3"]));
    assert!(!popup.is_open());

    let chosen = log();
    let _ = open_menu(&chosen, &cancelled);
    feed("<Down><Down><Down><Down><CR>"); // 4 downs on 3 items -> index 1
    assert_eq!(*chosen.borrow(), v(&["2"]));

    let chosen = log();
    let _ = open_menu(&chosen, &cancelled);
    feed("<Up><Up><CR>");
    assert_eq!(*chosen.borrow(), v(&["2"]));
    assert_eq!(cancelled.get(), 0);
}

#[nvim_oxi::test]
fn menu_enter_chooses_exactly_once() {
    setup();
    let (chosen, cancelled) = (log(), Rc::new(Cell::new(0)));
    let popup = open_menu(&chosen, &cancelled);
    feed("j<CR>");
    feed("<CR><Esc>q");
    assert_eq!(*chosen.borrow(), v(&["2"]));
    assert_eq!(cancelled.get(), 0);
    assert!(!popup.is_open());
}

#[nvim_oxi::test]
fn menu_esc_and_q_cancel_exactly_once() {
    setup();
    let (chosen, cancelled) = (log(), Rc::new(Cell::new(0)));
    let popup = open_menu(&chosen, &cancelled);
    feed("<Esc>");
    assert_eq!(cancelled.get(), 1);
    assert!(!popup.is_open());
    feed("<Esc>q<CR>");
    assert_eq!(cancelled.get(), 1);
    assert!(chosen.borrow().is_empty());

    let popup = open_menu(&chosen, &cancelled);
    feed("q");
    assert_eq!(cancelled.get(), 2);
    assert!(!popup.is_open());
    assert!(chosen.borrow().is_empty());
}

#[nvim_oxi::test]
fn menu_long_list_scrolls_window_cursor() {
    setup();
    let items: Vec<MenuItem<usize>> = (0..10).map(|i| MenuItem::new(format!("item{i}"), i)).collect();
    let menu = Menu::new(items, MenuOptions { size: Size::cells(20, 3), ..Default::default() }).unwrap();
    let popup = menu.popup().clone();
    let sel = menu.selected();
    menu.open(|_| {}).unwrap();
    feed("jjjjj");
    assert_eq!(sel.get_untracked(), 5);
    assert_eq!(popup.window().unwrap().cursor().unwrap().0, 6);
    let top: i64 = lua_eval(&format!(
        "return vim.api.nvim_win_call({}, function() return vim.fn.line('w0') end)",
        popup.window().unwrap().id()
    ));
    assert!(top > 1, "the view scrolled (top line {top})");
}

// ---- Input ----------------------------------------------------------------------

/// Put the Input window into a state where typed text appends: Neovim's
/// `startinsert` is part of `Input::open` (see BUG note below), so only enter
/// insert mode ourselves when it is not already active.
fn append_prefix() -> &'static str {
    let mode: String = lua_eval("return vim.api.nvim_get_mode().mode");
    if mode.starts_with('i') { "" } else { "A" }
}

#[nvim_oxi::test]
fn input_submit_with_typed_text() {
    setup();
    let got = log();
    let g = got.clone();
    let input = Input::new(InputOptions { initial: "ab".into(), ..Default::default() }).unwrap();
    let popup = input.popup().clone();
    let _ = input.open(move |s| g.borrow_mut().extend(s));
    assert_eq!(lines(popup.buffer()), v(&["ab"]));
    feed(&format!("{}hello<CR>", append_prefix()));
    let g = got.clone();
    wait(500, move || !g.borrow().is_empty());
    assert_eq!(*got.borrow(), v(&["abhello"]));
    assert!(!popup.is_open());
}

#[nvim_oxi::test]
fn input_submits_initial_text_and_cancels_once() {
    setup();
    let (got, cancels) = (log(), Rc::new(Cell::new(0)));
    let (g, c) = (got.clone(), cancels.clone());
    let input = Input::new(InputOptions { initial: "keep".into(), ..Default::default() }).unwrap();
    let popup = input.popup().clone();
    let _ = input.open(move |s| match s {
        Some(s) => g.borrow_mut().push(s),
        None => c.set(c.get() + 1),
    });
    feed("<CR>");
    feed("<CR><Esc>");
    assert_eq!(*got.borrow(), v(&["keep"]));
    assert_eq!(cancels.get(), 0);
    assert!(!popup.is_open());

    let input = Input::new(InputOptions::default()).unwrap();
    let popup = input.popup().clone();
    let (g, c) = (got.clone(), cancels.clone());
    let _ = input.open(move |s| match s {
        Some(s) => g.borrow_mut().push(s),
        None => c.set(c.get() + 1),
    });
    feed("<Esc>");
    feed("<Esc><CR>");
    assert_eq!(cancels.get(), 1);
    assert_eq!(got.borrow().len(), 1, "cancel never submits");
    assert!(!popup.is_open());
}

#[nvim_oxi::test]
fn input_on_change_and_value_signal_follow_buffer_edits() {
    setup();
    let changes = log();
    let c = changes.clone();
    let input = Input::new(InputOptions::default()).unwrap().on_change(move |s| c.borrow_mut().push(s.to_string()));
    let popup = input.popup().clone();
    let value = input.value();
    let _ = input.open(|_| {});
    feed(&format!("{}hey", append_prefix()));
    // Edits are observed through TextChanged/TextChangedI.
    lua_run(&format!("vim.api.nvim_exec_autocmds('TextChangedI', {{ buffer = {} }})", popup.buffer().id()));
    assert_eq!(value.get_untracked(), "hey");
    assert_eq!(*changes.borrow(), v(&["hey"]));
}

#[nvim_oxi::test]
fn input_window_is_a_one_row_float_with_title() {
    setup();
    let input =
        Input::new(InputOptions { title: Some(" Name ".into()), width: Size::cells(30, 9), ..Default::default() })
            .unwrap();
    let popup = input.popup().clone();
    let _ = input.open(|_| {});
    let c = popup.window().unwrap().config().unwrap();
    assert_eq!((c.width, c.height), (30, 1));
    let title: String =
        lua_eval(&format!("return vim.api.nvim_win_get_config({}).title[1][1]", popup.window().unwrap().id()));
    assert_eq!(title, " Name ");
    feed("<Esc>");
}

// Regression (was a bug before the internals rewrite): `wp_ui::lua::cmd` always fails (`vim.cmd` is a callable *table*, but the
// helper fetches it as a Function: "error converting Lua table to function").
// Consequences: `Input::open` returns Err (it ends with
// `lua::cmd("startinsert!")?`) and never enters insert mode, and
// `vim.ui.input` (vim_ui::register_input) raises a Lua error on every call.
// Expected: `lua::cmd("echo 1")` is Ok, Input::open is Ok and Neovim is in insert mode.
//
#[nvim_oxi::test]
fn lua_cmd_runs_ex_commands() {
    setup();
    assert!(wp_ui::lua::cmd("let g:ab_cmd_ran = 1").is_ok());
    let ran: i64 = lua_eval("return vim.g.ab_cmd_ran");
    assert_eq!(ran, 1);
}

#[nvim_oxi::test]
fn input_open_succeeds_and_starts_insert() {
    setup();
    let input = Input::new(InputOptions::default()).unwrap();
    assert!(input.open(|_| {}).is_ok());
}

// ---- vim.ui ---------------------------------------------------------------------

#[nvim_oxi::test]
fn vim_ui_select_returns_item_and_index_once() {
    setup();
    wp_ui::vim_ui::register_select().unwrap();
    lua_run(
        r#"
        _G.calls = 0
        vim.ui.select({ "alpha", "beta", "gamma" }, { prompt = "Pick:" }, function(item, idx)
          _G.calls = _G.calls + 1
          _G.item = item
          _G.idx = idx
        end)
        "#,
    );
    assert_eq!(win_count(), 2);
    feed("jj<CR>");
    feed("<CR><Esc>");
    assert_eq!(lua_eval::<i64>("return _G.calls"), 1);
    assert_eq!(lua_eval::<String>("return _G.item"), "gamma");
    assert_eq!(lua_eval::<i64>("return _G.idx"), 3);
    assert_eq!(win_count(), 1);
}

#[nvim_oxi::test]
fn vim_ui_select_cancel_gives_nil_once() {
    setup();
    wp_ui::vim_ui::register_select().unwrap();
    lua_run(
        r#"
        _G.calls = 0
        _G.item = "unset"
        vim.ui.select({ "a", "b" }, {}, function(item, idx)
          _G.calls = _G.calls + 1
          _G.item = item
          _G.idx = idx
        end)
        "#,
    );
    feed("<Esc>");
    feed("<Esc>q");
    assert_eq!(lua_eval::<i64>("return _G.calls"), 1);
    assert!(lua_eval::<Option<String>>("return _G.item").is_none());
    assert!(lua_eval::<Option<i64>>("return _G.idx").is_none());
    assert_eq!(win_count(), 1);
}

#[nvim_oxi::test]
fn vim_ui_select_uses_format_item_and_keeps_original_values() {
    setup();
    wp_ui::vim_ui::register_select().unwrap();
    lua_run(
        r#"
        _G.res = nil
        vim.ui.select({ { name = "x", id = 7 }, { name = "y", id = 9 } }, {
          prompt = "Choose:",
          format_item = function(it) return "item " .. it.name end,
        }, function(item) _G.res = item end)
        "#,
    );
    let buf = floats().iter().map(|w| w.buffer().unwrap()).next().unwrap();
    assert_eq!(lines(buf), v(&["▌ item x", "  item y"]));
    let title: String = lua_eval(&format!("return vim.api.nvim_win_get_config({}).title[1][1]", floats()[0].id()));
    assert_eq!(title, " Choose ");
    feed("j<CR>");
    assert_eq!(lua_eval::<i64>("return _G.res.id"), 9);
}

// Regression (same root cause as above, fixed): `vim.ui.input` raises "error converting Lua
// table to function" because Input::open fails on `lua::cmd`. Expected: the
// call succeeds and the callback fires exactly once with the submitted text
// (or nil when cancelled).
//
#[nvim_oxi::test]
fn vim_ui_input_submit_and_cancel() {
    setup();
    wp_ui::vim_ui::register_input().unwrap();
    lua_run(
        "_G.calls = 0; vim.ui.input({ prompt = 'Name:', default = 'x' }, function(v) _G.calls = _G.calls + 1; _G.v = v end)",
    );
    feed("<CR>");
    assert_eq!(lua_eval::<i64>("return _G.calls"), 1);
    assert_eq!(lua_eval::<String>("return _G.v"), "x");
}

// ---- notify ---------------------------------------------------------------------

fn sticky() -> NotifyOptions {
    NotifyOptions { timeout_ms: None, ..Default::default() }
}

fn title_of(w: Window) -> String {
    lua_eval(&format!("return vim.api.nvim_win_get_config({}).title[1][1]", w.id()))
}

#[nvim_oxi::test]
fn notify_shows_a_toast_float_without_stealing_focus() {
    setup();
    let main = current_win();
    let h = notify("hello there", sticky()).unwrap();
    assert_eq!(win_count(), 2);
    assert_eq!(current_win(), main);
    let toast = floats()[0];
    assert_eq!(trimmed(toast.buffer().unwrap()), v(&["hello there"]));
    assert_eq!(title_of(toast), " Info ");
    let c = toast.config().unwrap();
    assert_eq!(c.width, 40);
    assert_eq!(c.height, 1);
    h.dismiss();
    assert_eq!(win_count(), 1);
}

#[nvim_oxi::test]
fn notify_level_titles_custom_title_and_highlight() {
    setup();
    let _a = notify("w", NotifyOptions { level: Level::Warn, ..sticky() }).unwrap();
    let _b = notify("e", NotifyOptions { level: Level::Error, title: Some("Boom".into()), ..sticky() }).unwrap();
    let mut titles: Vec<String> = floats().into_iter().map(title_of).collect();
    titles.sort();
    assert_eq!(titles, v(&[" Boom ", " Warning "]));
    for w in floats() {
        let hl: String = lua_eval(&format!("return vim.wo[{}].winhighlight", w.id()));
        let want = if title_of(w).contains("Boom") { "AbNuiError" } else { "AbNuiWarn" };
        assert!(hl.contains(want), "{hl}");
    }
}

#[nvim_oxi::test]
fn notify_wraps_long_messages() {
    setup();
    let _h = notify("one two three four five", NotifyOptions { width: 20, ..sticky() }).unwrap();
    let toast = floats()[0];
    assert_eq!(trimmed(toast.buffer().unwrap()), v(&["one two three four", "five"]));
    assert_eq!(toast.config().unwrap().height, 2);
}

#[nvim_oxi::test]
fn notify_keeps_line_breaks() {
    setup();
    // Each line wraps on its own; the toast is as tall as all of them.
    let _h = notify("first line\nsecond one is longer than twenty", NotifyOptions { width: 20, ..sticky() }).unwrap();
    let toast = floats()[0];
    assert_eq!(trimmed(toast.buffer().unwrap()), v(&["first line", "second one is longer", "than twenty"]));
    assert_eq!(toast.config().unwrap().height, 3);
}

#[nvim_oxi::test]
fn notify_height_matches_the_wrapped_text() {
    setup();
    // 39 chars: wraps to 2 rows at the toast's width of 20, but would be 3
    // at 18 — the toast must be exactly as tall as what it shows.
    let _h = notify("abcdefghi abcdefghi abcdefghi abcdefghi", NotifyOptions { width: 20, ..sticky() }).unwrap();
    let toast = floats()[0];
    let shown = trimmed(toast.buffer().unwrap());
    assert!(shown.iter().all(|l| !l.is_empty()), "no blank rows: {shown:?}");
    assert_eq!(toast.config().unwrap().height as usize, shown.len());
}

#[nvim_oxi::test]
fn notify_stacks_newest_on_top_and_restacks_on_dismiss() {
    setup();
    let first = notify("first", sticky()).unwrap();
    let second = notify("second", sticky()).unwrap();
    let by_text = |t: &str| -> Window { floats().into_iter().find(|w| trimmed(w.buffer().unwrap())[0] == t).unwrap() };
    let (rf, rs) = (by_text("first").config().unwrap(), by_text("second").config().unwrap());
    assert_eq!(rs.row, 1.0);
    assert_eq!(rf.row, 4.0, "older toast is pushed down by the newest one's height (1 row + border)");
    assert_eq!(rs.col, rf.col);
    let (cols, _) = wp_ui::geometry::editor_size().unwrap();
    assert_eq!(rs.col, (cols - 40 - 3) as f64);

    second.dismiss();
    assert_eq!(win_count(), 2);
    assert_eq!(by_text("first").config().unwrap().row, 1.0);
    first.dismiss();
    assert_eq!(win_count(), 1);
    first.dismiss(); // dismissing twice is harmless
}

#[nvim_oxi::test]
fn dismiss_all_removes_every_toast() {
    setup();
    let _a = notify("a", sticky()).unwrap();
    let _b = notify("b", sticky()).unwrap();
    let _c = notify("c", sticky()).unwrap();
    assert_eq!(win_count(), 4);
    dismiss_all();
    assert_eq!(win_count(), 1);
    // New toasts still work afterwards.
    let _d = notify("d", sticky()).unwrap();
    assert_eq!(win_count(), 2);
    dismiss_all();
}

#[nvim_oxi::test]
fn notify_auto_dismisses_after_timeout() {
    setup();
    let _keep = notify("sticky", sticky()).unwrap();
    let _h = notify("temp", NotifyOptions { timeout_ms: Some(100), ..Default::default() }).unwrap();
    assert_eq!(win_count(), 3);
    assert!(wait(2000, || win_count() == 2), "the timed toast disappears");
    // The sticky one survives and moves back to the top.
    assert_eq!(floats()[0].config().unwrap().row, 1.0);
    assert_eq!(trimmed(floats()[0].buffer().unwrap()), v(&["sticky"]));
}

// ---- timers -----------------------------------------------------------------------

#[nvim_oxi::test]
fn timer_once_fires_exactly_once() {
    setup();
    let n = Rc::new(Cell::new(0));
    let n2 = n.clone();
    let _t = Timer::once(20, move || n2.set(n2.get() + 1)).unwrap();
    assert_eq!(n.get(), 0);
    assert!(wait(1000, {
        let n = n.clone();
        move || n.get() >= 1
    }));
    wait(100, || false);
    assert_eq!(n.get(), 1);
}

#[nvim_oxi::test]
fn timer_interval_repeats_until_stopped() {
    setup();
    let n = Rc::new(Cell::new(0));
    let n2 = n.clone();
    let t = Timer::interval(10, move || n2.set(n2.get() + 1)).unwrap();
    let nn = n.clone();
    assert!(wait(2000, move || nn.get() >= 3));
    t.stop().unwrap();
    wait(30, || false);
    let after_stop = n.get();
    wait(100, || false);
    assert_eq!(n.get(), after_stop, "no more ticks once stopped");
}

#[nvim_oxi::test]
fn timer_drop_cancels_pending_callback() {
    setup();
    let n = Rc::new(Cell::new(0));
    let n2 = n.clone();
    let t = Timer::once(30, move || n2.set(n2.get() + 1)).unwrap();
    drop(t);
    wait(150, || false);
    assert_eq!(n.get(), 0);

    let m = Rc::new(Cell::new(0));
    let m2 = m.clone();
    let t = Timer::interval(10, move || m2.set(m2.get() + 1)).unwrap();
    let mm = m.clone();
    assert!(wait(2000, move || mm.get() >= 1));
    drop(t);
    let at_drop = m.get();
    wait(100, || false);
    assert_eq!(m.get(), at_drop);
}

#[nvim_oxi::test]
fn timer_close_is_idempotent() {
    setup();
    let t = Timer::interval(50, || {}).unwrap();
    t.close().unwrap();
    t.close().unwrap();
}

#[nvim_oxi::test]
fn defer_runs_after_the_delay() {
    setup();
    let n = Rc::new(Cell::new(0));
    let n2 = n.clone();
    nvim::defer(30, move || n2.set(n2.get() + 1)).unwrap();
    assert_eq!(n.get(), 0);
    let nn = n.clone();
    assert!(wait(2000, move || nn.get() == 1));
    wait(100, || false);
    assert_eq!(n.get(), 1);
}

#[nvim_oxi::test]
fn schedule_runs_later_not_inline() {
    setup();
    let order = log();
    let o2 = order.clone();
    nvim::schedule(move || o2.borrow_mut().push("scheduled".into())).unwrap();
    order.borrow_mut().push("inline".into());
    assert!(wait(1000, {
        let order = order.clone();
        move || order.borrow().len() == 2
    }));
    assert_eq!(*order.borrow(), v(&["inline", "scheduled"]));
}

// ---- Spinner ----------------------------------------------------------------------

#[nvim_oxi::test]
fn spinner_state_advances_frames_and_stop_freezes_it() {
    setup();
    let st = SpinnerState::start(20).unwrap();
    let frame = st.frame();
    assert_eq!(frame.get_untracked(), 0);
    let f = frame.clone();
    assert!(wait(2000, move || f.get_untracked() >= 2));
    st.stop();
    wait(60, || false);
    let frozen = frame.get_untracked();
    wait(100, || false);
    assert_eq!(frame.get_untracked(), frozen);
}

#[nvim_oxi::test]
fn spinner_widget_repaints_each_tick() {
    setup();
    let st = SpinnerState::start(20).unwrap();
    let s2 = st.clone();
    let (_s, buf) = surf(6, 1, move || row![Spinner::new(&s2), Text::new(" busy")]);
    assert_eq!(trimmed(buf), v(&[&format!("{} busy", wp_ui::view::SPINNER_DOTS[0])]));
    let frame = st.frame();
    let f = frame.clone();
    assert!(wait(2000, move || f.get_untracked() >= 1));
    let n = frame.get_untracked();
    assert_eq!(trimmed(buf), v(&[&format!("{} busy", wp_ui::view::SPINNER_DOTS[n % wp_ui::view::SPINNER_DOTS.len()])]));
    // Custom frames and group.
    let s3 = st.clone();
    let (_s2, buf2) = surf(4, 1, move || Spinner::new(&s3).frames(&["-", "+"]).fg("Title"));
    let n = frame.get_untracked();
    assert_eq!(trimmed(buf2), v(&[["-", "+"][n % 2]]));
    assert_eq!(hl_at(buf2, 0, 0).as_deref(), Some("Title"));
    st.stop();
}
