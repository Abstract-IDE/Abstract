--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: none-ls.nvim
Source: https://github.com/nvimtools/none-ls.nvim

null-ls.nvim reloaded / Use Neovim as a language server
to inject LSP diagnostics, code actions, and more via Lua.
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
	"nvimtools/none-ls.nvim",
	lazy = true,
}

spec.setup = function()
	-- https://github.com/nvimtools/none-ls.nvim/blob/main/doc/BUILTINS.md
	local null = require("null-ls")

	local formatting = null.builtins.formatting
	-- local completion = null.builtins.completion
	-- local diagnostics = null.builtins.diagnostics
	-- local code_actions = null.builtins.code_actions

	-- register any number of sources simultaneously
	local sources = {}

	-- === FORMATTING === ---
	-- https://github.com/nvimtools/none-ls.nvim/tree/main/lua/null-ls/builtins/formatting

	-- Go
	if vim.fn.executable("gofmt") == 1 then
		sources[#sources + 1] = formatting.gofmt.with({})
	end

	-- === CODEACTION === --
	-- https://github.com/nvimtools/none-ls.nvim/tree/main/lua/null-ls/builtins/code_actions

	-- -- Javascript
	-- if vim.fn.executable("clang-format") == 1 then
	-- 	sources[#sources + 1] = code_actions.eslint.with({
	-- 		command = "eslint",
	-- 		filetypes = { "javascript", "javascriptreact", "typescript", "typescriptreact", "vue" },
	-- 		args = { "-f", "json", "--stdin", "--stdin-filename", "$FILENAME" },
	-- 		to_stdin = true,
	-- 	})
	-- end

	-- === DIAGNOSTICS === --
	-- -- https://github.com/nvimtools/none-ls.nvim/tree/main/lua/null-ls/builtins/diagnostics

	-- -- Django ("htmldjango")
	-- if vim.fn.executable("djlint") == 1 then
	-- 	sources[#sources+1] = diagnostics.djlint.with({
	-- 		command = "djlint",
	-- 		args = { "$FILENAME" },
	-- 	})
	-- end

	-- === COMPLETION === --
	-- https://github.com/nvimtools/none-ls.nvim/tree/main/lua/null-ls/builtins/completion

	-- === HOVER === --
	-- https://github.com/nvimtools/none-ls.nvim/tree/main/lua/null-ls/builtins/hover


	-- setup null-ls
	-- overider the config with user defined one ("~/.config/nvim/lua/override/none-ls.lua")
	sources = vim.tbl_extend("force", sources, require("override.none-ls").sources(null))

	-- setup null-ls
	null.setup({ debug = false, sources = sources })
end

return spec
