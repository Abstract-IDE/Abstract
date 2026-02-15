/*
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: vim-dadbod
Github: https://github.com/tpope/vim-dadbod

Dadbod is a Vim plugin for interacting with databases
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
*/

use crate::lua_spec;

pub struct Plugin;

impl Plugin {
    pub fn spec() -> crate::plugins::spec::SpecInfo {
        lua_spec!(raw r#"{
            "tpope/vim-dadbod",
            dependencies = {
                { "kristijanhusak/vim-dadbod-ui", lazy = true }, -- Simple UI for vim-dadbod.
                {
                    "kristijanhusak/vim-dadbod-completion",
                    ft = { "sql", "mysql", "plsql" },
                    lazy = true,
                },
            },
            lazy = true,
            cmd = { "DBUI", "DBUIToggle", "DBUIAddConnection", "DBUIFindBuffer" },
            init = function()
                -- Your DBUI configuration
                vim.g.db_ui_use_nerd_fonts = 1
            end,
        }"#)
    }
}
