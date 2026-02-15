/*
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: nvim-web-devicons
Source: https://github.com/nvim-tree/nvim-web-devicons

A lua fork of vim-devicons.
This plugin provides the same icons as well as colors for each icon.
Light and dark color variants are provided.
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
*/

use crate::lua_spec;

pub struct Plugin;

impl Plugin {
    pub fn spec() -> crate::plugins::spec::SpecInfo {
        lua_spec!(raw r#"{
            "nvim-tree/nvim-web-devicons",
            lazy = true
        }"#)
    }
}
