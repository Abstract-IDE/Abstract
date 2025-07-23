--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: abstract-window.nvim
Github: https://github.com/Abstract-IDE/abstract-window.nvim

Neovim window management plugin that provides a command
framework for window-related utilities.
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
	"Abstract-IDE/abstract-window.nvim",
	keys = require("abstract.configs.mapping").plugin["Abstract-IDE/abstract-window.nvim"],
}

spec.opts = {}

return spec
