//! Interactive widgets driven through real key presses.

use std::cell::RefCell;
use std::rc::Rc;

use wp_ui::prelude::*;

use crate::support::*;

type Log = Rc<RefCell<Vec<String>>>;

fn log() -> Log {
    Rc::new(RefCell::new(Vec::new()))
}

const SEL: &str = "AbNuiSelection";

// ---- Button -----------------------------------------------------------------

#[nvim_oxi::test]
fn button_focus_highlight_moves_with_tab() {
    setup();
    let (_s, buf) = surf(12, 2, || col![Button::new("A"), Button::new("B")]);
    assert_eq!(trimmed(buf), v(&["[ A ]", "[ B ]"]));
    assert_eq!(highlights(buf), vec![(0, 0, 5, SEL.into())]);
    feed("<Tab>");
    // The old position is no longer highlighted.
    assert_eq!(highlights(buf), vec![(1, 0, 5, SEL.into())]);
    feed("<Tab>");
    assert_eq!(highlights(buf), vec![(0, 0, 5, SEL.into())]);
    feed("<S-Tab>");
    assert_eq!(highlights(buf), vec![(1, 0, 5, SEL.into())]);
}

#[nvim_oxi::test]
fn button_press_with_enter_and_space_and_custom_style() {
    setup();
    let l = log();
    let l1 = l.clone();
    let (_s, buf) = surf(12, 2, move || {
        let l1 = l1.clone();
        col![Button::new("Go").on_press(move || l1.borrow_mut().push("go".into())), Button::new("Other").fg("ErrorMsg"),]
    });
    feed("<CR>");
    feed("<Space>");
    feed("x"); // not a press key
    assert_eq!(*l.borrow(), v(&["go", "go"]));
    // Unfocused button with a custom group.
    assert_eq!(hl_at(buf, 1, 0).as_deref(), Some("ErrorMsg"));
    feed("<Tab>");
    assert_eq!(hl_at(buf, 1, 0).as_deref(), Some(SEL));
    assert_eq!(hl_at(buf, 0, 0), None);
}

// ---- Checkbox / Toggle / RadioGroup -----------------------------------------

#[nvim_oxi::test]
fn checkbox_toggles_signal_and_callback() {
    setup();
    let on = Signal::new(false);
    let l = log();
    let (s2, l2) = (on.clone(), l.clone());
    let (_s, buf) = surf(14, 2, move || {
        let l2 = l2.clone();
        col![Checkbox::new("Enable", s2.clone()).on_change(move |b| l2.borrow_mut().push(b.to_string()))]
    });
    assert_eq!(trimmed(buf)[0], "[ ] Enable");
    assert_eq!(hl_at(buf, 0, 0).as_deref(), Some(SEL));
    feed("<CR>");
    assert!(on.get_untracked());
    assert_eq!(trimmed(buf)[0], "[x] Enable");
    feed("<Space>");
    assert!(!on.get_untracked());
    assert_eq!(trimmed(buf)[0], "[ ] Enable");
    assert_eq!(*l.borrow(), v(&["true", "false"]));
    // External writes repaint as well.
    on.set(true);
    assert_eq!(trimmed(buf)[0], "[x] Enable");
}

#[nvim_oxi::test]
fn toggle_renders_pill_and_flips() {
    setup();
    let on = Signal::new(false);
    let s2 = on.clone();
    let (_s, buf) = surf(14, 1, move || Toggle::new("Dark", s2.clone()));
    assert_eq!(trimmed(buf), v(&["●── Dark"]));
    assert_eq!(hl_at(buf, 0, 0).as_deref(), Some("AbNuiMuted"));
    feed("<CR>");
    assert!(on.get_untracked());
    assert_eq!(trimmed(buf), v(&["──● Dark"]));
    assert_eq!(hl_at(buf, 0, 0).as_deref(), Some("AbNuiSuccess"));
    // The label (not the pill) carries the focus highlight.
    assert_eq!(hl_at(buf, 0, 12).as_deref(), Some(SEL));
}

