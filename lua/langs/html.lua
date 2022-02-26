
-- ───────────────────────────────────────────────── --
--                        HTML
-- ───────────────────────────────────────────────── --

-- don't load config if document is not html
local filetype = vim.bo.filetype
if filetype ~= "html"	then
	return
end

local set = vim.opt -- global options

set.softtabstop = 2
set.shiftwidth = 2 -- spaces per tab (when shifting)
set.tabstop = 2 -- spaces per tab
set.expandtab = true -- expand tabs into spaces

