--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: messenger.nvim
Source: https://github.com/lsig/messenger.nvim

Reveal the commit message under the cursor - Written in Lua
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
	"lsig/messenger.nvim",
}

spec.config = function()
	require("messenger").setup({
		border = "rounded", -- Valid values: "none", "single", "double", "rounded", "solid", "shadow".
		heading_hl = "#89b4fa", -- Any hex color that your terminal supports
	})
	require("abstract.utils.map").set_map("lsig/messenger.nvim")
end

return spec
