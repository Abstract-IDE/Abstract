--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: telescope-file-browser.nvim
Github: https://github.com/nvim-telescope/telescope-file-browser.nvim

File Browser extension for telescope.nvim
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local M = {}
local actions = require("telescope._extensions.file_browser.actions")

M.config = {
	theme = "ivy",
	hijack_netrw = true,
	path = vim.loop.cwd(),
	cwd = vim.loop.cwd(),
	cwd_to_path = false,
	grouped = false,
	files = true,
	add_dirs = true,
	depth = 1,
	auto_depth = false,
	select_buffer = false,
	hidden = { file_browser = false, folder_browser = false },
	respect_gitignore = vim.fn.executable("fd") == 1,
	no_ignore = false,
	follow_symlinks = false,
	browse_files = require("telescope._extensions.file_browser.finders").browse_files,
	browse_folders = require("telescope._extensions.file_browser.finders").browse_folders,
	hide_parent_dir = false,
	collapse_dirs = false,
	prompt_path = false,
	quiet = false,
	dir_icon = "",
	dir_icon_hl = "Default",
	display_stat = { date = true, size = true, mode = true },
	use_fd = true,
	git_status = true,
	mappings = {
		["i"] = {
			["<A-c>"] = actions.create,
			["<S-CR>"] = actions.create_from_prompt,
			["<A-r>"] = actions.rename,
			["<A-m>"] = actions.move,
			["<A-y>"] = actions.copy,
			["<A-d>"] = actions.remove,
			["<C-o>"] = actions.open,
			["<C-g>"] = actions.goto_parent_dir,
			["<C-e>"] = actions.goto_home_dir,
			["<C-w>"] = actions.goto_cwd,
			["<C-t>"] = actions.change_cwd,
			["<C-f>"] = actions.toggle_browser,
			["<C-h>"] = actions.toggle_hidden,
			["<C-s>"] = actions.toggle_all,
			["<bs>"] = actions.backspace,
		},
		["n"] = {
			["c"] = actions.create,
			["r"] = actions.rename,
			["m"] = actions.move,
			["y"] = actions.copy,
			["d"] = actions.remove,
			["o"] = actions.open,
			["g"] = actions.goto_parent_dir,
			["e"] = actions.goto_home_dir,
			["w"] = actions.goto_cwd,
			["t"] = actions.change_cwd,
			["f"] = actions.toggle_browser,
			["h"] = actions.toggle_hidden,
			["s"] = actions.toggle_all,
		},
	},
}

return M
