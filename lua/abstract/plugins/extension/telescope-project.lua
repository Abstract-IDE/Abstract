--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: telescope.project.nvim
Github: https://github.com/nvim-telescope/telescope-project.nvim

An extension for telescope.nvim that allows you to switch between projects.

Default mappings (normal mode):
d 	    delete currently selected project
r 	    rename currently selected project
c 	    create a project*
s 	    search inside files within your project
b 	    browse inside files within your project
w 	    change to the selected project's directory without opening it
R 	    find a recently opened file within your project
f 	    find a file within your project (same as <CR>)
o 	    change current cd scope

Default mappings (insert mode):
<c-d> 	delete currently selected project
<c-v> 	rename currently selected project
<c-a> 	create a project*
<c-s> 	search inside files within your project
<c-b> 	browse inside files within your project
<c-l> 	change to the selected project's directory without opening it
<c-r> 	find a recently opened file within your project
<c-f> 	find a file within your project (same as <CR>)
<c-o> 	change current cd scope

defaults to your git root if used inside a git project, otherwise, it will use your current working directory
Workspace mappings (insert mode):
<c-w> 	change workspace
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local M = {}

M.config = {
	hidden_files = false, -- Show hidden files in selected project
	theme = "dropdown",
	order_by = "asc", -- Order projects by asc, desc, recent
	search_by = "title", -- Telescope finder search by field (title/path) string or table (default: title). Can also be a table {"title", "path"} to search by both title and path
	sync_with_nvim_tree = false,
	-- cd_scope = { "tab", "window" }, -- 	Array of cd scopes: tab, window, global table (default: {"tab", "window"})
	-- base_dirs = { -- Array of project base directory configurations
	-- 	"~/dev/src",
	-- 	{ "~/dev/src2" },
	-- 	{ "~/dev/src3",        max_depth = 4 },
	-- 	{ path = "~/dev/src4" },
	-- 	{ path = "~/dev/src5", max_depth = 2 },
	-- },
	-- default for on_project_selected = find project files
	-- on_project_selected = function(prompt_bufnr)
	-- 	-- Do anything you want in here. For example:
	-- 	local project_actions = require("telescope._extensions.project.actions")
	-- 	project_actions.change_working_directory(prompt_bufnr, false)
	-- 	require("harpoon.ui").nav_file(1)
	-- end,
}

return M
