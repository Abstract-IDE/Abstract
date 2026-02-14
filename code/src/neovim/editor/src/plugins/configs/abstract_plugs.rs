/*
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: abstract-plugs.nvim
Source: https://github.com/Abstract-IDE/abstract-plugs.nvim

collections of neovim plugins
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
*/

use crate::lua_spec;

pub struct Plugin;

impl Plugin {
    pub fn spec() -> crate::plugins::spec::SpecInfo {
        let config = Self::config();

        lua_spec!(format!(
            r#"{{
                "Abstract-IDE/abstract-plugs.nvim",
                config = {}
            }}"#,
            config,
        ).leak())
    }
}

impl Plugin {
    pub fn config() -> &'static str {
        r#"function()
            local abstract = require('abs')
            abstract.window().setup()
            abstract.terminal().setup({
                height = 0.4, -- value range between: 0.0 to 1.0
                width = 0.6, -- value range between: 0.0 to 1.0
                offset_row = 0.9, -- vertical: value range between: 0.0 to 1.0
                offset_col = 0.5, -- horizontal: value range between: 0.0 to 1.0
                border = "rounded",
            })
            abstract.whitespace().setup()
        end"#
    }
}
