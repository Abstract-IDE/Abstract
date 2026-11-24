//! Layout and content widgets, asserted on exact buffer text.

use wp_ui::prelude::*;

use crate::support::*;

// ---- Text -------------------------------------------------------------------

#[nvim_oxi::test]
fn text_plain_and_multiple_rows() {
    setup();
    let rows = render(20, 3, || col![Text::new("hello"), Text::new("world")]);
    assert_eq!(rows, v(&["hello", "world", ""]));
}

#[nvim_oxi::test]
fn text_word_wrap() {
    setup();
    let rows = render(11, 3, || Text::new("hello brave new world").wrap(Wrap::Word));
    assert_eq!(rows, v(&["hello brave", "new world", ""]));
}

#[nvim_oxi::test]
fn text_char_wrap_and_max_lines() {
    setup();
    let rows = render(4, 4, || Text::new("abcdefghij").wrap(Wrap::Char));
    assert_eq!(rows, v(&["abcd", "efgh", "ij", ""]));
    let rows = render(4, 4, || Text::new("abcdefghij").wrap(Wrap::Char).max_lines(2));
    assert_eq!(rows, v(&["abcd", "efgh", "", ""]));
}

#[nvim_oxi::test]
fn text_alignment() {
    setup();
    assert_eq!(render(6, 1, || Text::new("ab").align(TextAlign::Center)), v(&["  ab"]));
    assert_eq!(render(6, 1, || Text::new("ab").align(TextAlign::Right)), v(&["    ab"]));
    assert_eq!(render(6, 1, || Text::new("ab").align(TextAlign::Left)), v(&["ab"]));
}

#[nvim_oxi::test]
fn text_overflow_clip_vs_ellipsis() {
    setup();
    assert_eq!(render(7, 1, || Text::new("hello world")), v(&["hello w"]));
    assert_eq!(render(7, 1, || Text::new("hello world").ellipsis()), v(&["hello …"]));
    // Fits: no ellipsis.
    assert_eq!(render(12, 1, || Text::new("hello world").ellipsis()), v(&["hello world"]));
}

#[nvim_oxi::test]
fn text_styled_spans_via_macro() {
    setup();
    let (_s, buf) = surf(20, 1, || text!["a ", ("bold", "Title"), " c ", ("err", "ErrorMsg")]);
    assert_eq!(trimmed(buf), v(&["a bold c err"]));
    assert_eq!(hl_at(buf, 0, 0), None);
    assert_eq!(hl_at(buf, 0, 2).as_deref(), Some("Title"));
    assert_eq!(hl_at(buf, 0, 5).as_deref(), Some("Title"));
    assert_eq!(hl_at(buf, 0, 6), None);
    assert_eq!(hl_at(buf, 0, 8), None);
    assert_eq!(hl_at(buf, 0, 9).as_deref(), Some("ErrorMsg"));
    assert_eq!(hl_at(buf, 0, 11).as_deref(), Some("ErrorMsg"));
    assert_eq!(hl_at(buf, 0, 12), None);
}

#[nvim_oxi::test]
fn text_fg_and_builder_spans() {
    setup();
    let (_s, buf) = surf(10, 2, || {
        col![Text::new("all").fg("Title"), Text::new("a").span("b", "Comment").raw("c")]
    });
    assert_eq!(trimmed(buf), v(&["all", "abc"]));
    assert_eq!(highlights(buf), vec![(0, 0, 3, "Title".into()), (1, 1, 2, "Comment".into())]);
}

#[nvim_oxi::test]
fn text_wide_cjk_cells_and_byte_highlights() {
    setup();
    // Wide glyphs take two cells: the Spacer pushes "x" to the last column.
    let rows = render(10, 1, || row![Text::new("日本"), Spacer::new(), Text::new("x")]);
    assert_eq!(rows, v(&["日本     x"]));
    // Wrapping is by display cells.
    let rows = render(4, 3, || Text::new("日本語").wrap(Wrap::Char));
    assert_eq!(rows, v(&["日本", "語", ""]));
    // Highlight columns are byte offsets: 日 is 3 bytes.
    let (_s, buf) = surf(10, 1, || text![("日", "Title"), " a"]);
    assert_eq!(trimmed(buf), v(&["日 a"]));
    assert_eq!(highlights(buf), vec![(0, 0, 3, "Title".into())]);
}

