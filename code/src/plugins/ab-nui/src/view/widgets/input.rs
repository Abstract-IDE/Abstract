//! Text entry widgets: the single-line [`TextField`] and multi-line
//! [`TextArea`]. Input arrives through the runtime's key routing (no Neovim
//! insert mode), so both compose like any other widget.
//!
//! Cursor and scroll live in state bundles ([`FieldState`], [`AreaState`])
//! because the widget tree is rebuilt every frame — create the state once,
//! outside the build closure.

use std::rc::Rc;

use crate::reactive::Signal;
use crate::text::char_width;
use crate::theme::groups;
use crate::view::canvas::Canvas;
use crate::view::widget::{Area, Constraints, Cx, Key, Size, Widget};

/// Byte index of char `i` in `s` (or `s.len()` past the end).
fn byte_at(s: &str, i: usize) -> usize {
    s.char_indices().nth(i).map(|(b, _)| b).unwrap_or(s.len())
}

/// Delete the word before char index `cursor`; returns the new cursor.
fn delete_word_back(s: &mut String, cursor: usize) -> usize {
    let chars: Vec<char> = s.chars().collect();
    let mut i = cursor.min(chars.len());
    while i > 0 && chars[i - 1] == ' ' {
        i -= 1;
    }
    while i > 0 && chars[i - 1] != ' ' {
        i -= 1;
    }
    let start = byte_at(s, i);
    let end = byte_at(s, cursor.min(chars.len()));
    s.replace_range(start..end, "");
    i
}

/// The signals a [`TextField`] needs across rebuilds.
#[derive(Clone)]
pub struct FieldState {
    pub value: Signal<String>,
    /// Cursor as a char index into `value`.
    pub cursor: Signal<usize>,
    /// First visible char (horizontal scroll) — managed by the widget.
    pub scroll: Signal<usize>,
}

impl FieldState {
    pub fn new(initial: impl Into<String>) -> Self {
        let initial = initial.into();
        let len = initial.chars().count();
        Self { value: Signal::new(initial), cursor: Signal::new(len), scroll: Signal::new(0) }
    }

    /// Adopt an existing value signal.
    pub fn from_signal(value: Signal<String>) -> Self {
        let len = value.get_untracked().chars().count();
        Self { value, cursor: Signal::new(len), scroll: Signal::new(0) }
    }

    pub fn clear(&self) {
        self.cursor.set_silent(0);
        self.scroll.set_silent(0);
        self.value.set(String::new());
    }
}

/// A single-line text field. Focused, it handles chars, `<BS>`/`<Del>`,
/// `<Left>`/`<Right>`/`<Home>`/`<End>`, `<C-w>` (delete word) and `<C-u>`
/// (kill to start); `<CR>` fires `on_submit`.
pub struct TextField {
    state: FieldState,
    placeholder: String,
    mask: Option<char>,
    on_change: Rc<dyn Fn(&str)>,
    on_submit: Rc<dyn Fn(&str)>,
}

impl TextField {
    pub fn new(state: &FieldState) -> Self {
        Self {
            state: state.clone(),
            placeholder: String::new(),
            mask: None,
            on_change: Rc::new(|_| {}),
            on_submit: Rc::new(|_| {}),
        }
    }
    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = text.into();
        self
    }
    /// Password entry: paint `ch` for every char.
    pub fn mask(mut self, ch: char) -> Self {
        self.mask = Some(ch);
        self
    }
    pub fn on_change(mut self, f: impl Fn(&str) + 'static) -> Self {
        self.on_change = Rc::new(f);
        self
    }
    pub fn on_submit(mut self, f: impl Fn(&str) + 'static) -> Self {
        self.on_submit = Rc::new(f);
        self
    }

    fn display_text(&self, value: &str) -> String {
        match self.mask {
            Some(m) => std::iter::repeat_n(m, value.chars().count()).collect(),
            None => value.to_string(),
        }
    }
}

impl Widget for TextField {
    fn measure(&self, c: Constraints) -> Size {
        Size::new(c.max_w, 1)
    }

