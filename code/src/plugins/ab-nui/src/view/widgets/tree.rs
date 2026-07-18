//! `Tree` — a collapsible hierarchy with custom node rendering.

use std::collections::HashSet;
use std::rc::Rc;

use crate::reactive::Signal;
use crate::text::{Line, Span};
use crate::theme::groups;
use crate::view::canvas::Canvas;
use crate::view::widget::{Area, Constraints, Cx, Key, Size, Widget};
use crate::view::widgets::list::ItemCx;
use crate::view::widgets::scroll::draw_scrollbar;

/// A node: your data plus children.
#[derive(Clone)]
pub struct TreeNode<T> {
    pub data: T,
    pub children: Vec<TreeNode<T>>,
}

impl<T> TreeNode<T> {
    pub fn leaf(data: T) -> Self {
        Self { data, children: Vec::new() }
    }
    pub fn new(data: T, children: Vec<TreeNode<T>>) -> Self {
        Self { data, children }
    }
}

/// The signals a [`Tree`] needs across rebuilds.
#[derive(Clone)]
pub struct TreeState {
    /// Ids (per [`Tree::id`]) of expanded nodes.
    pub expanded: Signal<HashSet<String>>,
    /// Selected row in the FLATTENED visible list.
    pub selected: Signal<usize>,
    /// First visible row.
    pub offset: Signal<usize>,
}

impl TreeState {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { expanded: Signal::new(HashSet::new()), selected: Signal::new(0), offset: Signal::new(0) }
    }
}

/// One flattened, visible row.
struct FlatRow<T> {
    data: T,
    id: String,
    depth: u16,
    has_children: bool,
    expanded: bool,
    parent_row: Option<usize>,
}

type IdFn<T> = Rc<dyn Fn(&T) -> String>;
type RenderFn<T> = Rc<dyn Fn(&T, &ItemCx) -> Line>;

/// A collapsible tree. Keys (focused): `j`/`k` move, `l`/`<CR>` expand (or
/// activate a leaf), `h` collapse (or jump to parent), `<Space>` toggles.
pub struct Tree<T: Clone + 'static> {
    roots: Rc<Vec<TreeNode<T>>>,
    state: TreeState,
    id: IdFn<T>,
    render: RenderFn<T>,
    on_activate: Rc<dyn Fn(&T)>,
    show_scrollbar: bool,
}

impl<T: Clone + ToString + 'static> Tree<T> {
    /// A tree rendering nodes via `ToString` and identifying them by their
    /// rendered text. Set [`Tree::id`] when node texts aren't unique.
    pub fn new(roots: Vec<TreeNode<T>>, state: &TreeState) -> Self {
        Self {
            roots: Rc::new(roots),
            state: state.clone(),
            id: Rc::new(|d: &T| d.to_string()),
            render: Rc::new(|d: &T, _| Line::raw(d.to_string())),
            on_activate: Rc::new(|_| {}),
            show_scrollbar: true,
        }
    }
}

impl<T: Clone + 'static> Tree<T> {
    /// Stable node key for the expanded set (defaults to `ToString`).
    pub fn id(mut self, f: impl Fn(&T) -> String + 'static) -> Self {
        self.id = Rc::new(f);
        self
    }
    /// Custom node rendering (indent and `▸/▾` glyphs are painted for you).
    pub fn render(mut self, f: impl Fn(&T, &ItemCx) -> Line + 'static) -> Self {
        self.render = Rc::new(f);
        self
    }
    /// `<CR>`/`l` on a leaf.
    pub fn on_activate(mut self, f: impl Fn(&T) + 'static) -> Self {
        self.on_activate = Rc::new(f);
        self
    }
    pub fn scrollbar(mut self, show: bool) -> Self {
        self.show_scrollbar = show;
        self
    }

    /// Flatten the visible rows (DFS honoring the expanded set).
    fn flatten(&self, expanded: &HashSet<String>) -> Vec<FlatRow<T>> {
        fn walk<T: Clone>(
            nodes: &[TreeNode<T>],
            depth: u16,
            parent_row: Option<usize>,
            id: &IdFn<T>,
            expanded: &HashSet<String>,
            out: &mut Vec<FlatRow<T>>,
        ) {
            for node in nodes {
                let nid = id(&node.data);
                let is_expanded = expanded.contains(&nid);
                let row = out.len();
                out.push(FlatRow {
                    data: node.data.clone(),
                    id: nid,
                    depth,
                    has_children: !node.children.is_empty(),
                    expanded: is_expanded,
                    parent_row,
                });
                if is_expanded {
                    walk(&node.children, depth + 1, Some(row), id, expanded, out);
                }
            }
        }
        let mut out = Vec::new();
        walk(&self.roots, 0, None, &self.id, expanded, &mut out);
        out
    }
}

