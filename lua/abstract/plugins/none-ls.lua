--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin:    none-ls.nvim
Github:    https://github.com/nvimtools/none-ls.nvim

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

	-- get all installed Packages
	-- Packages(LSP, Formatter, Linter, DAP) are installed and managed by 'williamboman/mason.nvim'
	local installed_packages = require("mason-registry").get_installed_package_names()

	local formatting = null.builtins.formatting
	-- local completion = null.builtins.completion
	-- local diagnostics = null.builtins.diagnostics
	-- local code_actions = null.builtins.code_actions

	-- register any number of sources simultaneously
	local sources = {}

	-- ───────────────────────────────────────────────── --
	-- ─────────────────❰ FORMATTING ❱────────────────── --
	-- https://github.com/nvimtools/none-ls.nvim/tree/main/lua/null-ls/builtins/formatting

	for _, package in pairs(installed_packages) do
		-- lua
		if package == "stylua" then
			sources[#sources + 1] = formatting.stylua.with({})
		end
		-- Python
		if package == "black" then
			sources[#sources + 1] = formatting.black.with({})
		end
		-- Django ("htmldjango")
		if package == "djlint" then
			sources[#sources + 1] = formatting.djlint.with({
				command = "djlint",
				args = { "--reformat", "-" },
			})
		end
		-- "javascript", "javascriptreact", "typescript", "typescriptreact", "vue",
		-- "css", "scss", "less", "html", "json", "yaml", "markdown", "graphql"
		if package == "prettier" then
			sources[#sources + 1] = formatting.prettier.with({})
		end

		if package == "typstyle" then
			sources[#sources + 1] = formatting.typstyle.with({})
		end
	end

	-- Go
	if vim.fn.executable("gofmt") == 1 then
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
	-- overider the config with user defined one ("~/.config/nvim/lua/override/none-ls.lua")
	sources = vim.tbl_extend("force", sources, require("override.none-ls").setup(null, installed_packages))

	-- setup null-ls
	null.setup({ debug = false, sources = sources })
end

return spec
