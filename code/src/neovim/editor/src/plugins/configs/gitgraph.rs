/*
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: gitgraph.nvim
Source: https://github.com/isakbm/gitgraph.nvim

Git Graph plugin for neovim.
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
*/

use crate::core::keymaps;

pub struct Plugin;

impl Plugin {
    pub fn spec() -> &'static str {
        let config = Self::config();
        let keys = keymaps::KEYMAPS.get_map(keymaps::MapKey::GitGraph);

        format!(
            // language=lua
            r#"{{
                "isakbm/gitgraph.nvim",
                dependencies = {{ "sindrets/diffview.nvim" }},
                event = "BufEnter",
                keys = {keys},
                config = {config},
            }}"#
        )
        .leak()
    }
}

impl Plugin {
    pub fn config() -> &'static str {
        keymaps::MapLoader::signal(keymaps::MapKey::GitGraph);

        // language=lua
        r#"function()
            require('gitgraph').setup({
                symbols = {
                    merge_commit = "M",
                    commit = "*",
                },
                format = {
                    timestamp = "%H:%M:%S %d-%m-%Y",
                    fields = { "hash", "timestamp", "author", "branch_name", "tag" },
                },
            })
        end"#
    }
}
