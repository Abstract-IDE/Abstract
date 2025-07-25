--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: Bufdelete - snacks.nvim
Source: https://github.com/folke/snacks.nvim/blob/main/docs/bufdelete.md

Delete buffers without disrupting window layout.
If the buffer you want to close has changes, a prompt will be shown to save or discard.
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

---@class snacks.bufdelete.Opts
local config = {
	enabled = true,
}

return config
