/*
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: kulala.nvim
Github: https://github.com/mistweaverco/kulala.nvim

A minimal REST-Client Interface for Neovim.
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
*/

use crate::core::keymaps;
use crate::lua_spec;

pub struct Plugin;

impl Plugin {
    pub fn spec() -> crate::plugins::spec::SpecInfo {
        let opts = Self::opts();
        let keys = keymaps::MAPPING.get_map(keymaps::Key::Kulala);

        lua_spec!(raw format!(
            // language=lua
            r#"{{
                "mistweaverco/kulala.nvim",
                lazy = true,
                ft = "http",
                keys = {keys},
                opts = {opts},
            }}"#
        )
        .leak())
    }
}

impl Plugin {
    pub fn opts() -> &'static str {
        keymaps::MAPPING.signal(keymaps::Key::Kulala);
        // language=lua
        r#"function()
            return {
                -- split direction
                -- possible values: "vertical", "horizontal"
                split_direction = "horizontal",
                -- default_view, body or headers or headers_body
                default_view = "body",
                -- dev, test, prod, can be anything
                -- see: https://learn.microsoft.com/en-us/aspnet/core/test/http-files?view=aspnetcore-8.0#environment-files
                default_env = "dev",
                -- enable/disable debug mode
                debug = false,
                -- default formatters for different content types
                formatters = {
                    json = { "jq", "." },
                    xml = { "xmllint", "--format", "-" },
                    html = { "xmllint", "--format", "--html", "-" },
                },
                -- default icons
                icons = {
                    inlay = {
                        loading = "⏳",
                        done = "✅",
                        error = "❌",
                    },
                    lualine = "🐼",
                },
                -- additional cURL options
                -- see: https://curl.se/docs/manpage.html
                additional_curl_options = {},
                -- scratchpad default contents
                -- scratchpad_default_contents = {
                --  "@MY_TOKEN_NAME=my_token_value",
                --  "",
                --  "POST https://httpbin.org/post HTTP/1.1",
                --  "accept: application/json",
                --  "content-type: application/json",
                --  " @name scratchpad",
                --  "",
                --  "{",
                --  '  "foo": "bar"',
                --  "}",
                -- },
                -- enable winbar
                winbar = false,
            }
        end"#
    }
}
