
-- don't load config if document is not jua
local filetype = vim.bo.filetype
if filetype ~= "lua" then
	return
end
