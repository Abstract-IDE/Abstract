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

pub struct Plugin;

impl Plugin {
    pub fn spec() -> &'static str {
        let opts = Self::opts();

        let spec = format!(
            r#"{{
                "cbochs/grapple.nvim",
                event = {{ "BufReadPost", "BufNewFile" }},
                cmd = "Grapple",
                opts = {}
            }}"#,
            opts,
        );

        Box::leak(spec.into_boxed_str())
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
