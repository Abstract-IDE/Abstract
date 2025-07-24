--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: abstract-terminal.nvim
Github: https://github.com/Abstract-IDE/abstract-terminal.nvim

Simple terminal for neovim
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
	"Abstract-IDE/abstract-terminal.nvim",
	-- keys = require("abstract.configs.mapping").plugin["Abstract-IDE/abstract-terminal.nvim"],
}

spec.opts = {
	height = 0.4,  -- value range between: 0.0 to 1.0
	width = 0.6,   -- value range between: 0.0 to 1.0
	offset_row = 0.9, -- vertical: value range between: 0.0 to 1.0
	offset_col = 0.5, -- horizontal: value range between: 0.0 to 1.0
	border = "rounded",
	keymap = { toggle = "<C-t>" },
}


return spec
