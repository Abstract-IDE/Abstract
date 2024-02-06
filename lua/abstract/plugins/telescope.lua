--[[━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: telescope.nvim
Github: https://github.com/nvim-telescope/telescope.nvim

telescope.nvim is a highly extendable fuzzy finder over lists.
Built on the latest awesome features from neovim core. Telescope
is centered around modularity, allowing for easy customization.
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━]]

local spec = {
	"nvim-telescope/telescope.nvim",
	lazy = true,
	cmd = { "Telescope" },
	keys = { "t", "<C-f>", "<C-p>" },
	dependencies = {
		"nvim-lua/plenary.nvim",
		"nvim-telescope/telescope-file-browser.nvim",
		"nvim-telescope/telescope-media-files.nvim",
		"nvim-telescope/telescope-project.nvim",
		"nvim-telescope/telescope-ui-select.nvim",
		{ "nvim-telescope/telescope-fzf-native.nvim", build = "make" },
	},
}

spec.config = function()
	require("abstract.utils.map").set_plugin("nvim-telescope/telescope.nvim")

	local telescope = require("telescope")
	local actions = require("telescope.actions")
	local action_layout = require("telescope.actions.layout")

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
				filesize_limit = 10, -- MB
			},

			live_grep = {
				preview = {
					treesitter = false,
				},
			},
			mappings = {
				n = {
					["<M-p>"] = action_layout.toggle_preview,
				},
				i = {
					["<M-p>"] = action_layout.toggle_preview,
					["<C-s>"] = actions.cycle_previewers_next,
					["<C-a>"] = actions.cycle_previewers_prev,
				},
			},
			-- Developer configurations: Not meant for general override
			buffer_previewer_maker = require("telescope.previewers").buffer_previewer_maker,
		},
		pickers = {
			buffers = {
				sort_lastused = true,
				previewer = true,
				mappings = {
					i = {
						["<c-d>"] = "delete_buffer",
						-- ["<c-d>"] = actions.delete_buffer + actions.move_to_top,
					},
					n = {
						["dd"] = "delete_buffer",
					},
				},
			},
			find_files = {
				-- find_command = { "fd", "--type", "f", "--strip-cwd-prefix" },
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
end

return spec
