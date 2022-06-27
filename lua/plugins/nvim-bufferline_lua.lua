
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ───────────────────────────────────────────────── --
--   Plugin:    nvim-bufferline.lua
--   Github:    github.com/akinsho/nvim-bufferline.lua
-- ───────────────────────────────────────────────── --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --





-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━❰ configs ❱━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

-- safely import bufferline
local bufferline_imported, bufferline = pcall(require, 'bufferline')
if not bufferline_imported then return end

bufferline.setup {

	options = {

		mode = "buffers", -- set to "tabs" to only show tabpages instead
		numbers = "ordinal", -- "none" | "ordinal" | "buffer_id" | "both" | function({ ordinal, id, lower, raise }): string
		always_show_bufferline = false, -- don't show bufferline if there is only one file is opened

		close_command = "bdelete! %d", -- can be a string | function, see "Mouse actions"
		right_mouse_command = "bdelete! %d", -- can be a string | function, see "Mouse actions"
		left_mouse_command = "buffer %d", -- can be a string | function, see "Mouse actions"
		middle_mouse_command = nil, -- can be a string | function, see "Mouse actions"

		-- NOTE: this plugin is designed with this icon in mind,
		-- and so changing this is NOT recommended, this is intended
		-- as an escape hatch for people who cannot bear it for whatever reason
		indicator_icon = '▎',
		-- indicator_icon      = '',
		buffer_close_icon = '',
		modified_icon = '●',
		close_icon = '',
		left_trunc_marker = '',
		right_trunc_marker = '',

		--- name_formatter can be used to change the buffer's label in the bufferline.
		--- Please note some names can/will break the
		--- bufferline so use this at your discretion knowing that it has
		--- some limitations that will *NOT* be fixed.
		name_formatter = function(buf) -- buf contains a "name", "path" and "bufnr"
			-- remove extension from markdown files for example
			if buf.name:match('%.md') then return vim.fn.fnamemodify(buf.name, ':t:r') end
		end,

		max_name_length = 18,
		max_prefix_length = 15, -- prefix used when a buffer is de-duplicate
		tab_size = 18,

		show_close_icon = false,
		show_buffer_icons = true, -- disable filetype icons for buffers
		show_buffer_close_icons = false,
		show_tab_indicators = true,
		enforce_regular_tabs = true, -- if set true, tabs will be prevented from extending beyond the tab size and all tabs will be the same length

		view = "multiwindow",
		-- can also be a table containing 2 custom separators
		-- [focused and unfocused]. eg: { '|', '|' }
		separator_style = {"", ""}, -- options "slant" | "thick" | "thin" | { 'any', 'any' },
		offsets = {
			-- options function , text_" "h always_show_bufferline = false
			{filetype = "NvimTree", text = "Explorer", text_align = "left"},
		},
	},

	highlights = {
		fill = {guibg = {highlight = "BufferLineFill", attribute = "bg"}},
		background = { -- current tab
			guifg = {highlight = "BufferCurrent", attribute = "fg"},
			guibg = {highlight = "BufferCurrent", attribute = "bg"},
		},
		separator = {guifg = {highlight = "BufferSeparator", attribute = "fg"}},
		separator_selected = {
			guifg = {highlight = "BufferSeparatorSelected", attribute = "fg"},
		},
		close_button_selected = {
			guifg = {highlight = "BufferCloseButtonSelected", attribute = "fg"},
			guibg = {highlight = "BufferCloseButtonSelected", attribute = "bg"},
		},
		duplicate = {
			guifg = {highlight = "BufferCurrentSign", attribute = "fg"},
			guibg = {highlight = "BufferLineFill", attribute = "bg"},
		},
		duplicate_selected = {
			guifg = {highlight = "BufferCurrentSign", attribute = "fg"},
		},
		modified = {
			guifg = {highlight = "BufferCurrentSign", attribute = "fg"},
			guibg = {highlight = "BufferLineFill", attribute = "bg"},
		},
		modified_selected = {
			guifg = {highlight = "BufferCurrentSign", attribute = "fg"},
		},
		numbers = {
			guibg = {highlight = "BuffNumbers", attribute = "bg"},
		},

		tab_selected = {
			guifg = {highlight = "TabSelectedFG", attribute = "fg"},
			guibg = {highlight = "TabSelectedBG", attribute = "bg"},
		},
		tab = {
			guifg = {highlight = "TabFG", attribute = "fg"},
			guibg = {highlight = "TabBG", attribute = "bg"},
		},

		-- duplicate_visible = {
		-- },
		-- close_button = {
		-- },
		-- close_button_visible = {
		-- },
		-- tab_selected = {
		-- },
		-- buffer_visible = {
		-- },
		-- buffer_selected = {
		-- },
		-- modified_visible = {
		-- },
		-- separator_visible = {
		-- },
		-- indicator_selected = {
		-- },
	},

}

-- vim.cmd("autocmd BufDelete * if len(getbufinfo({'buflisted':1})) -1 < 1 | set showtabline=1 | endif")

-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━❰ end configs ❱━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --




-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━❰ Mappings ❱━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

local keymap = vim.api.nvim_set_keymap
local options = {noremap = true, silent = true}

-- Move to previous/next
keymap('n', '\\', ':BufferLineCycleNext<CR>', options)
keymap('n', '|', ':BufferLineCyclePrev<CR>', options)

-- Re-order to previous/next
keymap('n', '<Leader>.', ':BufferLineMoveNext<CR>', options)
keymap('n', '<Leader>,', ':BufferLineMovePrev<CR>', options)

-- Close buffer
-- nnoremap <silent>    <A-c> :BufferClose<CR>

-- Magic buffer-picking mode
keymap('n', '<Space><Space>', ':BufferLinePick<CR>', options)

-- go to buffer number
keymap('n', '<Leader>1', ':BufferLineGoToBuffer 1<CR>', options)
keymap('n', '<Leader>2', ':BufferLineGoToBuffer 2<CR>', options)
keymap('n', '<Leader>3', ':BufferLineGoToBuffer 3<CR>', options)
keymap('n', '<Leader>4', ':BufferLineGoToBuffer 4<CR>', options)
keymap('n', '<Leader>5', ':BufferLineGoToBuffer 5<CR>', options)
keymap('n', '<Leader>6', ':BufferLineGoToBuffer 6<CR>', options)
keymap('n', '<Leader>7', ':BufferLineGoToBuffer 7<CR>', options)
keymap('n', '<Leader>8', ':BufferLineGoToBuffer 8<CR>', options)
keymap('n', '<Leader>9', ':BufferLineGoToBuffer 9<CR>', options)

-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━❰ end Mappings ❱━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

