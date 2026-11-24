//! The guarantees the rendering core makes about *how much* it asks of
//! Neovim: unchanged frames cost nothing, changed frames send only the rows
//! that changed, and one callback causes one repaint.

use std::cell::Cell;
use std::rc::Rc;

use wp_ui::prelude::*;

use crate::support::*;

/// Count calls to the buffer-painting API functions. Must run before
/// [`setup`], so the library's cached handles are the counting wrappers.
fn count_api_calls() {
    lua_run_raw(
        r#"
        _G.calls = { nvim_buf_set_lines = 0, nvim_buf_set_extmark = 0, nvim_buf_clear_namespace = 0 }
        _G.line_ranges = {}
        for name in pairs(_G.calls) do
          local real = vim.api[name]
          vim.api[name] = function(...)
            _G.calls[name] = _G.calls[name] + 1
            if name == "nvim_buf_set_lines" then
              local args = { ... }
              table.insert(_G.line_ranges, args[2] .. ":" .. args[3])
            end
            return real(...)
          end
        end
        "#,
    );
}

/// Like `lua_run`, but usable before the library is initialised.
fn lua_run_raw(code: &str) {
    nvim_oxi::mlua::lua().load(code).exec().expect("lua");
}

fn calls(name: &str) -> i64 {
    lua_eval(&format!("return _G.calls.{name}"))
}

fn line_ranges() -> String {
    lua_eval("return table.concat(_G.line_ranges, ' ')")
}

fn reset_calls() {
    lua_run("for k in pairs(_G.calls) do _G.calls[k] = 0 end _G.line_ranges = {}");
}

fn five_lines(third: &str) -> Vec<Line> {
    vec![
        Line::raw("one"),
        Line::from_spans([Span::hl("two", "Title")]),
        Line::raw(third),
        Line::from_spans([Span::raw("fo"), Span::hl("ur", "ErrorMsg")]),
        Line::raw("five"),
    ]
}

#[nvim_oxi::test]
fn an_unchanged_frame_sends_nothing() {
    count_api_calls();
    setup();
    let popup = Popup::new(PopupOptions { size: Size::cells(20, 5), ..Default::default() }).unwrap();
    popup.open().unwrap();
    popup.set_content(&five_lines("three")).unwrap();
    reset_calls();

    popup.set_content(&five_lines("three")).unwrap();

    assert_eq!(calls("nvim_buf_set_lines"), 0);
    assert_eq!(calls("nvim_buf_set_extmark"), 0);
    assert_eq!(calls("nvim_buf_clear_namespace"), 0);
}

#[nvim_oxi::test]
fn a_changed_row_is_the_only_row_sent() {
    count_api_calls();
    setup();
    let popup = Popup::new(PopupOptions { size: Size::cells(20, 5), ..Default::default() }).unwrap();
    popup.open().unwrap();
    popup.set_content(&five_lines("three")).unwrap();
    reset_calls();

    popup.set_content(&five_lines("THREE")).unwrap();

    assert_eq!(line_ranges(), "2:3", "only row 2 is rewritten");
    assert_eq!(calls("nvim_buf_set_extmark"), 0, "row 2 has no highlights to re-apply");
    assert_eq!(lines(popup.buffer()), vec!["one", "two", "THREE", "four", "five"]);
    // The highlights of the untouched rows are still in place.
    assert_eq!(
        highlights(popup.buffer()),
        vec![(1, 0, 3, "Title".to_string()), (3, 2, 4, "ErrorMsg".to_string())]
    );
}

#[nvim_oxi::test]
fn a_highlight_only_change_does_not_resend_text() {
    count_api_calls();
    setup();
    let popup = Popup::new(PopupOptions { size: Size::cells(20, 5), ..Default::default() }).unwrap();
    popup.open().unwrap();
    popup.set_content(&five_lines("three")).unwrap();
    reset_calls();

    let mut next = five_lines("three");
    next[1] = Line::from_spans([Span::hl("two", "WarningMsg")]);
    popup.set_content(&next).unwrap();

    assert_eq!(calls("nvim_buf_set_lines"), 0);
    assert_eq!(calls("nvim_buf_set_extmark"), 1);
    assert_eq!(
        highlights(popup.buffer()),
        vec![(1, 0, 3, "WarningMsg".to_string()), (3, 2, 4, "ErrorMsg".to_string())]
    );
}

#[nvim_oxi::test]
fn a_different_row_count_repaints_in_full() {
    setup();
    let popup = Popup::new(PopupOptions { size: Size::cells(20, 5), ..Default::default() }).unwrap();
    popup.open().unwrap();
    popup.set_content(&five_lines("three")).unwrap();

    popup.set_content(&[Line::from_spans([Span::hl("only", "Title")])]).unwrap();
    assert_eq!(lines(popup.buffer()), vec!["only"]);
    assert_eq!(highlights(popup.buffer()), vec![(0, 0, 4, "Title".to_string())]);

    popup.set_content(&five_lines("three")).unwrap();
    assert_eq!(lines(popup.buffer()), vec!["one", "two", "three", "four", "five"]);
}

