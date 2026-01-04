/*
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: plenary.nvim
Github: https://github.com/nvim-lua/plenary.nvim

full; complete; entire; absolute; unqualified.
All the lua functions I don't want to write twice.
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
*/

pub struct Plugin;

impl Plugin {
    pub fn spec() -> &'static str {
        r#"{
            "nvim-lua/plenary.nvim",
            lazy = true
        }"#
    }
}
