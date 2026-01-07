/*
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: fff.nvim
Source: https://github.com/dmtrKovalenko/fff.nvim

Finally a smart fuzzy file picker for neovim.
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
*/

pub struct Plugin;

impl Plugin {
    pub fn spec() -> &'static str {
        let config = Self::config();
        let init = Self::init();
        let spec = format!(
            // language=lua
            r#"{{
                'nvim-treesitter/nvim-treesitter',
                lazy = false,
                build = ':TSUpdate',
                init = {init},
                config = {config},
            }}"#
        );

        Box::leak(spec.into_boxed_str())
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
        // language=lua
        r#"function()
            require('nvim-treesitter').setup {
                -- Directory to install parsers and queries to (prepended to `runtimepath` to have priority)
                -- NOTE!: Remember to run vim.opt.runtimepath:append("/some/path/to/store/parsers")
                install_dir = vim.fn.stdpath('data') .. '/rust/treesitter'
            }
        end"#
    }
}
