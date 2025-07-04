--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: tiny-code-action.nvim
Source: https://github.com/rachartier/tiny-code-action.nvim

A Neovim plugin that provides a simple way to
run and visualize code actions with Telescope.
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
	"rachartier/tiny-code-action.nvim",
	lazy = true,
	event = { "LspAttach" },
}

spec.opts = {}

return spec
