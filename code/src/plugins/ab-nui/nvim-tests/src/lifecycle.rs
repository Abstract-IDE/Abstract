//! `Surface` lifecycle, focus traversal, reactivity and multi-window layouts.

use std::cell::RefCell;
use std::rc::Rc;

use wp_ui::prelude::*;

use crate::support::*;

type Log = Rc<RefCell<Vec<String>>>;

fn log() -> Log {
    Rc::new(RefCell::new(Vec::new()))
}

fn opts(w: u32, h: u32) -> PopupOptions {
    PopupOptions { size: Size::cells(w, h), ..Default::default() }
}

fn three_buttons(l: &Log) -> impl Fn() -> Column + 'static {
    let l = l.clone();
    move || {
        let mk = |name: &'static str| {
            let l = l.clone();
            Button::new(name).on_press(move || l.borrow_mut().push(name.to_string()))
        };
        col![mk("a"), mk("b"), mk("c")]
    }
}

// ---- show / hide / reopen / toggle / close / drop -----------------------------

#[nvim_oxi::test]
fn surface_show_hide_reopen_toggle() {
    setup();
    let base = win_count();
    let (s, buf) = surf(10, 2, || Text::new("hi"));
    assert!(s.is_visible());
    assert_eq!(win_count(), base + 1);
    assert_eq!(current_win(), s.popup().window().unwrap());

    s.hide();
    assert!(!s.is_visible());
    assert_eq!(win_count(), base);
    assert!(buf.is_valid(), "the buffer survives hiding");

    s.reopen().unwrap();
    assert!(s.is_visible());
    assert_eq!(win_count(), base + 1);
    assert_eq!(trimmed(buf)[0], "hi");

    s.toggle().unwrap();
    assert!(!s.is_visible());
    s.toggle().unwrap();
    assert!(s.is_visible());
    // reopen on an already visible surface is a no-op.
    s.reopen().unwrap();
    assert_eq!(win_count(), base + 1);
}

#[nvim_oxi::test]
fn surface_hidden_still_reacts_and_reopens_current() {
    setup();
    let n = Signal::new(1);
    let n2 = n.clone();
    let (s, buf) = surf(10, 1, move || Text::new(format!("n={}", n2.get())));
    s.hide();
    n.set(2);
    assert_eq!(trimmed(buf), v(&["n=2"]));
    s.reopen().unwrap();
    assert_eq!(trimmed(buf), v(&["n=2"]));
    n.set(3);
    assert_eq!(trimmed(buf), v(&["n=3"]));
}

#[nvim_oxi::test]
fn surface_close_frees_buffer_window_and_stops_reacting() {
    setup();
    let base = win_count();
    let renders = Rc::new(RefCell::new(0));
    let n = Signal::new(0);
    let (n2, r2) = (n.clone(), renders.clone());
    let (s, buf) = surf(10, 1, move || {
        *r2.borrow_mut() += 1;
        Text::new(format!("{}", n2.get()))
    });
    let win = s.popup().window().unwrap();
    s.close();
    assert!(!buf.is_valid());
    assert!(!win.is_valid());
    assert_eq!(win_count(), base);
    assert!(!s.is_visible());
    let before = *renders.borrow();
    n.set(5);
    assert_eq!(*renders.borrow(), before, "the render effect was disposed");
}

#[nvim_oxi::test]
fn surface_drop_frees_buffer_and_window() {
    setup();
    let base = win_count();
    let (s, buf) = surf(10, 1, || Text::new("x"));
    let win = s.popup().window().unwrap();
    assert!(buf.is_valid());
    drop(s);
    assert!(!buf.is_valid());
    assert!(!win.is_valid());
    assert_eq!(win_count(), base);
}

#[nvim_oxi::test]
fn surface_clone_keeps_ui_alive_until_last_handle_drops() {
    setup();
    let (s, buf) = surf(10, 1, || Text::new("x"));
    let s2 = s.clone();
    drop(s);
    assert!(buf.is_valid());
    assert!(s2.is_visible());
    drop(s2);
    assert!(!buf.is_valid());
}

