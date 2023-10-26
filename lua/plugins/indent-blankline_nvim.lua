
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ───────────────────────────────────────────────── --
--   Plugin:    indent-blankline.nvim
--   Github:    github.com/lukas-reineke/indent-blankline.nvim
-- ───────────────────────────────────────────────── --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --





-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━❰ configs ❱━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

--[===[
    NOTE:
        to know more about filetype_exclude and buftype_exclude
        https://github.com/lukas-reineke/indent-blankline.nvim/issues/284
		:set buftype
--]===]

local ibl_import, ibl = pcall(require, "ibl")
if not ibl_import then return end

ibl.setup {

	debounce = 100,
	indent = {
		char = "▎", -- | ¦ ┆ ┊ ║ ▎
		smart_indent_cap = true,
		priority = 2,
		highlight = {
			"IndentBlanklineIndent1", "IndentBlanklineIndent2", "IndentBlanklineIndent3", "IndentBlanklineIndent4",
			"IndentBlanklineIndent5", "IndentBlanklineIndent6", "IndentBlanklineIndent7", "IndentBlanklineIndent8",
		},
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
			"rust", "lspinfo", "packer", "checkhealth", "help", "man", "gitcommit", "TelescopePrompt",
			"TelescopeResults", 'startify', 'alpha', 'dashboard', 'neogitstatus', 'NvimTree', '',
		},
		buftypes = { "terminal",'nofile', },
	}
}

-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━❰ end configs ❱━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

