use mlua::{Table, Value};
use wp_ui::geometry::{Position, Size};
use wp_ui::nvim::{Border, TitlePos};
use wp_ui::widget::PopupOptions;
use wp_ui::{Result, lua};

// -----------------------------------------------------------------------------
// Configuration
// -----------------------------------------------------------------------------

/// The `vim.g` variable holding the user's options table.
const OPTS_VAR: &str = "abstract_terminal_opts";

#[derive(Clone, Debug)]
pub struct Config {
    /// Float width as a fraction of the editor.
    pub width: f64,
    /// Float height as a fraction of the editor.
    pub height: f64,
    /// Vertical placement within the free space: 0.0 top, 1.0 bottom.
    pub offset_row: f64,
    /// Horizontal placement within the free space: 0.0 left, 1.0 right.
    pub offset_col: f64,
    pub border: Border,
    pub title: String,
    pub title_pos: TitlePos,
    /// Optional global key (normal, insert and terminal mode) that toggles the float.
    pub toggle_key: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            width: 0.6,
            height: 0.4,
            offset_row: 1.0,
            offset_col: 0.5,
            border: Border::Rounded,
            title: "Terminal".to_string(),
            title_pos: TitlePos::Right,
            toggle_key: None,
        }
    }
}

impl Config {
    /// Save the user's options table (or `nil`) to `vim.g.abstract_terminal_opts`.
    pub fn store(opts: Value) -> Result<()> {
        let g: Table = lua::vim()?.get("g")?;
        g.set(OPTS_VAR, opts)?;
        Ok(())
    }

    /// Load from `vim.g.abstract_terminal_opts`. Missing or mistyped fields
    /// keep their defaults.
    pub fn load() -> Result<Self> {
        let mut config = Self::default();
        let g: Table = lua::vim()?.get("g")?;
        let Value::Table(opts) = g.get::<Value>(OPTS_VAR)? else {
            return Ok(config);
        };

        let num = |key: &str| opts.get::<Option<f64>>(key).ok().flatten();
        let text = |key: &str| opts.get::<Option<String>>(key).ok().flatten();

        if let Some(v) = num("width") {
            config.width = v;
        }
        if let Some(v) = num("height") {
            config.height = v;
        }
        if let Some(v) = num("offset_row") {
            config.offset_row = v;
        }
        if let Some(v) = num("offset_col") {
            config.offset_col = v;
        }
        if let Some(v) = text("border") {
            config.border = parse_border(&v);
        }
        if let Some(v) = text("title") {
            config.title = v;
        }
        if let Some(v) = text("title_pos") {
            config.title_pos = parse_title_pos(&v);
        }
        if let Ok(Value::Table(keymap)) = opts.get::<Value>("keymap") {
            config.toggle_key = keymap.get::<Option<String>>("toggle").ok().flatten().filter(|k| !k.is_empty());
        }
        Ok(config)
    }

    /// The popup options every terminal float is created with.
    pub fn popup_options(&self) -> PopupOptions {
        PopupOptions {
            size: Size::ratio(self.width, self.height),
            position: Position::Ratio { row: self.offset_row, col: self.offset_col },
            border: self.border,
            title_pos: self.title_pos,
            ..Default::default()
        }
    }

    /// Border title for terminal `index` (0-based) of `count`.
    pub fn title(&self, index: usize, count: usize) -> String {
        format!(" {} {}/{} ", self.title, index + 1, count)
    }
}

fn parse_border(s: &str) -> Border {
    match s {
        "single" => Border::Single,
        "double" => Border::Double,
        "rounded" => Border::Rounded,
        "solid" => Border::Solid,
        "shadow" => Border::Shadow,
        _ => Border::None,
    }
}

fn parse_title_pos(s: &str) -> TitlePos {
    match s {
        "left" => TitlePos::Left,
        "center" => TitlePos::Center,
        _ => TitlePos::Right,
    }
}
