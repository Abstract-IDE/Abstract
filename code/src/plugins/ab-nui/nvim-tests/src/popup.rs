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
