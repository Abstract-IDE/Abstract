local M = {}

function M.setup()
	vim.api.nvim_exec([[ autocmd User AbstractLSPLoaded lua print('got MyPlugin event') ]], false)

	vim.g.ABSTRACT_LOADED = false
	require("abstract.bridge").setup()
	vim.g.ABSTRACT_LOADED = true
end

return M
