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
use crate::lua_spec;

pub struct Plugin;

impl Plugin {
    pub fn spec() -> crate::plugins::spec::SpecInfo {
        let config = Self::config();
        let keys = keymaps::MAPPING.get_map(keymaps::Key::GitGraph);

        lua_spec!(format!(
            // language=lua
            r#"{{
                "isakbm/gitgraph.nvim",
                dependencies = {{ "sindrets/diffview.nvim" }},
                event = "BufEnter",
                keys = {keys},
                config = {config},
            }}"#
        )
        .leak())
    }
}

impl Plugin {
    pub fn config() -> &'static str {
        keymaps::MAPPING.signal(keymaps::Key::GitGraph);

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
