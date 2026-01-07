/*
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: nvim-autopairs
Source: https://github.com/windwp/nvim-autopairs

autopairs for neovim written in lua
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
*/

pub struct Plugin;

impl Plugin {
    pub fn spec() -> &'static str {
        r#"{
            "windwp/nvim-autopairs",
            lazy = true,
            event = "InsertEnter",
            opts = {},
        }"#
    }
}
