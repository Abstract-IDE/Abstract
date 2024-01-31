--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: abstract-winbar
Github: https://github.com/Abstract-IDE/abstract-winbar

Neovim Winbar: Elevating Awesome. Simple, Dynamic, Navic-Powered.
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]



return {
	"Abstract-IDE/abstract-winbar",

	config = function()
		require("abstract-winbar").setup({
			-- exclude_filetypes = {},
		})
	end
}
