--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin:    nvim-ts-autotag
Github:    github.com/windwp/nvim-ts-autotag

 Use treesitter to auto close and auto rename html tag
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
	"windwp/nvim-ts-autotag",
	event = "InsertEnter",
	dependencies = {
		"nvim-treesitter/nvim-treesitter",
	},
}

spec.config = function()
	require("nvim-ts-autotag").setup({})
end

return spec
