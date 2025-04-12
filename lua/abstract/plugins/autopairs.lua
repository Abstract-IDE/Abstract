--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: nvim-autopairs
Source: https://github.com/windwp/nvim-autopairs

autopairs for neovim written in lua
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
	"windwp/nvim-autopairs",
	lazy = true,
	event = "InsertEnter",
	opts = {},
}

return spec
