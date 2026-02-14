/*
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: nvim-surround
Github: https://github.com/kylechui/nvim-surround

Add/change/delete surrounding delimiter pairs with ease.
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
*/

use crate::lua_spec;

pub struct Plugin;

impl Plugin {
    pub fn spec() -> crate::plugins::spec::SpecInfo {
        let opts = Self::opts();

        lua_spec!(format!(
            // language=lua
            r#"{{
                "kylechui/nvim-surround",
                version = "*", -- Use for stability; omit to use `main` branch for the latest features
                keys = {{ "c", "s" }},
                opts = {opts},
            }}"#
        )
        .leak())
    }
}

impl Plugin {
    pub fn opts() -> &'static str {
        // language=lua
        r#"{
            keymaps = { -- vim-surround style keymaps
                visual = "S",
                delete = "ds",
                change = "cs",
                insert = "<C-g>s",
                -- insert_line = "<C-g>S",
                -- normal = "ys",
                -- normal_cur = "yss",
                -- normal_line = "yS",
                -- normal_cur_line = "ySS",
                -- visual_line = "gS",
            },
            surrounds = {
                ["("] = { add = { "( ", " )" }, },
                [")"] = { add = { "(", ")" }, },
                ["{"] = { add = { "{ ", " }" }, },
                ["}"] = { add = { "{", "}" }, },
                ["<"] = { add = { "< ", " >" }, },
                [">"] = { add = { "<", ">" }, },
                ["["] = { add = { "[ ", " ]" }, },
                ["]"] = { add = { "[", "]" }, },
                ["'"] = { add = { "'", "'" }, },
                ['"'] = { add = { '"', '"' }, },
                ["`"] = { add = { "`", "`" }, },
            },
            aliases = {
                ["a"] = ">", -- Single character aliases apply everywhere
                ["b"] = ")",
                ["B"] = "}",
                ["r"] = "]",
                -- Table aliases only apply for changes/deletions
                ["q"] = { '"', "'", "`" },               -- Any quote character
                ["s"] = { ")", "]", "}", ">", "'", '"', "`" }, -- Any surrounding delimiter
            },

            highlight = { -- Highlight before inserting/changing surrounds
                duration = 0,
            },
            move_cursor = "begin",
        }"#
    }
}

