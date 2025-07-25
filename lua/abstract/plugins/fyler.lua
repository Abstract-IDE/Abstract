--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: fyler.nvim
Source: https://github.com/A7Lavinraj/fyler.nvim

A neovim file manager which can edit file system like a buffer with tree view
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
	"A7Lavinraj/fyler.nvim",
	dependencies = { "echasnovski/mini.icons" },
	-- branch = "stable",
	keys = require("abstract.configs.mapping").plugin["A7Lavinraj/fyler.nvim"],
}

spec.opts = {

	-- NETRW Hijacking:
	-- The plugin will replace most of the netrw command
	-- By default this option is disable to avoid any incompatibility
	default_explorer = false,

	-- Close open file:
	-- This enable user to close fyler window on opening a file
	close_on_select = true,

	-- Defines icon provider used by fyler, integrated ones are below:
	-- - "mini-icons"
	-- - "nvim-web-devicons"
	-- It also accepts `fun(type, name): icon: string, highlight: string`
	icon_provider = "nvim-web-devicons",

	-- Controls whether to show git status or not
	git_status = true,

	-- Controls behaviour of indentation marker
	indentscope = {
		enabled = true,
		group = "FylerDarkGrey",
		marker = "│",
	},

	-- Views configuration:
	-- Every view config contains following options to be customized
	-- `width` a number in [0, 1]
	-- `height` a number in [0, 1]

	-- `kind` could be as following:
	-- 'float',
	-- 'split:left',
	-- 'split:above',
	-- 'split:right',
	-- 'split:below'
	-- 'split:leftmost',
	-- 'split:abovemost',
	-- 'split:rightmost',
	-- 'split:belowmost'

	-- `border` could be as following:
	-- 'bold',
	-- 'double',
	-- 'none',
	-- 'rounded',
	-- 'shadow',
	-- 'single',
	-- 'solid'
	views = {
		explorer = {
			width = 0.8,
			height = 0.8,
			kind = "float",
			border = "single",
		},
		confirm = {
			width = 0.5,
			height = 0.4,
			kind = "float",
			border = "single",
		},
	},

	-- Mappings:
	-- mappings can be customized by action names which are local to thier view
	mappings = {
		-- For `explorer` actions checkout following link:
		-- https://github.com/A7Lavinraj/fyler.nvim/blob/main/lua/fyler/views/explorer/actions.lua
		explorer = {
			n = {
				["q"] = "CloseView",
				["<CR>"] = "Select",
			},
		},
	},
}

return spec
