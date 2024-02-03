--[[━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin:    goto-preview
Github:    https://github.com/rmagatti/goto-preview

A small Neovim plugin for previewing native LSP's goto definition, type definition,
implementation, declaration and references calls in floating windows.
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━]]

local spec = {
	"rmagatti/goto-preview",
	lazy = true,
	event = { "User AbstractLSPLoaded" },
	dependencies = {
		"nvim-telescope/telescope.nvim",
	},
}

spec.config = function()
	require("goto-preview").setup({
		width = 80, -- Width of the floating window
		height = 15, -- Height of the floating window
		border = { "↖", "─", "┐", "│", "┘", "─", "└", "│" }, -- Border characters of the floating window
		default_mappings = false, -- Bind default mappings
		debug = false, -- Print debug information
		opacity = nil, -- 0-100 opacity level of the floating window where 100 is fully transparent.
		resizing_mappings = false, -- Binds arrow keys to resizing the floating window.
		-- A function taking two arguments, a buffer and a window to be ran as a hook.
		post_open_hook = function()
			-- add preview window to buffer list
			local buffer_num = vim.api.nvim_get_current_buf() -- current buffer
			vim.api.nvim_buf_set_option(buffer_num, "buflisted")
		end,
		references = { -- Configure the telescope UI for showing the references cycling window.
			telescope = require("telescope.themes").get_dropdown({
				winblend = 15,
				layout_config = { prompt_position = "top", width = 64, height = 15 },
				border = {},
				previewer = false,
				shorten_path = false,
			}),
		},
		-- These two configs can also be passed down to the goto-preview definition and implementation calls for one off "peak" functionality.
		focus_on_open = true, -- Focus the floating window when opening it.
		dismiss_on_move = false, -- Dismiss the floating window when moving the cursor.
		force_close = true, -- passed into vim.api.nvim_win_close's second argument. See :h nvim_win_close
		bufhidden = "wipe", -- the bufhidden option to set on the floating window. See :h bufhidden
	})

	local options = { noremap = true }
	vim.api.nvim_set_keymap("n", "gpd", "<cmd>lua require('goto-preview').goto_preview_definition()<CR>", options)
	vim.api.nvim_set_keymap("n", "gpi", "<cmd>lua require('goto-preview').goto_preview_implementation()<CR>", options)
	-- Only set if telescope is installed
	vim.api.nvim_set_keymap("n", "gpr", "<cmd>lua require('goto-preview').goto_preview_references()<CR>", options)

	vim.api.nvim_set_keymap("n", "gP", "<cmd>lua require('goto-preview').close_all_win()<CR>", options)
end

return spec
