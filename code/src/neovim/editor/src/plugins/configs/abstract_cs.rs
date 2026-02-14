/*
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: Abstract-cs
Source: https://github.com/Abstract-IDE/Abstract-cs

Colorscheme for (neo)vim written in lua,
specially made for roshnivim with Tree-sitter support.
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
*/

use crate::lua_spec;

pub struct Plugin;

impl Plugin {
    pub fn spec() -> crate::plugins::spec::SpecInfo {
        let config = Self::config();
        lua_spec!(format!(
            // language=lua
            r#"{{
                "Abstract-IDE/Abstract-cs",
                branch = "rewrite-2",
                lazy = false,
                priority = 1000,
                config = {config},
            }}"#
        )
        .leak())
    }
}

impl Plugin {
    pub fn config() -> &'static str {
        // language=lua
        r#"function()
            require("abstract_cs").setup()
        end"#
    }
}
