
-- ───────────────────────────────────────────────── --
--              JAVASCRIPT, TYPESCRIPT
-- ───────────────────────────────────────────────── --

local filetype = vim.bo.filetype
if filetype ~= "javascript" and filetype ~= "typescript" and
	filetype ~= "javascriptreact" and filetype ~= "typescriptreact"
	then
		return
end

--[[
vim.opt.softtabstop = 2
vim.opt.shiftwidth = 2 -- spaces per tab (when shifting), when using the >> or << commands, shift lines by 4 spaces
vim.opt.tabstop = 2 -- spaces per tab
vim.opt.expandtab = true -- expand tabs into spaces
--]]

-- set filetype to typescriptreact if document if javascriptreact
if filetype == "javascriptreact" or filetype == "typescriptreact"
	then
		vim.api.nvim_command("set filetype=typescriptreact")
end

