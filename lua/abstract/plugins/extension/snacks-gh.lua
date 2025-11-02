--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: gh - snacks.nvim
Source: https://github.com/folke/snacks.nvim/blob/main/docs/gh.md

A modern GitHub CLI integration for Neovim that brings
GitHub issues and pull requests directly into your editor.
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local M = {}

function M.config(keymap)
	keymap("folke/snacks.nvim/gh")

	---@class snacks.gh.Config
	return {
		enabled = true,
	}
end

return M
