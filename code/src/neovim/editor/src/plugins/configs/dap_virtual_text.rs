/*
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: nvim-dap-virtual-text
Source: https://github.com/theHamsta/nvim-dap-virtual-text

This plugin adds virtual text support to nvim-dap.
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
*/

pub struct Plugin;

impl Plugin {
    pub fn spec() -> &'static str {
        r#"{
            "theHamsta/nvim-dap-virtual-text",
            virt_text_pos = 'eol',
        }"#
    }
}