#[nvim_oxi::test]
fn radio_group_vertical_navigation_wraps() {
    setup();
    let sel = Signal::new(0usize);
    let l = log();
    let (s2, l2) = (sel.clone(), l.clone());
    let (_s, buf) = surf(12, 3, move || {
        let l2 = l2.clone();
        RadioGroup::new(vec!["one".into(), "two".into(), "six".into()], s2.clone())
            .on_change(move |i| l2.borrow_mut().push(i.to_string()))
    });
    assert_eq!(trimmed(buf), v(&["(•) one", "( ) two", "( ) six"]));
    assert_eq!(hl_at(buf, 0, 0).as_deref(), Some(SEL));
    assert_eq!(hl_at(buf, 1, 0), None);
    feed("j");
    assert_eq!(trimmed(buf), v(&["( ) one", "(•) two", "( ) six"]));
    assert_eq!(hl_at(buf, 0, 0), None);
    assert_eq!(hl_at(buf, 1, 0).as_deref(), Some(SEL));
    feed("<Down>j");
    assert_eq!(sel.get_untracked(), 0); // wrapped past the end
    feed("k");
    assert_eq!(sel.get_untracked(), 2); // wrapped before the start
    assert_eq!(*l.borrow(), v(&["1", "2", "0", "2"]));
}

#[nvim_oxi::test]
fn radio_group_horizontal() {
    setup();
    let sel = Signal::new(0usize);
    let s2 = sel.clone();
    let (_s, buf) = surf(20, 1, move || RadioGroup::new(vec!["a".into(), "b".into()], s2.clone()).horizontal());
    assert_eq!(trimmed(buf), v(&["(•) a  ( ) b"]));
    feed("l");
    assert_eq!(trimmed(buf), v(&["( ) a  (•) b"]));
    feed("j"); // vertical key is ignored when horizontal
    assert_eq!(sel.get_untracked(), 1);
    feed("<Left>");
    assert_eq!(sel.get_untracked(), 0);
}

// ---- List ----------------------------------------------------------------

#[nvim_oxi::test]
fn list_navigation_and_selection_callback() {
    setup();
    let st = ListState::new();
    let picked = log();
    let (st2, p2) = (st.clone(), picked.clone());
    let (_s, buf) = surf(10, 4, move || {
        let p2 = p2.clone();
        List::new(vec!["a", "b", "c"], &st2).on_select(move |i| p2.borrow_mut().push(i.to_string()))
    });
    assert_eq!(trimmed(buf), v(&["▌ a", "  b", "  c", ""]));
    assert_eq!(hl_at(buf, 0, 0).as_deref(), Some(SEL));
    feed("j");
    assert_eq!(trimmed(buf), v(&["  a", "▌ b", "  c", ""]));
    assert_eq!(hl_at(buf, 0, 0), None);
    assert_eq!(hl_at(buf, 1, 0).as_deref(), Some(SEL));
    feed("<Down><Down>"); // wraps to the top
    assert_eq!(st.selected.get_untracked(), 0);
    feed("k"); // wraps to the bottom
    assert_eq!(st.selected.get_untracked(), 2);
    feed("g");
    assert_eq!(st.selected.get_untracked(), 0);
    feed("G");
    assert_eq!(st.selected.get_untracked(), 2);
    feed("<CR>");
    assert_eq!(*picked.borrow(), v(&["2"]));
}

#[nvim_oxi::test]
fn list_on_change_and_unfocused_selection_style() {
    setup();
    let st = ListState::new();
    let changed = log();
    let (st2, c2) = (st.clone(), changed.clone());
    let (_s, buf) = surf(10, 4, move || {
        let c2 = c2.clone();
        col![
            Button::new("b"),
            List::new(vec!["a", "b", "c"], &st2).on_change(move |i| c2.borrow_mut().push(i.to_string()))
        ]
    });
    // Focus is on the button, so the list's selection uses the dimmer group.
    assert_eq!(hl_at(buf, 1, 0).as_deref(), Some("AbNuiFocus"));
    feed("<Tab>j");
    assert_eq!(*changed.borrow(), v(&["1"]));
    assert_eq!(hl_at(buf, 2, 0).as_deref(), Some(SEL));
    assert_eq!(hl_at(buf, 1, 0), None);
}

