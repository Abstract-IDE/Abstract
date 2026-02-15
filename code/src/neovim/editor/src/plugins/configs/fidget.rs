/*
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: fidget.nvim
Source: https://github.com/j-hui/fidget.nvim

💫 Extensible UI for Neovim notifications and LSP progress messages.
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
*/

use crate::lua_spec;

pub struct Plugin;

impl Plugin {
    pub fn spec() -> crate::plugins::spec::SpecInfo {
        let opts = Self::opts();

        lua_spec!(raw format!(
            r#"{{
                "j-hui/fidget.nvim",
                lazy = true,
                event = {{ "LspAttach" }},
                opts = {}
            }}"#,
            opts,
        ).leak())
    }
}

impl Plugin {
    pub fn opts() -> &'static str {
        r##"{
            notification = {
                window = {
                    winblend = 100, -- Background color opacity in the notification window
                }
            }
        }"##
    }
}
