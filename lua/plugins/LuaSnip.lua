
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ───────────────────────────────────────────────── --
--   Plugin:    LuaSnip
--   Github:    github.com/L3MON4D3/LuaSnip
-- ───────────────────────────────────────────────── --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --





-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━❰ configs ❱━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

local ls = require("luasnip")

ls.config.set_config({
	history = false, -- If true, Snippets that were exited can still be jumped back into.
	update_events = "TextChanged,TextChangedI", -- Update more often, :h events for more info.
	region_check_events = "CursorMoved", -- Ref: https://github.com/L3MON4D3/LuaSnip/issues/91
})


-- enable html snippets in javascript/javascript(REACT)
ls.filetype_extend("javascriptreact", {"html"})
ls.filetype_extend("typescriptreact", {"html"})


-- this will lazy load all filetypes
require("luasnip/loaders/from_vscode").lazy_load()

-- lazy loading so you only get in memory snippets of languages you use
require'luasnip/loaders/from_vscode'.lazy_load({
	paths = {"~/.config/nvim/extra/snippets"},
})

-- ───────────────────────────────────────────────── --
-- NOTE: not working, need some fix
-- firendl-snippets
local filetype = vim.bo.filetype

if filetype == "javascriptreact" then
	require("luasnip/loaders/from_vscode").load({include = {"javascriptreact"}})
	return
end
if filetype == "typescriptreact" then
	require("luasnip/loaders/from_vscode").load({include = {"typescriptreact"}})
	return
end
-- ───────────────────────────────────────────────── --


-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━❰ end configs ❱━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

