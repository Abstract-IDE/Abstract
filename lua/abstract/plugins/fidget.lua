--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: fidget.nvim
Source: https://github.com/j-hui/fidget.nvim

💫 Extensible UI for Neovim notifications and LSP progress messages.
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
	"j-hui/fidget.nvim",
	lazy = true,
	event = { "LspAttach" },
	opts = {}
}

spec.opts = {
	notification = {
		window = {
			winblend = 100, -- Background color opacity in the notification window
		}
	}
}


return spec
