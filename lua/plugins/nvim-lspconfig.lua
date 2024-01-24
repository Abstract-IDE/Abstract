
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ───────────────────────────────────────────────── --
--   Plugin:    nvim-lspconfig
--   Github:    github.com/neovim/nvim-lspconfig
-- ───────────────────────────────────────────────── --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --




-- ───────────────────────────────────────────────── --
local _lspconfig, lspconfig = pcall(require, "lspconfig")
local _mlspconfig, mason_lspconfig = pcall(require, "mason-lspconfig")
if not _lspconfig or not _mlspconfig then return end

local lsp = vim.lsp
local api = vim.api
local fn = vim.fn
local handlers = vim.lsp.handlers
-- ───────────────────────────────────────────────── --


-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━❰ Mappings ❱━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

local function mappings(bufnr)

	local buf_set_keymap = function(...)
		api.nvim_buf_set_keymap(bufnr, ...)
	end
	local buf_set_option = function(...)
		api.nvim_buf_set_option(bufnr, ...)
	end
	local options = { noremap = true, silent = true }

	-- Enable completion triggered by <c-x><c-o>
	buf_set_option("omnifunc", "v:lua.vim.lsp.omnifunc")

	-- See `:help vim.lsp.*` for documentation on any of the below functions
	-- ───────────────────────────────────────────────── --
	buf_set_keymap("n", "<Space>e", "<cmd>lua vim.diagnostic.open_float()<CR>", options)
	buf_set_keymap("n", "<Space>n", "<cmd>lua vim.diagnostic.goto_next()<CR>", options)
	buf_set_keymap("n", "<Space>b", "<cmd>lua vim.diagnostic.goto_prev()<CR>", options)

	buf_set_keymap("n", "<Space>d", "<Cmd>lua vim.lsp.buf.definition()<CR>", options)
	buf_set_keymap("n", "<Space>D", "<Cmd>lua vim.lsp.buf.declaration()<CR>", options)
	buf_set_keymap("n", "<Space>T", "<cmd>lua vim.lsp.buf.type_definition()<CR>", options)
	buf_set_keymap("n", "<Space>i", "<cmd>lua vim.lsp.buf.implementation()<CR>", options)
	buf_set_keymap("n", "<Space>s", "<cmd>lua vim.lsp.buf.signature_help()<CR>", options)
	buf_set_keymap("n", "<Space>h", "<Cmd>lua vim.lsp.buf.hover()<CR>", options)
	buf_set_keymap("n", "K", "<Cmd>lua vim.lsp.buf.hover()<CR>", options)
	-- using 'filipdutescu/renamer.nvim' for rename
	-- buf_set_keymap('n', '<space>rn',	'<cmd>lua vim.lsp.buf.rename()<CR>', opts)
	buf_set_keymap("n", "<Space>r", "<cmd>Telescope lsp_references<CR>", options)
	buf_set_keymap("n", "<Space>f", "<cmd>lua vim.lsp.buf.format({ timeout_ms = 2000 })<CR>", options)

	buf_set_keymap("n", "<Space>a", "<cmd>lua vim.lsp.buf.code_action()<CR>", options)
	buf_set_keymap("x", "<Space>a", "<cmd>lua vim.lsp.buf.range_code_action()<CR>", options)

	-- buf_set_keymap('n', '<leader>wa',   '<cmd>lua vim.lsp.buf.add_workleader_folder()<CR>',          opts)
	-- buf_set_keymap('n', '<leader>wr',   '<cmd>lua vim.lsp.buf.remove_workleader_folder()<CR>',       opts)
	-- buf_set_keymap('n', '<leader>wl',   '<cmd>lua print(vim.inspect(vim.lsp.buf.list_workleader_folders()))<CR>', opts)
end

-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━❰ end Mappings ❱━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --




-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━❰ configs ❱━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