    fn paint(&self, cx: &mut Cx, area: Area, canvas: &mut Canvas) {
        let focused = cx.will_focus();
        let value = self.state.value.get();
        let text = self.display_text(&value);
        let chars: Vec<char> = text.chars().collect();
        let cursor = self.state.cursor.get_untracked().min(chars.len());
        // The cursor signal must repaint us when it moves.
        self.state.cursor.with(|_| ());

        canvas.fill(area, ' ', Some(Rc::from(groups::FIELD)));

        if chars.is_empty() && !focused && !self.placeholder.is_empty() {
            canvas.put_str(area.x, area.y, &self.placeholder, Some(Rc::from(groups::MUTED)));
        } else {
            // Horizontal scroll so the cursor stays visible (display-width aware,
            // leaving one cell for the cursor itself).
            let w = area.w.max(1);
            let mut scroll = self.state.scroll.get_untracked().min(chars.len());
            if cursor < scroll {
                scroll = cursor;
            }
            let width_between = |from: usize, to: usize| -> u16 {
                chars[from..to].iter().map(|&c| char_width(c)).sum()
            };
            while width_between(scroll, cursor) + 1 > w {
                scroll += 1;
            }
            self.state.scroll.set_silent(scroll);

            let mut x = area.x;
            for (i, &ch) in chars.iter().enumerate().skip(scroll) {
                let cw = char_width(ch);
                if x + cw > area.x + area.w {
                    break;
                }
                let hl = if focused && i == cursor { groups::CURSOR } else { groups::FIELD };
                canvas.set(x, area.y, ch, Some(Rc::from(hl)));
                x += cw;
            }
            // Cursor past the last char.
            if focused && cursor >= chars.len() && x < area.x + area.w {
                canvas.set(x, area.y, ' ', Some(Rc::from(groups::CURSOR)));
            }
        }

        // Key handling.
        let state = self.state.clone();
        let on_change = self.on_change.clone();
        let on_submit = self.on_submit.clone();
        cx.register(
            area,
            Rc::new(move |k| {
                let len = state.value.get_untracked().chars().count();
                let cur = state.cursor.get_untracked().min(len);
                let changed = |state: &FieldState, f: &Rc<dyn Fn(&str)>| {
                    f(&state.value.get_untracked());
                };
                match k {
                    Key::Char(c) => {
                        state.value.update(|s| {
                            let at = byte_at(s, cur);
                            s.insert(at, c);
                        });
                        state.cursor.set(cur + 1);
                        changed(&state, &on_change);
                        true
                    }
                    Key::Backspace if cur > 0 => {
                        state.value.update(|s| {
                            let start = byte_at(s, cur - 1);
                            let end = byte_at(s, cur);
                            s.replace_range(start..end, "");
                        });
                        state.cursor.set(cur - 1);
                        changed(&state, &on_change);
                        true
                    }
                    Key::Backspace => true, // consumed, nothing to delete
                    Key::Delete if cur < len => {
                        state.value.update(|s| {
                            let start = byte_at(s, cur);
                            let end = byte_at(s, cur + 1);
                            s.replace_range(start..end, "");
                        });
                        changed(&state, &on_change);
                        true
                    }
                    Key::Delete => true,
                    Key::Left => {
                        state.cursor.set(cur.saturating_sub(1));
                        true
                    }
                    Key::Right => {
                        state.cursor.set((cur + 1).min(len));
                        true
                    }
                    Key::Home => {
                        state.cursor.set(0);
                        true
                    }
                    Key::End => {
                        state.cursor.set(len);
                        true
                    }
                    Key::Ctrl('w') => {
                        let mut new_cur = cur;
                        state.value.update(|s| new_cur = delete_word_back(s, cur));
                        state.cursor.set(new_cur);
                        changed(&state, &on_change);
                        true
                    }
                    Key::Ctrl('u') => {
                        state.value.update(|s| {
                            let end = byte_at(s, cur);
                            s.replace_range(..end, "");
                        });
                        state.cursor.set(0);
                        changed(&state, &on_change);
                        true
                    }
                    Key::Enter => {
                        on_submit(&state.value.get_untracked());
                        true
                    }
                    _ => false,
                }
            }),
        );
    }
}

