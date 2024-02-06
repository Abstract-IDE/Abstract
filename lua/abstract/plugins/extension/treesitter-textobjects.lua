--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin:    nvim-treesitter-textobjects
Github:    https://github.com/nvim-treesitter/nvim-treesitter-textobjects

Syntax aware text-objects, select, move, swap, and peek support.
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local M = {}

M.textobjects = {
	-- Text objects: select
	-- similar to ip (inner paragraph) and ap (a paragraph).
	select = {
		enable = true,

		-- Automatically jump forward to textobj, similar to targets.vim
		lookahead = true,

		keymaps = {
			-- You can use the capture groups defined in textobjects.scm
			["if"] = { query = "@function.inner", desc = "inner part of a function region" },
			["af"] = { query = "@function.outer", desc = "outer part of a function region" },
			["ic"] = { query = "@class.inner", desc = "inner part of a class region" },
			["ac"] = { query = "@class.outer", desc = "outer part of a class region" },
			["as"] = { query = "@scope", query_group = "locals", desc = "language scope" },
		},
		-- You can choose the select mode (default is charwise 'v')
		--
		-- Can also be a function which gets passed a table with the keys
		-- * query_string: eg '@function.inner'
		-- * method: eg 'v' or 'o'
		-- and should return the mode ('v', 'V', or '<c-v>') or a table
		-- mapping query_strings to modes.
		selection_modes = {
			["@parameter.outer"] = "v", -- charwise
			["@function.outer"] = "V", -- linewise
			["@class.outer"] = "<c-v>", -- blockwise
		},
		-- If you set this to `true` (default is `false`) then any textobject is
		-- extended to include preceding or succeeding whitespace. Succeeding
		-- whitespace has priority in order to act similarly to eg the built-in
		-- `ap`.
		--
		-- Can also be a function which gets passed a table with the keys
		-- * query_string: eg '@function.inner'
		-- * selection_mode: eg 'v'
		-- and should return true of false
		include_surrounding_whitespace = true,
	},
	-- Text objects: swap
	-- to swap the node under the cursor with the next or previous one, like function parameters or arguments.
	swap = {
		-- enable = true,
		-- swap_next = {
		-- 	["<leader>a"] = "@parameter.inner",
		-- },
		-- swap_previous = {
		-- 	["<leader>A"] = "@parameter.inner",
		-- },
	},
	-- Text objects: move
	-- to jump to the next or previous text object. This is similar to ]m, [m, ]M, [M Neovim's mappings to jump to the next or previous function.
	move = {
		-- enable = true,
		-- set_jumps = true, -- whether to set jumps in the jumplist
		-- goto_next_start = {
		-- 	["]m"] = "@function.outer",
		-- 	["]]"] = { query = "@class.outer", desc = "Next class start" },
		-- 	--
		-- 	-- You can use regex matching (i.e. lua pattern) and/or pass a list in a "query" key to group multiple queires.
		-- 	["]o"] = "@loop.*",
		-- 	-- ["]o"] = { query = { "@loop.inner", "@loop.outer" } }
		-- 	--
		-- 	-- You can pass a query group to use query from `queries/<lang>/<query_group>.scm file in your runtime path.
		-- 	-- Below example nvim-treesitter's `locals.scm` and `folds.scm`. They also provide highlights.scm and indent.scm.
		-- 	["]s"] = { query = "@scope", query_group = "locals", desc = "Next scope" },
		-- 	["]z"] = { query = "@fold", query_group = "folds", desc = "Next fold" },
		-- },
		-- goto_next_end = {
		-- 	["]M"] = "@function.outer",
		-- 	["]["] = "@class.outer",
		-- },
		-- goto_previous_start = {
		-- 	["[m"] = "@function.outer",
		-- 	["[["] = "@class.outer",
		-- },
		-- goto_previous_end = {
		-- 	["[M"] = "@function.outer",
		-- 	["[]"] = "@class.outer",
		-- },
		-- -- Below will go to either the start or the end, whichever is closer.
		-- -- Use if you want more granular movements
		-- -- Make it even more gradual by adding multiple queries and regex.
		-- goto_next = {
		-- 	["]d"] = "@conditional.outer",
		-- },
		-- goto_previous = {
		-- 	["[d"] = "@conditional.outer",
		-- },
	},
	-- Textobjects: LSP interop
	-- peek_definition_code: show textobject surrounding definition as determined using Neovim's
	-- built-in LSP -- in a floating window. Press the shortcut twice to enter the floating window.
	lsp_interop = {
		enable = true,
		border = "rounded",
		floating_preview_opts = {},
		-- peek_definition_code = {
		-- 	["<leader>df"] = "@function.outer",
		-- 	["<leader>dF"] = "@class.outer",
		-- },
	},
}

return M