local function setup_lsp_config()
	-- options for lsp diagnostic
	-- ───────────────────────────────────────────────── --
	vim.diagnostic.config({
		float = {
			border = "single",
			focusable = true,
			style = "minimal",
			source = "always",
			header = "",
			prefix = "",
		},
	})

	handlers["textDocument/publishDiagnostics"] = lsp.with(lsp.diagnostic.on_publish_diagnostics, {
		underline = true,
		signs = true,
		update_in_insert = true,
		virtual_text = {
			true,
			spacing = 6,
			-- severity_limit='Error'  -- Only show virtual text on error
		},
	})

	handlers["textDocument/hover"] = lsp.with(handlers.hover, { border = "single" })
	handlers["textDocument/signatureHelp"] = lsp.with(handlers.signature_help, { border = "single" })

	-- show diagnostic on float window(like auto complete)
	-- vim.api.nvim_command [[ autocmd CursorHold  *.lua,*.sh,*.bash,*.dart,*.py,*.cpp,*.c,js lua vim.lsp.diagnostic.show_line_diagnostics() ]]

	-- set LSP diagnostic symbols/signs
	-- ─────────────────────────────────────────────────--
	fn.sign_define("DiagnosticSignError", { text = "●", texthl = "DiagnosticSignError" })
	fn.sign_define("DiagnosticSignWarn", { text = "●", texthl = "DiagnosticSignWarn" })
	fn.sign_define("DiagnosticSignInfo", { text = "●", texthl = "DiagnosticSignInfo" })
	fn.sign_define("DiagnosticSignHint", { text = "󰌵", texthl = "DiagnosticSignHint" })

	-- Auto-format files prior to saving them
	-- vim.api.nvim_command[[autocmd BufWritePre <buffer> lua vim.lsp.buf.formatting_sync(nil, 1000)]]
end


local lsp_options = {

	flags = {
		debounce_text_changes = 150,
	},

	on_attach = function(client, bufnr)
		-- lsp support on winbar
		local _navic, navic = pcall(require, "nvim-navic")
		if _navic then
			if client.server_capabilities.documentSymbolProvider then
				navic.attach(client, bufnr)
			end
		end

		---------------------
		-- NOTE: integrate with none-ls | null -ls
		-- Avoiding LSP formatting conflicts
		-- ref: https://github.com/jose-elias-alvarez/null-ls.nvim/wiki/Avoiding-LSP-formatting-conflicts
		-- 2nd red: https://github.com/neovim/nvim-lspconfig/issues/1891#issuecomment-1157964108
		-- client.server_capabilities.documentFormattingProvider = false
		-- client.server_capabilities.documentRangeFormattingProvider = false
		--------------------------

		-- enable Mappings
		mappings(bufnr)
	end,

	capabilities = (function()
		local _capabilities = vim.lsp.protocol.make_client_capabilities()

		-- enable LSP's builtin snippet support
		_capabilities.textDocument.completion.completionItem.snippetSupport = true

		local _cmp_lsp, cmp_lsp = pcall(require, "cmp_nvim_lsp")
		if not _cmp_lsp then return _capabilities end

		return vim.tbl_deep_extend("force", _capabilities, cmp_lsp.default_capabilities())
	end)(),
}


local function setup_lsp()

	local tbl_deep_extend = vim.tbl_deep_extend

	-- for Flutter and Dart
	-- don't put this on setup_handlers to set it because dart LSP is installed and maintained by akinsho/flutter-tools.nvim
	lspconfig["dartls"].setup(lsp_options)

	mason_lspconfig.setup_handlers({

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
		["lua_ls"] = function()
			lspconfig.lua_ls.setup(tbl_deep_extend("force", lsp_options, {
				settings = {
					Lua = {
						diagnostics = {
							-- Get the language server to recognize the 'vim', 'use' global
							globals = { "vim", "use", "require" },
						},
						workspace = {
							-- Make the server aware of Neovim runtime files
							library = api.nvim_get_runtime_file("", true),
							--  don't ask about working environment on every startup
							checkThirdParty = false,
						},
						-- Do not send telemetry data containing a randomized but unique identifier
						telemetry = { enable = false },
					},
				},
			}))
		end,
		["rust_analyzer"] = function()
			lspconfig.rust_analyzer.setup(tbl_deep_extend("force", lsp_options, {
				settings = {
					["rust-analyzer"] = {
						diagnostics = { enable = true, },
						checkOnSave = { enable = true, },
					},
				},
			}))
		end,
	})
end

-- make sure `lspconfig` is not loaded after `mason-lspconfig`.
-- Also, make sure not to set up any servers via `lspconfig` _before_ calling `mason-lspconfig`'s setup function.
setup_lsp_config() -- setup lsp configs (mainly UI)
setup_lsp() -- setup lsp (like pyright, ccls ...)

-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━❰ end configs ❱━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

