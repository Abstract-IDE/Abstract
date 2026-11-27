//! Regression tests for widget bugs: arithmetic overflow, painting outside a
//! widget's area, flex/table space distribution, scroll offsets, and the
//! `Select`/`ScrollView`/`RadioGroup` viewport handling.

use std::cell::RefCell;
use std::rc::Rc;

use wp_ui::prelude::*;
use wp_ui::view::{Constraints, Widget};

use crate::support::*;

const SEL: &str = "AbNuiSelection";

fn strs(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| s.to_string()).collect()
}

/// The last char of every row (the right border, for a boxed surface).
fn last_chars(rows: &[String]) -> Vec<char> {
    rows.iter().map(|r| r.chars().last().unwrap_or(' ')).collect()
}

// ---- 1. u16 overflow with greedy children / huge weights ------------------------

#[nvim_oxi::test]
fn fix_scroll_view_with_greedy_children_does_not_overflow() {
    setup();
    let st = AreaState::new("abc");
    let rows = render(10, 4, move || {
        ScrollView::new(Signal::new(0u16), col![TextArea::new(&st), TextArea::new(&st), TextArea::new(&st)])
    });
    assert_eq!(rows[0], "abc");
    let rows = render(6, 4, || {
        ScrollView::new(
            Signal::new(0u16),
            row![Center::new(Text::new("x")), Divider::vertical(), Scrollbar::new(10, 2, 0)],
        )
    });
    assert_eq!(rows.len(), 4);
}

#[nvim_oxi::test]
fn fix_huge_spacer_weight_does_not_overflow() {
    setup();
    // leftover (18) * weight (10_000) overflowed u16.
    let rows = render(3, 20, || col![Text::new("a"), Spacer::new().weight(10_000), Text::new("b")]);
    assert_eq!(rows[0], "a");
    assert_eq!(rows[19], "b");
}

// ---- 2. TextArea with no lines ------------------------------------------------------

#[nvim_oxi::test]
fn fix_text_area_with_empty_lines_vec() {
    setup();
    let st = AreaState::new("");
    st.lines.set(Vec::new());
    let st2 = st.clone();
    let (_s, buf) = surf(10, 3, move || TextArea::new(&st2));
    feed("<PageDown>");
    feed("a");
    assert_eq!(st.lines.get_untracked(), strs(&["a"]));
    st.lines.set(Vec::new());
    feed("<CR>");
    assert_eq!(st.lines.get_untracked(), strs(&["", ""]));
    assert_eq!(st.cursor.get_untracked(), (1, 0));

    st.clear();
    assert_eq!(st.lines.get_untracked(), strs(&[""]));
    assert_eq!(st.cursor.get_untracked(), (0, 0));
    assert_eq!(st.text(), "");
    assert_eq!(trimmed(buf), v(&["", "", ""]));
}

// ---- 3. painting is clipped to the widget's area --------------------------------------

#[nvim_oxi::test]
fn fix_container_child_text_cannot_overwrite_border() {
    setup();
    let rows = render(8, 3, || Container::new(Text::new("hello world")));
    assert_eq!(rows, v(&["╭──────╮", "│hello │", "╰──────╯"]));
}

#[nvim_oxi::test]
fn fix_widget_labels_are_clipped_to_their_area() {
    setup();
    // List rows.
    let st = ListState::new();
    let rows = render(8, 3, move || Container::new(List::new(strs(&["abcdefghijkl"]), &st).scrollbar(false)));
    assert_eq!(rows[1], "│▌ abcd│");
    // Tree rows.
    let ts = TreeState::new();
    let rows = render(8, 3, move || Container::new(Tree::new(vec![TreeNode::leaf("abcdefghijkl")], &ts)));
    assert_eq!(last_chars(&rows), vec!['╮', '│', '╯']);
    // Placeholders.
    let fs = FieldState::new("");
    // (A button first takes the focus, so the placeholders show.)
    let rows =
        render(8, 4, move || col![Button::new("x"), Container::new(TextField::new(&fs).placeholder("type something"))]);
    assert_eq!(rows[2], "│type s│");
    let a = AreaState::new("");
    let rows = render(8, 4, move || {
        col![Button::new("x"), Container::new(SizedBox::new(6, 1).child(TextArea::new(&a).placeholder("write here")))]
    });
    assert_eq!(rows[2], "│write │");
    // Button / Checkbox / Toggle / Select / TabBar labels.
    let rows = render(8, 3, || Container::new(Button::new("press me")));
    assert_eq!(rows[1], "│[ pres│");
    let rows = render(8, 3, || Container::new(Checkbox::new("check me", Signal::new(false))));
    assert_eq!(rows[1], "│[ ] ch│");
    let rows = render(8, 3, || Container::new(Toggle::new("toggle me", Signal::new(false))));
    assert_eq!(rows[1], "│●── to│");
    let ss = SelectState::new();
    let rows = render(8, 3, move || Container::new(Select::new(strs(&["a long option"]), &ss)));
    assert_eq!(rows[1], "│▾ a lo│");
    let rows = render(8, 3, || Container::new(TabBar::new(strs(&["first", "second"]), Signal::new(0))));
    assert_eq!(rows[1], "│ first│");
    let rows = render(8, 3, || Container::new(RadioGroup::new(strs(&["one", "two"]), Signal::new(0)).horizontal()));
    assert_eq!(rows[1], "│(•) on│");
}

