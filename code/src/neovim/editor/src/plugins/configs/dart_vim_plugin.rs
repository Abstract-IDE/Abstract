/*
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: dart-vim-plugin
Source: https://github.com/dart-lang/dart-vim-plugin

dart-vim-plugin provides filetype detection, syntax highlighting,
and indentation for Dart code in Vim.
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
*/

use crate::lua_spec;

pub struct Plugin;

impl Plugin {
    pub fn spec() -> crate::plugins::spec::SpecInfo {
        let config = Self::config();

        lua_spec!(raw format!(
            // language=lua
            r#"{{
                "dart-lang/dart-vim-plugin",
                lazy = true,
                ft = {{ "dart" }},
                config = {config},
            }}"#
        )
        .leak())
    }
}

impl Plugin {
    pub fn config() -> &'static str {
        // language=lua
        r#"function()
            -- Enable HTML syntax highlighting inside Dart strings  (default: false)
            vim.g.dart_html_in_string = "v.true"
            -- Enable Dart style guide syntax (like 2-space indentation)
            vim.g.dart_style_guide = 2
            -- Enable DartFmt execution on buffer save with
            vim.g.dart_format_on_save = 0
        end"#
    }
}
