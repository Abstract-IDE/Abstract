//! # ab-nui — a reactive UI library for Neovim
//!
//! `ab-nui` (imported as `wp_ui`) is a small, composable toolkit for building
//! Neovim UIs from Rust. It deliberately does **not** use nvim-oxi: every call
//! goes through Neovim's `vim.*` API via [`mlua`], using the host's live `Lua`
//! handle.
//!
//! ## Layers
//!
//! | Layer            | What it gives you                                        |
//! |------------------|---------------------------------------------------------|
//! | [`reactive`]     | `Signal`, `effect`, `memo` — automatic state management  |
//! | [`text`]         | `Span`/`Line` — styled content                           |
//! | [`geometry`]     | `Size`, `Rect`, `Position` — layout math                 |
//! | [`nvim`]         | `Buffer`, `Window`, keymaps, autocmds, extmarks          |
//! | [`widget`]       | `Popup`, `Menu`, `Input` — ready-made components         |
//! | [`theme`]        | highlight groups                                         |
//!
//! ## Setup
//!
//! Call [`init`] once, handing in the host's `Lua` (e.g. from nvim-oxi):
//!
//! ```ignore
//! wp_ui::init(&nvim_oxi::mlua::lua());
//! wp_ui::theme::setup().ok();
//! ```
//!
//! ## Example: a reactive counter popup
//!
//! ```ignore
//! use wp_ui::prelude::*;
//!
//! let count = Signal::new(0i64);
//! let popup = Popup::new(PopupOptions { title: Some(" Counter ".into()), ..Default::default() })?;
//! popup.open()?;
//!
//! // Re-renders automatically whenever `count` changes.
//! let c = count.clone();
//! popup.render_reactive(move || vec![Line::raw(format!("count: {}", c.get()))]);
//!
//! let inc = count.clone();
//! popup.on_key("n", "+", move || inc.update(|n| *n += 1))?;
//! popup.on_close_key("q")?;
//! ```

pub mod error;
pub mod geometry;
pub mod lua;
pub mod nvim;
pub mod reactive;
pub mod text;
pub mod theme;
pub mod view;
pub mod vim_ui;
pub mod widget;

pub use error::{Error, Result};
pub use lua::{init, is_initialized};

/// The common imports for building UIs.
pub mod prelude {
    pub use crate::geometry::{Dim, Position, Rect, Size};
    pub use crate::nvim::{
        Anchor, Border, Buffer, ExtmarkOpts, FloatConfig, Namespace, Relative, SplitConfig,
        SplitDir, Timer, TitlePos, VirtTextPos, Window,
    };
    pub use crate::reactive::{Memo, Signal, Store, effect, memo};
    pub use crate::text::{Line, Span, Wrap};
    pub use crate::widget::{Input, InputOptions, Menu, MenuItem, MenuOptions, Popup, PopupOptions};
    // Declarative view layer (Flutter-style widget tree).
    pub use crate::view::{
        Align, Alignment, AreaState, BorderChars, BoxStyle, Button, Center, Checkbox, ColWidth,
        Column, Container, CrossAxis, Divider, EdgeAlign, Element, Expanded, FieldState, Fraction,
        ItemCx, Key, KeyHints, Keyed, Layout, Level, List, ListState, ListView, MainAxis,
        NotifyHandle, NotifyOptions, Overflow, Padding, Pane, Positioned, ProgressBar, RadioGroup,
        Row, ScrollView, Scrollbar, Select, SelectState, SizedBox, Spacer, Spinner, SpinnerState,
        Stack, Surface, TabBar, Table, TableColumn, Text, TextAlign, TextArea, TextField, Toggle,
        Tree, TreeNode, TreeState, Widget, dismiss_all, notify,
    };
    pub use crate::{Error, Result, col, row, stack, store, text};
}