#[nvim_oxi::test]
fn list_scrolls_when_more_items_than_rows() {
    setup();
    let st = ListState::new();
    let st2 = st.clone();
    let items: Vec<String> = (0..10).map(|i| format!("item{i}")).collect();
    let (_s, buf) = surf(10, 3, move || List::new(items.clone(), &st2));
    let last = |buf: Buffer| -> Vec<char> { lines(buf).iter().map(|l| l.chars().last().unwrap()).collect() };
    let starts = |buf: Buffer| -> Vec<String> { lines(buf).iter().map(|l| l.chars().take(8).collect()).collect() };
    assert_eq!(starts(buf), v(&["▌ item0 ", "  item1 ", "  item2 "]));
    // Scrollbar thumb at the top.
    assert_eq!(last(buf), vec!['█', '│', '│']);
    feed("G");
    assert_eq!(starts(buf), v(&["  item7 ", "  item8 ", "▌ item9 "]));
    assert_eq!(last(buf), vec!['│', '│', '█']);
    feed("g");
    assert_eq!(starts(buf), v(&["▌ item0 ", "  item1 ", "  item2 "]));
    // Moving within the window does not scroll; moving past it does.
    feed("jjj");
    assert_eq!(starts(buf), v(&["  item1 ", "  item2 ", "▌ item3 "]));
    feed("<C-f>");
    assert_eq!(st.selected.get_untracked(), 6);
}

#[nvim_oxi::test]
fn list_without_scrollbar_and_custom_render() {
    setup();
    let st = ListState::new();
    let st2 = st.clone();
    let (_s, buf) = surf(10, 2, move || {
        List::with_render(vec![1, 2, 3, 4], &st2, |n, icx| {
            Line::raw(format!("n={n}{}", if icx.selected { "*" } else { "" }))
        })
        .scrollbar(false)
    });
    assert_eq!(trimmed(buf), v(&["▌ n=1*", "  n=2"]));
    feed("j");
    assert_eq!(trimmed(buf), v(&["  n=1", "▌ n=2*"]));
}

#[nvim_oxi::test]
fn list_multi_select_and_filter() {
    setup();
    let st = ListState::new();
    let st2 = st.clone();
    let (_s, buf) = surf(10, 3, move || List::new(vec!["a", "b", "c"], &st2).multi_select());
    feed("<Space>j");
    assert_eq!(st.marked.get_untracked(), vec![0]);
    assert_eq!(trimmed(buf), v(&["✓ a", "▌ b", "  c"]));
    feed("<Space>"); // marks b
    feed("k<Space>"); // unmarks a
    assert_eq!(st.marked.get_untracked(), vec![1]);

    let st = ListState::new();
    let st2 = st.clone();
    let (_s2, buf2) = surf(10, 3, move || List::new(vec!["a", "b", "c"], &st2).filter(|s| *s != "b"));
    assert_eq!(trimmed(buf2), v(&["▌ a", "  c", ""]));
    feed("j");
    assert_eq!(st.selected.get_untracked(), 2); // index into the original items
}

#[nvim_oxi::test]
fn list_selects_through_a_shared_signal() {
    setup();
    let sel = Signal::new(0usize);
    let state = ListState::from_selected(sel.clone());
    let picked = log();
    let p2 = picked.clone();
    let (_s, buf) = surf(10, 3, move || {
        let p2 = p2.clone();
        List::new(vec!["x", "y", "z"], &state)
            .scrollbar(false)
            .on_select(move |i| p2.borrow_mut().push(i.to_string()))
    });
    assert_eq!(trimmed(buf), v(&["▌ x", "  y", "  z"]));
    feed("jj<CR>");
    assert_eq!(sel.get_untracked(), 2);
    assert_eq!(*picked.borrow(), v(&["2"]));
    assert_eq!(trimmed(buf), v(&["  x", "  y", "▌ z"]));
}

// ---- TextField / TextArea --------------------------------------------------------

#[nvim_oxi::test]
fn text_field_typing_backspace_and_cursor() {
    setup();
    let st = FieldState::new("");
    let changes = log();
    let (st2, c2) = (st.clone(), changes.clone());
    let (_s, buf) = surf(10, 1, move || {
        let c2 = c2.clone();
        TextField::new(&st2).on_change(move |s| c2.borrow_mut().push(s.to_string()))
    });
    feed("hello");
    assert_eq!(st.value.get_untracked(), "hello");
    assert_eq!(trimmed(buf), v(&["hello"]));
    // The cursor cell sits right after the text.
    assert_eq!(hl_at(buf, 0, 5).as_deref(), Some("AbNuiCursor"));
    assert_eq!(hl_at(buf, 0, 0).as_deref(), Some("AbNuiField"));
    feed("<BS>");
    assert_eq!(st.value.get_untracked(), "hell");
    feed("<Left><Left>X");
    assert_eq!(st.value.get_untracked(), "heXll");
    feed("<Home>Y");
    assert_eq!(st.value.get_untracked(), "YheXll");
    assert_eq!(hl_at(buf, 0, 1).as_deref(), Some("AbNuiCursor"));
    assert_eq!(hl_at(buf, 0, 5).as_deref(), Some("AbNuiField"));
    feed("<End><Left><Del>"); // Delete removes the char under the cursor
    assert_eq!(st.value.get_untracked(), "YheXl");
    feed("<End><Right><Del><BS><BS><BS><BS><BS><BS><BS>");
    assert_eq!(st.value.get_untracked(), "");
    assert_eq!(*changes.borrow().last().unwrap(), "");
    assert_eq!(changes.borrow()[0], "h");
}

