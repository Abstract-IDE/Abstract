//! `col!` / `row!` — Flutter-like nesting sugar over `Column`/`Row`.
//!
//! They box each child into an [`crate::view::Element`], so you can write a tree
//! without `Box::new(...)` noise:
//!
//! ```ignore
//! col![
//!     Text::new("Title").fg("Title"),
//!     Spacer::new(),
//!     row![ Button::new("OK"), Button::new("Cancel") ],
//! ]
//! ```

/// Build a [`Column`](crate::view::Column) from a child list.
#[macro_export]
macro_rules! col {
    ($($child:expr),* $(,)?) => {
        $crate::view::Column::new(vec![ $( Box::new($child) as $crate::view::Element ),* ])
    };
}

/// Build a [`Row`](crate::view::Row) from a child list.
#[macro_export]
macro_rules! row {
    ($($child:expr),* $(,)?) => {
        $crate::view::Row::new(vec![ $( Box::new($child) as $crate::view::Element ),* ])
    };
}