impl<T: Clone + 'static> Widget for Tree<T> {
    fn measure(&self, c: Constraints) -> Size {
        let rows = self.state.expanded.with(|e| self.flatten(e));
        let probe = ItemCx { index: 0, selected: false, marked: false, focused: false, width: c.max_w };
        let w = rows
            .iter()
            .map(|r| (self.render)(&r.data, &probe).width() + r.depth * 2 + 4)
            .max()
            .unwrap_or(0);
        c.clamp(Size::new(w.min(c.max_w), (rows.len() as u16).min(c.max_h)))
    }

    fn paint(&self, cx: &mut Cx, area: Area, canvas: &mut Canvas) {
        let focused = cx.will_focus();
        let expanded = self.state.expanded.get(); // tracked: repaint on toggle
        let rows = self.flatten(&expanded);
        let n = rows.len();
        let h = area.h as usize;

        let selected = self.state.selected.get().min(n.saturating_sub(1));
        self.state.selected.set_silent(selected);

        // Scroll window keeps the selection visible.
        let mut offset = self.state.offset.get_untracked().min(n.saturating_sub(1));
        if selected < offset {
            offset = selected;
        } else if h > 0 && selected >= offset + h {
            offset = selected - h + 1;
        }
        self.state.offset.set_silent(offset);

        let bar_w = if self.show_scrollbar { 1 } else { 0 };
        for (screen_row, row_idx) in (offset..n.min(offset + h)).enumerate() {
            let row = &rows[row_idx];
            let is_sel = row_idx == selected;
            let glyph = match (row.has_children, row.expanded) {
                (true, true) => "▾ ",
                (true, false) => "▸ ",
                (false, _) => "  ",
            };
            let icx = ItemCx {
                index: row_idx,
                selected: is_sel,
                marked: false,
                focused,
                width: area.w.saturating_sub(row.depth * 2 + 2 + bar_w),
            };
            let mut line = Line::empty();
            line.spans.push(Span::raw(" ".repeat((row.depth * 2) as usize)));
            line.spans.push(Span::hl(glyph, groups::MUTED));
            let rendered = (self.render)(&row.data, &icx);
            if is_sel {
                for span in rendered.spans {
                    line.spans.push(Span {
                        text: span.text,
                        hl: Some(if focused { groups::SELECTION } else { groups::FOCUS }.to_string()),
                    });
                }
            } else {
                line.spans.extend(rendered.spans);
            }
            canvas.put_line(area.x, area.y + screen_row as u16, &line);
        }

        if self.show_scrollbar {
            draw_scrollbar(
                canvas,
                Area { x: area.x + area.w.saturating_sub(1), y: area.y, w: 1, h: area.h },
                n,
                h,
                offset,
            );
        }

        // Key handling operates on the flattened rows.
        let state = self.state.clone();
        let on_activate = self.on_activate.clone();
        let flat: Rc<Vec<FlatRow<T>>> = Rc::new(rows);
        cx.register(
            area,
            Rc::new(move |k| {
                let n = flat.len();
                if n == 0 {
                    return false;
                }
                let cur = state.selected.get_untracked().min(n - 1);
                let row = &flat[cur];
                let toggle = |expand: bool| {
                    let id = row.id.clone();
                    state.expanded.update(|e| {
                        if expand {
                            e.insert(id);
                        } else {
                            e.remove(&id);
                        }
                    });
                };
                match k {
                    Key::Char('j') | Key::Down => {
                        state.selected.set((cur + 1) % n);
                        true
                    }
                    Key::Char('k') | Key::Up => {
                        state.selected.set((cur + n - 1) % n);
                        true
                    }
                    Key::Char('g') | Key::Home => {
                        state.selected.set(0);
                        true
                    }
                    Key::Char('G') | Key::End => {
                        state.selected.set(n - 1);
                        true
                    }
                    Key::Char('l') | Key::Right => {
                        if row.has_children && !row.expanded {
                            toggle(true);
                        } else if !row.has_children {
                            on_activate(&row.data);
                        }
                        true
                    }
                    Key::Char('h') | Key::Left => {
                        if row.has_children && row.expanded {
                            toggle(false);
                        } else if let Some(parent) = row.parent_row {
                            state.selected.set(parent);
                        }
                        true
                    }
                    Key::Char(' ') => {
                        if row.has_children {
                            toggle(!row.expanded);
                        }
                        true
                    }
                    Key::Enter => {
                        if row.has_children {
                            toggle(!row.expanded);
                        } else {
                            on_activate(&row.data);
                        }
                        true
                    }
                    _ => false,
                }
            }),
        );
    }
}
