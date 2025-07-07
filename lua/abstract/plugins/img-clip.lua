--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: img-clip.nvim
Source: https://github.com/hakonharnes/img-clip.nvim

Embed images into any markup language, like LaTeX, Markdown or Typst
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
	"hakonharnes/img-clip.nvim",
	event = "VeryLazy",
	keys = require("abstract.configs.mapping").plugin["hakonharnes/img-clip.nvim"],
}

spec.opts = {}

return spec
