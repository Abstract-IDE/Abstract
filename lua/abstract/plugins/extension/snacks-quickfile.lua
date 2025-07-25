--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: Quickfile - snacks.nvim
Source: https://github.com/folke/snacks.nvim/blob/main/docs/quickfile.md

When doing nvim somefile.txt,
it will render the file as quickly as possible, before loading your plugins.
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

---@class snacks.quickfile.Config
local config = {
	-- -- any treesitter langs to exclude
	-- exclude = { "latex" },
}

return config
