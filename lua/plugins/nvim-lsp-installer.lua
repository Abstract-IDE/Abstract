
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ───────────────────────────────────────────────── --
--   Plugin:    nvim-lsp-installer
--   Github:    github.com/williamboman/nvim-lsp-installer
-- ───────────────────────────────────────────────── --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --





-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━❰ configs ❱━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

local lsp_installer = require("nvim-lsp-installer")

-- Provide settings first!
lsp_installer.settings {
	ui = {
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
local servers = {
	"sumneko_lua", -- for Lua
	-- "rust_analyzer",      -- for Rust
	-- "pyright",            -- for Python
	-- "clangd",             -- for C/C++
	-- "bashls",             -- for Bash
}

-- setup the LS
local On_attach = require("plugins.nvim-lspconfig").On_attach
make_server_ready(On_attach) -- LSP mappings

-- install the LS
for _, server in ipairs(servers) do install_server(server) end

-- ───────────────────────────────────────────────── --

-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━❰ end configs ❱━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

