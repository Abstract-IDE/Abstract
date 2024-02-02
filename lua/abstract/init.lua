local M = {}

function M.setup()
	vim.g.ABSTRACT_LOADED = false
	require("abstract.bridge").setup()
	vim.g.ABSTRACT_LOADED = true
end

return M
