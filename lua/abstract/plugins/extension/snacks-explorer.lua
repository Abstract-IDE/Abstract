--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: Explorer - snacks.nvim
Source: https://github.com/folke/snacks.nvim/blob/main/docs/explorer.md

A file explorer for snacks.
This is actually a picker in disguise. This module provide a shortcut to open
the explorer picker and a setup function to replace netrw with the explorer.
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

---@class snacks.explorer.Config
local config = {
	enabled = true,
	replace_netrw = true, -- Replace netrw with the snacks explorer
}

return config
