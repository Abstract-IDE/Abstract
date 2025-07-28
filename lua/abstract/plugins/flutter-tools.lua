--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: flutter-tools.nvim
Source: https://github.com/nvim-flutter/flutter-tools.nvim
        https://github.com/akinsho/flutter-tools.nvim (Old Source)

Tools to help create flutter apps in neovim using the native lsp
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
	"akinsho/flutter-tools.nvim",
	ft = { "dart" },
}

spec.config = function()
	require("flutter-tools").setup {
		widget_guides = {
			enabled = true,
		},
	}
end

return spec