#[nvim_oxi::test]
fn text_field_word_and_line_deletion_and_submit() {
    setup();
    let st = FieldState::new("");
    let submitted = log();
    let (st2, s2) = (st.clone(), submitted.clone());
    let (_s, _buf) = surf(20, 1, move || {
        let s2 = s2.clone();
        TextField::new(&st2).on_submit(move |s| s2.borrow_mut().push(s.to_string()))
    });
    feed("one two three<C-w>");
    assert_eq!(st.value.get_untracked(), "one two ");
    feed("<C-w>");
    assert_eq!(st.value.get_untracked(), "one ");
    feed("x<CR>");
    assert_eq!(*submitted.borrow(), v(&["one x"]));
    feed("<C-u>");
    assert_eq!(st.value.get_untracked(), "");
}

#[nvim_oxi::test]
fn text_field_placeholder_mask_and_initial_value() {
    setup();
    let st = FieldState::new("");
    let st2 = st.clone();
    let (_s, buf) = surf(12, 2, move || col![Button::new("b"), TextField::new(&st2).placeholder("type...")]);
    // Unfocused and empty: placeholder in the muted group.
    assert_eq!(trimmed(buf)[1], "type...");
    assert_eq!(hl_at(buf, 1, 0).as_deref(), Some("AbNuiMuted"));
    feed("<Tab>");
    assert_eq!(trimmed(buf)[1], "");
    assert_eq!(hl_at(buf, 1, 0).as_deref(), Some("AbNuiCursor"));

    let st = FieldState::new("日本");
    let st2 = st.clone();
    let (_s2, buf2) = surf(10, 1, move || TextField::new(&st2).mask('*'));
    assert_eq!(trimmed(buf2), v(&["**"]));
    feed("ab");
    assert_eq!(st.value.get_untracked(), "日本ab");
    assert_eq!(trimmed(buf2), v(&["****"]));
}

#[nvim_oxi::test]
fn text_field_wide_chars_and_horizontal_scroll() {
    setup();
    let st = FieldState::new("日本語");
    let st2 = st.clone();
    let (_s, buf) = surf(8, 1, move || TextField::new(&st2));
    assert_eq!(trimmed(buf), v(&["日本語"]));
    // Longer than the field: the view scrolls so the cursor stays visible.
    feed("abcdefghij");
    let row = trimmed(buf)[0].clone();
    assert!(row.ends_with("ghij"), "row = {row:?}");
    assert!(wp_ui::text::display_width(&row) <= 8);
    feed("<Home>");
    assert!(trimmed(buf)[0].starts_with("日本語"));
}

#[nvim_oxi::test]
fn text_area_editing_and_navigation() {
    setup();
    let st = AreaState::new("");
    let changes = log();
    let (st2, c2) = (st.clone(), changes.clone());
    let (_s, buf) = surf(10, 3, move || {
        let c2 = c2.clone();
        TextArea::new(&st2).on_change(move |ls| c2.borrow_mut().push(ls.join("|")))
    });
    feed("ab<CR>cd");
    assert_eq!(st.text(), "ab\ncd");
    assert_eq!(trimmed(buf), v(&["ab", "cd", ""]));
    assert_eq!(hl_at(buf, 1, 2).as_deref(), Some("AbNuiCursor"));
    feed("<Up><End>!");
    assert_eq!(st.text(), "ab!\ncd");
    feed("<Down><Home><BS>"); // joins with the previous line
    assert_eq!(st.text(), "ab!cd");
    assert_eq!(trimmed(buf), v(&["ab!cd", "", ""]));
    feed("<Left><Left><CR>");
    assert_eq!(st.text(), "a\nb!cd");
    feed("<Up><End><Del>"); // joins the next line up
    assert_eq!(st.text(), "ab!cd");
    assert_eq!(changes.borrow().last().unwrap(), "ab!cd");
}

