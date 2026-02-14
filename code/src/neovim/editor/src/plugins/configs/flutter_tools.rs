/*
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: flutter-tools.nvim
Source: https://github.com/nvim-flutter/flutter-tools.nvim

Tools to help create flutter apps in neovim using the native lsp
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
*/

use crate::lua_spec;

pub struct Plugin;

impl Plugin {
    pub fn spec() -> crate::plugins::spec::SpecInfo {
        let opts = Self::opts();

        lua_spec!(format!(
            // language=lua
            r#"{{
                "nvim-flutter/flutter-tools.nvim",
                lazy = false,
                opts = {opts},
            }}"#
        )
        .leak())
    }
}

impl Plugin {
    pub fn opts() -> &'static str {
        // language=lua
        r#"
            {
                widget_guides = {
                    enabled = true,
                },

            }
        "#
    }
}
