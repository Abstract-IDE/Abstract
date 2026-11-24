//! Painting styled rows into a buffer — with diffing.
//!
//! A frame is a list of [`Row`]s: the row's text plus the byte ranges to
//! highlight. A [`Renderer`] remembers the last frame it painted into its
//! buffer and, for the next one, sends Neovim only the rows that changed —
//! in a single call into Lua (see [`crate::lua`]'s driver), however many rows
//! and highlights are involved.
//!
//! For a typical interaction (moving a selection, ticking a spinner, typing a
//! character) that is one or two rows instead of the whole buffer, which also
//! means Neovim only has to redraw those rows.

use std::cell::RefCell;
use std::rc::Rc;

use mlua::{Table, Value};

use crate::error::{Error, Result};
use crate::lua;
use crate::nvim::buffer::Buffer;
use crate::nvim::extmark::Namespace;
use crate::text::Line;

/// A highlighted byte range `[start, end)` of a row.
#[derive(Clone, Debug)]
pub struct Mark {
    pub start: u32,
    pub end: u32,
    pub hl: Rc<str>,
}

impl PartialEq for Mark {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start
            && self.end == other.end
            && (Rc::ptr_eq(&self.hl, &other.hl) || self.hl == other.hl)
    }
}

/// One buffer line of a frame: its text and its highlights (in column order).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Row {
    pub text: String,
    pub marks: Vec<Mark>,
}

impl Row {
    /// Flatten a styled [`Line`]: concatenate the span texts and turn each
    /// highlighted span into a [`Mark`] (adjacent spans sharing a group merge).
    pub fn from_line(line: &Line) -> Self {
        let mut row = Row { text: String::with_capacity(line.spans.iter().map(|s| s.text.len()).sum()), marks: Vec::new() };
        for span in &line.spans {
            let start = row.text.len() as u32;
            row.text.push_str(&span.text);
            let end = row.text.len() as u32;
            let Some(hl) = &span.hl else { continue };
            if start == end {
                continue;
            }
            match row.marks.last_mut() {
                Some(last) if last.end == start && *last.hl == **hl => last.end = end,
                _ => row.marks.push(Mark { start, end, hl: Rc::from(hl.as_str()) }),
            }
        }
        row
    }
}

/// What a [`Renderer`] knows about its buffer.
struct State {
    /// The frame currently in the buffer.
    rows: Vec<Row>,
    /// The buffer's `changedtick` after that frame was painted; `None` when
    /// the buffer's contents are unknown (nothing painted yet, or it was
    /// changed behind our back).
    tick: Option<i64>,
}

/// Paints frames into one buffer/namespace, diffing against the previous one.
pub struct Renderer {
    buf: Buffer,
    ns: Namespace,
    state: RefCell<State>,
}

impl Renderer {
    pub fn new(buf: Buffer, ns: Namespace) -> Self {
        Self { buf, ns, state: RefCell::new(State { rows: Vec::new(), tick: None }) }
    }

    /// Forget the last frame, so the next [`render`](Self::render) repaints
    /// the whole buffer. Needed after writing to the buffer by other means.
    pub fn invalidate(&self) {
        if let Ok(mut state) = self.state.try_borrow_mut() {
            state.tick = None;
        }
    }

    /// Make the buffer show `rows`.
    pub fn render(&self, rows: Vec<Row>) -> Result<()> {
        // Take the previous frame out instead of holding a borrow across the
        // call into Neovim: should that call re-enter this renderer, the
        // nested render simply sees an unknown buffer and paints in full.
        let (prev, tick) = {
            let Ok(mut state) = self.state.try_borrow_mut() else {
                return Err(Error::invalid("re-entrant render"));
            };
            (std::mem::take(&mut state.rows), state.tick.take())
        };

        let mut painted = None;
        if let Some(tick) = tick
            && prev.len() == rows.len()
        {
            let runs = changed_runs(&prev, &rows);
            if runs.is_empty() {
                painted = Some(tick);
            } else {
                let encoded = encode_runs(&rows, &runs)?;
                let new_tick: i64 =
                    lua::driver_fn("render")?.call((self.buf.id(), self.ns.id(), tick, rows.len(), false, encoded))?;
                match new_tick {
                    -2 => return Err(Error::invalid("buffer is no longer valid")),
                    // -1: the buffer is not what we diffed against → repaint.
                    t if t >= 0 => painted = Some(t),
                    _ => {}
                }
            }
        }

        let tick = match painted {
            Some(tick) => tick,
            None => {
                let whole = Run { start: 0, len: rows.len(), text_changed: true };
                let encoded = encode_runs(&rows, &[whole])?;
                let tick: i64 =
                    lua::driver_fn("render")?.call((self.buf.id(), self.ns.id(), -1, rows.len(), true, encoded))?;
                if tick < 0 {
                    return Err(Error::invalid("buffer is no longer valid"));
                }
                tick
            }
        };

        if let Ok(mut state) = self.state.try_borrow_mut() {
            // If a nested render already recorded a frame, the buffer is a mix
            // of both: leave it marked unknown so the next frame repaints.
            if state.tick.is_none() && state.rows.is_empty() {
                state.rows = rows;
                state.tick = Some(tick);
            } else {
                state.tick = None;
                state.rows.clear();
            }
        }
        Ok(())
    }
}

