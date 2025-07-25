--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: Window - abstract-plugs.nvim
Source: https://github.com/Abstract-IDE/abstract-plugs.nvim

Neovim window management plugin that provides
a command framework for window-related utilities.
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local M = {}

M.setup = function(plug, set_map)
	set_map("Abstract-IDE/abstract-plugs.nvim/window")
	plug.window().setup()
end

return M
