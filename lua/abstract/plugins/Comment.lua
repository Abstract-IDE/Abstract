--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: Comment.nvim
Github: https://github.com/numToStr/Comment.nvim

🧠 💪 // Smart and powerful comment plugin for neovim.
Supports treesitter, dot repeat, left-right/up-down motions, hooks, and more
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
	"numToStr/Comment.nvim",
	dependencies = { "JoosepAlviste/nvim-ts-context-commentstring" },
	keys = { "cc", "gc", "gb", { "cc", mode = "v" }, { "gc", mode = "v" }, { "gb", mode = "v" } },
}

spec.config = function()
	require("Comment").setup({
		---Add a space b/w comment and the line
		---@type boolean
		padding = true,

		---Whether the cursor should stay at its position
		---NOTE: This only affects NORMAL mode mappings and doesn't work with dot-repeat
		---@type boolean
		sticky = true,

		---Lines to be ignored while comment/uncomment.
		---Could be a regex string or a function that returns a regex string.
		---Example: Use '^$' to ignore empty lines
		---@type string|function
		ignore = "^$",

		---Pre-hook, called before commenting the line
		---@type function|nil
		-- NOTE: implemented with JoosepAlviste/nvim-ts-context-commentstring
		pre_hook = require("abstract.plugins.ts-context-commentstring").config(),

		---Post-hook, called after commenting is done
		---@type function|nil
		post_hook = nil,

		mappings = {
			basic = true, ---Includes `gcc`, `gcb`, `gc[count]{motion}` and `gb[count]{motion}`
			extra = true, ---Includes `gco`, `gcO`, `gcA`
			extended = false, ---Includes `g>`, `g<`, `g>[count]{motion}` and `g<[count]{motion}`
		},

		---LHS of toggle mapping in NORMAL
		---@type table
		toggler = {
			line = "cc", ---line-comment keymap
			block = "cb", ---block-comment keymap
		},

		---LHS of operator-pending mapping in VISUAL mode
		---@type table
		opleader = {
			line = "gc", ---line-comment keymap
			block = "gb", ---block-comment keymap
		},
	})
end

return spec
