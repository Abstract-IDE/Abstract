/*
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: abstract-cursor
Source: https://github.com/Abstract-IDE/abstract-cursor

dynamic cursor
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
*/

pub struct Plugin;

impl Plugin {
    pub fn spec() -> &'static str {
        let opts = Self::opts();
        let spec = format!(
            // language=lua
            r#"{{
                "Abstract-IDE/abstract-cursor",
                event = {{"BufRead"}},
                opts = {opts},
            }}"#
        );

        Box::leak(spec.into_boxed_str())
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
