/*
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: mason.nvim
Source: https://github.com/mason-org/mason.nvim

Git Graph plugin for neovim.
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
*/

use super::none_ls;
use crate::core::lsp::Lsp;

pub struct Plugin;

impl Plugin {
    pub fn spec() -> &'static str {
        let config = Self::config();

        let config_mason_lspconfig = PluginMasonLspConfig::spec();
        let config_mason_null_ls = PluginMasonNullLs::spec();
        let config_mason_nvim_dap = PluginMasonNvimDap::spec();

        format!(
            // language=lua
            r#"{{
                "mason-org/mason.nvim",
                dependencies = {{
                    {config_mason_lspconfig},
                    {config_mason_null_ls},
                    {config_mason_nvim_dap},
                }},
                lazy=true,
                event = {{ "CmdlineEnter", "BufRead", "BufNewFile", "InsertEnter" }},
                config={config},
            }}"#
        )
        .leak()
    }

    pub fn opts() -> &'static str {
        r#"{
            -- Controls to which degree logs are written to the log file. It's useful to set this to vim.log.levels.DEBUG when
            -- debugging issues with package installations.
            log_level = vim.log.levels.INFO,
            -- Limit for the maximum amount of packages to be installed at the same time. Once this limit is reached, any further
            -- packages that are requested to be installed will be put in a queue.
            max_concurrent_installers = 4,

            ui = {
                -- Whether to automatically check for new versions when opening the :Mason window.
                check_outdated_packages_on_open = true,
                -- The border to use for the UI window. Accepts same border values as |nvim_open_win()|.
                border = "rounded",
                -- Width of the window. Accepts:
                -- - Integer greater than 1 for fixed width.
                -- - Float in the range of 0-1 for a percentage of screen width.
                width = 0.8,
                -- Height of the window. Accepts:
                -- - Integer greater than 1 for fixed height.
                -- - Float in the range of 0-1 for a percentage of screen height.
                height = 0.9,

                icons = {
                    -- The list icon to use for installed packages.
                    package_installed = "✓",
                    -- The list icon to use for packages that are installing, or queued for installation.
                    package_pending = "➜",
                    -- The list icon to use for packages that are not installed.
                    package_uninstalled = "✗",
                },
            },
        }"#
    }

    pub fn config() -> &'static str {
        // keymaps::MAPPING.signal(keymaps::Key::Mason);
        let mason_opts = Self::opts();

        let setup_lsp = Lsp::setup().unwrap_or("");
        let setup_mason_lspconfig = PluginMasonLspConfig::setup();
        let setup_mason_nvim_dap = PluginMasonNvimDap::setup();
        let setup_mason_null_ls = PluginMasonNullLs::setup();
        let setup_none_ls = none_ls::Plugin::setup();

        // language=lua
        format!(
            r#"function()
                require("mason").setup({mason_opts})

                -- === must be loaded after mason ===
                -- WARN! order matters
                {setup_lsp}
                {setup_mason_lspconfig}
                {setup_mason_null_ls}
                {setup_mason_nvim_dap}
                {setup_none_ls}

            end"#
        )
        .leak()
    }
}

/*
────────────────────────────────────────────────
Plugin: mason-lspconfig.nvim
Source: https://github.com/mason-org/mason-lspconfig.nvim

Extension to mason.nvim that makes it easier to use lspconfig with mason.nvim.
────────────────────────────────────────────────
*/

pub struct PluginMasonLspConfig;

impl PluginMasonLspConfig {
    pub fn spec() -> &'static str {
        r#"{
            "mason-org/mason-lspconfig.nvim",
            lazy=true,
            dependencies = {
                "neovim/nvim-lspconfig",
                lazy=true,
            },
        }"#
    }

    pub fn setup() -> &'static str {
        // language=lua
        r#"
        require("mason-lspconfig").setup({
            automatic_enable = {
                true, -- will automatically enable (vim.lsp.enable()) installed servers
                exclude = {
                    "rust_analyzer",
                    "ts_ls",
                },
            },

            -- A list of servers to automatically install if they're not already installed. Example: { "rust_analyzer@nightly", "lua_ls" }
            ---@type string[]
            -- ensure_installed = ensure_installed,
        })
        "#
    }
}

/*
────────────────────────────────────────────────
Plugin: mason-null-ls.nvim
Source: https://github.com/jay-babu/mason-null-ls.nvim

mason-null-ls bridges mason.nvim with the null-ls plugin
 - making it easier to use both plugins together.
────────────────────────────────────────────────
*/

pub struct PluginMasonNullLs;

impl PluginMasonNullLs {
    pub fn spec() -> &'static str {
        r#"{
            "jay-babu/mason-null-ls.nvim",
            lazy=true,
            event = { "BufReadPre", "BufNewFile" },
        }"#
    }

    pub fn setup() -> &'static str {
        // language=lua
        r#"
        require("mason-null-ls").setup({
            -- A list of sources to install if they're not already installed.
            ensure_installed = {},
            -- Enable or disable null-ls methods to get set up
            -- This setting is useful if some functionality is handled by other plugins such as `conform` and `nvim-lint`
            methods = {
                formatting = true,
                code_actions = true,
            },
            automatic_installation = false,
            handlers = {},
        })
        "#
    }
}

/*
────────────────────────────────────────────────
Plugin: mason-nvim-dap.nvim
Source: https://github.com/jay-babu/mason-nvim-dap.nvim

mason-nvim-dap bridges mason.nvim with the nvim-dap plugin
- making it easier to use both plugins together.
────────────────────────────────────────────────
*/

pub struct PluginMasonNvimDap;

impl PluginMasonNvimDap {
    pub fn spec() -> &'static str {
        r#"{
            "jay-babu/mason-nvim-dap.nvim",
            lazy=true,
        }"#
    }

    pub fn setup() -> &'static str {
        // language=lua
        r#"
        require("mason-nvim-dap").setup({
            automatic_installation = false,
            ensure_installed = { "python", "delve" },
            handlers = {
                function(cnf)
                    -- all sources with no handler get passed here

                    -- Keep original functionality
                    require('mason-nvim-dap').default_setup(cnf)
                end,
                -- python = function(cnf)
                --     config.adapters = {
                --         type = "executable",
                --         command = "/usr/bin/python3",
                --         args = {
                --             "-m",
                --             "debugpy.adapter",
                --         },
                --     }
                --     require('mason-nvim-dap').default_setup(cnf) -- don't forget this!
                -- end,
            },

        })
        "#
    }
}
