--[[
--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: mason-lspconfig.nvim
Source: https://github.com/williamboman/mason-lspconfig.nvim

Extension to mason.nvim that makes it easier to use lspconfig with mason.nvim.
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
	"mason-org/mason-lspconfig.nvim",
	lazy = true,
}

spec.setup = function()
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
		ensure_installed = {
			"bashls",
			"cssls",
			"eslint",
			"html",
			"jsonls",
			"lua_ls",
			"basedpyright",
			"ts_ls", -- managed by typescript-tools
		},
	})
end

return spec
