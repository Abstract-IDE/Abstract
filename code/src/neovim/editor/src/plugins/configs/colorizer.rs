/*
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: nvim-colorizer.lua
Source: https://github.com/catgoose/nvim-colorizer.lua

A high-performance color highlighter for Neovim which has
no external dependencies! Written in performant Luajit.

NOTE:
Originally NvChad (https://github.com/NvChad/nvim-colorizer.lua)
forked it from https://github.com/norcalli/nvim-colorizer.lua
now it moved to https://github.com/catgoose/nvim-colorizer.lua
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
                "catgoose/nvim-colorizer.lua",
                event = {{ "BufReadPre", "BufNewFile", "InsertEnter" }},
                opts = {}
            }}"#,
            opts,
        ).leak())
    }
}

impl Plugin {
    pub fn opts() -> &'static str {
        r#"{
            user_default_options = {
                -- RGB = true,       -- #RGB hex codes
                -- RRGGBB = true,    -- #RRGGBB hex codes
                names = false, -- "Name" codes like Blue or blue
                RRGGBBAA = true, -- #RRGGBBAA hex codes
                AARRGGBB = true, -- 0xAARRGGBB hex codes
                rgb_fn = true, -- CSS rgb() and rgba() functions
                hsl_fn = true, -- CSS hsl() and hsla() functions
                -- css = true,          -- Enable all CSS features: rgb_fn, hsl_fn, names, RGB, RRGGBB
                -- css_fn = true,       -- Enable all CSS *functions*: rgb_fn, hsl_fn
                -- Available modes for `mode`: foreground, background,  virtualtext
                mode = "background", -- Set the display mode.
                virtualtext = "■",
            },

            -- all the sub-options of filetypes apply to buftypes
            filetypes = {
                "*", -- Highlight all files, but customize some others.
                css = { rgb_fn = true, names = true },
                -- html = { names = false; }, -- Disable parsing "names" like Blue or Gray
                -- '!vim', -- Exclude vim from highlighting.
                -- 'javascript', -- Highlight for javascript file
                -- html = { mode = 'foreground'; }
            },

            buftypes = {},
        }"#
    }
}
