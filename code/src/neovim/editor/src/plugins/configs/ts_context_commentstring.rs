/*
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: nvim-ts-context-commentstring
Source: https://github.com/JoosepAlviste/nvim-ts-context-commentstring

Neovim treesitter plugin for setting the commentstring based on the cursor location in a file.
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
*/

use crate::lua_spec;

pub struct Plugin;

impl Plugin {
    pub fn spec() -> crate::plugins::spec::SpecInfo {
        // language=lua
        lua_spec!(r#"{
            "JoosepAlviste/nvim-ts-context-commentstring",
            lazy = true,
        }"#)
    }
}

impl Plugin {
    pub fn setup() -> &'static str {
        // language=lua
        r#"function()
            -- skip backwards compatibility routines and speed up loading.
            vim.g.skip_ts_context_commentstring_module = true

            require("ts_context_commentstring").setup({
                enable_autocmd = false,
            })

            -- Integration of nvim-ts-context-commentstring to numToStr/Comment.nvim
            return require("ts_context_commentstring.integrations.comment_nvim").create_pre_hook()
        end"#
    }
}