#[nvim_oxi::test]
fn text_wide_truncation_with_ellipsis() {
    setup();
    // 7 cells: budget 6 for text -> three wide glyphs, then the ellipsis.
    let rows = render(7, 1, || Text::new("日本語日本").ellipsis());
    assert_eq!(rows, v(&["日本語…"]));
}

// ---- Column / Row / Spacer / Expanded --------------------------------------

#[nvim_oxi::test]
fn row_spacer_pushes_apart() {
    setup();
    assert_eq!(render(10, 1, || row![Text::new("L"), Spacer::new(), Text::new("R")]), v(&["L        R"]));
}

#[nvim_oxi::test]
fn spacer_weights_share_leftover() {
    setup();
    let rows = render(12, 1, || {
        row![Text::new("a"), Spacer::new(), Text::new("b"), Spacer::new().weight(3), Text::new("c")]
    });
    assert_eq!(rows, v(&["a  b      c"]));
}

#[nvim_oxi::test]
fn row_main_axis_modes() {
    setup();
    assert_eq!(render(10, 1, || row![Text::new("ab")].main(MainAxis::Center)), v(&["    ab"]));
    assert_eq!(render(10, 1, || row![Text::new("ab")].main(MainAxis::End)), v(&["        ab"]));
    assert_eq!(
        render(9, 1, || row![Text::new("a"), Text::new("b"), Text::new("c")].main(MainAxis::SpaceBetween)),
        v(&["a   b   c"])
    );
}

#[nvim_oxi::test]
fn column_main_axis_modes() {
    setup();
    assert_eq!(render(3, 5, || col![Text::new("x")].main(MainAxis::Center)), v(&["", "", "x", "", ""]));
    assert_eq!(render(3, 3, || col![Text::new("x")].main(MainAxis::End)), v(&["", "", "x"]));
}

#[nvim_oxi::test]
fn cross_axis_modes() {
    setup();
    let rows = render(6, 5, || row![Text::new("x"), col![Text::new("a"), Text::new("b"), Text::new("c")]].cross(CrossAxis::Center));
    assert_eq!(rows, v(&["", " a", "xb", " c", ""]));
    let rows = render(5, 2, || col![Text::new("ab"), Text::new("c")].cross(CrossAxis::End));
    assert_eq!(rows, v(&["   ab", "    c"]));
    let rows = render(4, 3, || row![Divider::vertical(), Text::new("x")].cross(CrossAxis::Stretch));
    assert_eq!(rows, v(&["│x", "│", "│"]));
}

#[nvim_oxi::test]
fn expanded_fills_remaining_space() {
    setup();
    let rows = render(9, 5, || {
        col![Text::new("top"), Expanded::new(Center::new(Text::new("mid"))), Text::new("bot")]
    });
    assert_eq!(rows, v(&["top", "", "   mid", "", "bot"]));
}

#[nvim_oxi::test]
fn expanded_weights() {
    setup();
    let rows = render(9, 1, || {
        row![Expanded::new(Text::new("a")), Expanded::new(Text::new("b")).weight(2)]
    });
    assert_eq!(rows, v(&["a  b"]));
}

// ---- Padding / Center / Align / SizedBox ------------------------------------

#[nvim_oxi::test]
fn padding_variants() {
    setup();
    assert_eq!(render(6, 4, || Padding::all(1, Text::new("hi"))), v(&["", " hi", "", ""]));
    assert_eq!(render(8, 3, || Padding::symmetric(2, 1, Text::new("hi"))), v(&["", "  hi", ""]));
    assert_eq!(render(8, 3, || Padding::only(3, 1, 0, 0, Text::new("hi"))), v(&["", "   hi", ""]));
    assert_eq!(render(8, 2, || Padding::left(2, Text::new("hi"))), v(&["  hi", ""]));
    assert_eq!(render(8, 3, || Padding::top(2, Text::new("hi"))), v(&["", "", "hi"]));
}

#[nvim_oxi::test]
fn padding_shrinks_wrapping_width() {
    setup();
    let rows = render(8, 4, || Padding::all(1, Text::new("abcdefghij").wrap(Wrap::Char)));
    assert_eq!(rows, v(&["", " abcdef", " ghij", ""]));
}

