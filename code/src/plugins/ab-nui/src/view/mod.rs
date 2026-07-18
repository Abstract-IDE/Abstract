//! The declarative view layer — a Flutter-style widget tree on top of the core.
//!
//! Everything is a [`Widget`]: text, layout, spacing, buttons, lists. You build
//! a tree (often with the [`col!`](crate::col)/[`row!`](crate::row) macros),
//! mount it on a [`Surface`], and it rebuilds itself when the [`Signal`]s it
//! reads change — interactions (focus, keys, presses) are part of the tree.
//!
//! ```ignore
//! use wp_ui::prelude::*;
//!
//! let count = Signal::new(0);
//! Surface::float(PopupOptions { title: Some(" Counter ".into()), ..Default::default() })?
//!     .show(move || {
//!         let c = count.clone();
//!         col![
//!             Center::new(Text::new(format!("count: {}", c.get())).fg("Title")),
//!             Spacer::new(),
//!             row![
//!                 Button::new("-").on_press({ let c = count.clone(); move || c.update(|n| *n -= 1) }),
//!                 Spacer::new(),
//!                 Button::new("+").on_press({ let c = count.clone(); move || c.update(|n| *n += 1) }),
//!             ],
//!         ]
//!     })?;
//! ```
//!
//! [`Signal`]: crate::reactive::Signal

pub mod canvas;
pub mod macros;
pub mod multi;
pub mod notify;
pub mod runtime;
pub mod widget;
pub mod widgets;

pub use multi::{Layout, Pane};
pub use notify::{Level, NotifyHandle, NotifyOptions, dismiss_all, notify};
pub use runtime::Surface;
pub use widget::{
    Alignment, Area, Constraints, CrossAxis, Cx, Element, Focusable, Key, MainAxis, Size, Widget,
};
pub use widgets::{
    Align, AreaState, BorderChars, BoxStyle, Button, Center, Checkbox, ColWidth, Column, Container,
    Divider, EdgeAlign, Expanded, FieldState, Fraction, ItemCx, KeyHints, Keyed, List, ListState, ListView,
    Overflow, Padding, Positioned, ProgressBar, RadioGroup, Row, SPINNER_DOTS, ScrollView, Scrollbar,
    Select, SelectState, SizedBox, Spacer, Spinner, SpinnerState, Stack, TabBar, Table, TableColumn,
    Text, TextAlign, TextArea, TextField, Toggle, Tree, TreeNode, TreeState,
};
