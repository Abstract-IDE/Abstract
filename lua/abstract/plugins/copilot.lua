--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: copilot.lua
Source: https://github.com/zbirenbaum/copilot.lua

Fully featured & enhanced replacement for copilot.vim
complete with API for interacting with Github Copilot
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
	"zbirenbaum/copilot.lua",
	lazy = true,
	event = "InsertEnter",
}

spec.config = function()
	require("copilot").setup({
		panel = {
			enabled = true,
			auto_refresh = true,
			keymap = {
				jump_prev = "[[",
				jump_next = "]]",
				accept = "<CR>",
				refresh = "gr",
				open = "<M-CR>",
			},
			layout = {
				position = "bottom", -- | top | left | right
				ratio = 0.4,
			},
		},
		suggestion = {
			enabled = true,
			auto_trigger = true,
			debounce = 75,
			keymap = {
				accept_word = false,
				accept_line = false,
			},
		},
		filetypes = {
			yaml = false,
			markdown = false,
			help = false,
			["*"] = true,
		},
		copilot_node_command = "node", -- Node.js version must be > 18.x
		server_opts_overrides = {},
	})
	require("abstract.utils.map").set_map("zbirenbaum/copilot.lua", true)
end

return spec