// -----------------------------------------------------------------------------
// TextArea
// -----------------------------------------------------------------------------

/// The signals a [`TextArea`] needs across rebuilds.
#[derive(Clone)]
pub struct AreaState {
    pub lines: Signal<Vec<String>>,
    /// Cursor as `(row, char col)`.
    pub cursor: Signal<(usize, usize)>,
    /// `(first visible row, first visible char col)` — managed by the widget.
    pub scroll: Signal<(usize, usize)>,
}

impl AreaState {
    pub fn new(initial: impl Into<String>) -> Self {
        let lines: Vec<String> = {
            let s = initial.into();
            if s.is_empty() { vec![String::new()] } else { s.lines().map(str::to_string).collect() }
        };
        Self { lines: Signal::new(lines), cursor: Signal::new((0, 0)), scroll: Signal::new((0, 0)) }
    }

    /// The full text, newline-joined.
    pub fn text(&self) -> String {
        self.lines.get_untracked().join("\n")
    }
}

/// A callback invoked with the current lines on every edit.
type LinesHandler = Rc<dyn Fn(&[String])>;

/// A multi-line editor: chars, `<CR>` (newline), `<BS>`/`<Del>` (with line
/// join), arrows, `<Home>`/`<End>`, `<PageUp>`/`<PageDown>`. Greedy: fills the
/// available area (wrap it in `SizedBox`/`Expanded` to size it).
pub struct TextArea {
    state: AreaState,
    placeholder: String,
    on_change: LinesHandler,
}

impl TextArea {
    pub fn new(state: &AreaState) -> Self {
        Self { state: state.clone(), placeholder: String::new(), on_change: Rc::new(|_| {}) }
    }
    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = text.into();
        self
    }
    pub fn on_change(mut self, f: impl Fn(&[String]) + 'static) -> Self {
        self.on_change = Rc::new(f);
        self
    }
}

impl Widget for TextArea {
    fn measure(&self, c: Constraints) -> Size {
        Size::new(c.max_w, c.max_h)
    }