#[nvim_oxi::test]
fn center_and_align() {
    setup();
    assert_eq!(render(10, 3, || Center::new(Text::new("hi"))), v(&["", "    hi", ""]));
    assert_eq!(render(6, 3, || Align::new(Alignment::BottomRight, Text::new("x"))), v(&["", "", "     x"]));
    assert_eq!(render(6, 3, || Align::new(Alignment::TopRight, Text::new("x"))), v(&["     x", "", ""]));
    assert_eq!(render(6, 3, || Align::new(Alignment::Bottom, Text::new("x"))), v(&["", "", "  x"]));
    assert_eq!(render(6, 3, || Align::new(Alignment::Left, Text::new("x"))), v(&["", "x", ""]));
}

#[nvim_oxi::test]
fn sized_box_gaps() {
    setup();
    assert_eq!(render(8, 1, || row![SizedBox::w(3), Text::new("x")]), v(&["   x"]));
    assert_eq!(render(8, 4, || col![Text::new("a"), SizedBox::h(2), Text::new("b")]), v(&["a", "", "", "b"]));
    assert_eq!(render(8, 2, || row![SizedBox::new(2, 1), Text::new("a"), SizedBox::new(2, 1), Text::new("b")]), v(&["  a  b", ""]));
}

// ---- Container / Divider ----------------------------------------------------

#[nvim_oxi::test]
fn container_border_styles() {
    setup();
    let single = render(8, 3, || Container::new(Text::new("hi")).border(BoxStyle::Single));
    assert_eq!(single, v(&["┌──────┐", "│hi    │", "└──────┘"]));
    let rounded = render(6, 3, || Container::new(Text::new("hi")));
    assert_eq!(rounded, v(&["╭────╮", "│hi  │", "╰────╯"]));
    let double = render(6, 3, || Container::new(Text::new("hi")).border(BoxStyle::Double));
    assert_eq!(double, v(&["╔════╗", "║hi  ║", "╚════╝"]));
    let thick = render(6, 3, || Container::new(Text::new("hi")).border(BoxStyle::Thick));
    assert_eq!(thick, v(&["┏━━━━┓", "┃hi  ┃", "┗━━━━┛"]));
    let none = render(6, 3, || Container::new(Text::new("hi")).border(BoxStyle::None));
    assert_eq!(none, v(&["hi", "", ""]));
    let chars = BorderChars { tl: '+', tr: '+', bl: '+', br: '+', h: '-', v: '|' };
    let custom = render(6, 3, move || Container::new(Text::new("hi")).border(BoxStyle::Custom(chars)));
    assert_eq!(custom, v(&["+----+", "|hi  |", "+----+"]));
}

#[nvim_oxi::test]
fn container_title_footer_and_padding() {
    setup();
    let rows = render(10, 3, || Container::empty().title("T"));
    assert_eq!(rows, v(&["╭── T ───╮", "│        │", "╰────────╯"]));
    let rows = render(10, 3, || Container::empty().title("T").title_pos(EdgeAlign::Left));
    assert_eq!(rows[0], "╭ T ─────╮");
    let rows = render(10, 3, || Container::empty().title("T").title_pos(EdgeAlign::Right));
    assert_eq!(rows[0], "╭───── T ╮");
    let rows = render(10, 3, || Container::empty().footer("F").footer_pos(EdgeAlign::Left));
    assert_eq!(rows[2], "╰ F ─────╯");
    let rows = render(7, 5, || Container::new(Text::new("x")).padding(1));
    assert_eq!(rows, v(&["╭─────╮", "│     │", "│ x   │", "│     │", "╰─────╯"]));
    let rows = render(7, 4, || Container::new(Text::new("x")).padding_only(2, 1, 0, 0));
    assert_eq!(rows, v(&["╭─────╮", "│     │", "│  x  │", "╰─────╯"]));
}

#[nvim_oxi::test]
fn container_border_and_title_highlights() {
    setup();
    let (_s, buf) = surf(10, 3, || Container::empty().title("T").border_hl("ErrorMsg"));
    assert_eq!(hl_at(buf, 0, 0).as_deref(), Some("ErrorMsg"));
    assert_eq!(hl_at(buf, 1, 0).as_deref(), Some("ErrorMsg"));
    // "╭──" is 9 bytes, then " T " carries the title group.
    assert_eq!(hl_at(buf, 0, 10).as_deref(), Some("AbNuiTitle"));
    assert_eq!(hl_at(buf, 0, 12).as_deref(), Some("ErrorMsg"));
    // The interior is unstyled.
    assert_eq!(hl_at(buf, 1, 3), None);
}

