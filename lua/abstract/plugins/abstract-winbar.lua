--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: abstract-winbar
Github: https://github.com/Abstract-IDE/abstract-winbar

Neovim Winbar: Elevating Awesome. Simple, Dynamic, Navic-Powered.
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
	"Abstract-IDE/abstract-winbar",
	dependencies = {
		"SmiteshP/nvim-navic", -- Simple winbar/statusline plugin that shows your current code context
	},
}

spec.config = function()
	require("abstract-winbar").setup({
		-- exclude_filetypes = {},
	})
end

return spec
