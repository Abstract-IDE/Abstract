local M = {}

function M.lsp_loaded()
	vim.api.nvim_exec([[ autocmd User AbstractLSPLoaded lua return ]], false)
end

return M
