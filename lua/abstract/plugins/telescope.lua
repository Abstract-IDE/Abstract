--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: telescope.nvim
Github: https://github.com/nvim-telescope/telescope.nvim

telescope.nvim is a highly extendable fuzzy finder over lists.
Built on the latest awesome features from neovim core. Telescope
is centered around modularity, allowing for easy customization.
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
	"nvim-telescope/telescope.nvim",
	cmd = { "Telescope" },
	keys = { "t", "<C-f>", "<C-p>" },
	dependencies = {
		"nvim-lua/plenary.nvim",
		"nvim-telescope/telescope-file-browser.nvim",
		"nvim-telescope/telescope-fzf-native.nvim",
		"nvim-telescope/telescope-media-files.nvim",
		"nvim-telescope/telescope-project.nvim",
		"nvim-telescope/telescope-ui-select.nvim",
	},
}

spec.config = function()
	local telescope = require("telescope")
	telescope.setup({
		defaults = {
			-- stylua: ignore
			vimgrep_arguments = {
				"rg", "--color=never", "--no-heading", "--with-filename", "--line-number", "--column", "--smart-case",
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
				vertical = { mirror = false },
				width = 0.8,
				height = 0.9,
				preview_cutoff = 80,
			},
			-- stylua: ignore
			file_ignore_patterns = {
				"%.7z", "%.burp", "%.bz2", "%.cache", "%.class", "%.dll", "%.docx", "%.dylib", "%.epub", "%.exe",
				"%.flac", "%.ipynb", "%.jar", "%.lock", "%.met", "%.mkv", "%.mp4", "%.pdb", "%.pdf", "%.rar", "%.so",
				"%.sqlite3", "%.tar", "%.tar.gz", "%.zip",
				".dart_tool/", ".git/", ".github/", ".gradle/", ".idea/", ".vscode/",
				"__pycache__/", "__pycache__/*",
				"bin/Debug", "build/", "gradle/", "node_modules/", "node_modules/*",
				"obj/Debug", "smalljre_*/*", "target/", "vendor/*", "venv/",
			},

			file_sorter = require("telescope.sorters").get_fuzzy_file,
			generic_sorter = require("telescope.sorters").get_generic_fuzzy_sorter,
			file_previewer = require("telescope.previewers").vim_buffer_cat.new,
			grep_previewer = require("telescope.previewers").vim_buffer_vimgrep.new,
			qflist_previewer = require("telescope.previewers").vim_buffer_qflist.new,

			prompt_prefix = "🔎 ",
			selection_caret = "➤ ",
			entry_prefix = "  ",
			winblend = 0,
			border = {},
			borderchars = { "─", "│", "─", "│", "╭", "╮", "╯", "╰" }, -- ╭ ╮ ╰ ╯ ┌ ┐ └ ┘ │ ─
			color_devicons = true,
			use_less = true,
			set_env = { ["COLORTERM"] = "truecolor" }, -- default = nil,
			path_display = { "truncate" }, -- How file paths are displayed ()

			preview = {
				msg_bg_fillchar = " ",
				treesitter = false,
			},

			live_grep = {
				preview = {
					treesitter = false,
				},
			},

			-- Developer configurations: Not meant for general override
			buffer_previewer_maker = require("telescope.previewers").buffer_previewer_maker,
		},
		pickers = {
			buffers = {
				mappings = {
					i = {
						["<c-d>"] = "delete_buffer",
					},
					n = {
						dd = "delete_buffer",
					},
				},
			},
		},
		extensions = {
			["ui-select"] = require("abstract.plugins.telescope-ui-select").config,
			file_browser = require("abstract.plugins.telescope-file-browser").config,
			project = require("abstract.plugins.telescope-project").config,
			fzf = require("abstract.plugins.telescope-fzf-native").config,
			media_files = require("abstract.plugins.telescope-media-files").config,
		},
	})

	-- To get telescope-extension loaded and working with telescope,
	-- you need to call load_extension, somewhere after setup function:
	telescope.load_extension("fzf")
	telescope.load_extension("file_browser")
	telescope.load_extension("media_files")
	telescope.load_extension("ui-select")
	telescope.load_extension("project")

	-- Mappings
	local keymap = vim.api.nvim_set_keymap
	local opts = { silent = true, noremap = true }

	-- Launch Telescope without any argument
	keymap("n", "tt", "<cmd>lua require('telescope.builtin').builtin() <CR>", opts)
	-- Lists available Commands
	keymap("n", "tc", "<cmd>lua require('telescope.builtin').commands() <CR>", opts)
	-- Lists available help tags and opens a new window with the relevant help info on
	keymap("n", "th", "<cmd>lua require('telescope.builtin').help_tags() <CR>", opts)
	-- Show all availabe MAPPING
	keymap("n", "tm", "<cmd>lua require('telescope.builtin').keymaps() <CR>", opts)
	-- Show buffers/opened files
	keymap("n", "<C-b>", "<cmd>lua require('telescope.builtin').buffers() <CR>", opts)
	-- Show and grep current buffer
	keymap("n", "tw", "<cmd>lua require('telescope.builtin').current_buffer_fuzzy_find() <CR>", opts)
	keymap("n", "tg", "<cmd>lua require('telescope.builtin').live_grep() <CR>", opts)

	--       --> Find Files
	-- NOTE1: to get project root's directory, https://github.com/Abstract-IDE/penvim plugin is used.
	-- any config related to project root is in seperate config file (lua/plugin_confs/penvim.lua)
	-- to change settings related to working directory, refer to penvim.lua config file

	-- Find files from current file's project
	keymap("n", "<C-p>", ":Telescope find_files <cr>", opts)
	-- Show all files from current working directory
	keymap(
		"n",
		"<C-f>",
		"<cmd>lua require('telescope.builtin').find_files( { cwd = vim.fn.expand('%:p:h') }) <CR>",
		opts
	)
end

return spec
