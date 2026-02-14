/*
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: nvim-autopairs
Source: https://github.com/windwp/nvim-autopairs

autopairs for neovim written in lua
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
*/

use crate::lua_spec;

pub struct Plugin;

impl Plugin {
    pub fn spec() -> crate::plugins::spec::SpecInfo {
        lua_spec!(r#"{
            "windwp/nvim-autopairs",
            lazy = true,
            event = "InsertEnter",
            opts = {},
        }"#)
    }
}
