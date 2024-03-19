--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin:    windows.nvim
Github:    https://github.com/anuvyklack/windows.nvim

Automatically expand width of the current window.
Maximizes and restore it. And all this with nice animations!
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
	"anuvyklack/windows.nvim",
	dependencies = {
		"anuvyklack/middleclass",
	},
}

spec.config = function()
	require("windows").setup({
		autowidth = {
			enable = false,
			winwidth = 5,
		},
		ignore = {
			buftype = { "quickfix" },
			filetype = { "NvimTree", "neo-tree", "undotree", "gundo" },
		},
	})
	require("abstract.utils.map").set_plugin("anuvyklack/windows.nvim")
end

return spec
