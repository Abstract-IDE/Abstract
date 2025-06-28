--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: flutter-tools.nvim
Source: https://github.com/akinsho/flutter-tools.nvim

Tools to help create flutter apps in neovim using the native lsp
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
	"akinsho/flutter-tools.nvim",
	ft = { "dart" },
}

spec.config = true

return spec
