local group = vim.api.nvim_create_augroup("AbstractAutocmdGroup", { clear = true })

vim.api.nvim_create_autocmd({ "TermOpen" }, {
	desc = "hide line number",
	group = group,
	pattern = "*",
	command = "setlocal nonumber norelativenumber",
})
