//! Widgets — composable UI components built on [`Popup`].
//!
//! `Popup` is the primitive (a float over a managed buffer with reactive
//! content and Rust keymaps). `Menu` and `Input` are full components built from
//! it; use them directly, or use `Popup` + [`crate::reactive`] to build your own.

pub mod input;
pub mod menu;
pub mod popup;

pub use input::{Input, InputOptions};
pub use menu::{Menu, MenuItem, MenuOptions};
pub use popup::{Popup, PopupOptions};
