/*
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: helpview.nvim
Source: https://github.com/OXY2DEV/helpview.nvim

An impractical way to view vimdoc/help files
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
*/

pub struct Plugin;

impl Plugin {
    pub fn spec() -> &'static str {
        r#"{
            "OXY2DEV/helpview.nvim",
            ft = "help",
            opts = {},
        }"#
    }
}