#[nvim_oxi::test]
fn divider_horizontal_vertical_and_style() {
    setup();
    let rows = render(6, 3, || col![Text::new("a"), Divider::new(), Text::new("b")]);
    assert_eq!(rows, v(&["a", "──────", "b"]));
    let rows = render(4, 1, || Divider::new().ch('='));
    assert_eq!(rows, v(&["===="]));
    let rows = render(3, 3, || row![Text::new("a"), Divider::vertical(), Text::new("b")]);
    assert_eq!(rows, v(&["a│b", " │", " │"]));
    let (_s, buf) = surf(4, 2, || col![Divider::new(), Divider::new().fg("Title")]);
    assert_eq!(hl_at(buf, 0, 0).as_deref(), Some("AbNuiBorder"));
    assert_eq!(hl_at(buf, 1, 0).as_deref(), Some("Title"));
}

// ---- Stack / Positioned / Fraction -------------------------------------------

#[nvim_oxi::test]
fn stack_overlays_and_positions() {
    setup();
    let rows = render(6, 2, || stack![Text::new("aaaa"), Positioned::new(1, 0, Text::new("X"))]);
    assert_eq!(rows, v(&["aXaa", ""]));
    let rows = render(4, 2, || stack![Text::new("ab"), Text::new("Z")].align(Alignment::BottomRight));
    assert_eq!(rows, v(&["", "  aZ"]));
    let rows = render(5, 2, || stack![Text::new("abc"), Positioned::new(0, 1, Text::new("d"))]);
    assert_eq!(rows, v(&["abc", "d"]));
}

#[nvim_oxi::test]
fn stack_later_child_wins_highlight() {
    setup();
    let (_s, buf) = surf(5, 1, || stack![Text::new("aaa").fg("Title"), Text::new("b")]);
    assert_eq!(trimmed(buf), v(&["baa"]));
    assert_eq!(hl_at(buf, 0, 0), None);
    assert_eq!(hl_at(buf, 0, 1).as_deref(), Some("Title"));
}

#[nvim_oxi::test]
fn fraction_sizes_child_relative_to_parent() {
    setup();
    let rows = render(20, 1, || row![Fraction::w(0.5, Divider::new()), Text::new("x")]);
    assert_eq!(rows, v(&["──────────x"]));
    let rows = render(6, 6, || col![Fraction::h(0.5, Container::empty()), Text::new("end")]);
    assert_eq!(rows, v(&["╭╮", "││", "╰╯", "end", "", ""]));
    let rows = render(10, 1, || row![Fraction::new(0.3, 1.0, Divider::new()), Text::new("x")]);
    assert_eq!(rows, v(&["───x"]));
}

#[nvim_oxi::test]
fn progress_bar_and_key_hints_text() {
    setup();
    assert_eq!(render(10, 1, || ProgressBar::new(0.5)), v(&["█████░░░░░"]));
    assert_eq!(render(10, 1, || ProgressBar::new(0.5).label("50%")), v(&["███50%░░░░"]));
    assert_eq!(render(10, 1, || ProgressBar::new(0.0)), v(&["░░░░░░░░░░"]));
    assert_eq!(render(10, 1, || ProgressBar::new(2.0)), v(&["██████████"]));
    assert_eq!(render(6, 1, || ProgressBar::new(1.0).chars('#', '-')), v(&["######"]));
    assert_eq!(render(30, 2, || KeyHints::new(vec![("q", "quit"), ("Tab", "focus")])), v(&["q quit · Tab focus", ""]));
    // Hints wrap on word boundaries when narrow.
    assert_eq!(render(10, 3, || KeyHints::new(vec![("q", "quit"), ("Tab", "focus")])), v(&["q quit ·", "Tab focus", ""]));
}

#[nvim_oxi::test]
fn progress_and_hints_highlights() {
    setup();
    let (_s, buf) = surf(10, 3, || col![ProgressBar::new(0.5), KeyHints::new(vec![("q", "quit"), ("Tab", "go")])]);
    assert_eq!(hl_at(buf, 0, 0).as_deref(), Some("AbNuiAccent"));
    assert_eq!(hl_at(buf, 0, 23).as_deref(), Some("AbNuiMuted"));
    // Key names are accented, labels and separators muted.
    assert_eq!(hl_at(buf, 1, 0).as_deref(), Some("AbNuiAccent"));
    assert_eq!(hl_at(buf, 1, 2).as_deref(), Some("AbNuiMuted"));
    assert_eq!(hl_at(buf, 2, 0).as_deref(), Some("AbNuiAccent"));
}