    fn paint(&self, cx: &mut Cx, area: Area, canvas: &mut Canvas) {
        let focused = cx.will_focus();
        let lines = self.state.lines.get();
        let (crow, ccol) = {
            let (r, c) = self.state.cursor.get();
            let r = r.min(lines.len().saturating_sub(1));
            (r, c.min(lines.get(r).map(|l| l.chars().count()).unwrap_or(0)))
        };

        canvas.fill(area, ' ', Some(Rc::from(groups::FIELD)));

        let empty = lines.len() == 1 && lines[0].is_empty();
        if empty && !focused && !self.placeholder.is_empty() {
            canvas.put_str(area.x, area.y, &self.placeholder, Some(Rc::from(groups::MUTED)));
        } else {
            // Scroll to keep the cursor visible.
            let (mut top, mut left) = self.state.scroll.get_untracked();
            let h = area.h.max(1) as usize;
            let w = area.w.max(1) as usize;
            if crow < top {
                top = crow;
            } else if crow >= top + h {
                top = crow - h + 1;
            }
            if ccol < left {
                left = ccol;
            } else if ccol >= left + w {
                left = ccol - w + 1;
            }
            self.state.scroll.set_silent((top, left));

            for (row, line) in lines.iter().enumerate().skip(top).take(h) {
                let y = area.y + (row - top) as u16;
                let mut x = area.x;
                for (i, ch) in line.chars().enumerate().skip(left) {
                    let cw = char_width(ch);
                    if x + cw > area.x + area.w {
                        break;
                    }
                    let hl = if focused && row == crow && i == ccol { groups::CURSOR } else { groups::FIELD };
                    canvas.set(x, y, ch, Some(Rc::from(hl)));
                    x += cw;
                }
                // Cursor past the end of its line.
                if focused && row == crow && ccol >= line.chars().count() && x < area.x + area.w {
                    canvas.set(x, y, ' ', Some(Rc::from(groups::CURSOR)));
                }
            }
        }

        // Key handling.
        let state = self.state.clone();
        let on_change = self.on_change.clone();
        let page = area.h.max(1) as usize;
        cx.register(
            area,
            Rc::new(move |k| {
                let lines = state.lines.get_untracked();
                let (mut row, mut col) = state.cursor.get_untracked();
                row = row.min(lines.len().saturating_sub(1));
                let line_len = |r: usize| lines.get(r).map(|l| l.chars().count()).unwrap_or(0);
                col = col.min(line_len(row));
                let changed = |state: &AreaState, f: &LinesHandler| {
                    f(&state.lines.get_untracked());
                };
                match k {
                    Key::Char(c) => {
                        state.lines.update(|ls| {
                            let at = byte_at(&ls[row], col);
                            ls[row].insert(at, c);
                        });
                        state.cursor.set((row, col + 1));
                        changed(&state, &on_change);
                        true
                    }
                    Key::Enter => {
                        state.lines.update(|ls| {
                            let at = byte_at(&ls[row], col);
                            let rest = ls[row].split_off(at);
                            ls.insert(row + 1, rest);
                        });
                        state.cursor.set((row + 1, 0));
                        changed(&state, &on_change);
                        true
                    }
                    Key::Backspace => {
                        if col > 0 {
                            state.lines.update(|ls| {
                                let start = byte_at(&ls[row], col - 1);
                                let end = byte_at(&ls[row], col);
                                ls[row].replace_range(start..end, "");
                            });
                            state.cursor.set((row, col - 1));
                            changed(&state, &on_change);
                        } else if row > 0 {
                            let prev_len = line_len(row - 1);
                            state.lines.update(|ls| {
                                let cur = ls.remove(row);
                                ls[row - 1].push_str(&cur);
                            });
                            state.cursor.set((row - 1, prev_len));
                            changed(&state, &on_change);
                        }
                        true
                    }
                    Key::Delete => {
                        if col < line_len(row) {
                            state.lines.update(|ls| {
                                let start = byte_at(&ls[row], col);
                                let end = byte_at(&ls[row], col + 1);
                                ls[row].replace_range(start..end, "");
                            });
                            changed(&state, &on_change);
                        } else if row + 1 < lines.len() {
                            state.lines.update(|ls| {
                                let next = ls.remove(row + 1);
                                ls[row].push_str(&next);
                            });
                            changed(&state, &on_change);
                        }
                        true
                    }
                    Key::Left => {
                        if col > 0 {
                            state.cursor.set((row, col - 1));
                        } else if row > 0 {
                            state.cursor.set((row - 1, line_len(row - 1)));
                        }
                        true
                    }
                    Key::Right => {
                        if col < line_len(row) {
                            state.cursor.set((row, col + 1));
                        } else if row + 1 < lines.len() {
                            state.cursor.set((row + 1, 0));
                        }
                        true
                    }
                    Key::Up => {
                        if row > 0 {
                            state.cursor.set((row - 1, col.min(line_len(row - 1))));
                        }
                        true
                    }
                    Key::Down => {
                        if row + 1 < lines.len() {
                            state.cursor.set((row + 1, col.min(line_len(row + 1))));
                        }
                        true
                    }
                    Key::Home => {
                        state.cursor.set((row, 0));
                        true
                    }
                    Key::End => {
                        state.cursor.set((row, line_len(row)));
                        true
                    }
                    Key::PageDown => {
                        let r = (row + page).min(lines.len() - 1);
                        state.cursor.set((r, col.min(line_len(r))));
                        true
                    }
                    Key::PageUp => {
                        let r = row.saturating_sub(page);
                        state.cursor.set((r, col.min(line_len(r))));
                        true
                    }
                    _ => false,
                }
            }),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn byte_at_handles_multibyte() {
        assert_eq!(byte_at("aé日", 0), 0);
        assert_eq!(byte_at("aé日", 1), 1);
        assert_eq!(byte_at("aé日", 2), 3);
        assert_eq!(byte_at("aé日", 3), 6);
    }

    #[test]
    fn delete_word_back_removes_word_and_spaces() {
        let mut s = String::from("hello world  ");
        let end = s.chars().count();
        let cur = delete_word_back(&mut s, end);
        assert_eq!(s, "hello ");
        assert_eq!(cur, 6);
    }
}
