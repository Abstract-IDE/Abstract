--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: grug-far.nvim
Source: https://github.com/MagicDuck/grug-far.nvim

Find And Replace plugin for neovim
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
	"MagicDuck/grug-far.nvim",
	keys = require("abstract.configs.mapping").plugin["MagicDuck/grug-far.nvim"],
}
spec.opts = {}

return spec
