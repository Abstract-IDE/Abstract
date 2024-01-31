--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin:    mason.nvim
Github:    https://github.com/williamboman/mason.nvim

Companion plugin for nvim-lspconfig that allows you to seamlessly install LSP servers
locally (inside :echo stdpath("data")).
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]


return {
	"williamboman/mason.nvim",
	cmd = "Mason",

	config = function()
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
	end,
}
