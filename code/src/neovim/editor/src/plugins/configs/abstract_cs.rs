/*
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: Abstract-cs
Source: https://github.com/Abstract-IDE/Abstract-cs

Colorscheme for (neo)vim written in lua,
specially made for roshnivim with Tree-sitter support.
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
*/

pub struct Plugin;

impl Plugin {
    pub fn spec() -> &'static str {
        let config = Self::config();
        let spec = format!(
            // language=lua
            r#"{{
                "Abstract-IDE/Abstract-cs",
                branch = "rewrite-2",
                config = {config},
            }}"#
        );

        Box::leak(spec.into_boxed_str())
    }
}

impl Plugin {
    pub fn config() -> &'static str {
        // language=lua
        r#"function()
            require("abstract_cs").setup()
        end"#
    }
}
