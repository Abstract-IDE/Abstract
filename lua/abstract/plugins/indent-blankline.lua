--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: indent-blankline.nvim
Github: https://github.com/lukas-reineke/indent-blankline.nvim

This plugin adds indentation guides to Neovim. It uses Neovim's virtual text feature and no conceal
NOTE:
	to know more about filetype_exclude and buftype_exclude
	https://github.com/lukas-reineke/indent-blankline.nvim/issues/284
	:set buftype
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]



return {  -- Indent guides for Neovim
	"lukas-reineke/indent-blankline.nvim",
	main = "ibl",
	event = { "BufReadPost", "BufNewFile" },
	opts = {},
	config = function()
		local _ibl, ibl = pcall(require, "ibl")
		if not _ibl then
			return
		end

		ibl.setup({

			debounce = 100,
			indent = {
				char = "▎", -- | ¦ ┆ ┊ ║ ▎
				tab_char = "▎",
				smart_indent_cap = true,
				priority = 2,
			},
			whitespace = {
				highlight = { "Whitespace", "NonText" },
				remove_blankline_trail = true,
			},
			scope = {
				enabled = true,
			},
			exclude = {
				filetypes = {
					"rust",
					"lspinfo",
					"packer",
					"checkhealth",
					"help",
					"man",
					"gitcommit",
					"TelescopePrompt",
					"TelescopeResults",
					"startify",
					"alpha",
					"dashboard",
					"neogitstatus",
					"NvimTree",
					"",
				},
				buftypes = { "terminal", "nofile" },
			},
		})
	end,
}
