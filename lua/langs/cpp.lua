
-- don't load config if document is not c++
local filetype = vim.bo.filetype
if filetype ~= "cpp" then
	return
end
