
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
-- some shorthands...
-- local s = ls.snippet
-- local sn = ls.snippet_node
-- local t = ls.text_node
-- local i = ls.insert_node
-- local f = ls.function_node
-- local c = ls.choice_node
-- local d = ls.dynamic_node

-- Every unspecified option will be set to the default.
ls.config.set_config({
	history = false,
	-- Update more often, :h events for more info.
	updateevents = "TextChanged,TextChangedI",
	-- ref: https://github.com/L3MON4D3/LuaSnip/issues/91
	region_check_events = "CursorMoved",
})

ls.snippets = {all = {}, html = {}}

-- enable html snippets in javascript/javascript(REACT)
-- ls.snippets.javascript = ls.snippets.html
ls.snippets.javascriptreact = ls.snippets.html
ls.snippets.typescriptreact = ls.snippets.html

-- enable html snippets in django (htmldjango)
ls.snippets.htmldjango = ls.snippets.html

-- -- this will load according to filetype mentioned in include={}
-- require("luasnip/loaders/from_vscode").load({
-- 	include = {
-- 		"dart", "python", "javascript", "typescript", "css", "sass", "html", "php",
-- 	}
-- })
-- [its slowdown the startup speed. code needs some optimization ]
-- this will load for all filetype
require("luasnip/loaders/from_vscode").load()

--[[
-- Beside defining your own snippets you can also load snippets from "vscode-like" packages
-- that expose snippets in json files, for example <https://github.com/rafamadriz/friendly-snippets>.
-- Mind that this will extend  `ls.snippets` so you need to do it after your own snippets or you
-- will need to extend the table yourself instead of setting a new one.
]]

-- require("luasnip/loaders/from_vscode").load({ include = { "python" } }) -- Load only python snippets
-- require("luasnip/loaders/from_vscode").load({ paths = { "~/.config/nvim/extra/snippets" } }) -- Load snippets from my-snippets folder

-- You can also use lazy loading so you only get in memory snippets of languages you use
require'luasnip/loaders/from_vscode'.lazy_load({
	paths = {"~/.config/nvim/extra/snippets"},
})


-- ───────────────────────────────────────────────── --
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

