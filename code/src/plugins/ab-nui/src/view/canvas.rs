//! A cell grid that widgets paint onto, which compiles down to buffer
//! [`Line`]s + highlights. This is the bridge between the declarative widget
//! tree and the existing rendering core.

use std::rc::Rc;

use crate::text::{Line, Span};
use crate::view::widget::Area;

/// One terminal cell: a character and an optional highlight group.
#[derive(Clone)]
pub struct Cell {
    pub ch: char,
    pub hl: Option<Rc<str>>,
}

impl Default for Cell {
    fn default() -> Self {
        Self { ch: ' ', hl: None }
    }
}

/// A fixed-size grid of cells (row-major).
pub struct Canvas {
    pub w: u16,
    pub h: u16,
    cells: Vec<Cell>,
}

impl Canvas {
    pub fn new(w: u16, h: u16) -> Self {
        let len = (w as usize) * (h as usize);
        Self { w, h, cells: vec![Cell::default(); len] }
    }

    fn index(&self, x: u16, y: u16) -> Option<usize> {
        if x < self.w && y < self.h {
            Some(y as usize * self.w as usize + x as usize)
        } else {
            None
        }
    }

    /// Paint a single cell.
    pub fn set(&mut self, x: u16, y: u16, ch: char, hl: Option<Rc<str>>) {
        if let Some(i) = self.index(x, y) {
            self.cells[i] = Cell { ch, hl };
        }
    }

    /// Paint a string starting at `(x, y)`, clipped to the canvas width.
    pub fn put_str(&mut self, x: u16, y: u16, s: &str, hl: Option<Rc<str>>) {
        for (i, ch) in s.chars().enumerate() {
            let col = x + i as u16;
            if col >= self.w {
                break;
            }
            self.set(col, y, ch, hl.clone());
        }
    }

    /// Fill a rectangular area with a character.
    pub fn fill(&mut self, area: Area, ch: char, hl: Option<Rc<str>>) {
        for y in area.y..area.y.saturating_add(area.h) {
            for x in area.x..area.x.saturating_add(area.w) {
                self.set(x, y, ch, hl.clone());
            }
        }
    }

    /// Convert the grid into one styled [`Line`] per row, merging adjacent
    /// cells that share a highlight into a single [`Span`].
    pub fn to_lines(&self) -> Vec<Line> {
        let mut lines = Vec::with_capacity(self.h as usize);
        for y in 0..self.h {
            let mut spans: Vec<Span> = Vec::new();
            let mut run = String::new();
            let mut run_hl: Option<Rc<str>> = None;
            let mut started = false;

            for x in 0..self.w {
                let cell = &self.cells[y as usize * self.w as usize + x as usize];
                if !started {
                    run.push(cell.ch);
                    run_hl = cell.hl.clone();
                    started = true;
                } else if same_hl(&run_hl, &cell.hl) {
                    run.push(cell.ch);
                } else {
                    spans.push(make_span(&run, &run_hl));
                    run.clear();
                    run.push(cell.ch);
                    run_hl = cell.hl.clone();
                }
            }
            if started {
                spans.push(make_span(&run, &run_hl));
            }
            lines.push(Line { spans });
        }
        lines
    }
}

fn same_hl(a: &Option<Rc<str>>, b: &Option<Rc<str>>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(x), Some(y)) => x.as_ref() == y.as_ref(),
        _ => false,
    }
}

fn make_span(text: &str, hl: &Option<Rc<str>>) -> Span {
    Span { text: text.to_string(), hl: hl.as_ref().map(|r| r.to_string()) }
}
