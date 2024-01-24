
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ───────────────────────────────────────────────── --
--   Plugin:    LuaSnip
--   Github:    github.com/L3MON4D3/LuaSnip
-- ───────────────────────────────────────────────── --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --




-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━❰ configs ❱━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

local _luasnip, luasnip = pcall(require, "luasnip")
if not _luasnip then return end

local from_vscode = require("luasnip/loaders/from_vscode")

local filetype = vim.bo.filetype
local NVIM_DIR = vim.fn.stdpath('config')

luasnip.config.set_config({
	history = false, -- If true, Snippets that were exited can still be jumped back into.
	update_events = "TextChanged,TextChangedI", -- Update more often, :h events for more info.
	region_check_events = "CursorMoved", -- Ref: https://github.com/L3MON4D3/LuaSnip/issues/91
})

-- enable html snippets in javascript/javascript(REACT)
luasnip.filetype_extend("javascriptreact", {"html"})
luasnip.filetype_extend("typescriptreact", {"html"})

-- enable html snippets in Django (htmldjango)
luasnip.filetype_extend("htmldjango", {"html"})

-- this will lazy load all filetypes
from_vscode.lazy_load()

-- lazy loading so you only get in memory snippets of languages you use
from_vscode.lazy_load({
	paths = {  NVIM_DIR .. "/extra/snippets"},
})

-- ───────────────────────────────────────────────── --
-- NOTE: not working, need some fix
-- firendl-snippets

if filetype == "javascriptreact" then
	from_vscode.load({include = {"javascriptreact"}})
	return
end
if filetype == "typescriptreact" then
	from_vscode.load({include = {"typescriptreact"}})
	return
end
-- ───────────────────────────────────────────────── --


-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━❰ end configs ❱━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