/// A maximal range of consecutive rows that differ from the previous frame.
struct Run {
    start: usize,
    len: usize,
    /// Whether any row's text (not just its highlights) changed.
    text_changed: bool,
}

fn changed_runs(prev: &[Row], next: &[Row]) -> Vec<Run> {
    let mut runs: Vec<Run> = Vec::new();
    for (i, (old, new)) in prev.iter().zip(next).enumerate() {
        let text_changed = old.text != new.text;
        if !text_changed && old.marks == new.marks {
            continue;
        }
        match runs.last_mut() {
            Some(run) if run.start + run.len == i => {
                run.len += 1;
                run.text_changed |= text_changed;
            }
            _ => runs.push(Run { start: i, len: 1, text_changed }),
        }
    }
    runs
}

/// Encode runs for the driver: a flat list of
/// `start_row, row_count, lines | false, marks` quads, where `marks` is a flat
/// list of `row, start_col, end_col, hl_group` quads.
fn encode_runs(rows: &[Row], runs: &[Run]) -> Result<Table> {
    let l = lua::lua()?;
    let out = l.create_table_with_capacity(runs.len() * 4, 0)?;
    for run in runs {
        let slice = &rows[run.start..run.start + run.len];
        out.raw_push(run.start)?;
        out.raw_push(run.len)?;
        if run.text_changed {
            out.raw_push(l.create_sequence_from(slice.iter().map(|r| r.text.as_str()))?)?;
        } else {
            out.raw_push(false)?;
        }
        let mark_count: usize = slice.iter().map(|r| r.marks.len()).sum();
        let marks = l.create_table_with_capacity(mark_count * 4, 0)?;
        for (offset, row) in slice.iter().enumerate() {
            for mark in &row.marks {
                marks.raw_push(run.start + offset)?;
                marks.raw_push(mark.start)?;
                marks.raw_push(mark.end)?;
                marks.raw_push(Value::String(l.create_string(&*mark.hl)?))?;
            }
        }
        out.raw_push(marks)?;
    }
    Ok(out)
}

/// Render styled [`Line`]s into `buf`, painting span highlights via `ns`.
///
/// This is the stateless form: it replaces the whole buffer and every extmark
/// of the namespace. Anything that repaints the same buffer repeatedly should
/// keep a [`Renderer`] instead, so only changes are sent.
pub fn render(buf: Buffer, ns: Namespace, lines: &[Line]) -> Result<()> {
    Renderer::new(buf, ns).render(lines.iter().map(Row::from_line).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::text::Span;

    fn row(text: &str, marks: &[(u32, u32, &str)]) -> Row {
        Row {
            text: text.into(),
            marks: marks.iter().map(|&(start, end, hl)| Mark { start, end, hl: Rc::from(hl) }).collect(),
        }
    }

    #[test]
    fn from_line_uses_byte_offsets_and_merges_neighbours() {
        let line = Line::from_spans([
            Span::raw("é "),
            Span::hl("ab", "A"),
            Span::hl("cd", "A"),
            Span::hl("", "B"),
            Span::hl("ef", "B"),
            Span::raw("gh"),
        ]);
        assert_eq!(Row::from_line(&line), row("é abcdefgh", &[(3, 7, "A"), (7, 9, "B")]));
    }

    #[test]
    fn identical_frames_have_no_runs() {
        let a = vec![row("one", &[(0, 3, "A")]), row("two", &[])];
        assert!(changed_runs(&a, &a.clone()).is_empty());
    }

    #[test]
    fn runs_group_consecutive_changes() {
        let prev = vec![row("a", &[]), row("b", &[]), row("c", &[]), row("d", &[]), row("e", &[])];
        let mut next = prev.clone();
        next[1].text = "B".into();
        next[2].marks.push(Mark { start: 0, end: 1, hl: Rc::from("X") });
        next[4].marks.push(Mark { start: 0, end: 1, hl: Rc::from("X") });
        let runs = changed_runs(&prev, &next);
        assert_eq!(runs.len(), 2);
        assert_eq!((runs[0].start, runs[0].len, runs[0].text_changed), (1, 2, true));
        // Only a highlight changed here: the text need not be resent.
        assert_eq!((runs[1].start, runs[1].len, runs[1].text_changed), (4, 1, false));
    }

    #[test]
    fn marks_compare_by_group_name() {
        let a = Mark { start: 0, end: 1, hl: Rc::from("Same") };
        let b = Mark { start: 0, end: 1, hl: Rc::from("Same") };
        assert_eq!(a, b);
        assert_ne!(a, Mark { start: 0, end: 1, hl: Rc::from("Other") });
    }
}
