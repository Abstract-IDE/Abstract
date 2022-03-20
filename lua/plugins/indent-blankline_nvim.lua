
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

require("indent_blankline").setup {

	show_end_of_line = true,
	space_char_blankline = " ",
	show_current_context = true,
	show_current_context_start = true,
	show_trailing_blankline_indent = false,
	use_treesitter = true,

	char_list = {'│'}, -- {'|', '¦', '┆', '┊'},
    -- context_char = '┃' -- pecifies the character to be used for the current context indent line
    -- context_patterns = {'class, function', 'method', '^if'},

	filetype_exclude = {
		'help',
		'startify',
		'alpha',
		'dashboard',
		'packer',
		'neogitstatus',
		'NvimTree',
		'lsp-installer',
	},

	buftype_exclude = {
		'terminal',
		'nofile',
	},

	char_highlight_list = {
		"IndentBlanklineIndent1",
		"IndentBlanklineIndent2",
		"IndentBlanklineIndent3",
		"IndentBlanklineIndent4",
		"IndentBlanklineIndent5",
		"IndentBlanklineIndent6",
		"IndentBlanklineIndent7",
		"IndentBlanklineIndent8",
    },
}

-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━❰ end configs ❱━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

