/*
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: flutter-tools.nvim
Source: https://github.com/nvim-flutter/flutter-tools.nvim

Tools to help create flutter apps in neovim using the native lsp
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
*/

pub struct Plugin;

impl Plugin {
    pub fn spec() -> &'static str {
        let opts = Self::opts();

        format!(
            // language=lua
            r#"{{
                "nvim-flutter/flutter-tools.nvim",
                lazy = false,
                opts = {opts},
            }}"#
        )
        .leak()
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
