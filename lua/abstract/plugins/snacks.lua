--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: snacks.nvim
Source: github.com/folke/snacks.nvim

A collection of small QoL plugins for Neovim.
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
	"folke/snacks.nvim",
	priority = 1000,
	lazy = false,
	keys = require("abstract.configs.mapping").plugin["folke/snacks.nvim"],
}

---@type snacks.Config
spec.opts = {

	-- Delete buffers without disrupting window layout.
	---@class snacks.bufdelete.Opts
	bufdelete = {
		enabled = true,
	},

	-- Better vim.ui.input
	input = {
		enabled = true,
	},
	bigfile = require("abstract.plugins.extension.snacks-bigfile"),
	dashboard = require("abstract.plugins.extension.snacks-dashboard"),
	-- explorer = require("abstract.plugins.extension.snacks-picker"),
	indent = require("abstract.plugins.extension.snacks-indent"),
	lazygit = require("abstract.plugins.extension.snacks-lazygit"),
	notifier = require("abstract.plugins.extension.snacks-notifier"),
	-- NOTE: using voldikss/vim-floaterm for terminal
	-- terminal = require("abstract.plugins.extension.snacks-terminal"),
}

return spec
