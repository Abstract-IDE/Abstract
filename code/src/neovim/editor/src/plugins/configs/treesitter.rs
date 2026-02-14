/*
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: nvim-treesitter
Source: https://github.com/nvim-treesitter/nvim-treesitter

Nvim Treesitter configurations and abstraction layer
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
*/

use crate::utils::constants::NVIM_TREESITTER_HOME;
use crate::lua_spec;

pub struct Plugin;

impl Plugin {
    pub fn spec() -> crate::plugins::spec::SpecInfo {
        let config = Self::config();
        let init = Self::init();
        lua_spec!(format!(
            // language=lua
            r#"{{
                'nvim-treesitter/nvim-treesitter',
                lazy = false,
                build = ':TSUpdate',
                init = {init},
                config = {config},
            }}"#
        ).leak())
    }
}

impl Plugin {
    pub fn init() -> &'static str {
        // language=lua
        r#"function()
            local register = vim.treesitter.language.register
            register("html", { "htmldjango" })
            register("bash", { "zsh" } )
            register('xml',  { 'svg', 'xslt' })
        end"#
    }
}

impl Plugin {
    pub fn config() -> &'static str {
        let nvim_treesitter_home: &str = &NVIM_TREESITTER_HOME;
        // language=lua
        format!(
            r#"function()
                require('nvim-treesitter').setup({{
                    -- Directory to install parsers and queries to (prepended to `runtimepath` to have priority)
                    -- NOTE!: Remember to run vim.opt.runtimepath:append("/some/path/to/store/parsers")
                    -- NOTE!: we are adding to rtp using lazy.nvim
                    install_dir = {nvim_treesitter_home:?},
                }})
            end"#
        )
        .leak()
    }
}
