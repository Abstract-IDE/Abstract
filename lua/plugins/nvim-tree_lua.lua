
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ───────────────────────────────────────────────── --
--   Plugin:    nvim-tree.lua
--   Github:    github.com/kyazdani42/nvim-tree.lua
-- ───────────────────────────────────────────────── --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --





-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━❰ configs ❱━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

local keymap = vim.api.nvim_set_keymap
local options = {noremap = true, silent = true}
local tree_cb = require'nvim-tree.config'.nvim_tree_callback
local cmd = vim.cmd -- execute Vim commands
cmd('autocmd ColorScheme * highlight highlight NvimTreeBg guibg=None')
cmd('autocmd FileType NvimTree setlocal winhighlight=Normal:NvimTreeBg')


-- each of these are documented in `:help nvim-tree.OPTION_NAME`
require'nvim-tree'.setup {
	disable_netrw = false,
	hijack_netrw = false,
	open_on_setup = false,
	ignore_ft_on_setup = {},
	-- auto_close = false,
	open_on_tab = false,
	hijack_cursor = false,
	update_cwd = false,
	hijack_directories = {enable = true, auto_open = true},
	diagnostics = {
		enable = false,
		icons = {hint = "", info = "", warning = "", error = ""},
	},
	git = {enable = false},
	update_focused_file = {enable = true, update_cwd = false, ignore_list = {}},
	system_open = {cmd = nil, args = {}},
	filters = {dotfiles = false, custom = {}},

	renderer = {
		indent_markers = {
			enable = true, -- show indent markers when folders are open
		},
		-- highlight_opened_files = "all", -- Highlight icons and/or names for opened files. Value can be `"none"`, `"icon"`, `"name"` or `"all"`.
	},

	view = {
		width = 25,
		height = 30,
		hide_root_folder = false,
		side = 'left',
		-- auto_resize = true,

-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━❰ end configs ❱━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --




-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━❰ Mappings ❱━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

		mappings = {
			custom_only = false,
			list = {
				{key = "g?", cb = tree_cb("toggle_help")},
				-- { key = "<C-v>",  cb = tree_cb("vsplit") },
				-- { key = "<C-x>",  cb = tree_cb("split") },
				-- { key = "<C-t>",  cb = tree_cb("tabnew") },
				-- { key = "<",      cb = tree_cb("prev_sibling") },
				-- { key = ">",      cb = tree_cb("next_sibling") },
				-- { key = "P",      cb = tree_cb("parent_node") },
				-- { key = "<BS>",   cb = tree_cb("close_node") },
				-- { key = "<S-CR>", cb = tree_cb("close_node") },
				-- { key = "<Tab>",  cb = tree_cb("preview") },
				-- { key = "K",      cb = tree_cb("first_sibling") },
				-- { key = "J",      cb = tree_cb("last_sibling") },
				-- { key = "I",      cb = tree_cb("toggle_ignored") },
				-- { key = "H",      cb = tree_cb("toggle_dotfiles") },
				-- { key = "R",      cb = tree_cb("refresh") },
				-- { key = "a",      cb = tree_cb("create") },
				-- { key = "d",      cb = tree_cb("remove") },
				-- { key = "r",      cb = tree_cb("rename") },
				-- { key = "<C-r>",  cb = tree_cb("full_rename") },
				-- { key = "x",      cb = tree_cb("cut") },
				-- { key = "c",      cb = tree_cb("copy") },
				-- { key = "p",      cb = tree_cb("paste") },
				-- { key = "y",      cb = tree_cb("copy_name") },
				-- { key = "Y",      cb = tree_cb("copy_path") },
				-- { key = "gy",     cb = tree_cb("copy_absolute_path") },
				-- { key = "[c",     cb = tree_cb("prev_git_item") },
				-- { key = "]c",     cb = tree_cb("next_git_item") },
				-- { key = "-",      cb = tree_cb("dir_up") },
				-- { key = "q",      cb = tree_cb("close") },
				-- { key = {"<CR>", "o", "<2-LeftMouse>"}, cb = tree_cb("edit") },
				-- { key = {"<2-RightMouse>", "<C-]>"},    cb = tree_cb("cd") },
			},
		},

	},
}

-- Toggle Nvim-Tree
keymap('n', '<leader>f', ':NvimTreeToggle<CR>', options)

-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━❰ end Mappings ❱━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

