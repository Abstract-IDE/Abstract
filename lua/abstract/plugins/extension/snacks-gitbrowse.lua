--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: GitBrowse - snacks.nvim
Source: https://github.com/folke/snacks.nvim/blob/main/docs/gitbrowse.md

Open the repo of the active file in the browser (e.g., GitHub)
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]
local M = {}

---@return snacks.gitbrowse.Config
function M.config(keymap)
	keymap("folke/snacks.nvim/gitbrowse")

	---@type snacks.gitbrowse.Config
	return {}
end

return M
