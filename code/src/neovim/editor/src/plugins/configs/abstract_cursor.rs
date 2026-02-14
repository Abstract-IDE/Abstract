/*
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: abstract-cursor
Source: https://github.com/Abstract-IDE/abstract-cursor

dynamic cursor
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
*/

use crate::lua_spec;

pub struct Plugin;

impl Plugin {
    pub fn spec() -> crate::plugins::spec::SpecInfo {
        let opts = Self::opts();
        lua_spec!(format!(
            // language=lua
            r#"{{
                "Abstract-IDE/abstract-cursor",
                event = {{"BufRead"}},
                opts = {opts},
            }}"#
        ).leak())
    }
}

impl Plugin {
    pub fn opts() -> &'static str {
        // language=lua
        r##"{
            Visual = {
                enable = true,
                colors = {},
            },

            CursorLine = {
                enable = true,
                colors = {},
            },

            CursorLineNr = {
                enable = true,
                colors = {
                    i = {
                        fg = "#ac3131",
                        reverse=true,
                    },
                    v = {
                        fg = "#d1d1d1",
                        reverse=true,
                    },
                        reverse=true,
                    V = {
                        fg = "#ffffff",
                        reverse=true,
                    },
                    ["^V"] = {
                        fg = "#d1d1d1",
                        reverse=true,
                    },
                },
            }
        }"##
    }
}
