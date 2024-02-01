--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: mason-lspconfig.nvim
Github: https://github.com/williamboman/mason-lspconfig.nvim

Extension to mason.nvim that makes it easier to use lspconfig with mason.nvim.
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
	"williamboman/mason-lspconfig.nvim",
	lazy = true,
	dependencies = {
		"b0o/schemastore.nvim", -- A Neovim Lua plugin providing access to the SchemaStore catalog.
	},
}

local function mason_lspconfig(lsp_options)
	local tbl_deep_extend = vim.tbl_deep_extend
	local lspconfig = require("lspconfig")

	-- for Flutter and Dart
	-- don't put this on setup_handlers to set it because dart LSP is installed and maintained by akinsho/flutter-tools.nvim
	lspconfig["dartls"].setup(lsp_options)

	require("mason-lspconfig").setup_handlers({

		function(server_name)
			lspconfig[server_name].setup(lsp_options)
		end,

		["clangd"] = function()
			lspconfig.clangd.setup(
				tbl_deep_extend("force", lsp_options, { capabilities = { offsetEncoding = { "utf-16" } } })
			)
		end,
		["html"] = function()
			lspconfig.html.setup(tbl_deep_extend("force", lsp_options, { filetypes = { "html", "htmldjango" } }))
		end,
		["cssls"] = function()
			lspconfig.cssls.setup(tbl_deep_extend("force", lsp_options, {
				capabilities = {
					textDocument = { completion = { completionItem = { snippetSupport = true } } },
				},
			}))
		end,
		-- ["lua_ls"] = function()
		-- 	lspconfig.lua_ls.setup(tbl_deep_extend("force", lsp_options, {
		-- 		settings = {
		-- 			Lua = {
		-- 				diagnostics = {
		-- 					-- Get the language server to recognize the 'vim', 'use' global
		-- 					globals = { "vim", "use", "require" },
		-- 				},
		-- 				workspace = {
		-- 					-- Make the server aware of Neovim runtime files
		-- 					library = vim.api.nvim_get_runtime_file("", true),
		-- 					--  don't ask about working environment on every startup
		-- 					checkThirdParty = false,
		-- 				},
		-- 				-- Do not send telemetry data containing a randomized but unique identifier
		-- 				telemetry = { enable = false },
		-- 			},
		-- 		},
		-- 	}))
		-- end,
		["rust_analyzer"] = function()
			lspconfig.rust_analyzer.setup(tbl_deep_extend("force", lsp_options, {
				settings = {
					["rust-analyzer"] = {
						diagnostics = { enable = true },
						checkOnSave = { enable = true },
					},
				},
			}))
		end,
		["jsonls"] = function()
			lspconfig.jsonls.setup(tbl_deep_extend("force", lsp_options, {
				settings = {
					json = {
						schemas = require("schemastore").json.schemas(),
						validate = { enable = true },
					},
				},
			}))
		end,

		["yamlls"] = function()
			lspconfig.yamlls.setup(tbl_deep_extend("force", lsp_options, {
				settings = {
					yaml = {
						schemaStore = {
							-- You must disable built-in schemaStore support if you want to use
							-- this plugin and its advanced options like `ignore`.
							enable = false,
							-- Avoid TypeError: Cannot read properties of undefined (reading 'length')
							url = "",
						},
						schemas = require("schemastore").yaml.schemas(),
					},
				},
			}))
		end,
	})
end

spec.config = function()
	local lsp_options = require("abstract.plugins.lspconfig").config()
	mason_lspconfig(lsp_options)
end

return spec
