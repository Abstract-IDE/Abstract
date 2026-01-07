/*
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: Neotest
Source: https://github.com/nvim-neotest/neotest

An extensible framework for interacting with tests within NeoVim.
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
*/

pub struct Plugin;

impl Plugin {
    pub fn spec() -> &'static str {
        let config = Self::config();

        format!(
            // language=lua
            r#"{{
                "nvim-neotest/neotest",
                dependencies = {{"antoinemadec/FixCursorHold.nvim"}},
                config = {config},
            }}"#
        )
        .leak()
    }
}

impl Plugin {
    pub fn config() -> &'static str {
        // language=lua
        r#"function()

        end"#
    }
}