// ---- 4. fixed children are clamped to the space that is left -----------------------

#[nvim_oxi::test]
fn fix_fixed_children_never_exceed_parent() {
    setup();
    let fs = FieldState::new("");
    let rows = render(10, 3, move || Container::new(row![Text::new("ab"), TextField::new(&fs)]));
    assert_eq!(last_chars(&rows), vec!['╮', '│', '╯']);
    let rows = render(5, 4, || Container::new(col![Text::new("a"), Text::new("b"), Text::new("c")]));
    assert_eq!(rows, v(&["╭───╮", "│a  │", "│b  │", "╰───╯"]));
}

// ---- 5. flex rounding ----------------------------------------------------------------

#[nvim_oxi::test]
fn fix_flex_children_fill_all_cells() {
    setup();
    let rows = render(10, 1, || {
        row![
            Expanded::new(Divider::new().ch('a')),
            Expanded::new(Divider::new().ch('b')),
            Expanded::new(Divider::new().ch('c'))
        ]
    });
    assert_eq!(rows, v(&["aaaabbbccc"]));
}

// ---- 6. table column widths ---------------------------------------------------------

#[nvim_oxi::test]
fn fix_table_fixed_columns_clamped() {
    setup();
    let st = ListState::new();
    let rows = render(10, 5, move || {
        let cols = vec![
            TableColumn::new("aaaaaa").width(ColWidth::Fixed(6)),
            TableColumn::new("bbbbbb").width(ColWidth::Fixed(6)),
        ];
        let body = vec![vec![Line::raw("123456"), Line::raw("654321")]];
        Container::new(Table::new(cols, body, &st).scrollbar(false))
    });
    assert_eq!(last_chars(&rows), vec!['╮', '│', '│', '│', '╯']);
    // The second column only has one cell left: its header truncates to `…`.
    assert_eq!(rows[1], "│aaaaaa …│");
}

// ---- 7. scroll offsets never leave blank rows ------------------------------------------

#[nvim_oxi::test]
fn fix_list_tree_table_offset_clamped_to_last_page() {
    setup();
    let items: Vec<String> = (0..10).map(|i| format!("i{i}")).collect();

    let st = ListState::new();
    st.selected.set(9);
    st.offset.set(9);
    let it = items.clone();
    let rows = render(6, 3, move || List::new(it.clone(), &st).scrollbar(false));
    assert_eq!(rows, v(&["  i7", "  i8", "▌ i9"]));

    let ts = TreeState::new();
    ts.selected.set(9);
    ts.offset.set(9);
    let nodes: Vec<TreeNode<String>> = items.iter().cloned().map(TreeNode::leaf).collect();
    let rows = render(6, 3, move || Tree::new(nodes.clone(), &ts).scrollbar(false));
    assert_eq!(rows, v(&["  i7", "  i8", "  i9"]));

    let tst = ListState::new();
    tst.selected.set(9);
    tst.offset.set(9);
    let rows = render(6, 3, move || {
        let body: Vec<Vec<Line>> = items.iter().map(|i| vec![Line::raw(i.as_str())]).collect();
        Table::new(vec![TableColumn::new("h")], body, &tst).headerless().scrollbar(false)
    });
    assert_eq!(rows, v(&["i7", "i8", "i9"]));
}

