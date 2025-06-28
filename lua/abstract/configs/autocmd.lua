local M = {}

M.groups = {
	Abstract = vim.api.nvim_create_augroup("AbstractAutocmdGroup", { clear = true }),
	Lsp = vim.api.nvim_create_augroup("AbstractAutocmdLspGroup", { clear = true }),
}

M.setup = function()
	vim.api.nvim_create_autocmd({ "TermOpen" }, {
		desc = "hide line number",
		group = M.groups.Abstract,
		pattern = "*",
		command = "setlocal nonumber norelativenumber",
	})
end

return M
