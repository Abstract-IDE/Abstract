--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: nvim-web-devicons
Source: https://github.com/nvim-tree/nvim-web-devicons

A lua fork of vim-devicons.
This plugin provides the same icons as well as colors for each icon.
Light and dark color variants are provided.
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
	"nvim-tree/nvim-web-devicons",
	lazy = true,
}

spec.config = function()
	require("nvim-web-devicons").get_icons()
end

return spec