#[nvim_oxi::test]
fn surface_window_closed_externally_is_not_visible() {
    setup();
    let (s, buf) = surf(10, 1, || Text::new("x"));
    lua_run("vim.cmd('close')");
    assert!(!s.is_visible());
    assert!(buf.is_valid());
    s.reopen().unwrap();
    assert!(s.is_visible());
}

#[nvim_oxi::test]
fn surface_buffer_wiped_externally_stops_effect() {
    setup();
    let n = Signal::new(0);
    let count = Rc::new(RefCell::new(0));
    let (n2, c2) = (n.clone(), count.clone());
    let (s, buf) = surf(10, 1, move || {
        *c2.borrow_mut() += 1;
        Text::new(format!("{}", n2.get()))
    });
    buf.delete().unwrap();
    let before = *count.borrow();
    n.set(1);
    assert_eq!(*count.borrow(), before);
    drop(s); // freeing an already-wiped surface must not panic
}

// ---- focus ------------------------------------------------------------------

#[nvim_oxi::test]
fn tab_and_shift_tab_traversal_order() {
    setup();
    let l = log();
    let (_s, _buf) = surf(10, 3, three_buttons(&l));
    feed("<CR><Tab><CR><Tab><CR><Tab><CR>");
    assert_eq!(*l.borrow(), v(&["a", "b", "c", "a"]), "Tab wraps around");
    l.borrow_mut().clear();
    feed("<S-Tab><CR><S-Tab><CR>");
    assert_eq!(*l.borrow(), v(&["c", "b"]));
}

#[nvim_oxi::test]
fn initial_focus_selects_widget() {
    setup();
    let l = log();
    let s = Surface::float(opts(10, 3)).unwrap().initial_focus(2).show(three_buttons(&l)).unwrap();
    assert_eq!(hl_at(s.popup().buffer(), 2, 0).as_deref(), Some("AbNuiSelection"));
    assert_eq!(hl_at(s.popup().buffer(), 0, 0), None);
    feed("<CR>");
    assert_eq!(*l.borrow(), v(&["c"]));
}

#[nvim_oxi::test]
fn focus_wrap_false_stops_at_ends() {
    setup();
    let l = log();
    let s = Surface::float(opts(10, 3)).unwrap().focus_wrap(false).show(three_buttons(&l)).unwrap();
    feed("<S-Tab><CR>"); // stays on the first
    feed("<Tab><Tab><Tab><Tab><CR>"); // stops on the last
    assert_eq!(*l.borrow(), v(&["a", "c"]));
    drop(s);
}

#[nvim_oxi::test]
fn tab_is_consumed_by_a_widget_that_wants_it() {
    setup();
    // A TextField doesn't take Tab, so Tab moves focus away from it.
    let st = FieldState::new("");
    let st2 = st.clone();
    let (_s, buf) = surf(10, 2, move || col![TextField::new(&st2), Button::new("b")]);
    feed("<Tab>");
    assert_eq!(hl_at(buf, 1, 0).as_deref(), Some("AbNuiSelection"));
    // Typing now goes to nobody in the field.
    feed("zz");
    assert_eq!(st.value.get_untracked(), "");
}

#[nvim_oxi::test]
fn keys_go_to_the_focused_widget_only() {
    setup();
    let a = FieldState::new("");
    let b = FieldState::new("");
    let (a2, b2) = (a.clone(), b.clone());
    let (_s, _buf) = surf(10, 2, move || col![TextField::new(&a2), TextField::new(&b2)]);
    feed("one<Tab>two<S-Tab>!");
    assert_eq!(a.value.get_untracked(), "one!");
    assert_eq!(b.value.get_untracked(), "two");
}

