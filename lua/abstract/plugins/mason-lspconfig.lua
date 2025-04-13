--[[
--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: mason-lspconfig.nvim
Source: https://github.com/williamboman/mason-lspconfig.nvim

Extension to mason.nvim that makes it easier to use lspconfig with mason.nvim.

Resources:
https://github.com/neovim/nvim-lspconfig/blob/master/doc/server_configurations.md
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
	"williamboman/mason-lspconfig.nvim",
	lazy = true,
}

local mason_lspconfig = function(hook)
	local lspconfig = require("lspconfig")

	local function set_lspconfig(server, options)
		local lsp_options = vim.tbl_deep_extend("force", hook, options)
		lspconfig[server].setup(lsp_options)
	end

	local server_config = {}

	-- some lsp comes with toolchain so we need configure them separately
	server_config.toolchain_server = {
		["sourcekit"] = {
			executable = "swift",
			config = {
				capabilities = {
					workspace = {
						didChangeWatchedFiles = {
							dynamicRegistration = true,
						},
					},
				},
			},
		},
		["mojo"] = {
			executable = "mojo",
			config = {},
		},
	}

	-- and this one is for installed lsp
	server_config.installed_server = {
		function(server_name)
			set_lspconfig(server_name, {})
		end,
		["ts_ls"] = function()
			-- Typescript LSP is maintained by https://github.com/pmizio/typescript-tools.nvim
		end,
		-- NOTE: for now lets disable it as its slow; like if you make even single change in rust file and save, whole things starts load and its really slow
		-- ["rust_analyzer"] = function()
		-- 	-- Rust LSP is maintained by https://github.com/mrcjkb/rustaceanvim
		-- 	require("abstract.plugins.rustaceanvim").setup(hook)
		-- end,
		["jdtls"] = function()
			-- Java LSP is maintained by https://github.com/nvim-java/nvim-java
			require("abstract.plugins.nvim-java").setup()
			set_lspconfig("jdtls", {})
		end,
		["html"] = function()
			set_lspconfig("html", {
				filetypes = { "html", "htmldjango" },
			})
		end,
		["jsonls"] = function()
			set_lspconfig("jsonls", {
				settings = {
					json = {
						schemas = require("schemastore").json.schemas(),
						validate = { enable = true },
					},
				},
			})
		end,
		["pyright"] = function()
			set_lspconfig("pyright", {
				settings = {
					python = {
						analysis = {
							autoSearchPaths = true,
							-- diagnosticMode = "workspace", -- options: "workspace" | "openFilesOnly"
							useLibraryCodeForTypes = true,
						},
					},
				},
			})
		end,
		["yamlls"] = function()
			set_lspconfig("yamlls", {
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
			})
		end,
		["cssls"] = function()
			set_lspconfig("cssls", {
				capabilities = {
					textDocument = {
						completion = {
							completionItem = {
								snippetSupport = true,
							},
						},
					},
				},
			})
		end,
	}

	-- override with user defined config ("~/.config/nvim/lua/override/lsp.lua")
	local user_config = require("override.lsp").setup(set_lspconfig)
	server_config = vim.tbl_deep_extend("force", server_config, user_config)

	for server, server_setup in pairs(server_config.toolchain_server) do
		if vim.fn.executable(server_setup.executable) == 1 then
			set_lspconfig(server, server_setup.config)
		end
	end

	return server_config.installed_server
end

spec.setup = function()
	local hook = require("abstract.plugins.lspconfig").setup()
	require("mason-lspconfig").setup({
		-- A list of servers to automatically install if they're not already installed. Example: { "rust_analyzer@nightly", "lua_ls" }
		-- This setting has no relation with the `automatic_installation` setting.
		---@type string[]
		ensure_installed = {
			"bashls",
			"cssls",
			"eslint",
			"html",
			"jsonls",
			"lua_ls",
			"basedpyright",
			"ts_ls", -- managed by typescript-tools
		},

		-- Whether servers that are set up (via lspconfig) should be automatically installed if they're not already installed.
		-- This setting has no relation with the `ensure_installed` setting.
		-- Can either be:
		--   - false: Servers are not automatically installed.
		--   - true: All servers set up via lspconfig are automatically installed.
		--   - { exclude: string[] }: All servers set up via lspconfig, except the ones provided in the list, are automatically installed.
		--       Example: automatic_installation = { exclude = { "rust_analyzer", "solargraph" } }
		---@type boolean
		automatic_installation = false,

		-- See `:h mason-lspconfig.setup_handlers()`
		---@type table<string, fun(server_name: string)>?
		handlers = mason_lspconfig(hook),
	})
end

return spec
