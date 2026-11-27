//! Regression tests for view-runtime bugs: `Surface` remount after `:bdelete`,
//! focus-key tracking, toast re-anchoring/overflow/re-entrancy, hiding the last
//! window, resize filtering, `<C-h>` as Backspace, `Surface::closer` and the
//! closed/mounted errors.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use wp_ui::prelude::*;

use crate::support::*;

type Log = Rc<RefCell<Vec<String>>>;

fn log() -> Log {
    Rc::new(RefCell::new(Vec::new()))
}

fn sticky() -> NotifyOptions {
    NotifyOptions { timeout_ms: None, ..Default::default() }
}

fn keymap_count(buf: Buffer) -> i64 {
    lua_eval(&format!("return #vim.api.nvim_buf_get_keymap({}, 'n')", buf.id()))
}

fn focus_first_normal_window() {
    lua_run("vim.api.nvim_set_current_win(vim.fn.win_getid(1))");
}

// ---- 1. `:bdelete` unloads the buffer ---------------------------------------------

#[nvim_oxi::test]
fn reopen_after_bdelete_restores_keys_content_and_options() {
    setup();
    let l = log();
    let l2 = l.clone();
    let (s, buf) = surf(10, 2, move || {
        let l = l2.clone();
        col![Text::new("hi"), Button::new("b").on_press(move || l.borrow_mut().push("b".into()))]
    });
    let maps = keymap_count(buf);
    assert!(maps > 0);
    ex(&format!("bdelete {}", buf.id()));
    assert!(!s.is_visible());

    s.reopen().unwrap();
    assert!(s.is_visible());
    assert_eq!(keymap_count(buf), maps, "the key table is reinstalled");
    assert_eq!(trimmed(buf), v(&["hi", "[ b ]"]), "the frame is repainted in full");
    let bufhidden: String = lua_eval(&format!("return vim.bo[{}].bufhidden", buf.id()));
    assert_eq!(bufhidden, "hide");
    feed("<Tab><CR>");
    assert_eq!(*l.borrow(), v(&["b"]));
}

#[nvim_oxi::test]
fn reopen_after_bdelete_restores_surface_on_key_maps() {
    setup();
    let hits = Rc::new(Cell::new(0));
    let (s, buf) = surf(10, 1, || Text::new("x"));
    let h = hits.clone();
    s.on_key("Q", move || h.set(h.get() + 1)).unwrap();
    ex(&format!("bdelete {}", buf.id()));
    s.reopen().unwrap();
    feed("Q");
    assert_eq!(hits.get(), 1);
}

// ---- 2. focused_key follows the focused widget --------------------------------------

fn keyed_ab(l: &Log, extra: &Signal<bool>, show_b: &Signal<bool>) -> impl Fn() -> Column + 'static {
    let (l, extra, show_b) = (l.clone(), extra.clone(), show_b.clone());
    move || {
        let mut kids: Vec<Element> = Vec::new();
        let mk = |name: &'static str| {
            let l = l.clone();
            Keyed::new(Button::new(name).on_press(move || l.borrow_mut().push(name.into()))).focus_key(name)
        };
        if extra.get() {
            kids.push(Box::new(mk("new")));
        }
        kids.push(Box::new(mk("a")));
        if show_b.get() {
            kids.push(Box::new(mk("b")));
        }
        Column::new(kids)
    }
}

#[nvim_oxi::test]
fn initial_focus_key_is_tracked_without_tab() {
    setup();
    let (l, extra, show_b) = (log(), Signal::new(false), Signal::new(true));
    let (_s, _buf) = surf(12, 3, keyed_ab(&l, &extra, &show_b));
    // Focused on "a" from the first paint; inserting before it must not
    // move the keys to the new widget.
    extra.set(true);
    feed("<CR>");
    assert_eq!(*l.borrow(), v(&["a"]));
}

#[nvim_oxi::test]
fn stale_focus_key_is_dropped_when_its_widget_disappears() {
    setup();
    let (l, extra, show_b) = (log(), Signal::new(false), Signal::new(true));
    let (_s, _buf) = surf(12, 3, keyed_ab(&l, &extra, &show_b));
    feed("<Tab>"); // focus "b"
    show_b.set(false); // focus clamps to "a"
    show_b.set(true); // "b" is back: focus stays on "a"
    feed("<CR>");
    assert_eq!(*l.borrow(), v(&["a"]));
}

// ---- 3-5. toasts --------------------------------------------------------------------

