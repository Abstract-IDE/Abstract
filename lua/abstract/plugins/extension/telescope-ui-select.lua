--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: telescope-ui-select.nvim
Github: https://github.com/nvim-telescope/telescope-ui-select.nvim

It sets vim.ui.select to telescope.
That means for example that neovim core stuff can fill the telescope picker.
Example would be lua vim.lsp.buf.code_action().
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local M = {}

M.config = {
	require("telescope.themes").get_dropdown({
		winblend = 15,
		layout_config = {
			prompt_position = "top",
			width = 64,
			height = 15,
		},
		border = {},
		previewer = false,
		shorten_path = false,
	}),
	-- pseudo code / specification for writing custom displays, like the one
	-- for "codeactions"
	-- specific_opts = {
	--   [kind] = {
	--     make_indexed = function(items) -> indexed_items, width,
	--     make_displayer = function(widths) -> displayer
	--     make_display = function(displayer) -> function(e)
	--     make_ordinal = function(e) -> string
	--   },
	--   -- for example to disable the custom builtin "codeactions" display
	--      do the following
	--   codeactions = false,
	-- }
}

return M
