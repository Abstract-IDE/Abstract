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
pub mod runtime;
pub mod widget;
pub mod widgets;

pub use runtime::Surface;
pub use widget::{Alignment, Area, Constraints, CrossAxis, Cx, Element, Key, MainAxis, Size, Widget};
pub use widgets::{
    Align, Button, BoxStyle, Center, Column, Container, Divider, Expanded, ListView, Padding, Row,
    SizedBox, Spacer, Text, TextField,
};
