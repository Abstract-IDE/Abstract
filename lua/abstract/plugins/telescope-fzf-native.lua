--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: telescope-fzf-native.nvim
Github: https://github.com/nvim-telescope/telescope-fzf-native.nvim

FZF sorter for telescope written in c
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local M = {}

M.config = {
	fuzzy = true, -- false will only do exact matching
	override_generic_sorter = true, -- override the generic sorter
	override_file_sorter = true, -- override the file sorter
	case_mode = "smart_case", -- or "ignore_case" or "respect_case". the default case_mode is "smart_case"
}

return M
