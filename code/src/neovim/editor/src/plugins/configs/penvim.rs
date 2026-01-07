/*
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: penvim
Source: https://github.com/Abstract-IDE/penvim

project's root directory and documents indentation detector with project based config loader
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
*/

pub struct Plugin;

impl Plugin {
    pub fn spec() -> &'static str {
        let opts = Self::opts();

        format!(
            // language=lua
            r#"{{
                "Abstract-IDE/penvim",
                opts = {opts},
            }}"#
        )
        .leak()
    }
}

impl Plugin {
    pub fn opts() -> &'static str {
        // language=lua
        r#"{
            project_env = {
                enable = true,
                config_name = ".__nvim__.lua",
            },

            rooter = {
                enable = true,
                patterns = { ".__nvim__.lua" },
            },

            indentor = {
                enable = true,
                indent_length = 4,
                indent_type = "auto", -- auto|tab|space
            },

            -- langs = {
            -- 	enable = true,
            -- }
        }"#
    }
}
