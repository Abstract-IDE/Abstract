//! Floating terminals, built on ab-nui (`wp_ui`).
//!
//! Each terminal is a real terminal buffer shown in its own ab-nui [`Popup`]
//! (`Popup::with_buffer`), so sizing, placement, re-anchoring on resize and
//! theming all come from the UI library. Only one terminal float is visible
//! at a time; the others stay alive, hidden, and can be cycled through.
//!
//! [`Popup`]: wp_ui::widget::Popup

mod config;
mod init;
mod state;
mod terminal;

pub use init::{setup, setup_with_opts};
