//! The built-in widget set. All implement [`crate::view::Widget`].

pub mod button;
pub mod hints;
pub mod input;
pub mod keyed;
pub mod layout;
pub mod list;
pub mod progress;
pub mod scroll;
pub mod select;
pub mod table;
pub mod tabs;
pub mod text;
pub mod toggle;
pub mod tree;

pub use button::Button;
pub use hints::KeyHints;
pub use input::{AreaState, FieldState, TextArea, TextField};
pub use keyed::Keyed;
pub use layout::{
    Align, BorderChars, BoxStyle, Center, Column, Container, Divider, EdgeAlign, Expanded, Fraction,
    Padding, Positioned, Row, SizedBox, Spacer, Stack,
};
pub use list::{ItemCx, List, ListState, ListView};
pub use progress::{ProgressBar, SPINNER_DOTS, Spinner, SpinnerState};
pub use scroll::{ScrollView, Scrollbar};
pub use select::{Select, SelectState};
pub use table::{ColWidth, Table, TableColumn};
pub use tabs::TabBar;
pub use text::{Overflow, Text, TextAlign};
pub use toggle::{Checkbox, RadioGroup, Toggle};
pub use tree::{Tree, TreeNode, TreeState};
