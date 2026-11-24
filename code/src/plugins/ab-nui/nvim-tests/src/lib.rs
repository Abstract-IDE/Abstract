//! Integration tests for ab-nui (`wp_ui`), executed inside a real Neovim.
//!
//! Every `#[nvim_oxi::test]` runs in its own headless Neovim instance, so the
//! tests are isolated from each other and can assert on what actually ends up
//! in buffers, windows and extmarks.

mod support;

mod bench;
mod interactive;
mod layout;
mod lifecycle;
mod nvim_api;
mod popup;
mod rendering;
mod surface;
mod widgets;
