--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: abstract-cursor
Github: https://github.com/Abstract-IDE/abstract-cursor

dynamic cursor
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

return {
	"Abstract-IDE/abstract-cursor",
	config = function()
		require("abstract-cursor").setup({
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
	end
}