#[nvim_oxi::test]
fn toasts_reanchor_on_editor_resize() {
    setup();
    resize(80, 24);
    let _h = notify("hello", sticky()).unwrap();
    assert_eq!(floats()[0].config().unwrap().col, (80 - 40 - 3) as f64);
    resize(120, 24);
    assert_eq!(floats()[0].config().unwrap().col, (120 - 40 - 3) as f64);
    dismiss_all();
}

#[nvim_oxi::test]
fn toasts_that_do_not_fit_are_hidden_until_there_is_room() {
    setup();
    resize(80, 12);
    let cmdheight: i64 = lua_eval("return vim.o.cmdheight");
    let bottom = 12 - cmdheight;
    let handles: Vec<NotifyHandle> = (0..5).map(|i| notify(format!("t{i}"), sticky()).unwrap()).collect();
    let fits = || {
        floats().into_iter().all(|w| {
            let c = w.config().unwrap();
            c.row as i64 + c.height + 2 <= bottom
        })
    };
    assert!(fits(), "no toast extends past the bottom");
    // Newest on top: t4, t3, t2 fit (rows 1, 4, 7); t1 and t0 wait.
    let shown = |w: &Window| trimmed(w.buffer().unwrap())[0].clone();
    let mut texts: Vec<String> = floats().iter().map(shown).collect();
    texts.sort();
    assert_eq!(texts, v(&["t2", "t3", "t4"]));
    // Dismissing one makes room for the next older one.
    handles[4].dismiss();
    assert!(fits());
    let mut texts: Vec<String> = floats().iter().map(shown).collect();
    texts.sort();
    assert_eq!(texts, v(&["t1", "t2", "t3"]));
    // The revealed toast keeps its level styling.
    let t1 = floats().into_iter().find(|w| shown(w) == "t1").unwrap();
    let hl: String = lua_eval(&format!("return vim.wo[{}].winhighlight", t1.id()));
    assert!(hl.contains("AbNuiInfo"), "{hl}");
    // Growing the editor reveals the rest.
    resize(80, 40);
    assert_eq!(floats().len(), 4);
    dismiss_all();
    assert!(floats().is_empty());
}

#[nvim_oxi::test]
fn notify_from_an_autocmd_fired_by_dismissal_is_not_lost() {
    setup();
    let first = notify("first", sticky()).unwrap();
    let fired = Rc::new(Cell::new(false));
    let ok = Rc::new(Cell::new(false));
    let (f, o) = (fired.clone(), ok.clone());
    wp_ui::nvim::autocmd(&["BufWipeout"], None, None, move |_| {
        if !f.replace(true) {
            o.set(notify("from autocmd", sticky()).is_ok());
        }
    })
    .unwrap();
    first.dismiss();
    assert!(fired.get());
    assert!(ok.get(), "notify() inside the autocmd succeeds");
    let texts: Vec<String> = floats().into_iter().map(|w| trimmed(w.buffer().unwrap())[0].clone()).collect();
    assert_eq!(texts, v(&["from autocmd"]));
    dismiss_all();
}

#[nvim_oxi::test]
fn dismiss_all_tolerates_reentrant_notify() {
    setup();
    let _a = notify("a", sticky()).unwrap();
    let _b = notify("b", sticky()).unwrap();
    let fired = Rc::new(Cell::new(false));
    let f = fired.clone();
    wp_ui::nvim::autocmd(&["BufWipeout"], None, None, move |_| {
        if !f.replace(true) {
            let _ = notify("late", sticky());
        }
    })
    .unwrap();
    dismiss_all();
    assert!(fired.get());
    let texts: Vec<String> = floats().into_iter().map(|w| trimmed(w.buffer().unwrap())[0].clone()).collect();
    assert_eq!(texts, v(&["late"]));
    dismiss_all();
}

// ---- 6. Esc on a split surface that is the last window ---------------------------

#[nvim_oxi::test]
fn esc_on_last_window_split_keeps_state_consistent() {
    setup();
    let mut cfg = SplitConfig::new(SplitDir::Right);
    cfg.size = Some(20);
    let s = Surface::split(cfg).unwrap().show(|| Text::new("only")).unwrap();
    let win = s.popup().window().unwrap();
    lua_run(&format!("vim.api.nvim_set_current_win({})", win.id()));
    ex("only");
    assert_eq!(win_count(), 1);
    feed("<Esc>");
    assert!(win.is_valid(), "the last window cannot close");
    assert!(s.is_visible(), "so the surface is still visible");
    s.reopen().unwrap();
    assert_eq!(win_count(), 1, "reopen does not open a second window");
    s.toggle().unwrap();
    assert_eq!(win_count(), 1);
}

// ---- 7. resize filtering -------------------------------------------------------------

fn counting(n: &Rc<Cell<u32>>) -> impl Fn() -> Center + 'static {
    let n = n.clone();
    move || {
        n.set(n.get() + 1);
        Center::new(Text::new("x"))
    }
}

