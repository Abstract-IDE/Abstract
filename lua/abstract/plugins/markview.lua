--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: markview.nvim
Source: https://github.com/OXY2DEV/markview.nvim

A hackable markdown, Typst, latex, html(inline) & YAML previewer for Neovim
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
	"OXY2DEV/markview.nvim",
	-- Do not lazy load this plugin as it is already lazy-loaded.
	-- Lazy-loading may cause more time for the previews to load when starting Neovim!
	lazy = false,
}


---@class mkv.config
spec.config = function()
	-- NOTE: for now lets disable for all filetype. later we will provide a mapping to enable/disable
	require("abstract.utils.map").set_map("OXY2DEV/markview.nvim")

	require("markview").setup({
		experimental = {
			check_rtp = false
		},
		html = {
			enable = false,
		},
		latex = {
			enable = false,
		},
		markdown = {
			enable = false,
		},
		markdown_inline = {
			enable = false,
		},
		typst = {
			enable = false,
		},
		yaml = {
			enable = false,
		}
	})
end


return spec
