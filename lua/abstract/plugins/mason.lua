--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: mason.nvim
Source: https://github.com/williamboman/mason.nvim

Companion plugin for nvim-lspconfig that allows you to seamlessly install LSP servers
locally (inside :echo stdpath("data")).
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
	"mason-org/mason.nvim",
	lazy = true,
	event = { "CmdlineEnter", "BufRead", "BufNewFile", "InsertEnter" },
}

spec.config = function()
	require("mason").setup({
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
	})

	-- === must load after mason ===

	local ensure_installed = require('abstract.plugins.lspconfig').setup()
	require('abstract.plugins.mason-lspconfig').setup(ensure_installed)

	require("abstract.plugins.mason-nvim-dap").setup()

	-- null/none-ls
	require("abstract.plugins.mason-null-ls").setup()
	require("abstract.plugins.none-ls").setup()
end

return spec
