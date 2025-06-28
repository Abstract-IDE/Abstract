--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: lazydev.nvim
Source: https://github.com/folke/lazydev.nvim

lazydev.nvim is a plugin that properly configures LuaLS for editing
your Neovim config by lazily updating your workspace libraries.
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
	"folke/lazydev.nvim",
	ft = "lua",
}

spec.opts = {
	runtime = vim.env.VIMRUNTIME,
	library = {},
	integrations = {
		-- Fixes lspconfig's workspace management for LuaLS
		-- Only create a new workspace if the buffer is not part
		-- of an existing workspace or one of its libraries
		lspconfig = true,
		-- add the cmp source for completion of:
		-- `require "modname"`
		-- `---@module "modname"`
		cmp = true,
		-- same, but for Coq
		coq = false,
	},
	enabled = function(root_dir)
		return vim.g.lazydev_enabled == nil and true or vim.g.lazydev_enabled
	end,
}

return spec
