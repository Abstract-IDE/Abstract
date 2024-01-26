--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: abstract-cursor
Github: https://github.com/Abstract-IDE/abstract-cursor

dynamic cursor
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local _cursor, cursor = pcall(require, "abstract-cursor")
if not _cursor then
	return
end

cursor.setup({
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
})
