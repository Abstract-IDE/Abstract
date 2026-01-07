/*
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: rustaceanvim
Source: https://github.com/mrcjkb/rustaceanvim

Supercharge your Rust experience in Neovim!
A heavily modified fork of rust-tools.nvim
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
                "mrcjkb/rustaceanvim",
                version = "^6", -- Recommended
                lazy = false, -- This plugin is already lazy
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
            vim.g.rustaceanvim = {
                -- Plugin configuration
                tools = {
                    float_win_config = {
                        border = "rounded",
                    },
                },
                -- DAP configuration
                dap = {},
            }
        end"#
    }
}
