/*
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: markview.nvim
Source: https://github.com/OXY2DEV/markview.nvim

A hackable markdown, Typst, latex, html(inline) & YAML previewer for Neovim
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
*/

use crate::core::keymaps;
use crate::lua_spec;

pub struct Plugin;

impl Plugin {
    pub fn spec() -> crate::plugins::spec::SpecInfo {
        let config = Self::config();

        lua_spec!(raw format!(
            // language=lua
            r#"{{
                "OXY2DEV/markview.nvim",
                -- Do not lazy load this plugin as it is already lazy-loaded.
                -- Lazy-loading may cause more time for the previews to load when starting Neovim!
                lazy = false,
                config = {config},
            }}"#
        )
        .leak())
    }
}

impl Plugin {
    pub fn config() -> &'static str {
        keymaps::MAPPING.signal(keymaps::Key::Markview);
        // language=lua
        r#"function()
            -- NOTE: for now lets disable for all filetype. later we will provide a mapping to enable/disable
            require("markview").setup({
                experimental = {
                    check_rtp = false
                },
                html = {
                    enable = false,
                },
                latex = {
                    enable = false,
                },
                markdown = {
                    enable = false,
                },
                markdown_inline = {
                    enable = false,
                },
                typst = {
                    enable = false,
                },
                yaml = {
                    enable = false,
                }
            })
        end"#
    }
}