#[nvim_oxi::test]
fn text_area_initial_text_and_vertical_scroll() {
    setup();
    let st = AreaState::new("l0\nl1\nl2\nl3\nl4");
    let st2 = st.clone();
    let (_s, buf) = surf(6, 3, move || TextArea::new(&st2));
    assert_eq!(trimmed(buf), v(&["l0", "l1", "l2"]));
    feed("<PageDown>");
    // Cursor moved down by a page; the viewport follows it.
    assert_eq!(st.cursor.get_untracked().0, 3);
    assert_eq!(trimmed(buf), v(&["l1", "l2", "l3"]));
}

// ---- Select ---------------------------------------------------------------------

#[nvim_oxi::test]
fn select_open_navigate_choose() {
    setup();
    let st = SelectState::new();
    let chosen = log();
    let (st2, c2) = (st.clone(), chosen.clone());
    let (s, buf) = surf(12, 5, move || {
        let c2 = c2.clone();
        col![
            Select::new(vec!["red".into(), "green".into(), "blue".into()], &st2)
                .on_change(move |i| c2.borrow_mut().push(i.to_string()))
        ]
    });
    assert_eq!(trimmed(buf), v(&["▾ red", "", "", "", ""]));
    feed("<CR>");
    assert!(st.open.get_untracked());
    assert_eq!(trimmed(buf), v(&["▾ red", " red", " green", " blue", ""]));
    assert_eq!(hl_at(buf, 1, 1).as_deref(), Some(SEL));
    assert_eq!(hl_at(buf, 2, 1).as_deref(), Some("AbNuiField"));
    feed("jj<CR>");
    assert!(!st.open.get_untracked());
    assert_eq!(st.selected.get_untracked(), 2);
    assert_eq!(*chosen.borrow(), v(&["2"]));
    assert_eq!(trimmed(buf), v(&["▾ blue", "", "", "", ""]));
    assert!(s.is_visible());
}

#[nvim_oxi::test]
fn select_esc_is_consumed_when_open_and_hides_when_closed() {
    setup();
    let st = SelectState::new();
    let st2 = st.clone();
    let (s, buf) = surf(12, 4, move || Select::new(vec!["a".into(), "b".into()], &st2));
    feed("<CR>");
    assert!(st.open.get_untracked());
    feed("<Esc>");
    assert!(!st.open.get_untracked());
    assert!(s.is_visible(), "Esc closed the dropdown, not the surface");
    assert_eq!(trimmed(buf)[1], "");
    // Closed select doesn't consume Esc -> surface hides.
    feed("<Esc>");
    assert!(!s.is_visible());
}

// ---- TabBar ---------------------------------------------------------------------

#[nvim_oxi::test]
fn tab_bar_switches_content_and_highlights() {
    setup();
    let active = Signal::new(0usize);
    let changes = log();
    let (a2, c2) = (active.clone(), changes.clone());
    let (_s, buf) = surf(24, 2, move || {
        let c2 = c2.clone();
        let page = ["page one", "page two", "page three"][a2.get()];
        col![
            TabBar::new(vec!["One".into(), "Two".into(), "Three".into()], a2.clone())
                .on_change(move |i| c2.borrow_mut().push(i.to_string())),
            Text::new(page)
        ]
    });
    assert_eq!(trimmed(buf), v(&[" One │ Two │ Three", "page one"]));
    // Focused active tab uses the selection group; inactive tabs their own.
    assert_eq!(hl_at(buf, 0, 1).as_deref(), Some(SEL));
    assert_eq!(hl_at(buf, 0, 9).as_deref(), Some("AbNuiTabInactive"));
    feed("l");
    assert_eq!(trimmed(buf)[1], "page two");
    assert_eq!(hl_at(buf, 0, 1).as_deref(), Some("AbNuiTabInactive"));
    assert_eq!(hl_at(buf, 0, 9).as_deref(), Some(SEL));
    feed("<Right><Right>"); // wraps to the first
    assert_eq!(active.get_untracked(), 0);
    feed("h"); // wraps back to the last
    assert_eq!(active.get_untracked(), 2);
    assert_eq!(trimmed(buf)[1], "page three");
    assert_eq!(*changes.borrow(), v(&["1", "2", "0", "2"]));
}

// ---- Table ----------------------------------------------------------------------

fn people() -> Vec<Vec<Line>> {
    vec![
        vec![Line::raw("1"), Line::raw("alice")],
        vec![Line::raw("2"), Line::raw("bob")],
        vec![Line::raw("3"), Line::raw("carol")],
    ]
}