#[nvim_oxi::test]
fn hidden_surface_does_not_rebuild_on_resize() {
    setup();
    let n = Rc::new(Cell::new(0));
    let s = Surface::float(PopupOptions { size: Size::ratio(0.5, 0.5), ..Default::default() })
        .unwrap()
        .show(counting(&n))
        .unwrap();
    s.hide();
    let before = n.get();
    resize(100, 40);
    ex("silent! doautocmd WinResized");
    assert_eq!(n.get(), before, "no rebuild while hidden");
    s.reopen().unwrap();
    // Re-measured at the new size on reopen: 50x20 -> "x" at row 9, col 24.
    assert_eq!(trimmed(s.popup().buffer())[9], format!("{}x", " ".repeat(24)));
}

#[nvim_oxi::test]
fn float_surface_ignores_unrelated_window_resizes() {
    setup();
    let n = Rc::new(Cell::new(0));
    let _s = Surface::float(PopupOptions { size: Size::cells(10, 3), ..Default::default() })
        .unwrap()
        .show(counting(&n))
        .unwrap();
    focus_first_normal_window();
    ex("vsplit");
    ex("vertical resize 20");
    let before = n.get();
    ex("silent! doautocmd WinResized");
    assert_eq!(n.get(), before, "dragging an unrelated split does not rebuild the float");
}

#[nvim_oxi::test]
fn split_surface_rebuilds_when_its_own_window_resizes() {
    setup();
    let n = Rc::new(Cell::new(0));
    let mut cfg = SplitConfig::new(SplitDir::Right);
    cfg.size = Some(20);
    let s = Surface::split(cfg).unwrap().show(counting(&n)).unwrap();
    let win = s.popup().window().unwrap();
    let before = n.get();
    lua_run(&format!("vim.api.nvim_win_set_width({}, 30)", win.id()));
    ex("silent! doautocmd WinResized");
    assert!(n.get() > before);
    let h = win.height().unwrap() as usize;
    let row = trimmed(s.popup().buffer())[(h - 1) / 2].clone();
    assert_eq!(row, format!("{}x", " ".repeat(14)));
}

// ---- 8. <C-h> is Backspace -------------------------------------------------------------

#[nvim_oxi::test]
fn ctrl_h_acts_as_backspace() {
    setup();
    let st = FieldState::new("");
    let st2 = st.clone();
    let (_s, _buf) = surf(10, 1, move || TextField::new(&st2));
    feed("hello<C-h>");
    assert_eq!(st.value.get_untracked(), "hell");
}

// ---- 9. Surface::closer -------------------------------------------------------------------

#[nvim_oxi::test]
fn closer_closes_from_inside_the_tree() {
    setup();
    let base = win_count();
    let s = Surface::float(PopupOptions { size: Size::cells(10, 1), ..Default::default() }).unwrap();
    let close = Rc::new(s.closer());
    let s = s
        .show(move || {
            let c = close.clone();
            Button::new("Close").on_press(move || c())
        })
        .unwrap();
    let buf = s.popup().buffer();
    feed("<CR>");
    assert_eq!(win_count(), base);
    assert!(!buf.is_valid());
    assert!(s.reopen().is_err());
}

#[nvim_oxi::test]
fn closer_does_not_keep_the_surface_alive() {
    setup();
    let base = win_count();
    let s = Surface::float(PopupOptions { size: Size::cells(10, 1), ..Default::default() }).unwrap();
    let close = Rc::new(s.closer());
    let s = s
        .show(move || {
            let c = close.clone();
            Button::new("Close").on_press(move || c())
        })
        .unwrap();
    let buf = s.popup().buffer();
    drop(s);
    assert_eq!(win_count(), base);
    assert!(!buf.is_valid(), "dropping the last handle frees it");
}

// ---- 10. show twice / use after close --------------------------------------------------

#[nvim_oxi::test]
fn show_twice_is_an_error() {
    setup();
    let (s, _buf) = surf(10, 1, || Text::new("a"));
    let err = s.clone().show(|| Text::new("b")).err().expect("second show fails");
    assert!(err.to_string().contains("already"), "{err}");
}

#[nvim_oxi::test]
fn reopen_and_toggle_after_close_report_closed() {
    setup();
    let (s, _buf) = surf(10, 1, || Text::new("a"));
    s.close();
    let err = s.reopen().unwrap_err();
    assert_eq!(err.to_string(), "surface is closed");
    let err = s.toggle().unwrap_err();
    assert_eq!(err.to_string(), "surface is closed");
    s.hide(); // harmless
    s.close(); // idempotent
}
