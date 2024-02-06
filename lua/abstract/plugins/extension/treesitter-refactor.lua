--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin:    nvim-treesitter-refactor
Github:    https://github.com/nvim-treesitter/nvim-treesitter-refactor

Refactor modules for nvim-treesitter
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local M = {}

M.refactor = {
	-- Highlight current scope
	-- Highlights the block from the current scope where the cursor is.
	highlight_definitions = {
		enable = true,
		-- Set to false if you have an `updatetime` of ~100.
		clear_on_cursor_move = true,
	},
	-- Smart rename
	-- Renames the symbol under the cursor within the current scope (and current file).
	smart_rename = {
		-- enable = true,
		-- -- Assign keymaps to false to disable them, e.g. `smart_rename = false`.
		-- keymaps = {
		-- 	smart_rename = "grr",
		-- },
	},
	-- Navigation
	-- Provides "go to definition" for the symbol under the cursor, and lists the definitions from the current file.
	-- If you use goto_definition_lsp_fallback instead of goto_definition in the config below vim.lsp.buf.definition
	-- is used if nvim-treesitter can not resolve the variable. goto_next_usage/goto_previous_usage go to
	-- the next usage of the identifier under the cursor.
	navigation = {
		-- enable = true,
		-- -- Assign keymaps to false to disable them, e.g. `goto_definition = false`.
		-- keymaps = {
		-- 	goto_definition = "gnd",
		-- 	list_definitions = "gnD",
		-- 	list_definitions_toc = "gO",
		-- 	goto_next_usage = "<a-*>",
		-- 	goto_previous_usage = "<a-#>",
		-- },
	},
}

return M
