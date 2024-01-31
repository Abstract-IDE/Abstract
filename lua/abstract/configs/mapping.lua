return {
	-- none-ls
	["<Space>"] = {
		f = { "<cmd>lua vim.lsp.buf.format({ timeout_ms = 2000 })<CR>", "Format code", noremap = true, silent = true },
	},
}
