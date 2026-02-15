/*
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: nvim-ts-autotag
Source: github.com/windwp/nvim-ts-autotag

Use treesitter to auto close and auto rename html tag
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
*/

use crate::lua_spec;

pub struct Plugin;

impl Plugin {
    pub fn spec() -> crate::plugins::spec::SpecInfo {
        // language=lua
        lua_spec!(raw r#"{
            "windwp/nvim-ts-autotag",
            event = { "InsertEnter", "LspAttach" },
            -- stylua: ignore
            ft = {
                "astro", "glimmer", "handlebars", "hbs", "html", "javascript",
                "javascriptreact", "jsx", "markdown", "php", "rescript",
                "svelte", "tsx", "typescript", "typescriptreact", "vue", "xml",
            },
            opts = {},
        }"#)
    }
}
