
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

local builtins = require("null-ls.builtins")
local formatting = builtins.formatting
-- local completion = builtins.completion
-- local diagnostics = builtins.diagnostics
-- local code_actions = builtins.code_actions

-- ───────────────────────────────────────────────── --
-- ─────────────────❰ FORMATTING ❱────────────────── --
-- https://github.com/jose-elias-alvarez/null-ls.nvim/tree/main/lua/null-ls/builtins/formatting

-- register any number of sources simultaneously
local sources = {}
local ld = false

-- Lua
if vim.fn.executable("lua-format") == 1 then
	ld = true
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
	ld = true
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
	ld = true
	sources[#sources+1] = formatting.prettier.with({
		command = "prettier",
		args = {"--stdin-filepath", "$FILENAME"},
	})
end

-- Python
if vim.fn.executable("black") == 1 then
	ld = true
	sources[#sources+1] = formatting.black.with({
		command = "black",
		args = {"--quiet", "--fast", "-"},
	})
end

if ld then
	require("null-ls").setup({sources = sources})
end

-- ───────────────❰ end FORMATTING ❱──────────────── --
-- ───────────────────────────────────────────────── --


-- ───────────────────────────────────────────────── --
-- ─────────────────❰ COMPLETION ❱────────────────── --
-- https://github.com/jose-elias-alvarez/null-ls.nvim/tree/main/lua/null-ls/builtins/completion
-- ───────────────❰ end COMPLETION ❱──────────────── --
-- ───────────────────────────────────────────────── --

-- ───────────────────────────────────────────────── --
-- ─────────────────❰ DIAGNOSTICS ❱───────────────── --
-- https://github.com/jose-elias-alvarez/null-ls.nvim/tree/main/lua/null-ls/builtins/diagnostics
-- ───────────────❰ end DIAGNOSTICS ❱─────────────── --
-- ───────────────────────────────────────────────── --

-- ───────────────────────────────────────────────── --
-- ─────────────────❰ CODEACTION ❱────────────────── --
-- https://github.com/jose-elias-alvarez/null-ls.nvim/tree/main/lua/null-ls/builtins/code_actions
-- ───────────────❰ end CODEACTION ❱──────────────── --
-- ───────────────────────────────────────────────── --

-- ───────────────────────────────────────────────── --
-- ───────────────────❰ HOVER ❱───────────────────── --
-- https://github.com/jose-elias-alvarez/null-ls.nvim/tree/main/lua/null-ls/builtins/hover
-- ─────────────────❰ end HOVER ❱─────────────────── --
-- ───────────────────────────────────────────────── --







-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━❰ end configs ❱━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
--                                                   --
--                                                   --
--                                                   --
--                                                   --
--                                                   --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━❰ Mappings ❱━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

local keymap = vim.api.nvim_set_keymap
keymap('n', '<Space>fm', '<ESC>:lua vim.lsp.buf.formatting()<CR>', {noremap = true, silent = true})
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━❰ end Mappings ❱━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

