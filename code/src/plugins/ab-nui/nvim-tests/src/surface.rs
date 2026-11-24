use wp_ui::prelude::*;

use crate::support::*;

#[nvim_oxi::test]
fn surface_counter_reacts_to_button_presses() {
    setup();
    let count = Signal::new(0i64);
    let c = count.clone();
    let surface = Surface::float(PopupOptions { size: Size::cells(30, 5), ..Default::default() })
        .unwrap()
        .show(move || {
            let dec = c.clone();
            let inc = c.clone();
            col![
                Text::new(format!("count: {}", c.get())),
                row![
                    Button::new("-1").on_press(move || dec.update(|n| *n -= 1)),
                    Spacer::new(),
                    Button::new("+1").on_press(move || inc.update(|n| *n += 1)),
                ],
            ]
        })
        .unwrap();
    let buf = surface.popup().buffer();
    assert!(surface.is_visible());
    assert_eq!(trimmed(buf)[0], "count: 0");

    // The first button ("-1") starts focused.
    feed("<CR>");
    assert_eq!(count.get_untracked(), -1);
    assert_eq!(trimmed(buf)[0], "count: -1");

    // Tab moves focus to "+1".
    feed("<Tab><CR><CR>");
    assert_eq!(count.get_untracked(), 1);
    assert_eq!(trimmed(buf)[0], "count: 1");

    // Esc hides without destroying state; reopen brings it back.
    feed("<Esc>");
    assert!(!surface.is_visible());
    surface.reopen().unwrap();
    assert!(surface.is_visible());
    assert_eq!(trimmed(buf)[0], "count: 1");

    surface.close();
    assert!(!buf.is_valid());
}