#[nvim_oxi::test]
fn table_header_rows_and_selection() {
    setup();
    let st = ListState::new();
    let picked = log();
    let (st2, p2) = (st.clone(), picked.clone());
    let (_s, buf) = surf(14, 5, move || {
        let p2 = p2.clone();
        Table::new(vec![TableColumn::new("Id").width(ColWidth::Fixed(4)), TableColumn::new("Name")], people(), &st2)
            .on_select(move |i| p2.borrow_mut().push(i.to_string()))
    });
    assert_eq!(trimmed(buf), v(&["Id   Name", "─────────────", "1    alice", "2    bob", "3    carol"]));
    assert_eq!(hl_at(buf, 0, 0).as_deref(), Some("AbNuiHeader"));
    assert_eq!(hl_at(buf, 1, 0).as_deref(), Some("AbNuiBorder"));
    assert_eq!(hl_at(buf, 2, 0).as_deref(), Some(SEL));
    assert_eq!(hl_at(buf, 3, 0), None);
    feed("j");
    assert_eq!(hl_at(buf, 2, 0), None);
    assert_eq!(hl_at(buf, 3, 0).as_deref(), Some(SEL));
    feed("<CR>");
    assert_eq!(*picked.borrow(), v(&["1"]));
    feed("G");
    assert_eq!(st.selected.get_untracked(), 2);
    feed("k");
    assert_eq!(st.selected.get_untracked(), 1);
}

#[nvim_oxi::test]
fn table_alignment_headerless_and_scrolling() {
    setup();
    let st = ListState::new();
    let st2 = st.clone();
    let (_s, buf) = surf(10, 2, move || {
        Table::new(
            vec![TableColumn::new("N").width(ColWidth::Fixed(3)).align(TextAlign::Right), TableColumn::new("T")],
            vec![
                vec![Line::raw("1"), Line::raw("a")],
                vec![Line::raw("22"), Line::raw("b")],
                vec![Line::raw("333"), Line::raw("c")],
            ],
            &st2,
        )
        .headerless()
        .scrollbar(false)
    });
    assert_eq!(trimmed(buf), v(&["  1 a", " 22 b"]));
    feed("G");
    assert_eq!(trimmed(buf), v(&[" 22 b", "333 c"]));
}

// ---- Tree -----------------------------------------------------------------------

fn tree_roots() -> Vec<TreeNode<&'static str>> {
    vec![
        TreeNode::new("root", vec![TreeNode::leaf("a"), TreeNode::new("sub", vec![TreeNode::leaf("deep")])]),
        TreeNode::leaf("other"),
    ]
}

#[nvim_oxi::test]
fn tree_expand_collapse_and_navigation() {
    setup();
    let st = TreeState::new();
    let activated = log();
    let (st2, a2) = (st.clone(), activated.clone());
    let (_s, buf) = surf(16, 6, move || {
        let a2 = a2.clone();
        Tree::new(tree_roots(), &st2).on_activate(move |d| a2.borrow_mut().push(d.to_string()))
    });
    assert_eq!(trimmed(buf), v(&["▸ root", "  other", "", "", "", ""]));
    assert_eq!(hl_at(buf, 0, 4).as_deref(), Some(SEL));
    feed("<CR>"); // expand
    assert_eq!(trimmed(buf), v(&["▾ root", "    a", "  ▸ sub", "  other", "", ""]));
    feed("jjl"); // on "sub": expand with l
    assert_eq!(trimmed(buf), v(&["▾ root", "    a", "  ▾ sub", "      deep", "  other", ""]));
    feed("jl"); // l on a leaf activates it
    assert_eq!(*activated.borrow(), v(&["deep"]));
    feed("h"); // leaf: h jumps to the parent row
    assert_eq!(st.selected.get_untracked(), 2);
    feed("h"); // expanded parent: h collapses
    assert_eq!(trimmed(buf), v(&["▾ root", "    a", "  ▸ sub", "  other", "", ""]));
    feed("gh"); // root: collapse
    assert_eq!(trimmed(buf), v(&["▸ root", "  other", "", "", "", ""]));
    feed("<Space>");
    assert_eq!(trimmed(buf)[1], "    a");
    feed("jj<CR>"); // Enter on a collapsed branch toggles, not activates
    assert_eq!(*activated.borrow(), v(&["deep"]));
}

