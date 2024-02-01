--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin:    hop.nvim
Github:    https://github.com/smoka7/hop.nvim
           (forked of: https://github.com/phaazon/hop.nvim)

Hop is an EasyMotion-like plugin allowing you to jump
anywhere in a document with as few keystrokes as possible
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
	"smoka7/hop.nvim",
	version = "*",
	opts = {},
}

spec.config = function()
	local hop = require("hop")
	hop.setup({
		keys = "qwertyuiopasdfghjklzxcvbnm",
		jump_on_sole_occurrence = false,
	})

	local keymap = vim.api.nvim_set_keymap
	local options = { noremap = true, silent = true }

	keymap("n", "f", "<cmd>lua require'hop'.hint_words()<cr>", options)

	-- local directions = require("hop.hint").HintDirection
	-- vim.keymap.set("", "f", function()
	-- 	hop.hint_char1({ direction = directions.AFTER_CURSOR, current_line_only = true })
	-- end, { remap = true })
	-- vim.keymap.set("", "F", function()
	-- 	hop.hint_char1({ direction = directions.BEFORE_CURSOR, current_line_only = true })
	-- end, { remap = true })
	-- vim.keymap.set("", "t", function()
	-- 	hop.hint_char1({ direction = directions.AFTER_CURSOR, current_line_only = true, hint_offset = -1 })
	-- end, { remap = true })
	-- vim.keymap.set("", "T", function()
	-- 	hop.hint_char1({ direction = directions.BEFORE_CURSOR, current_line_only = true, hint_offset = 1 })
	-- end, { remap = true })
end

return spec
