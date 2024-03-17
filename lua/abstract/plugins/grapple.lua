--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin:    grapple-nvim
Github:    https://github.com/cbochs/grapple.nvim

Grapple is a plugin that aims to provide immediate navigation
to important files (and their last known cursor location).
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
	"cbochs/grapple.nvim",
	event = { "BufReadPost", "BufNewFile" },
	cmd = "Grapple",
	dependencies = {
		{ "nvim-tree/nvim-web-devicons", lazy = true },
	},
}

spec.config = function()
	require("grapple").setup({

		---Show icons next to tags or scopes in Grapple windows
		---Requires "nvim-tree/nvim-web-devicons"
		---@type boolean
		icons = true,

		---How a tag's path should be rendered in Grapple windows
		---  "relative": show tag path relative to the scope's resolved path
		---  "basename": show tag path basename and directory hint
		---@type "basename" | "relative"
		style = "basename",
	})
	require("abstract.utils.map").set_plugin("cbochs/grapple.nvim")
end

return spec
