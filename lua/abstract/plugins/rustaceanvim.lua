--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: rustaceanvim
Source: https://github.com/mrcjkb/rustaceanvim

Supercharge your Rust experience in Neovim!
A heavily modified fork of rust-tools.nvim
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
	"mrcjkb/rustaceanvim",
	version = "^6", -- Recommended
	lazy = false, -- This plugin is already lazy
}

spec.config = function()
	vim.g.rustaceanvim = {

		-- Plugin configuration
		tools = {
			float_win_config = {
				border = "rounded",
			},
		},
		-- DAP configuration
		dap = {},
	}
end

return spec
