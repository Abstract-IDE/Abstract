--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: neotab.nvim
Github: https://github.com/kawre/neotab.nvim/

Simple yet convenient Neovim plugin for tabbing in and out of brackets, parentheses, quotes, and more.
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]




local _neotab, neotab = pcall(require, "neotab")
if not _neotab then return end

neotab.setup({
	tabkey = "<Tab>",
	act_as_tab = true, -- fallback to tab, if `tabout` action is not available
	behavior = "nested", ---@type ntab.behavior
	pairs = { ---@type ntab.pair[]
		{ open = "(", close = ")" },
		{ open = "[", close = "]" },
		{ open = "{", close = "}" },
		{ open = "'", close = "'" },
		{ open = '"', close = '"' },
		{ open = "`", close = "`" },
		{ open = "<", close = ">" },
	},
	exclude = {},
	smart_punctuators = {
		enabled = false,
		semicolon = {
			enabled = false,
			ft = { "cs", "c", "cpp", "java" },
		},
		escape = {
			enabled = false,
			triggers = {}, ---@type table<string, ntab.trigger>
		},
	},
})
