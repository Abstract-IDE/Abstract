--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: neotab.nvim
Github: https://github.com/L3MON4D3/LuaSnip

Snippet Engine for Neovim written in Lua.
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
	"L3MON4D3/LuaSnip",
	version = "v2.*", -- follow latest release.
	build = "make install_jsregexp", -- install jsregexp (optional!).
	lazy = true,
	dependencies = {
		{ "rafamadriz/friendly-snippets" }, -- Snippets collection for a set of different programming languages for faster development.
		{ "Neevash/awesome-flutter-snippets", ft = "dart" }, -- collection snippets and shortcuts for commonly used Flutter functions and classes
	},
}

spec.config = function()
	local luasnip = require("luasnip")

	luasnip.config.set_config({
		history = false, -- If true, Snippets that were exited can still be jumped back into.
		update_events = "TextChanged,TextChangedI", -- Update more often, :h events for more info.
		region_check_events = "CursorMoved", -- Ref: https://github.com/L3MON4D3/LuaSnip/issues/91
	})

	-- Add snippets from a framework to a filetype.
	luasnip.filetype_extend("javascriptreact", { "html" })
	luasnip.filetype_extend("typescriptreact", { "html" })
	luasnip.filetype_extend("htmldjango", { "html" })

	local from_vscode = require("luasnip.loaders.from_vscode")

	-- this will lazy load all filetypes
	from_vscode.lazy_load()

	-- lazy loading so you only get in memory snippets of languages you use
	-- local NVIM_DIR = vim.fn.stdpath("config")
	-- from_vscode.lazy_load({
	-- 	paths = { NVIM_DIR .. "/extra/snippets" },
	-- })
end

return spec
