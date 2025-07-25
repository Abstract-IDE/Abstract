--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: quicker.nvim
Source: https://github.com/stevearc/quicker.nvim


Improved UI and workflow for the Neovim quickfix
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
	"stevearc/quicker.nvim",
	event = "FileType qf",
}
---@module "quicker"
---@type quicker.SetupOptions
spec.opts = {
	follow = {
		-- When quickfix window is open, scroll to closest item to the cursor
		enabled = true,
	},
	keys = {
		{
			">",
			function()
				require("quicker").expand({ before = 2, after = 2, add_to_existing = true })
			end,
			desc = "Expand quickfix context",
		},
		{
			"<",
			function()
				require("quicker").collapse()
			end,
			desc = "Collapse quickfix context",
		},
	},
}


return spec
