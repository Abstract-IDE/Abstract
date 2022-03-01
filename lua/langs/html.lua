
-- ───────────────────────────────────────────────── --
--                        HTML
-- ───────────────────────────────────────────────── --

-- don't load config if document is not html
local filetype = vim.bo.filetype
if filetype ~= "html"	then
	return
end

--[[
local set = vim.opt -- global options

vim.opt.softtabstop = 2
vim.opt.shiftwidth = 2 -- spaces per tab (when shifting)
vim.opt.tabstop = 2 -- spaces per tab
vim.opt.expandtab = true -- expand tabs into spaces
--]]

