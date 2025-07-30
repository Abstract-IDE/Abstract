--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: windows.nvim
Source: https://github.com/anuvyklack/windows.nvim

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
	keys = require("abstract.configs.mapping").plugin["anuvyklack/windows.nvim"],
}

spec.opts = {
	autowidth = {
		enable = false,
		winwidth = 5,
	},
	ignore = {
		buftype = { "quickfix" },
		filetype = { "NvimTree", "neo-tree", "undotree", "gundo" },
	},
}

return spec
