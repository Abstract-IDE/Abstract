
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ───────────────────────────────────────────────── --
--   Plugin:    null-ls.nvim
--   Github:    github.com/jose-elias-alvarez/null-ls.nvim
-- ───────────────────────────────────────────────── --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --





-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━❰ configs ❱━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

-- https://github.com/jose-elias-alvarez/null-ls.nvim/blob/main/doc/BUILTINS.md

-- safley import null-ls
local null_imported_ok, null = pcall(require, "null-ls")
if not null_imported_ok then return end

local formatting = null.builtins.formatting
-- local completion = null.builtins.completion
-- local diagnostics = null.builtins.diagnostics
-- local code_actions = null.builtins.code_actions

-- register any number of sources simultaneously
local sources = {}
local load = false


-- ───────────────────────────────────────────────── --
-- ─────────────────❰ FORMATTING ❱────────────────── --
-- https://github.com/jose-elias-alvarez/null-ls.nvim/tree/main/lua/null-ls/builtins/formatting

-- Lua
if vim.fn.executable("lua-format") == 1 then
	load = true
	sources[#sources+1] = formatting.lua_format.with({
		command = "lua-format",
		args = {
			"--indent-width",
			"1",
			"--tab-width",
			"4",
			"--use-tab",
			"--chop-down-table",
			"--extra-sep-at-table-end",
		},
	})
end

 -- C, C++, CS, Java
if vim.fn.executable("clang-format") == 1 then
	load = true
	sources[#sources+1] = formatting.clang_format.with({
		command = "clang-format",
		args = {
			"-assume-filename",
			"$FILENAME",
			"-style",
			"{BasedOnStyle: Microsoft, UseTab: Always}",
		},
		to_stdin = true,
	})
end

-- "javascript", "javascriptreact", "typescript", "typescriptreact", "vue",
-- "css", "scss", "less", "html", "json", "yaml", "markdown", "graphql"
if vim.fn.executable("prettier") == 1 then
	load = true
	sources[#sources+1] = formatting.prettier.with({
		command = "prettier",
		args = {"--stdin-filepath", "$FILENAME"},
	})
end

-- Python
if vim.fn.executable("black") == 1 then
	load = true
	sources[#sources+1] = formatting.black.with({
		command = "black",
		args = {"--quiet", "--fast", "-"},
	})
end

-- Django ("htmldjango")
if vim.fn.executable("djlint") == 1 then
	load = true
	sources[#sources+1] = formatting.djlint.with({
		command = "djlint",
		args = { "--reformat", "-"},
	})
end

-- Rust
if vim.fn.executable("rustfmt") == 1 then
	load = true
	sources[#sources+1] = formatting.rustfmt.with({
		command = "rustfmt",
	})
end

-- ───────────────❰ end FORMATTING ❱──────────────── --
-- ───────────────────────────────────────────────── --


-- ───────────────────────────────────────────────── --
-- ─────────────────❰ CODEACTION ❱────────────────── --
-- https://github.com/jose-elias-alvarez/null-ls.nvim/tree/main/lua/null-ls/builtins/code_actions

--[===[
-- Javascript
if vim.fn.executable("clang-format") == 1 then
	load = true
	sources[#sources+1] = code_actions.eslint.with({
		command = "eslint",
		filetypes = { "javascript", "javascriptreact", "typescript", "typescriptreact", "vue" },
		args = { "-f", "json", "--stdin", "--stdin-filename", "$FILENAME" },
		to_stdin = true,
	})
end
---]===]

-- ───────────────❰ end CODEACTION ❱──────────────── --
-- ───────────────────────────────────────────────── --


-- ───────────────────────────────────────────────── --
-- ─────────────────❰ DIAGNOSTICS ❱───────────────── --
-- -- https://github.com/jose-elias-alvarez/null-ls.nvim/tree/main/lua/null-ls/builtins/diagnostics
-- -- Django ("htmldjango")
-- if vim.fn.executable("djlint") == 1 then
-- 	load = true
-- 	sources[#sources+1] = diagnostics.djlint.with({
-- 		command = "djlint",
-- 		args = { "$FILENAME" },
-- 	})
-- end
-- ───────────────❰ end DIAGNOSTICS ❱─────────────── --
-- ───────────────────────────────────────────────── --


-- ───────────────────────────────────────────────── --
-- ─────────────────❰ COMPLETION ❱────────────────── --
-- https://github.com/jose-elias-alvarez/null-ls.nvim/tree/main/lua/null-ls/builtins/completion
-- ───────────────❰ end COMPLETION ❱──────────────── --
-- ───────────────────────────────────────────────── --

-- ───────────────────────────────────────────────── --
-- ───────────────────❰ HOVER ❱───────────────────── --
-- https://github.com/jose-elias-alvarez/null-ls.nvim/tree/main/lua/null-ls/builtins/hover
-- ─────────────────❰ end HOVER ❱─────────────────── --
-- ───────────────────────────────────────────────── --


if load then
	null.setup({
		sources = sources
	})
end

-- give border to null-ls window
vim.api.nvim_create_autocmd("FileType", {
  pattern = "null-ls-info",
  callback = function() vim.api.nvim_win_set_config(0, { border = "rounded" }) end,
})

-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━❰ end configs ❱━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --




-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━❰ Mappings ❱━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

local keymap = vim.api.nvim_set_keymap
keymap('n', 'gf', '<ESC>:lua vim.lsp.buf.format{ async=true }<CR>', {noremap = true, silent = true})

-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━❰ end Mappings ❱━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