#[nvim_oxi::test]
fn surface_on_key_custom_binding() {
    setup();
    let l = log();
    let (s, _buf) = surf(10, 1, || Text::new("x"));
    let l2 = l.clone();
    s.on_key("<F5>", move || l2.borrow_mut().push("f5".into())).unwrap();
    feed("<F5><F5>");
    assert_eq!(*l.borrow(), v(&["f5", "f5"]));
}

#[nvim_oxi::test]
fn esc_hides_when_nothing_consumes_it() {
    setup();
    let base = win_count();
    let l = log();
    let (s, _buf) = surf(10, 3, three_buttons(&l));
    feed("<Esc>");
    assert!(!s.is_visible());
    assert_eq!(win_count(), base);
}

#[nvim_oxi::test]
fn esc_in_keyed_handler_can_be_consumed() {
    setup();
    let l = log();
    let l2 = l.clone();
    let (s, _buf) = surf(10, 1, move || {
        let l2 = l2.clone();
        Keyed::new(Button::new("b")).on_key(Key::Esc, move || l2.borrow_mut().push("esc".into()))
    });
    feed("<Esc>");
    assert!(s.is_visible());
    assert_eq!(*l.borrow(), v(&["esc"]));
}

// ---- reactivity ---------------------------------------------------------------

#[nvim_oxi::test]
fn signal_and_store_reads_repaint() {
    setup();
    let sig = Signal::new(1);
    let store = Store::new(10);
    let (s2, t2) = (sig.clone(), store.clone());
    let (_s, buf) = surf(12, 1, move || Text::new(format!("{} {}", s2.get(), t2.get())));
    assert_eq!(trimmed(buf), v(&["1 10"]));
    sig.set(2);
    assert_eq!(trimmed(buf), v(&["2 10"]));
    store.set(20);
    assert_eq!(trimmed(buf), v(&["2 20"]));
    store.update(|n| *n += 1);
    sig.update(|n| *n += 1);
    assert_eq!(trimmed(buf), v(&["3 21"]));
}

#[nvim_oxi::test]
fn two_signals_changed_back_to_back_end_in_final_frame() {
    setup();
    let a = Signal::new(0);
    let b = Signal::new(0);
    let frames = Rc::new(RefCell::new(Vec::<String>::new()));
    let (a2, b2, f2) = (a.clone(), b.clone(), frames.clone());
    let (_s, buf) = surf(12, 1, move || {
        let s = format!("a={} b={}", a2.get(), b2.get());
        f2.borrow_mut().push(s.clone());
        Text::new(s)
    });
    a.set(1);
    b.set(2);
    assert_eq!(trimmed(buf), v(&["a=1 b=2"]));
    assert_eq!(frames.borrow().last().unwrap(), "a=1 b=2");
}

#[nvim_oxi::test]
fn handler_updating_two_signals_leaves_consistent_frame() {
    setup();
    let a = Signal::new(0);
    let b = Signal::new(0);
    let (a2, b2) = (a.clone(), b.clone());
    let (_s, buf) = surf(12, 2, move || {
        let (a3, b3) = (a2.clone(), b2.clone());
        col![
            Text::new(format!("a={} b={}", a2.get(), b2.get())),
            Button::new("go").on_press(move || {
                a3.update(|n| *n += 1);
                b3.update(|n| *n += 10);
            })
        ]
    });
    feed("<CR><CR>");
    assert_eq!(trimmed(buf)[0], "a=2 b=20");
}

#[nvim_oxi::test]
fn stale_text_highlights_are_removed_on_repaint() {
    setup();
    let styled = Signal::new(true);
    let s2 = styled.clone();
    let (_s, buf) = surf(10, 1, move || if s2.get() { Text::new("hello").fg("Title") } else { Text::new("hello") });
    assert_eq!(highlights(buf), vec![(0, 0, 5, "Title".into())]);
    styled.set(false);
    assert_eq!(highlights(buf), vec![]);
    styled.set(true);
    assert_eq!(highlights(buf), vec![(0, 0, 5, "Title".into())]);
}

