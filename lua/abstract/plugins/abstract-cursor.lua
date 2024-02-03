--[[━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin:    abstract-cursor
Github:    https://github.com/Abstract-IDE/abstract-cursor

dynamic cursor
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━]]

local spec = {
	"Abstract-IDE/abstract-cursor",
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
				fg = "#000000",
				bg = "#ac3131",
				bold = true,
			},
			v = {
				fg = "#000000",
				bg = "#d1d1d1",
				bold = true,
			},
			V = {
				fg = "#000000",
				bg = "#d1d1d1",
				bold = true,
			},
			["^V"] = {
				fg = "#000000",
				bg = "#d1d1d1",
				bold = true,
			},
		},
	},
}

return spec
