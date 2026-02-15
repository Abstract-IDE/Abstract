/*
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: csvview.nvim
Source: https://github.com/hat0uma/csvview.nvim

lightweight CSV file viewer plugin for Neovim.
With this plugin, you can easily view and edit CSV files within Neovim.
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
*/

use crate::{lua_spec, plugins::spec::SpecInfo};

pub struct Plugin;

impl Plugin {
    pub fn spec() -> SpecInfo {
        let opts = Self::opts();

        lua_spec!(
            raw format!(
                r#"{{
                'hat0uma/csvview.nvim',
                lazy = true,
                ft = "csv",
                opts = {}
            }}"#,
                opts,
            )
            .leak()
        )
    }
}

impl Plugin {
    pub fn opts() -> &'static str {
        r##"{
            view = {
                --- minimum width of a column
                --- @type integer
                min_column_width = 5,

                --- spacing between columns
                --- @type integer
                spacing = 2,

                --- The display method of the delimiter
                --- "highlight" highlights the delimiter
                --- "border" displays the delimiter with `│`
                --- see `Features` section of the README.
                ---@type "highlight" | "border"
                display_mode = "highlight",
            },
        }"##
    }
}
