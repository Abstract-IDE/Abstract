
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ───────────────────────────────────────────────── --
--   Plugin:    nvim-lsp-installer
--   Github:    github.com/williamboman/nvim-lsp-installer
-- ───────────────────────────────────────────────── --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --





-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━❰ configs ❱━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

-- safely import nvim-lsp-installer
local installer_imported_ok, lsp_installer = pcall(require, "nvim-lsp-installer")
if not installer_imported_ok then return end

-- Provide settings first!
lsp_installer.settings {
	ui = {
		-- Whether to automatically check for outdated servers when opening the UI window.
		check_outdated_servers_on_open = true,
		-- The border to use for the UI window. Accepts same border values as |nvim_open_win()|.
		border = "rounded",
		icons = {
			server_installed = "✓",
			server_pending = "➜",
			server_uninstalled = "✗",
		},
	},
	-- Limit for the maximum amount of servers to be installed at the same time. Once this limit is reached, any further
	-- servers that are requested to be installed will be put in a queue.
	max_concurrent_installers = 4,
}

-- ───────────────────────────────────────────────── --
local function make_server_ready(attach)
	lsp_installer.on_server_ready(function(server)
		local opts = {}
		opts.on_attach = attach


		-- for lua
		if server.name == "sumneko_lua" then
			-- only apply these settings for the "sumneko_lua" server
			opts.settings = {
				Lua = {
					diagnostics = {
						-- Get the language server to recognize the 'vim', 'use' global
						globals = {'vim', 'use', 'require'},
					},
					workspace = {
						-- Make the server aware of Neovim runtime files
						library = vim.api.nvim_get_runtime_file("", true),
					},
					-- Do not send telemetry data containing a randomized but unique identifier
					telemetry = {enable = false},
				},
			}
		end

		-- for clangd (c/c++)
		-- [https://github.com/jose-elias-alvarez/null-ls.nvim/issues/428]
		if server.name == "clangd" then
			local capabilities = vim.lsp.protocol.make_client_capabilities()
			capabilities.offsetEncoding = { "utf-16" }
			opts.capabilities = capabilities
		end

		-- for html
		if server.name == "html" then
			opts.filetypes = {"html", "htmldjango"}
		end


		-- This setup() function is exactly the same as lspconfig's setup function (:help lspconfig-quickstart)
		server:setup(opts)
		vim.cmd [[ do User LspAttachBuffers ]]
	end)
end
-- ───────────────────────────────────────────────── --

-- ───────────────────────────────────────────────── --
local function install_server(server)
	local lsp_installer_servers = require 'nvim-lsp-installer.servers'
	local ok, server_analyzer = lsp_installer_servers.get_server(server)
	if ok then
		if not server_analyzer:is_installed() then
			server_analyzer:install(server) -- will install in background
			-- lsp_installer.install(server)     -- install window will popup
		end
	end
end
-- ───────────────────────────────────────────────── --

-- ───────────────────────────────────────────────── --

-- setup the LS

local on_attach = require("plugins.nvim-lspconfig").on_attach

make_server_ready(on_attach) -- LSP mappings

-- local servers = {
	-- "sumneko_lua",        -- for Lua
	-- "rust_analyzer",      -- for Rust
	-- "pyright",            -- for Python
	-- "clangd",             -- for C/C++
	-- "bashls",             -- for Bash
-- }
-- install the LS
-- for _, server in ipairs(servers) do install_server(server) end

-- ───────────────────────────────────────────────── --

-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━❰ end configs ❱━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

