//! Multi-window layouts: several [`Surface`]s arranged as one UI (nui.nvim
//! `Layout`-style). A telescope shape is one `Layout` + three surfaces whose
//! build closures share the same `Signal`s.
//!
//! ```ignore
//! let layout = Layout::new(
//!     Size::ratio(0.8, 0.8),
//!     Position::Center,
//!     Pane::row(vec![
//!         Pane::surface(list_pane, Dim::Ratio(0.4)),
//!         Pane::surface(preview_pane, Dim::Ratio(0.6)),
//!     ]),
//! );
//! layout.open()?;
//! ```

use std::cell::Cell;
use std::rc::{Rc, Weak};

use crate::error::Result;
use crate::geometry::{Dim, Position, Rect, Size};
use crate::nvim;
use crate::view::runtime::Surface;

/// One node of a layout tree: a surface, or a row/column of nested panes.
/// The [`Dim`] is the pane's extent along its PARENT's axis.
pub enum Pane {
    Surface(Surface, Dim),
    Row(Vec<Pane>, Dim),
    Col(Vec<Pane>, Dim),
}

impl Pane {
    /// A surface pane taking `dim` of the parent's axis.
    pub fn surface(surface: Surface, dim: Dim) -> Self {
        Pane::Surface(surface, dim)
    }
    /// Children side-by-side, filling the parent.
    pub fn row(children: Vec<Pane>) -> Self {
        Pane::Row(children, Dim::Ratio(1.0))
    }
    /// Children stacked, filling the parent.
    pub fn col(children: Vec<Pane>) -> Self {
        Pane::Col(children, Dim::Ratio(1.0))
    }
    /// Set this pane's extent along its parent's axis.
    pub fn sized(self, dim: Dim) -> Self {
        match self {
            Pane::Surface(s, _) => Pane::Surface(s, dim),
            Pane::Row(c, _) => Pane::Row(c, dim),
            Pane::Col(c, _) => Pane::Col(c, dim),
        }
    }

    fn dim(&self) -> Dim {
        match self {
            Pane::Surface(_, d) | Pane::Row(_, d) | Pane::Col(_, d) => *d,
        }
    }

    /// Assign `rect` (outer, including the float's border) to this pane.
    fn assign(&self, rect: Rect) -> Result<()> {
        match self {
            Pane::Surface(surface, _) => {
                // A bordered float's text area is 2 cells smaller than the box.
                let w = rect.width.saturating_sub(2).max(1);
                let h = rect.height.saturating_sub(2).max(1);
                let popup = surface.popup();
                popup.set_position(Position::At { row: rect.row, col: rect.col })?;
                popup.set_size(Size::cells(w, h))?;
                Ok(())
            }
            Pane::Row(children, _) => Self::assign_axis(children, rect, true),
            Pane::Col(children, _) => Self::assign_axis(children, rect, false),
        }
    }

    /// Carve `rect` along one axis: each child gets its resolved [`Dim`]; the
    /// last child absorbs the remainder.
    fn assign_axis(children: &[Pane], rect: Rect, horizontal: bool) -> Result<()> {
        let total = if horizontal { rect.width } else { rect.height };
        let mut used: u32 = 0;
        for (i, child) in children.iter().enumerate() {
            let extent = if i + 1 == children.len() {
                total.saturating_sub(used)
            } else {
                child.dim().resolve(total).min(total - used)
            };
            let piece = if horizontal {
                Rect { col: rect.col + used as f64, width: extent, ..rect }
            } else {
                Rect { row: rect.row + used as f64, height: extent, ..rect }
            };
            child.assign(piece)?;
            used += extent;
        }
        Ok(())
    }

    fn for_each_surface(&self, f: &mut impl FnMut(&Surface)) {
        match self {
            Pane::Surface(s, _) => f(s),
            Pane::Row(children, _) | Pane::Col(children, _) => {
                for c in children {
                    c.for_each_surface(f);
                }
            }
        }
    }
}

struct LayoutInner {
    size: Size,
    position: Position,
    root: Pane,
    group: Cell<Option<i64>>,
}

impl LayoutInner {
    fn relayout(&self) -> Result<()> {
        let rect = Rect::resolve(self.size, self.position)?;
        self.root.assign(rect)
    }
}

impl Drop for LayoutInner {
    fn drop(&mut self) {
        if let Some(group) = self.group.take() {
            let _ = nvim::del_augroup(group);
        }
    }
}

/// Arranges several float surfaces within one region. Cloning shares the
/// layout; the surfaces are owned by the panes, so dropping the last `Layout`
/// handle frees the whole UI.
#[derive(Clone)]
pub struct Layout {
    inner: Rc<LayoutInner>,
}

impl Layout {
    pub fn new(size: Size, position: Position, root: Pane) -> Self {
        Self { inner: Rc::new(LayoutInner { size, position, root, group: Cell::new(None) }) }
    }

    /// Position every pane and open all surfaces. Also installs a resize hook
    /// (once) so the arrangement follows editor resizes.
    pub fn open(&self) -> Result<()> {
        self.inner.relayout()?;
        self.inner.root.for_each_surface(&mut |s| {
            let _ = s.reopen();
        });
        // Panes' own positions are managed here, after the surfaces' own
        // relayout (both react to VimResized; ours must win → install later).
        if self.inner.group.get().is_none() {
            let group = nvim::unique_augroup("AbNuiLayout")?;
            self.inner.group.set(Some(group));
            let weak: Weak<LayoutInner> = Rc::downgrade(&self.inner);
            nvim::autocmd(&["VimResized"], None, Some(group), move |_| {
                if let Some(inner) = weak.upgrade() {
                    let _ = inner.relayout();
                }
            })?;
        }
        Ok(())
    }

    /// Hide every pane (state survives; [`Layout::open`] restores).
    pub fn close(&self) {
        self.inner.root.for_each_surface(&mut |s| s.hide());
    }

    /// Toggle visibility of the whole arrangement.
    pub fn toggle(&self) -> Result<()> {
        if self.is_visible() {
            self.close();
            Ok(())
        } else {
            self.open()
        }
    }

    /// Whether any pane is currently shown.
    pub fn is_visible(&self) -> bool {
        let mut visible = false;
        self.inner.root.for_each_surface(&mut |s| visible |= s.is_visible());
        visible
    }

    /// Re-resolve the arrangement (e.g. after changing sizes).
    pub fn relayout(&self) -> Result<()> {
        self.inner.relayout()
    }
}
