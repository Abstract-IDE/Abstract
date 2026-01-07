/*
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: hovercraft.nvim
Source: https://github.com/patrickpichler/hovercraft.nvim

hovercraft.nvim is a plug and play framework for writing custom hover provider.
It brings a few providers out of the box, such as a LSP, as well as a Dictionary.
It allows for basic customizations of the hover window
(such as defining custom borders, set the max width and so on).
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
*/

use crate::core::keymaps;

pub struct Plugin;

impl Plugin {
    pub fn spec() -> &'static str {
        let opts = Self::opts();
        let keys = keymaps::KEYMAPS.get_map(keymaps::MapKey::Hovercraft);

        format!(
            // language=lua
            r#"{{
                "patrickpichler/hovercraft.nvim",
                lazy = true,
                keys = {keys},
                config = {opts},
            }}"#
        )
        .leak()
    }
}

impl Plugin {
    pub fn opts() -> &'static str {
        // language=lua
        r#"function()
            return {
                providers = {
                    providers = {
                        { "LSP", require("hovercraft.provider.lsp.hover").new() },
                        { "Man", require("hovercraft.provider.man").new() },
                        { "Dictionary", require("hovercraft.provider.dictionary").new() },
                        -- { "Git Blame", require("hovercraft.provider.git.blame").new() },
                        -- { 'Diagnostics',  Provider.Diagnostics.new(), },
                        -- { 'LSP',          Provider.Lsp.Hover.new(), },
                        -- { 'Man',          Provider.Man.new(), },
                        -- { 'Github Issue', Provider.Github.Issue.new(), },
                        -- { 'Github Repo',  Provider.Github.Repo.new(), },
                        -- { 'Github User',  Provider.Github.User.new(), },
                        -- { 'Diagnostics',  Provider.Diagnostics.new(), },
                    },
                },

                window = {
                    border = "rounded",
                },

                keys = {
                    {
                        "<C-u>",
                        function()
                            require("hovercraft").scroll({ delta = -2 })
                        end,
                    },
                    {
                        "<C-d>",
                        function()
                            require("hovercraft").scroll({ delta = 2 })
                        end,
                    },
                    {
                        "<TAB>",
                        function()
                            require("hovercraft").hover_next()
                        end,
                    },
                    {
                        "<S-TAB>",
                        function()
                            require("hovercraft").hover_next({ step = -1 })
                        end,
                    },
                },
            }
        end"#
    }
}
