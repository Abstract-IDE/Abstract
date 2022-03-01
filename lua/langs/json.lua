
-- ───────────────────────────────────────────────── --
--              JSON
-- ───────────────────────────────────────────────── --
-- don't load config if document is not json
local filetype = vim.bo.filetype
if filetype ~= "json" then
	return
end

--[[
vim.opt.softtabstop = 2
vim.opt.shiftwidth = 2 -- spaces per tab (when shifting), when using the >> or << commands, shift lines by 4 spaces
vim.opt.tabstop = 2 -- spaces per tab
vim.opt.expandtab = true -- expand tabs into spaces
--]]

