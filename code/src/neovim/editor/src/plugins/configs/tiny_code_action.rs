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

pub struct Plugin;

impl Plugin {
    pub fn spec() -> &'static str {
        r#"{
            "rachartier/tiny-code-action.nvim",
            lazy = true,
            event = { "LspAttach" },
            opts = {},
        }"#
    }
}
