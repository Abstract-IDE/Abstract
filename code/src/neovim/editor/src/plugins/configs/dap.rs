/*
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: nvim-dap
Source: https://github.com/mfussenegger/nvim-dap

Debug Adapter Protocol client implementation for Neovim
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
*/

use crate::core::keymaps;

pub struct Plugin;

impl Plugin {
    pub fn spec() -> &'static str {
        keymaps::MAPPING.signal(keymaps::Key::Dap);

        // language=lua
        r#"{
            "mfussenegger/nvim-dap",
        }"#
    }
}
