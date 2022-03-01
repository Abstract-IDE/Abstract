
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ───────────────────────────────────────────────── --
--   PluginName: telescope.nvim
--   Github:     github.com/nvim-telescope/telescope.nvim
-- ───────────────────────────────────────────────── --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --





-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━❰ configs ❱━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

require('telescope').setup {

	defaults = {
		vimgrep_arguments = {
			'rg',
			'--color=never',
			'--no-heading',
			'--with-filename',
			'--line-number',
			'--column',
			'--smart-case',
		},

		initial_mode = "insert",
		selection_strategy = "reset",
		sorting_strategy = "ascending",
		layout_strategy = "horizontal",
		layout_config = {
			horizontal = {mirror = false, prompt_position = "top"},
			vertical = {mirror = false},
		},
		file_ignore_patterns = {},
		file_sorter = require'telescope.sorters'.get_fuzzy_file,
		generic_sorter = require'telescope.sorters'.get_generic_fuzzy_sorter,
		file_previewer = require'telescope.previewers'.vim_buffer_cat.new,
		grep_previewer = require'telescope.previewers'.vim_buffer_vimgrep.new,
		qflist_previewer = require'telescope.previewers'.vim_buffer_qflist.new,

		prompt_prefix = "🔎︎ ",
		selection_caret = "➤ ",
		entry_prefix = "  ",
		winblend = 0,
		border = {},
		borderchars = {'─', '│', '─', '│', '╭', '╮', '╯', '╰'},
		color_devicons = true,
		use_less = true,
		set_env = {['COLORTERM'] = 'truecolor'}, -- default = nil,
		path_display = {'absolute'}, -- How file paths are displayed ()

		-- Developer configurations: Not meant for general override
		buffer_previewer_maker = require'telescope.previewers'.buffer_previewer_maker,
	},

	extensions = {

		fzf = {
			fuzzy = true, -- false will only do exact matching
			override_generic_sorter = false, -- override the generic sorter
			override_file_sorter = true, -- override the file sorter
			case_mode = "smart_case", -- or "ignore_case" or "respect_case". the default case_mode is "smart_case"
		},

		file_browser = {
			theme = "ivy",
		},
	},

}
-- To get telescope-extension loaded and working with telescope,
-- you need to call load_extension, somewhere after setup function:
require('telescope').load_extension('fzf')
require("telescope").load_extension('file_browser')


-- replace the simple file browsing capabilities of netrw with telescope file_browse extension
--  for example, nvim ~/downloads will open directory in Telescope
-- local bufnr = vim.api.nvim_get_current_buf() -- get bufffer number
-- local curr_file = vim.api.nvim_buf_get_name(bufnr) -- get buffer/filename with location
-- local is_dir = require("funcs.is_dir").is_dir(curr_file)
-- if is_dir then vim.cmd([[au BufEnter * :Telescope file_browser <CR> ]]) end

-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━❰ end configs ❱━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --




-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━❰ Mappings ❱━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

local keymap = vim.api.nvim_set_keymap

--      --> Launch Telescope without any argument
keymap('n', "\\\\", "<cmd>lua require('telescope.builtin').builtin() <CR>",
       {silent = true, noremap = true})

--       --> show all availabe MAPPING
keymap('n', "<leader>M", "<cmd>lua require('telescope.builtin').keymaps() <CR>",
       {silent = true, noremap = true})

--       --> show buffers/opened files
keymap('n', "<C-b>", "<cmd>lua require('telescope.builtin').buffers() <CR>",
       {silent = true, noremap = true})

--       --> Find Files
-- NOTE1: to get project root's directory, extra plugin (github.com/ygm2/rooter.nvim) is used.
-- any config related to project root is in seperate config file (lua/plugin_confs/rooter_nvim.lua)
-- to change settings related to working directory, refer to rooter_nvim.lua config file

-- Find files from current file's project
keymap('n', "<C-p>", ":Telescope find_files <cr>",
       {silent = true, noremap = true})
-- show all files from current working directory
keymap('n', "<C-f>",
       "<cmd>lua require('telescope.builtin').find_files( { cwd = vim.fn.expand('%:p:h') }) <CR>",
       {silent = true, noremap = true})

--       --> Code Action
-- for example, in flutter/dart you can wrap or delete widgets using code action.
-- for more see :help builtin.lsp_code_actions() or :help builtin.lsp_range_code_actions()
keymap('n', "<Space>ca",
       "<cmd>lua  require('telescope.builtin').lsp_code_actions( {layout_config={width=50, height=20} } ) <CR>",
       {silent = true, noremap = true})
keymap('x', "<leader>ca",
       "<cmd>lua  require('telescope.builtin').lsp_range_code_actions( {layout_config={width=50, height=25} } ) <CR>",
       {silent = true, noremap = true})

-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━❰ end Mappings ❱━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

