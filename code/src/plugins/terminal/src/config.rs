use nvim_oxi::api::{self, types::WindowBorder, types::WindowTitlePosition};
use nvim_oxi::conversion::FromObject;
use serde::Deserialize;

// -----------------------------------------------------------------------------
// Configuration
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct Config {
    pub width: f64,
    pub height: f64,
    pub offset_row: f64,
    pub offset_col: f64,
    pub border: String,
    pub title: String,
    pub title_pos: String,
    pub keymap: Option<KeymapConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KeymapConfig {
    pub toggle: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            width: 0.6,
            height: 0.4,
            offset_row: 1.0,
            offset_col: 0.5,
            border: "rounded".to_string(),
            title: "Terminal".to_string(),
            title_pos: "right".to_string(),
            keymap: None,
        }
    }
}

impl Config {
    /// Load config from `vim.g.abstract_terminal_opts`.
    /// Falls back to defaults if the variable is not set or cannot be decoded.
    pub fn load() -> Self {
        let mut config = Self::default();
        if let Ok(opts) = api::get_var::<nvim_oxi::Dictionary>("abstract_terminal_opts") {
            if let Some(w) = opts.get(&nvim_oxi::String::from("width")).and_then(|v| f64::from_object(v.clone()).ok()) {
                config.width = w;
            }
            if let Some(h) = opts.get(&nvim_oxi::String::from("height")).and_then(|v| f64::from_object(v.clone()).ok()) {
                config.height = h;
            }
            if let Some(r) = opts.get(&nvim_oxi::String::from("offset_row")).and_then(|v| f64::from_object(v.clone()).ok()) {
                config.offset_row = r;
            }
            if let Some(c) = opts.get(&nvim_oxi::String::from("offset_col")).and_then(|v| f64::from_object(v.clone()).ok()) {
                config.offset_col = c;
            }
            if let Some(b) = opts.get(&nvim_oxi::String::from("border")).and_then(|v| String::from_object(v.clone()).ok()) {
                config.border = b;
            }
            if let Some(t) = opts.get(&nvim_oxi::String::from("title")).and_then(|v| String::from_object(v.clone()).ok()) {
                config.title = t;
            }
            if let Some(p) = opts.get(&nvim_oxi::String::from("title_pos")).and_then(|v| String::from_object(v.clone()).ok()) {
                config.title_pos = p;
            }
            if let Some(k) = opts.get(&nvim_oxi::String::from("keymap")).and_then(|v| nvim_oxi::Dictionary::from_object(v.clone()).ok()) {
                let mut keymap_config = KeymapConfig { toggle: None };
                if let Some(t) = k.get(&nvim_oxi::String::from("toggle")).and_then(|v| String::from_object(v.clone()).ok()) {
                    keymap_config.toggle = Some(t);
                }
                config.keymap = Some(keymap_config);
            }
        }
        config
    }

    pub fn border(&self) -> WindowBorder {
        match self.border.as_str() {
            "single" => WindowBorder::Single,
            "double" => WindowBorder::Double,
            "rounded" => WindowBorder::Rounded,
            "shadow" => WindowBorder::Shadow,
            "solid" => WindowBorder::Solid,
            _ => WindowBorder::None,
        }
    }

    pub fn title_position(&self) -> WindowTitlePosition {
        match self.title_pos.as_str() {
            "left" => WindowTitlePosition::Left,
            "center" => WindowTitlePosition::Center,
            _ => WindowTitlePosition::Right,
        }
    }
}