#[nvim_oxi::test]
fn highlight_moves_with_content_not_left_behind() {
    setup();
    let pos = Signal::new(0usize);
    let p2 = pos.clone();
    let (_s, buf) = surf(10, 1, move || {
        let p = p2.get();
        let mut t = Text::from_line(Line::empty());
        for (i, ch) in "abcdef".chars().enumerate() {
            t = if i == p { t.span(ch.to_string(), "ErrorMsg") } else { t.raw(ch.to_string()) };
        }
        t
    });
    assert_eq!(highlights(buf), vec![(0, 0, 1, "ErrorMsg".into())]);
    pos.set(3);
    assert_eq!(highlights(buf), vec![(0, 3, 4, "ErrorMsg".into())]);
}

// ---- split host ---------------------------------------------------------------

#[nvim_oxi::test]
fn surface_split_opens_a_real_split() {
    setup();
    let base = win_count();
    let mut cfg = SplitConfig::new(SplitDir::Right);
    cfg.size = Some(20);
    let s = Surface::split(cfg).unwrap().show(|| col![Text::new("hello"), Button::new("b")]).unwrap();
    let buf = s.popup().buffer();
    let win = s.popup().window().unwrap();
    assert_eq!(win_count(), base + 1);
    assert_eq!(win.config().unwrap().relative, "");
    assert_eq!(win.width().unwrap(), 20);
    assert_eq!(trimmed(buf)[0], "hello");
    assert_eq!(trimmed(buf)[1], "[ b ]");
    assert_eq!(hl_at(buf, 1, 0).as_deref(), Some("AbNuiSelection"));
    s.hide();
    assert_eq!(win_count(), base);
    s.reopen().unwrap();
    assert_eq!(win_count(), base + 1);
    assert_eq!(trimmed(buf)[0], "hello");
    s.close();
    assert_eq!(win_count(), base);
}

// ---- resize ---------------------------------------------------------------------

#[nvim_oxi::test]
fn surface_relayouts_after_editor_resize() {
    setup();
    let (cols0, _) = wp_ui::geometry::editor_size().unwrap();
    let s = Surface::float(PopupOptions { size: Size::ratio(0.5, 0.5), ..Default::default() })
        .unwrap()
        .show(|| Center::new(Text::new("mid")))
        .unwrap();
    let win = s.popup().window().unwrap();
    assert_eq!(win.config().unwrap().width, (cols0 as f64 * 0.5).round() as i64);
    resize(100, 40);
    let c = win.config().unwrap();
    assert_eq!(c.width, 50);
    assert_eq!(c.height, 20);
    // Centered text is re-measured at the new size: 50 wide -> col 23, row 9.
    let rows = trimmed(s.popup().buffer());
    assert_eq!(rows.len(), 20);
    assert_eq!(rows[9], format!("{}mid", " ".repeat(23)));
}

// ---- Layout / Pane --------------------------------------------------------------

fn pane_surface(label: &'static str) -> Surface {
    Surface::float(opts(10, 3)).unwrap().show(move || Text::new(label)).unwrap()
}

#[nvim_oxi::test]
fn layout_row_of_two_surfaces_positions_and_sizes() {
    setup();
    let base = win_count();
    let (a, b) = (pane_surface("left"), pane_surface("right"));
    let layout = Layout::new(
        Size::cells(60, 20),
        Position::At { row: 2.0, col: 4.0 },
        Pane::row(vec![Pane::surface(a.clone(), Dim::Ratio(0.5)), Pane::surface(b.clone(), Dim::Ratio(0.5))]),
    );
    layout.open().unwrap();
    assert!(layout.is_visible());
    assert_eq!(win_count(), base + 2);
    let ca = a.popup().window().unwrap().config().unwrap();
    let cb = b.popup().window().unwrap().config().unwrap();
    // Each pane is 30 wide including its border (text area 28 x 18).
    assert_eq!((ca.width, ca.height, ca.row, ca.col), (28, 18, 2.0, 4.0));
    assert_eq!((cb.width, cb.height, cb.row, cb.col), (28, 18, 2.0, 34.0));
    assert_eq!(trimmed(a.popup().buffer())[0], "left");
    assert_eq!(trimmed(b.popup().buffer())[0], "right");
}

