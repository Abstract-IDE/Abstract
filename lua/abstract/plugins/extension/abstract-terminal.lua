--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: Terminal - abstract-plugs.nvim
Source: https://github.com/Abstract-IDE/abstract-plugs.nvim

Simple terminal for neovim
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local M = {}

M.setup = function(plug, set_map)
	plug.terminal().setup({
		height = 0.4, -- value range between: 0.0 to 1.0
		width = 0.6, -- value range between: 0.0 to 1.0
		offset_row = 0.9, -- vertical: value range between: 0.0 to 1.0
		offset_col = 0.5, -- horizontal: value range between: 0.0 to 1.0
		border = "rounded",
	})
	set_map("Abstract-IDE/abstract-plugs.nvim/terminal")
end

return M
