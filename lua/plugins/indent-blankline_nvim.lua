
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

local _ibl, ibl = pcall(require, "ibl")
if not _ibl then return end

ibl.setup {

	debounce = 100,
	indent = {
		char = "▎", -- | ¦ ┆ ┊ ║ ▎
		tab_char= "▎",
		smart_indent_cap = true,
		priority = 2,
		highlight = {
			"IblIndent1", "IblIndent2", "IblIndent3", "IblIndent4",
			"IblIndent5", "IblIndent6", "IblIndent7", "IblIndent8",
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

