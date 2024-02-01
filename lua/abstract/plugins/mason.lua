--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin:    mason.nvim
Github:    https://github.com/williamboman/mason.nvim

Companion plugin for nvim-lspconfig that allows you to seamlessly install LSP servers
locally (inside :echo stdpath("data")).

!!! WARNINGS !!!:
0. mason.nvim is optimized to load as little as possible during setup. Lazy-loading the plugin, or somehow deferring the setup, is not recommended.
1. make sure `lspconfig` is not loaded after `mason-lspconfig`.
2. Also, make sure not to set up any servers via `lspconfig` _before_ calling `mason-lspconfig`'s setup function.
3. It's important that you set up the plugins in the following order:
    mason.nvim
    mason-lspconfig.nvim
    Setup servers via lspconfig
4. Pay extra attention to this if you lazy-load plugins, or somehow "chain" the loading of plugins via your plugin manager.
	require("mason").setup()
	require("mason-lspconfig").setup()

	After setting up mason-lspconfig you may set up servers via lspconfig
	require("lspconfig").lua_ls.setup {}
	require("lspconfig").rust_analyzer.setup {}
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
	"williamboman/mason.nvim",
	event = { "BufReadPre", "BufNewFile", "InsertEnter" },
}

local mason = function()
	require("mason").setup({
		ui = {
			-- The border to use for the UI window. Accepts same border values as |nvim_open_win()|.
			border = "rounded",
			icons = {
				-- The list icon to use for installed packages.
				package_installed = "✓",
				-- The list icon to use for packages that are installing, or queued for installation.
				package_pending = "➜",
				-- The list icon to use for packages that are not installed.
				package_uninstalled = "✗",
			},
		},
		-- Controls to which degree logs are written to the log file. It's useful to set this to vim.log.levels.DEBUG when
		-- debugging issues with package installations.
		log_level = vim.log.levels.INFO,
		-- Limit for the maximum amount of packages to be installed at the same time. Once this limit is reached, any further
		-- packages that are requested to be installed will be put in a queue.
		max_concurrent_installers = 4,
	})
end

spec.config = function()
	mason()
	require("abstract.plugins.mason-lspconfig").config()
	require("abstract.plugins.none-ls").config()
end

return spec