// ---- ScrollView / Scrollbar -----------------------------------------------------

#[nvim_oxi::test]
fn scrollbar_widget_thumb_position() {
    setup();
    let rows = render(3, 3, || row![Text::new("ab"), Scrollbar::new(10, 3, 0)]);
    assert_eq!(rows, v(&["ab█", "  │", "  │"]));
    let rows = render(3, 3, || row![Text::new("ab"), Scrollbar::new(10, 3, 7)]);
    assert_eq!(rows, v(&["ab│", "  │", "  █"]));
    // Content that fits needs no bar.
    let rows = render(3, 3, || row![Text::new("ab"), Scrollbar::new(3, 3, 0)]);
    assert_eq!(rows, v(&["ab", "", ""]));
}

#[nvim_oxi::test]
fn scroll_view_keys_drive_clamped_offset_signal() {
    setup();
    let off = Signal::new(0u16);
    let o2 = off.clone();
    let tick = Signal::new(0);
    let t2 = tick.clone();
    let (_s, buf) = surf(8, 3, move || {
        t2.get();
        ScrollView::new(
            o2.clone(),
            col![Text::new("l0"), Text::new("l1"), Text::new("l2"), Text::new("l3"), Text::new("l4")],
        )
    });
    let heads = |buf: Buffer| -> Vec<String> { lines(buf).iter().map(|l| l.chars().take(2).collect()).collect() };
    let bar = |buf: Buffer| -> Vec<char> { lines(buf).iter().map(|l| l.chars().last().unwrap()).collect() };
    assert_eq!(heads(buf), v(&["l0", "l1", "l2"]));
    assert_eq!(bar(buf), vec!['█', '█', '│']);
    feed("j");
    assert_eq!(off.get_untracked(), 1);
    feed("jjjj"); // clamps at the maximum offset
    assert_eq!(off.get_untracked(), 2);
    feed("g");
    assert_eq!(off.get_untracked(), 0);
    feed("G");
    assert_eq!(off.get_untracked(), 2);
    feed("k");
    assert_eq!(off.get_untracked(), 1);
    // Whatever repaints the surface applies the offset.
    tick.set(1);
    assert_eq!(heads(buf), v(&["l1", "l2", "l3"]));
    feed("G");
    tick.set(2);
    assert_eq!(heads(buf), v(&["l2", "l3", "l4"]));
    assert_eq!(bar(buf), vec!['│', '█', '█']);
}

#[nvim_oxi::test]
fn scroll_view_without_scrollbar_uses_initial_offset() {
    setup();
    let (_s, buf) = surf(4, 2, move || {
        ScrollView::new(Signal::new(1u16), col![Text::new("a"), Text::new("b"), Text::new("c")]).scrollbar(false)
    });
    assert_eq!(trimmed(buf), v(&["b", "c"]));
}

// Regression (was a bug before the internals rewrite): scrolling a ScrollView with j/k/G/<PageDown> changes the offset Signal
// but the window is NOT repainted (ScrollView reads the offset with
// get_untracked, so the surface's effect never subscribes to it). Expected per
// README ("a generic viewport that makes any child scrollable"): the visible
// rows change immediately.
//
#[nvim_oxi::test]
fn scroll_view_keys_repaint_immediately() {
    setup();
    let off = Signal::new(0u16);
    let o2 = off.clone();
    let (_s, buf) = surf(8, 3, move || {
        ScrollView::new(
            o2.clone(),
            col![Text::new("l0"), Text::new("l1"), Text::new("l2"), Text::new("l3"), Text::new("l4")],
        )
    });
    feed("j");
    assert_eq!(trimmed(buf)[0].chars().take(2).collect::<String>(), "l1");
    off.set(0); // external write should repaint too
    assert_eq!(trimmed(buf)[0].chars().take(2).collect::<String>(), "l0");
}

// ---- Keyed ----------------------------------------------------------------------

#[nvim_oxi::test]
fn keyed_custom_key_handler_runs_before_widget() {
    setup();
    let l = log();
    let l2 = l.clone();
    let (_s, _buf) = surf(12, 2, move || {
        let (a, b) = (l2.clone(), l2.clone());
        col![
            Keyed::new(Button::new("A").on_press(move || a.borrow_mut().push("press".into())))
                .on_key(Key::Char('x'), move || b.borrow_mut().push("x".into())),
            Button::new("B"),
        ]
    });
    feed("x<CR>");
    assert_eq!(*l.borrow(), v(&["x", "press"]));
    // On another widget the handler is not active.
    feed("<Tab>x");
    assert_eq!(*l.borrow(), v(&["x", "press"]));
}

