use std::sync::atomic::{AtomicBool, Ordering};

use wp_ui::prelude::*;

use crate::support::*;

#[nvim_oxi::test]
fn popup_renders_styled_lines() {
    setup();
    let popup = Popup::new(PopupOptions { size: Size::cells(20, 3), ..Default::default() }).unwrap();
    popup.open().unwrap();
    assert!(popup.is_open());

    popup
        .set_content(&[
            Line::raw("plain"),
            Line::from_spans([Span::raw("ab "), Span::hl("red", "ErrorMsg"), Span::raw(" cd")]),
        ])
        .unwrap();

    assert_eq!(lines(popup.buffer()), vec!["plain", "ab red cd"]);
    assert_eq!(highlights(popup.buffer()), vec![(1, 3, 6, "ErrorMsg".to_string())]);

    popup.close();
    assert!(!popup.is_open());
}

#[nvim_oxi::test]
fn popup_render_reactive_follows_signal() {
    setup();
    let count = Signal::new(0i64);
    let popup = Popup::new(PopupOptions { size: Size::cells(20, 3), ..Default::default() }).unwrap();
    popup.open().unwrap();
    let c = count.clone();
    popup.render_reactive(move || vec![Line::raw(format!("count: {}", c.get()))]);
    assert_eq!(lines(popup.buffer()), vec!["count: 0"]);

    count.set(7);
    assert_eq!(lines(popup.buffer()), vec!["count: 7"]);

    // After close the effect is disposed: further writes do nothing (and the
    // wiped buffer is not touched).
    popup.close();
    count.set(8);
}

#[nvim_oxi::test]
fn popup_hide_does_not_trip_its_own_winclosed_handler() {
    setup();
    let popup = Popup::new(PopupOptions { size: Size::cells(20, 3), ..Default::default() }).unwrap();
    popup.open().unwrap();

    // Closing the window fires the popup's `WinClosed` handler; `hide` must not
    // still be holding the window slot when it does. That panic would be caught
    // at the Lua boundary and swallowed, so watch for it with a hook.
    static PANICKED: AtomicBool = AtomicBool::new(false);
    let previous = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| PANICKED.store(true, Ordering::SeqCst)));
    popup.hide();
    std::panic::set_hook(previous);

    assert!(!PANICKED.load(Ordering::SeqCst), "a callback panicked while hiding");
    assert!(!popup.is_open());
    assert!(floats().is_empty());

    // ...and it can come back.
    popup.open().unwrap();
    assert!(popup.is_open());
    popup.close();
}

#[nvim_oxi::test]
fn popup_ratio_position_places_within_free_space_and_follows_resize() {
    setup();
    resize(100, 30);
    let opts = |row, col| PopupOptions {
        size: Size::cells(20, 10),
        position: Position::Ratio { row, col },
        ..Default::default()
    };

    // Bottom-right corner of the free space around the bordered box (22x12)
    // in the usable area (100x29: the command line is not usable).
    let popup = Popup::new(opts(1.0, 1.0)).unwrap();
    popup.open().unwrap();
    let cfg = popup.window().unwrap().config().unwrap();
    assert_eq!((cfg.row, cfg.col), (17.0, 78.0));

    // Re-anchored on resize, unlike `Position::At`.
    resize(60, 20);
    let cfg = popup.window().unwrap().config().unwrap();
    assert_eq!((cfg.row, cfg.col), (7.0, 38.0));
    popup.close();

    // Fractions in between are floored; out-of-range values are clamped.
    resize(100, 30);
    let popup = Popup::new(opts(0.5, 2.0)).unwrap();
    popup.open().unwrap();
    let cfg = popup.window().unwrap().config().unwrap();
    assert_eq!((cfg.row, cfg.col), (8.0, 78.0));
    popup.close();
}

#[nvim_oxi::test]
fn popup_with_buffer_shows_a_foreign_buffer_and_leaves_it_alone() {
    setup();
    let buf = Buffer::scratch().unwrap();
    buf.set_all(["mine"]).unwrap();
    buf.set_option("bufhidden", "hide").unwrap();

    let popup = Popup::with_buffer(buf, PopupOptions { size: Size::cells(20, 3), ..Default::default() }).unwrap();
    assert_eq!(popup.buffer(), buf);
    popup.open().unwrap();
    let win = popup.window().unwrap();
    assert_eq!(win.buffer().unwrap(), buf);

    // The popup themes the window but leaves the buffer's and the window's own
    // options to the caller.
    assert_eq!(buf.get_option::<String>("bufhidden").unwrap(), "hide");
    assert!(!win.get_option::<bool>("cursorline").unwrap());
    assert!(win.get_option::<String>("winhighlight").unwrap().contains("AbNuiNormal"));

    // Closing the popup never deletes the buffer.
    popup.close();
    assert!(!popup.is_open());
    assert!(buf.is_valid());
    assert_eq!(lines(buf), vec!["mine"]);

    // An invalid buffer is rejected up front.
    buf.delete().unwrap();
    assert!(Popup::with_buffer(buf, PopupOptions::default()).is_err());
}

#[nvim_oxi::test]
fn popup_set_title_updates_open_and_hidden_popups() {
    setup();
    let popup =
        Popup::new(PopupOptions { size: Size::cells(20, 3), title: Some(" one ".into()), ..Default::default() })
            .unwrap();
    popup.open().unwrap();
    let title = |p: &Popup| -> String {
        lua_eval(&format!("return vim.api.nvim_win_get_config({}).title[1][1]", p.window().unwrap().id()))
    };
    assert_eq!(title(&popup), " one ");

    popup.set_title(" two ").unwrap();
    assert_eq!(title(&popup), " two ");

    // Set while hidden: used on the next open.
    popup.hide();
    popup.set_title(" three ").unwrap();
    popup.open().unwrap();
    assert_eq!(title(&popup), " three ");
    popup.close();
}