#[nvim_oxi::test]
fn an_edit_behind_the_renderers_back_is_repaired() {
    setup();
    let popup = Popup::new(PopupOptions { size: Size::cells(20, 5), ..Default::default() }).unwrap();
    popup.open().unwrap();
    popup.set_content(&five_lines("three")).unwrap();

    // Something else rewrites a row the next frame does not change.
    popup.buffer().set_lines(0, 1, ["CLOBBERED"]).unwrap();
    assert_eq!(lines(popup.buffer())[0], "CLOBBERED");

    // The next frame differs from the last one only in row 2, but the
    // renderer notices the buffer is not what it left and repaints it all.
    popup.set_content(&five_lines("THREE")).unwrap();
    assert_eq!(lines(popup.buffer()), vec!["one", "two", "THREE", "four", "five"]);
}

#[nvim_oxi::test]
fn a_locked_buffer_is_still_painted_and_stays_locked() {
    setup();
    let popup = Popup::new(PopupOptions { size: Size::cells(20, 5), ..Default::default() }).unwrap();
    popup.open().unwrap();
    popup.buffer().lock().unwrap();

    popup.set_content(&five_lines("three")).unwrap();
    popup.set_content(&five_lines("THREE")).unwrap();

    assert_eq!(lines(popup.buffer())[2], "THREE");
    assert!(!popup.buffer().get_option::<bool>("modifiable").unwrap());
    assert!(popup.buffer().get_option::<bool>("readonly").unwrap());
}

#[nvim_oxi::test]
fn one_key_press_is_one_repaint_however_many_signals_it_writes() {
    setup();
    let (a, b, c) = (Signal::new(0u32), Signal::new(0u32), Signal::new(0u32));
    let builds = Rc::new(Cell::new(0u32));
    let (ra, rb, rc, nb) = (a.clone(), b.clone(), c.clone(), builds.clone());
    let (_surface, buf) = surf(20, 4, move || {
        nb.set(nb.get() + 1);
        let (wa, wb, wc) = (ra.clone(), rb.clone(), rc.clone());
        col![
            Text::new(format!("{} {} {}", ra.get(), rb.get(), rc.get())),
            Button::new("bump").on_press(move || {
                wa.update(|n| *n += 1);
                wb.update(|n| *n += 10);
                wc.update(|n| *n += 100);
            }),
        ]
    });
    let before = builds.get();

    feed("<CR>");

    assert_eq!(trimmed(buf)[0], "1 10 100");
    assert_eq!(builds.get() - before, 1, "three writes, one rebuild");
}

#[nvim_oxi::test]
fn batch_groups_writes_made_outside_callbacks() {
    setup();
    let (a, b) = (Signal::new(1u32), Signal::new(2u32));
    let builds = Rc::new(Cell::new(0u32));
    let (ra, rb, nb) = (a.clone(), b.clone(), builds.clone());
    let (_surface, buf) = surf(20, 2, move || {
        nb.set(nb.get() + 1);
        Text::new(format!("{} + {}", ra.get(), rb.get()))
    });
    let before = builds.get();

    batch(|| {
        a.set(10);
        b.set(20);
        // Nothing is repainted until the batch ends.
        assert_eq!(trimmed(buf)[0], "1 + 2");
    });

    assert_eq!(trimmed(buf)[0], "10 + 20");
    assert_eq!(builds.get() - before, 1);
}

#[nvim_oxi::test]
fn a_toast_installs_no_key_table() {
    setup();
    notify("hello", NotifyOptions { timeout_ms: None, ..Default::default() }).unwrap();
    let toast = floats().into_iter().next().expect("the toast's window");
    let maps: i64 = lua_eval(&format!(
        "return #vim.api.nvim_buf_get_keymap({}, 'n')",
        toast.buffer().unwrap().id()
    ));
    assert_eq!(maps, 0);
    dismiss_all();
}

#[nvim_oxi::test]
fn a_stopped_timer_never_fires_again() {
    setup();
    let ticks = Rc::new(Cell::new(0u32));
    let t = ticks.clone();
    let timer = Timer::interval(5, move || t.set(t.get() + 1)).unwrap();
    {
        let ticks = ticks.clone();
        assert!(wait(1000, move || ticks.get() >= 2));
    }
    timer.stop().unwrap();
    let seen = ticks.get();
    wait(60, || false);
    assert_eq!(ticks.get(), seen);
}
