--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: Terminal - snacks.nvim
Source: https://github.com/folke/snacks.nvim/blob/main/docs/terminal.md

Create and toggle terminal windows.
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

---@class snacks.terminal.Config
---@field win? snacks.win.Config|{}
---@field shell? string|string[] The shell to use. Defaults to `vim.o.shell`
---@field override? fun(cmd?: string|string[], opts?: snacks.terminal.Opts) Use this to use a different terminal implementation
local config = {
	win = { style = "terminal" },
	start_insert = true, -- start insert mode when starting the terminal
	auto_insert = false, -- start insert mode when entering the terminal buffer
	auto_close = true, -- close the terminal buffer when the process exits
	-- interactive = true, -- shortcut for `start_insert`, `auto_close` and `auto_insert` (default: true)
}

return config
