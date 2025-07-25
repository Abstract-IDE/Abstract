--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: Indent - snacks.nvim
Source: https://github.com/folke/snacks.nvim/blob/main/docs/indent.md

Visualize indent guides and scopes based on treesitter or indent.
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

---@class snacks.indent.Config
local config = {
	enabled = true,
	indent = {
		enabled = true, -- enable indent guides
		priority = 1,
		char = "│",
		only_scope = false, -- only show indent guides of the scope
		only_current = false, -- only show indent guides in the current window
		hl = "SnacksIndent", ---@type string|string[] hl groups for indent guides
	},
	---@class snacks.indent.Scope.Config: snacks.scope.Config
	scope = {
		enabled = true, -- enable highlighting the current scope
		priority = 200,
		char = "│", -- │ | ¦ ┆ ┊ ║ ▎
		underline = true, -- underline the start of the scope
		only_current = false, -- only show scope in the current window
		hl = "SnacksIndentScope", ---@type string|string[] hl group for scopes
	},
	chunk = {
		-- when enabled, scopes will be rendered as chunks, except for the
		-- top-level scope which will be rendered as a scope.
		enabled = false,
		-- only show chunk scopes in the current window
		only_current = false,
		priority = 100,
		hl = "SnacksIndentChunk", ---@type string|string[] hl group for chunk scopes
		char = {
			corner_top = "╭", --    ╭   ┌
			corner_bottom = "╰", -- ╰   └
			horizontal = "─",
			vertical = "│",
			arrow = ">",
		},
	},

	---@class snacks.indent.animate: snacks.animate.Config
	---@field style? "out"|"up_down"|"down"|"up"
	animate = {
		enabled = true, -- vim.fn.has("nvim-0.10") == 1,
		--- * out: animate outwards from the cursor
		--- * up: animate upwards from the cursor
		--- * down: animate downwards from the cursor
		--- * up_down: animate up or down based on the cursor position
		style = "out",
		easing = "linear",
		duration = {
			step = 20, -- ms per step
			total = 300, -- maximum duration
		},
	},
}

return config
