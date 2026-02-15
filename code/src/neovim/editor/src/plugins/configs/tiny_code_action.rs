/*
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: tiny-code-action.nvim
Source: https://github.com/rachartier/tiny-code-action.nvim

A Neovim plugin that provides a simple way to
run and visualize code actions with Telescope.
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
*/

use crate::lua_spec;

pub struct Plugin;

impl Plugin {
    pub fn spec() -> crate::plugins::spec::SpecInfo {
        lua_spec!(raw r#"{
            "rachartier/tiny-code-action.nvim",
            lazy = true,
            event = { "LspAttach" },
            opts = {},
        }"#)
    }
}