// ---- 8. TextArea horizontal scroll by display cells -------------------------------------

#[nvim_oxi::test]
fn fix_text_area_scrolls_by_display_width() {
    setup();
    let st = AreaState::new("日本語日本語日本語日");
    st.cursor.set((0, 10));
    let st2 = st.clone();
    let (_s, buf) = surf(10, 1, move || TextArea::new(&st2));
    // Chars 6..10 (8 cells) plus the cursor cell fit in 10 cells.
    assert_eq!(trimmed(buf), v(&["日本語日"]));
    assert_eq!(hl_at(buf, 0, 12).as_deref(), Some("AbNuiCursor"));
}

// ---- 9. Select dropdown -------------------------------------------------------------

#[nvim_oxi::test]
fn fix_select_dropdown_inside_scroll_view_lands_under_anchor() {
    setup();
    let st = SelectState::new();
    let st2 = st.clone();
    let (_s, buf) = surf(14, 8, move || {
        Container::new(ScrollView::new(
            Signal::new(0u16),
            col![Text::new("top"), Select::new(strs(&["red", "green"]), &st2)],
        ))
    });
    feed("<CR>");
    assert!(st.open.get_untracked());
    let rows = trimmed(buf);
    assert_eq!(rows[2], "│▾ red       │");
    assert!(rows[3].starts_with("│ red"), "{rows:?}");
    assert!(rows[4].starts_with("│ green"), "{rows:?}");
    assert_eq!(hl_at(buf, 3, 4).as_deref(), Some(SEL));
}

#[nvim_oxi::test]
fn fix_select_dropdown_scrolls_to_hover_and_pads_by_width() {
    setup();
    {
        let st = SelectState::new();
        let st2 = st.clone();
        let opts: Vec<String> = (0..10).map(|i| format!("opt{i}")).collect();
        let (_s, buf) = surf(12, 4, move || Select::new(opts.clone(), &st2));
        feed("<CR>jjjj");
        let rows = trimmed(buf);
        assert_eq!(&rows[1..], &v(&[" opt2", " opt3", " opt4"])[..]);
        assert_eq!(hl_at(buf, 3, 1).as_deref(), Some(SEL));
        feed("<Esc>");
    }

    // Wide options are padded to the same display width as narrow ones.
    let st = SelectState::new();
    let st2 = st.clone();
    let (_s, buf) = surf(12, 4, move || Select::new(strs(&["日本", "ab"]), &st2));
    feed("<CR>");
    let width_of = |row: i64| -> u16 {
        let line = &lines(buf)[row as usize];
        let (_, s, e, _) = highlights(buf).into_iter().find(|h| h.0 == row).expect("dropdown row");
        wp_ui::text::display_width(&line[s as usize..e as usize])
    };
    assert_eq!(width_of(1), 8);
    assert_eq!(width_of(2), 8);
}

// ---- 10. Padding::measure ---------------------------------------------------------------

#[nvim_oxi::test]
fn fix_padding_measure_respects_constraints() {
    setup();
    let s = Padding::all(2, Text::new("x")).measure(Constraints::loose(3, 3));
    assert!(s.w <= 3 && s.h <= 3, "{s:?}");
}

// ---- 11. Fraction / SizedBox paint at their own size --------------------------------------

#[nvim_oxi::test]
fn fix_fraction_and_sized_box_paint_their_size() {
    setup();
    let rows = render(12, 3, || Container::new(Fraction::new(0.5, 1.0, Divider::new())));
    assert_eq!(rows[1], "│─────     │");
    let rows = render(8, 3, || Container::new(SizedBox::new(3, 1).child(Divider::new())));
    assert_eq!(rows[1], "│───   │");
}

// ---- 12. vertical RadioGroup keeps the selection visible ----------------------------------

#[nvim_oxi::test]
fn fix_radio_group_scrolls_to_selection() {
    setup();
    let rows = render(10, 3, || RadioGroup::new(strs(&["a", "b", "c", "d", "e", "f"]), Signal::new(4)));
    assert_eq!(rows, v(&["( ) c", "( ) d", "(•) e"]));
}

// ---- 13. ScrollView focus + greedy content --------------------------------------------------

