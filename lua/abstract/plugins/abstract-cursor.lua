--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: abstract-cursor
Source: https://github.com/Abstract-IDE/abstract-cursor

dynamic cursor
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
	"Abstract-IDE/abstract-cursor",
	event = { "BufRead" },
}

spec.opts = {
	Visual = {
		enable = true,
		colors = {},
	},

	CursorLine = {
		enable = true,
		colors = {},
	},

	CursorLineNr = {
		enable = true,
		colors = {
			i = {
				fg = "#ac3131",
				reverse=true,
			},
			v = {
				fg = "#d1d1d1",
				reverse=true,
			},
				reverse=true,
			V = {
				fg = "#ffffff",
				reverse=true,
			},
			["^V"] = {
				fg = "#d1d1d1",
				reverse=true,
			},
		},
	},
}

return spec
