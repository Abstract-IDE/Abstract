
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ───────────────────────────────────────────────── --
--   PluginName: telescope.nvim
--   Github:     github.com/nvim-telescope/telescope.nvim
-- ───────────────────────────────────────────────── --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --





-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━❰ configs ❱━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

-- safely import telescope
local telescope_imported_ok, telescope = pcall(require, 'telescope')
if not telescope_imported_ok then return end

telescope.setup {

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
			horizontal = {
				mirror = false,
				prompt_position = "top",
				preview_width = 0.4,
			},
			vertical = { mirror = false, },
			width = 0.8,
			height = 0.9,
            preview_cutoff = 80,
		},
		file_ignore_patterns = {
			"__pycache__/", "__pycache__/*",

			"build/",       "env/",    "gradle/",  "node_modules/", "node_modules/*",
			"smalljre_*/*", "target/", "vendor/*",

			".dart_tool/",  ".git/",   ".github/", ".gradle/",      ".idea/",        ".vscode/",

			"%.sqlite3",    "%.ipynb", "%.lock",   "%.pdb",
			"%.dll",        "%.class", "%.exe",    "%.cache", "%.pdf",  "%.dylib",
			"%.jar",        "%.docx",  "%.met",    "%.burp",  "%.mp4",  "%.mkv", "%.rar",
			"%.zip",        "%.7z",    "%.tar",    "%.bz2",   "%.epub", "%.flac","%.tar.gz",
		},

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
		path_display = {'truncate'}, -- How file paths are displayed ()

		preview = {
			msg_bg_fillchar = " ",
			treesitter = false,
		},

		live_grep = {
			preview = {
				treesitter = false
			}
		},

		-- Developer configurations: Not meant for general override
		buffer_previewer_maker = require'telescope.previewers'.buffer_previewer_maker,
	},

	extensions = {

		fzf = {
			fuzzy = true, -- false will only do exact matching
			override_generic_sorter = true, -- override the generic sorter
			override_file_sorter = true, -- override the file sorter
			case_mode = "smart_case", -- or "ignore_case" or "respect_case". the default case_mode is "smart_case"
		},

		media_files = {
			-- filetypes whitelist
			filetypes = {"png", "jpg", "mp4", "webm", "pdf"},
			find_cmd = "fd" -- find command (defaults to `fd`)
		},

		file_browser = {
			theme = "ivy",
			-- disables netrw and use telescope-file-browser in its place
			hijack_netrw = true,
		},

		["ui-select"] = {
			require("telescope.themes").get_dropdown {
				winblend = 15,
				layout_config = {
					prompt_position = "top",
					width = 64,
					height = 15,
				},
				border = {},
				previewer = false,
				shorten_path = false,
			},
		}

	},

}
-- To get telescope-extension loaded and working with telescope,
-- you need to call load_extension, somewhere after setup function:
require('telescope').load_extension('fzf')
require("telescope").load_extension('file_browser')
require('telescope').load_extension('media_files')
require("telescope").load_extension("ui-select")

-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━❰ end configs ❱━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --




-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━❰ Mappings ❱━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

local keymap = vim.api.nvim_set_keymap
local options = { silent = true, noremap = true }

--      --> Launch Telescope without any argument
keymap('n', "tt",   "<cmd>lua require('telescope.builtin').builtin() <CR>", options)

--      --> Lists available help tags and opens a new window with the relevant help info on
keymap('n', "th",   "<cmd>lua require('telescope.builtin').help_tags() <CR>", options)

--       --> show all availabe MAPPING
keymap('n', "tm", "<cmd>lua require('telescope.builtin').keymaps() <CR>", options)

--       --> show buffers/opened files
keymap('n', "<C-b>", "<cmd>lua require('telescope.builtin').buffers() <CR>", options)
keymap('n', "<tb>",  "<cmd>lua require('telescope.builtin').buffers() <CR>", options)

--       --> show and grep current buffer
keymap('n', "tw", "<cmd>lua require('telescope.builtin').current_buffer_fuzzy_find() <CR>", options)
keymap('n', "tg", "<cmd>lua require('telescope.builtin').live_grep() <CR>", options)

--       --> Find Files
-- NOTE1: to get project root's directory, extra plugin (github.com/ygm2/rooter.nvim) is used.
-- any config related to project root is in seperate config file (lua/plugin_confs/rooter_nvim.lua)
-- to change settings related to working directory, refer to rooter_nvim.lua config file

-- Find files from current file's project
keymap('n', "<C-p>", ":Telescope find_files <cr>", options)
keymap('n', "tp",    ":Telescope find_files <cr>", options)
-- show all files from current working directory
keymap('n', "<C-f>", "<cmd>lua require('telescope.builtin').find_files( { cwd = vim.fn.expand('%:p:h') }) <CR>", options)
keymap('n', "tf",    "<cmd>lua require('telescope.builtin').find_files( { cwd = vim.fn.expand('%:p:h') }) <CR>", options)

-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━❰ end Mappings ❱━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

