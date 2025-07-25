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
}

spec.config = function()
	require("abstract.utils.map").set_map("folke/snacks.nvim")

	require("snacks").setup({
		-- Delete buffers without disrupting window layout.
		---@class snacks.bufdelete.Opts
		bufdelete = {
			enabled = true,
		},

		-- Better vim.ui.input
		input = {
			enabled = true,
		},

		-- Plugins
		bigfile = require("abstract.plugins.extension.snacks-bigfile"),
		dashboard = require("abstract.plugins.extension.snacks-dashboard"),
		indent = require("abstract.plugins.extension.snacks-indent"),
		lazygit = require("abstract.plugins.extension.snacks-lazygit"),
		notifier = require("abstract.plugins.extension.snacks-notifier"),
		quickfile = require("abstract.plugins.extension.snacks-quickfile"),

		-- Maybe use in future
		--------------------------
		-- picker = require("abstract.plugins.extension.snacks-picker"),
		-- explorer = require("abstract.plugins.extension.snacks-explorer"),
		-- NOTE: using voldikss/vim-floaterm for terminal
		-- terminal = require("abstract.plugins.extension.snacks-terminal"),
	})
end

return spec