#[nvim_oxi::test]
fn layout_nested_panes_and_close_toggle() {
    setup();
    let base = win_count();
    let (a, b, c) = (pane_surface("a"), pane_surface("b"), pane_surface("c"));
    let layout = Layout::new(
        Size::cells(60, 20),
        Position::At { row: 0.0, col: 0.0 },
        Pane::row(vec![
            Pane::surface(a.clone(), Dim::Ratio(0.4)),
            Pane::col(vec![Pane::surface(b.clone(), Dim::Ratio(0.5)), Pane::surface(c.clone(), Dim::Ratio(0.5))])
                .sized(Dim::Ratio(0.6)),
        ]),
    );
    layout.open().unwrap();
    assert_eq!(win_count(), base + 3);
    let cfg = |s: &Surface| s.popup().window().unwrap().config().unwrap();
    let (ca, cb, cc) = (cfg(&a), cfg(&b), cfg(&c));
    assert_eq!((ca.width, ca.height, ca.col), (22, 18, 0.0));
    assert_eq!((cb.width, cb.height, cb.row, cb.col), (34, 8, 0.0, 24.0));
    assert_eq!((cc.width, cc.height, cc.row, cc.col), (34, 8, 10.0, 24.0));

    layout.close();
    assert!(!layout.is_visible());
    assert_eq!(win_count(), base);
    layout.toggle().unwrap();
    assert!(layout.is_visible());
    assert_eq!(win_count(), base + 3);
    assert_eq!(cfg(&b).col, 24.0);
}

#[nvim_oxi::test]
fn layout_relayouts_after_editor_resize() {
    setup();
    let (a, b) = (pane_surface("l"), pane_surface("r"));
    let layout = Layout::new(
        Size::ratio(0.5, 0.5),
        Position::At { row: 0.0, col: 0.0 },
        Pane::row(vec![Pane::surface(a.clone(), Dim::Ratio(0.5)), Pane::surface(b.clone(), Dim::Ratio(0.5))]),
    );
    layout.open().unwrap();
    resize(100, 40);
    let ca = a.popup().window().unwrap().config().unwrap();
    let cb = b.popup().window().unwrap().config().unwrap();
    // 100x40 editor -> 50x20 layout -> two 25-wide panes (23 + border).
    assert_eq!((ca.width, ca.height, ca.col), (23, 18, 0.0));
    assert_eq!((cb.width, cb.height, cb.col), (23, 18, 25.0));
}

#[nvim_oxi::test]
fn panes_share_state_through_signals() {
    setup();
    let query = Signal::new(String::new());
    let (q1, q2) = (query.clone(), query.clone());
    let list = Surface::float(opts(10, 3)).unwrap().show(move || Text::new(format!("q:{}", q1.get()))).unwrap();
    let preview = Surface::float(opts(10, 3))
        .unwrap()
        .show(move || Text::new(format!("len:{}", q2.get().len())))
        .unwrap();
    let layout = Layout::new(
        Size::cells(40, 10),
        Position::Center,
        Pane::row(vec![Pane::surface(list.clone(), Dim::Ratio(0.5)), Pane::surface(preview.clone(), Dim::Ratio(0.5))]),
    );
    layout.open().unwrap();
    query.set("abc".into());
    assert_eq!(trimmed(list.popup().buffer())[0], "q:abc");
    assert_eq!(trimmed(preview.popup().buffer())[0], "len:3");
}