#[nvim_oxi::test]
fn fix_scroll_view_scrolls_to_focused_child() {
    setup();
    let pressed = Rc::new(RefCell::new(Vec::<usize>::new()));
    let off = Signal::new(0u16);
    let (p2, o2) = (pressed.clone(), off.clone());
    let (_s, buf) = surf(10, 3, move || {
        let buttons: Vec<Element> = (0..6)
            .map(|i| {
                let p = p2.clone();
                Box::new(Button::new(format!("b{i}")).on_press(move || p.borrow_mut().push(i))) as Element
            })
            .collect();
        ScrollView::new(o2.clone(), Column::new(buttons)).scrollbar(false)
    });
    assert_eq!(trimmed(buf), v(&["[ b0 ]", "[ b1 ]", "[ b2 ]"]));
    feed("<Tab><Tab><Tab><Tab>");
    assert_eq!(trimmed(buf), v(&["[ b2 ]", "[ b3 ]", "[ b4 ]"]));
    assert_eq!(hl_at(buf, 2, 0).as_deref(), Some(SEL));
    assert_eq!(off.get_untracked(), 2);
    feed("<CR>");
    assert_eq!(*pressed.borrow(), vec![4]);
}

#[nvim_oxi::test]
fn fix_scroll_view_greedy_child_gets_viewport_height() {
    setup();
    let st = AreaState::new("abc");
    // Content measured as the whole unbounded height used to make a ~32k-row
    // canvas (and a scrollbar); a greedy child now just fills the viewport.
    let rows = render(8, 3, move || ScrollView::new(Signal::new(0u16), TextArea::new(&st)));
    assert_eq!(rows, v(&["abc", "", ""]));
}

// ---- 14. text: tabs ---------------------------------------------------------------------

#[nvim_oxi::test]
fn fix_tabs_measure_and_paint_as_one_cell() {
    setup();
    assert_eq!(render(10, 1, || row![Text::new("a\tb"), Text::new("|")]), v(&["a b|"]));
    assert_eq!(Text::new("a\tb").measure(Constraints::loose(10, 1)).w, 3);
}

// ---- measure respects constraints ---------------------------------------------------------

#[nvim_oxi::test]
fn fix_every_widget_measure_respects_constraints() {
    setup();
    let c = Constraints::loose(3, 1);
    let fs = FieldState::new("hello");
    let a = AreaState::new("hello\nworld");
    let ls = ListState::new();
    let ts = TreeState::new();
    let ss = SelectState::new();
    let widgets: Vec<(&str, Element)> = vec![
        ("text", Box::new(Text::new("hello world"))),
        ("button", Box::new(Button::new("hello"))),
        ("checkbox", Box::new(Checkbox::new("hello", Signal::new(true)))),
        ("toggle", Box::new(Toggle::new("hello", Signal::new(true)))),
        ("radio", Box::new(RadioGroup::new(strs(&["aaaa", "bbbb"]), Signal::new(0)))),
        ("radio-h", Box::new(RadioGroup::new(strs(&["aaaa", "bbbb"]), Signal::new(0)).horizontal())),
        ("select", Box::new(Select::new(strs(&["hello"]), &ss))),
        ("tabs", Box::new(TabBar::new(strs(&["aaaa", "bbbb"]), Signal::new(0)))),
        ("list", Box::new(List::new(strs(&["hello", "world"]), &ls))),
        ("tree", Box::new(Tree::new(vec![TreeNode::leaf("hello"), TreeNode::leaf("world")], &ts))),
        ("field", Box::new(TextField::new(&fs))),
        ("area", Box::new(TextArea::new(&a))),
        ("progress", Box::new(ProgressBar::new(0.5))),
        ("divider", Box::new(Divider::vertical())),
        ("center", Box::new(Center::new(Text::new("hello")))),
        ("padding", Box::new(Padding::all(2, Text::new("hello")))),
        ("container", Box::new(Container::new(Text::new("hello")))),
        ("sized", Box::new(SizedBox::new(10, 10))),
        ("scroll", Box::new(ScrollView::new(Signal::new(0u16), col![Text::new("a"), Text::new("b")]))),
        ("hints", Box::new(KeyHints::new(vec![("q", "quit")]))),
    ];
    for (name, w) in &widgets {
        let s = w.measure(c);
        assert!(s.w <= 3 && s.h <= 1, "{name}: {s:?}");
    }
}