#[nvim_oxi::test]
fn keyed_focus_follows_key_when_tree_changes_shape() {
    setup();
    let show_extra = Signal::new(true);
    let se = show_extra.clone();
    let l = log();
    let l2 = l.clone();
    let (_s, buf) = surf(12, 3, move || {
        let mut kids: Vec<Element> = Vec::new();
        if se.get() {
            kids.push(Box::new(Button::new("extra")));
        }
        let (t, o) = (l2.clone(), l2.clone());
        kids.push(Box::new(
            Keyed::new(Button::new("target").on_press(move || t.borrow_mut().push("target".into()))).focus_key("t"),
        ));
        kids.push(Box::new(Button::new("other").on_press(move || o.borrow_mut().push("other".into()))));
        Column::new(kids)
    });
    assert_eq!(trimmed(buf), v(&["[ extra ]", "[ target ]", "[ other ]"]));
    feed("<Tab>"); // focus "target"
    assert_eq!(hl_at(buf, 1, 0).as_deref(), Some(SEL));
    show_extra.set(false);
    assert_eq!(trimmed(buf), v(&["[ target ]", "[ other ]", ""]));
    // Without the key, index 1 would now be "other": keys follow the key.
    feed("<CR>");
    assert_eq!(*l.borrow(), v(&["target"]));
    // Growing the tree again: keys still follow the key.
    show_extra.set(true);
    feed("<CR>");
    assert_eq!(*l.borrow(), v(&["target", "target"]));
}

// Regression (was a bug before the internals rewrite): right after the tree changes shape, the repaint highlights the widget at
// the OLD focus index ("other") even though keys already route to the keyed
// target: focus correction happens silently after painting, so the frame on
// screen is stale until something else repaints. Expected: highlight on the
// target ("[ target ]", row 0) immediately.
//
#[nvim_oxi::test]
fn keyed_focus_highlight_follows_key_immediately() {
    setup();
    let show_extra = Signal::new(true);
    let se = show_extra.clone();
    let (_s, buf) = surf(12, 3, move || {
        let mut kids: Vec<Element> = Vec::new();
        if se.get() {
            kids.push(Box::new(Button::new("extra")));
        }
        kids.push(Box::new(Keyed::new(Button::new("target")).focus_key("t")));
        kids.push(Box::new(Button::new("other")));
        Column::new(kids)
    });
    feed("<Tab>");
    show_extra.set(false);
    assert_eq!(highlights(buf), vec![(0, 0, 10, SEL.into())]); // actual: [(1, 0, 9, SEL)]
}

#[nvim_oxi::test]
fn focus_index_clamps_when_widgets_disappear() {
    setup();
    let two = Signal::new(true);
    let tick = Signal::new(0);
    let log = log();
    let (t2, k2, l2) = (two.clone(), tick.clone(), log.clone());
    let (_s, buf) = surf(12, 2, move || {
        k2.get();
        let l = l2.clone();
        if t2.get() {
            col![Button::new("a"), Button::new("b")]
        } else {
            col![Button::new("a").on_press(move || l.borrow_mut().push("a".into()))]
        }
    });
    feed("<Tab>");
    assert_eq!(hl_at(buf, 1, 0).as_deref(), Some(SEL));
    two.set(false);
    // Keys reach the only remaining widget...
    feed("<CR>");
    assert_eq!(*log.borrow(), v(&["a"]));
    // ...and once repainted it carries the focus highlight.
    tick.set(1);
    assert_eq!(highlights(buf), vec![(0, 0, 5, SEL.into())]);
}

// Regression (was a bug before the internals rewrite): when the focused widget disappears (focus index beyond the new widget
// count), the frame painted for that change shows NO focused widget; the index
// is only clamped (silently) after painting, so the highlight appears one
// repaint late. Expected: the surviving widget is highlighted immediately.
//
#[nvim_oxi::test]
fn focus_clamp_highlights_immediately() {
    setup();
    let two = Signal::new(true);
    let t2 = two.clone();
    let (_s, buf) =
        surf(12, 2, move || if t2.get() { col![Button::new("a"), Button::new("b")] } else { col![Button::new("a")] });
    feed("<Tab>");
    two.set(false);
    assert_eq!(highlights(buf), vec![(0, 0, 5, SEL.into())]); // actual: []
}
