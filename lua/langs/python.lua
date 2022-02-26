
-- ───────────────────────────────────────────────── --
--                        PYTHON
-- ───────────────────────────────────────────────── --

-- don't load config if document is not python
local filetype = vim.bo.filetype
if filetype ~= "python"	then
	return
end



local set = vim.opt -- global options

set.softtabstop = 4
set.shiftwidth = 4 -- spaces per tab (when shifting), when using the >> or << commands, shift lines by 4 spaces
set.tabstop = 4 -- spaces per tab
set.expandtab = true -- expand tabs into spaces

