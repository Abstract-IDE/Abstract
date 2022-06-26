
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ───────────────────────────────────────────────── --
--   Plugin:    nvim-lsp-installer
--   Github:    github.com/williamboman/nvim-lsp-installer
-- ───────────────────────────────────────────────── --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --





-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━❰ configs ❱━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

local lspinstaller_imported, lspinstaller = pcall(require, 'nvim-lsp-installer')
if not lspinstaller_imported then return end


lspinstaller.setup{

    -- A list of servers to automatically install if they're not already installed. Example: { "rust_analyzer", "sumneko_lua" }
    -- This setting has no relation with the `automatic_installation` setting.
    -- ensure_installed = {"sumneko_lua"},

    -- Whether servers that are set up (via lspconfig) should be automatically installed if they're not already installed.
    -- This setting has no relation with the `ensure_installed` setting.
    -- Can either be:
    --   - false: Servers are not automatically installed.
    --   - true: All servers set up via lspconfig are automatically installed.
    --   - { exclude: string[] }: All servers set up via lspconfig, except the ones provided in the list, are automatically installed.
    --       Example: automatic_installation = { exclude = { "rust_analyzer", "solargraph" } }
    automatic_installation = false,

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
        keymaps = {
            -- Keymap to expand a server in the UI
            toggle_server_expand = "<CR>",
            -- Keymap to install the server under the current cursor position
            install_server = "i",
            -- Keymap to reinstall/update the server under the current cursor position
            update_server = "u",
            -- Keymap to check for new version for the server under the current cursor position
            check_server_version = "c",
            -- Keymap to update all installed servers
            update_all_servers = "U",
            -- Keymap to check which installed servers are outdated
            check_outdated_servers = "C",
            -- Keymap to uninstall a server
            uninstall_server = "X",
        },
	},

	-- The directory in which to install all servers.
	-- install_root_dir = path.concat { vim.fn.stdpath "data", "lsp_servers" },

	-- Controls to which degree logs are written to the log file. It's useful to set this to vim.log.levels.DEBUG when
	-- debugging issues with server installations.
	log_level = vim.log.levels.INFO,

	-- Limit for the maximum amount of servers to be installed at the same time. Once this limit is reached, any further
	-- servers that are requested to be installed will be put in a queue.
	max_concurrent_installers = 4,
}


-- always call require("nvim-lsp-installer") after require("nvim-lsp-installer").setup {}, this is the way
local lspconfig_imported, lspconfig = pcall(require, 'lspconfig')
if not lspconfig_imported then return end

local attach_imported, attach = pcall(require, "plugins.nvim-lspconfig")
if not attach_imported then return end
local on_attach = (attach).on_attach

local capabilities = vim.lsp.protocol.make_client_capabilities()
local options = {
	on_attach = on_attach,
	flags = {
		debounce_text_changes = 150,
	},
	capabilities = require('cmp_nvim_lsp').update_capabilities(capabilities),
}

local installed_servers = lspinstaller.get_installed_servers()
-- don't setup servers if atleast one server is installed, or it will throw an error
if #installed_servers == 0 then
	return
end


for _, server in ipairs(installed_servers) do

	-- for lua
	if server.name == "sumneko_lua" then
		options.settings = {
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
		capabilities = vim.lsp.protocol.make_client_capabilities()
		capabilities.offsetEncoding = { "utf-16" }
		options.capabilities = capabilities
	end

	-- for html
	if server.name == "html" then
		options.filetypes = {"html", "htmldjango"}
	end

	if server.name == "cssls" then

		--[==[
			Neovim does not currently include built-in snippets.
			`vscode-css-language-server` only provides completions when snippet support is enabled.
			To enable completion, install a snippet plugin and add the following override to your
			language client capabilities during setup. Enable (broadcasting) snippet capability for completion
		--]==]
		capabilities = vim.lsp.protocol.make_client_capabilities()
		capabilities.textDocument.completion.completionItem.snippetSupport = true
		options.capabilities = capabilities
	end

	lspconfig[server.name].setup(options)
end

-- for Flutter and Dart
-- don't put this on loop to set it because dart LSP installed and maintained by akinsho/flutter-tools.nvim
lspconfig["dartls"].setup(options)

-- ───────────────────────────────────────────────── --

-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━❰ end configs ❱━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

