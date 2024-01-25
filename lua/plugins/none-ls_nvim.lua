--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: none-ls.nvim
Github: https://github.com/nvimtools/none-ls.nvim

 null-ls.nvim reloaded / Use Neovim as a language server
 to inject LSP diagnostics, code actions, and more via Lua.
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]




-- https://github.com/nvimtools/none-ls.nvim/blob/main/doc/BUILTINS.md

local _null, null = pcall(require, "null-ls")
-- Packages(LSP, Formatter, Linter, DAP) are installed and managed by 'williamboman/mason.nvim'
local _packages, packages = pcall(require, "mason-registry")
if not _null or not _packages then return end

-- get all installed Packages
local installed_packages = packages.get_installed_package_names()

local formatting = null.builtins.formatting
-- local completion = null.builtins.completion
-- local diagnostics = null.builtins.diagnostics
-- local code_actions = null.builtins.code_actions

-- register any number of sources simultaneously
local sources = {}
local load = false

-- ───────────────────────────────────────────────── --
-- ─────────────────❰ FORMATTING ❱────────────────── --
-- https://github.com/nvimtools/none-ls.nvim/tree/main/lua/null-ls/builtins/formatting

for _, package in pairs(installed_packages) do
	-- Lua
	if package == "luaformatter" then
		load = true
		sources[#sources + 1] = formatting.lua_format.with({
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
		goto loop_continue
	end
	if package == "stylua" then
		load = true
		sources[#sources + 1] = formatting.stylua.with({
			command = "stylua",
			args = {
				"--search-parent-directories",
				"--stdin-filepath",
				"$FILENAME",
				"-",
			},
		})
		goto loop_continue
	end
	-- Python
	if package == "black" then
		load = true
		sources[#sources + 1] = formatting.black.with({
			command = "black",
			args = { "--quiet", "--fast", "-" },
		})
		goto loop_continue
	end
	-- Django ("htmldjango")
	if package == "djlint" then
		load = true
		sources[#sources + 1] = formatting.djlint.with({
			command = "djlint",
			args = { "--reformat", "-" },
		})
		goto loop_continue
	end
	-- "javascript", "javascriptreact", "typescript", "typescriptreact", "vue",
	-- "css", "scss", "less", "html", "json", "yaml", "markdown", "graphql"
	if package == "prettier" then
		load = true
		sources[#sources + 1] = formatting.prettier.with({
			command = "prettier",
			args = { "--stdin-filepath", "$FILENAME" },
		})
		goto loop_continue
	end
	-- C, C++, CS, Java
	if package == "clang-format" then
		load = true
		sources[#sources + 1] = formatting.clang_format.with({
			command = "clang-format",
			args = {
				"-assume-filename",
				"$FILENAME",
			},
			to_stdin = true,
		})
		goto loop_continue
	end
	-- Rust
	if package == "rustfmt" then
		load = true
		sources[#sources + 1] = formatting.rustfmt.with({
			command = "rustfmt",
		})
		goto loop_continue
	end

	::loop_continue::
end

-- Go
if vim.fn.executable("gofmt") == 1 then
	load = true
	sources[#sources + 1] = formatting.gofmt.with({})
end
-- ───────────────❰ end FORMATTING ❱──────────────── --
-- ───────────────────────────────────────────────── --

-- ───────────────────────────────────────────────── --
-- ─────────────────❰ CODEACTION ❱────────────────── --
-- https://github.com/nvimtools/none-ls.nvim/tree/main/lua/null-ls/builtins/code_actions

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
-- -- https://github.com/nvimtools/none-ls.nvim/tree/main/lua/null-ls/builtins/diagnostics
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
-- https://github.com/nvimtools/none-ls.nvim/tree/main/lua/null-ls/builtins/completion
-- ───────────────❰ end COMPLETION ❱──────────────── --
-- ───────────────────────────────────────────────── --

-- ───────────────────────────────────────────────── --
-- ───────────────────❰ HOVER ❱───────────────────── --
-- https://github.com/nvimtools/none-ls.nvim/tree/main/lua/null-ls/builtins/hover
-- ─────────────────❰ end HOVER ❱─────────────────── --
-- ───────────────────────────────────────────────── --

-- setup null-ls
if load then
	null.setup({ debug = false, sources = sources })
end

local keymap = vim.api.nvim_set_keymap
keymap("n", "<Space>f", "<ESC>:lua vim.lsp.buf.format({ timeout_ms = 2000 })<CR>", { noremap = true, silent = true })
