local group = vim.api.nvim_create_augroup("AbstractAutoGroup", { clear = true })

vim.api.nvim_create_autocmd("TextYankPost", {
	desc = "highlight text on yank",
	pattern = "*",
	group = group,
	callback = function()
		vim.highlight.on_yank({
			higroup = "Search",
			timeout = 150,
			on_visual = true,
		})
	end,
})

vim.api.nvim_create_autocmd("BufWinEnter", {
	desc = "jump to the last position when reopening a file",
	pattern = "*",
	group = group,
	command = [[ if line("'\"") > 0 && line("'\"") <= line("$") | exe "normal! g`\"" | endif ]],
})

vim.api.nvim_create_autocmd("BufWritePre", {
	desc = "remove whitespaces on save",
	pattern = "*",
	group = group,
	command = "%s/\\s\\+$//e",
})

vim.api.nvim_create_autocmd({ "BufEnter", "FileType" }, {
	desc = "don't auto comment new line",
	pattern = "*",
	group = group,
	command = "setlocal formatoptions-=c formatoptions-=r formatoptions-=o",
})

-- vim.api.nvim_create_autocmd(
-- 	"VimResized",
-- 	{
-- 		desc = "auto resize splited windows",
-- 		pattern = "*",
-- 		group = group,
-- 		command = "tabdo wincmd =",
-- 	}
-- )

vim.api.nvim_create_autocmd("BufWinEnter", {
	desc = "clear the last used search pattern",
	pattern = "*",
	group = group,
	command = "let @/ = ''",
})

vim.api.nvim_create_autocmd("FileType", {
	desc = "Give border to required windows",
	pattern = { "null-ls-info", "lspinfo" },
	group = group,
	callback = function()
		vim.api.nvim_win_set_config(0, { border = "rounded" })
	end,
})
