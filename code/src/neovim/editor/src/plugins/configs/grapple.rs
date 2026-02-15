/*
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: grapple-nvim
Source: https://github.com/cbochs/grapple.nvim

Grapple is a plugin that aims to provide immediate navigation
to important files (and their last known cursor location).
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
*/

use crate::lua_spec;

pub struct Plugin;

impl Plugin {
    pub fn spec() -> crate::plugins::spec::SpecInfo {
        let opts = Self::opts();

        lua_spec!(raw format!(
            r#"{{
                "cbochs/grapple.nvim",
                event = {{ "BufReadPost", "BufNewFile" }},
                cmd = "Grapple",
                opts = {},
            }}"#,
            opts,
        ).leak())
    }
}

impl Plugin {
    pub fn opts() -> &'static str {
        r#"{
            ---Show icons next to tags or scopes in Grapple windows
            ---Requires "nvim-tree/nvim-web-devicons"
            ---@type boolean
            icons = true,

            ---How a tag's path should be rendered in Grapple windows
            ---  "relative": show tag path relative to the scope's resolved path
            ---  "basename": show tag path basename and directory hint
            ---@type "basename" | "relative"
            style = "basename",
        }"#
    }
}
