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
	local keymap = require("abstract.utils.map").set_map
	require("abstract.utils.map").set_map("folke/snacks.nvim")

	require("snacks").setup({

		-- Plugins
		bigfile = require("abstract.plugins.extension.snacks-bigfile"),
		bufdelete = require("abstract.plugins.extension.snacks-bufdelete").config(keymap),
		dashboard = require("abstract.plugins.extension.snacks-dashboard"),
		gitbrowse = require("abstract.plugins.extension.snacks-gitbrowse").config(keymap),
		indent = require("abstract.plugins.extension.snacks-indent"),
		input = require("abstract.plugins.extension.snacks-input"),
		lazygit = require("abstract.plugins.extension.snacks-lazygit").config(keymap),
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
