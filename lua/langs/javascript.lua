
-- ───────────────────────────────────────────────── --
--              JAVASCRIPT, TYPESCRIPT
-- ───────────────────────────────────────────────── --

local filetype = vim.bo.filetype
if filetype ~= "javascript" and filetype ~= "typescript" and
	filetype ~= "javascriptreact" and filetype ~= "typescriptreact"
	then
		return
end

local set = vim.opt -- global options

set.softtabstop = 2
set.shiftwidth = 2 -- spaces per tab (when shifting), when using the >> or << commands, shift lines by 4 spaces
set.tabstop = 2 -- spaces per tab
set.expandtab = true -- expand tabs into spaces


-- set filetype to typescriptreact if document if javascriptreact
if filetype == "javascriptreact" or filetype == "typescriptreact"
	then
		vim.api.nvim_command("set filetype=typescriptreact")
end

