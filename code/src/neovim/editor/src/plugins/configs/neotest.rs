/*
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: Neotest
Source: https://github.com/nvim-neotest/neotest

An extensible framework for interacting with tests within NeoVim.
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
*/

use crate::lua_spec;

pub struct Plugin;

impl Plugin {
    pub fn spec() -> crate::plugins::spec::SpecInfo {
        let config = Self::config();

        lua_spec!(format!(
            // language=lua
            r#"{{
                "nvim-neotest/neotest",
                dependencies = {{"antoinemadec/FixCursorHold.nvim"}},
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

        end"#
    }
}
