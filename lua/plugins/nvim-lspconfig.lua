
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ───────────────────────────────────────────────── --
--   Plugin:    nvim-lspconfig
--   Github:    github.com/neovim/nvim-lspconfig
-- ───────────────────────────────────────────────── --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --





-- ───────────────────────────────────────────────── --
local lspconfig_imported, lspconfig = pcall(require, 'lspconfig')
if not lspconfig_imported then return end

local imported_mason, mason = pcall(require, 'mason')
if not imported_mason then return end

local lsp = vim.lsp
local handlers = lsp.handlers
-- ───────────────────────────────────────────────── --


-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━❰ Mappings ❱━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

-- Use an on_attach function to only map the following keys
-- after the language server attaches to the current buffer
-- ───────────────────────────────────────────────── --
local on_attach = function(client, bufnr)

	local function buf_set_keymap(...) vim.api.nvim_buf_set_keymap(bufnr, ...) end
	local function buf_set_option(...) vim.api.nvim_buf_set_option(bufnr, ...) end

	---------------------
	-- Avoiding LSP formatting conflicts
	-- ref: https://github.com/jose-elias-alvarez/null-ls.nvim/wiki/Avoiding-LSP-formatting-conflicts
	-- 2nd red: https://github.com/neovim/nvim-lspconfig/issues/1891#issuecomment-1157964108
	client.server_capabilities.documentFormattingProvider = false
	client.server_capabilities.documentRangeFormattingProvider = false
	--------------------------

	-- Enable completion triggered by <c-x><c-o>
	buf_set_option('omnifunc', 'v:lua.vim.lsp.omnifunc')

	-- Mappings.
	local options = {noremap = true, silent = true}

	-- See `:help vim.lsp.*` for documentation on any of the below functions
	-- ───────────────────────────────────────────────── --
	buf_set_keymap('n', '<Space>e',  '<cmd>lua vim.diagnostic.open_float()<CR>', options)
	buf_set_keymap('n', '<Space>q',  '<cmd>lua vim.diagnostic.set_loclist({})<CR>', options)
	buf_set_keymap('n', '<Space>n',  '<cmd>lua vim.diagnostic.goto_next()<CR>', options)
	buf_set_keymap('n', '<Space>b',  '<cmd>lua vim.diagnostic.goto_prev()<CR>', options)

	buf_set_keymap('n', '<Space>d',  '<Cmd>lua vim.lsp.buf.definition()<CR>', options)
	buf_set_keymap('n', '<Space>D',  '<Cmd>lua vim.lsp.buf.declaration()<CR>', options)
	buf_set_keymap('n', '<Space>T',  '<cmd>lua vim.lsp.buf.type_definition()<CR>', options)
	buf_set_keymap('n', '<Space>i',  '<cmd>lua vim.lsp.buf.implementation()<CR>', options)
	buf_set_keymap('n', '<Space>s',  '<cmd>lua vim.lsp.buf.signature_help()<CR>', options)
	buf_set_keymap('n', '<Space>h',  '<Cmd>lua vim.lsp.buf.hover()<CR>', options)
	buf_set_keymap('n', 'K',                  '<Cmd>lua vim.lsp.buf.hover()<CR>', options)
	-- using 'filipdutescu/renamer.nvim' for rename
	-- buf_set_keymap('n', '<space>rn',	'<cmd>lua vim.lsp.buf.rename()<CR>', opts)
	buf_set_keymap('n', '<Space>r',  '<cmd>Telescope lsp_references<CR>', options)
	buf_set_keymap("n", "<Space>f",  '<cmd>lua vim.lsp.buf.format{ async=true }<CR>', options)

	buf_set_keymap('n', '<Space>a',  '<cmd>lua vim.lsp.buf.code_action()<CR>',       options)
	buf_set_keymap('x', '<Space>a',  '<cmd>lua vim.lsp.buf.range_code_action()<CR>', options)

	-- buf_set_keymap('n', '<leader>wa',    '<cmd>lua vim.lsp.buf.add_workleader_folder()<CR>',          opts)
	-- buf_set_keymap('n', '<leader>wr',    '<cmd>lua vim.lsp.buf.remove_workleader_folder()<CR>',       opts)
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
			border = "rounded",
			focusable = true,
			style = "minimal",
			source = "always",
			header = "",
			prefix = "",
		},
	})

	handlers["textDocument/publishDiagnostics"] = lsp.with(
		lsp.diagnostic.on_publish_diagnostics,
		{
			underline = true,
			signs = true,
			update_in_insert = true,
			virtual_text = {
				true,
				spacing = 6,
				-- severity_limit='Error'  -- Only show virtual text on error
			},
		}
	)

	handlers["textDocument/hover"] = lsp.with(handlers.hover, {border = "rounded"})
	handlers["textDocument/signatureHelp"] = lsp.with(handlers.signature_help, {border = "rounded"})

	-- show diagnostic on float window(like auto complete)
	-- vim.api.nvim_command [[ autocmd CursorHold  *.lua,*.sh,*.bash,*.dart,*.py,*.cpp,*.c,js lua vim.lsp.diagnostic.show_line_diagnostics() ]]

	-- set LSP diagnostic symbols/signs
	-- ─────────────────────────────────────────────────--
	vim.api.nvim_command [[ sign define DiagnosticSignError text= texthl=DiagnosticSignError linehl= numhl= ]]
	vim.api.nvim_command [[ sign define DiagnosticSignWarn  text= texthl=DiagnosticSignWarn  linehl= numhl= ]]
	vim.api.nvim_command [[ sign define DiagnosticSignInfo  text= texthl=DiagnosticSignInfo  linehl= numhl= ]]
	vim.api.nvim_command [[ sign define DiagnosticSignHint  text= texthl=DiagnosticSignHint  linehl= numhl= ]]

	vim.api.nvim_command [[ hi DiagnosticUnderlineError cterm=underline gui=underline guisp=#840000 ]]
	vim.api.nvim_command [[ hi DiagnosticUnderlineHint cterm=underline  gui=underline guisp=#07454b ]]
	vim.api.nvim_command [[ hi DiagnosticUnderlineWarn cterm=underline  gui=underline guisp=#2f2905 ]]
	vim.api.nvim_command [[ hi DiagnosticUnderlineInfo cterm=underline  gui=underline guisp=#265478 ]]

	-- Auto-format files prior to saving them
	-- vim.api.nvim_command[[autocmd BufWritePre <buffer> lua vim.lsp.buf.formatting_sync(nil, 1000)]]

	--[[
	" to change colors, it's better to define in color scheme
	" highlight LspDiagnosticsUnderlineError         guifg=#EB4917 gui=undercurl
	" highlight LspDiagnosticsUnderlineWarning       guifg=#EBA217 gui=undercurl
	" highlight LspDiagnosticsUnderlineInformation   guifg=#17D6EB gui=undercurl
	" highlight LspDiagnosticsUnderlineHint          guifg=#17EB7A gui=undercurl
	--]]
end


-- ───────────────────────────────────────────────── --
-- setup LSPs
-- ───────────────────────────────────────────────── --
local function setup_lsp(on_attach)

	local capabilities = lsp.protocol.make_client_capabilities()
	local lsp_options = {
		on_attach = on_attach,
		flags = {
			debounce_text_changes = 150,
		},
		capabilities = require('cmp_nvim_lsp').update_capabilities(capabilities),
	}

	require("mason-lspconfig").setup_handlers({

		function (server_name)
			require("lspconfig")[server_name].setup(lsp_options)
		end,

		["clangd"] = function ()
			require("lspconfig").clangd.setup(
				vim.tbl_deep_extend(
					"force", lsp_options,
					{ capabilities = { offsetEncoding = { "utf-16" } } }
				)
			)
		end,
		["html"] = function ()
			require("lspconfig").html.setup(
				vim.tbl_deep_extend(
					"force", lsp_options,
					{ filetypes = {"html", "htmldjango"} }
				)
			)
		end,
		["cssls"] = function ()
			require("lspconfig").cssls.setup(
				vim.tbl_deep_extend(
					"force", lsp_options,
					{
						capabilities = {
							textDocument = { completion= { completionItem = { snippetSupport = true } } }
						},
					}
				)
			)
		end,
		["sumneko_lua"] = function ()
			require("lspconfig").sumneko_lua.setup(
				vim.tbl_deep_extend(
					"force", lsp_options,
					{
						settings = {
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
					}
				)
			)
		end
	})
end


-- NOTE: always call require("lspconfig") after require("nvim-lsp-installer").setup {}, this is the way

-- import nvim-lsp-installer configs
local imported_mason_config, mason_config = pcall(require, "plugins.mason_nvim")
if not imported_mason_config then return end

mason.setup(mason_config.setup) -- setup mason
setup_lsp_config() -- setup lsp configs (mainly UI)
setup_lsp(on_attach) -- setup lsp (like pyright, ccls ...)

-- ───────────────────────────────────────────────── --
-- end LSP setup
-- ───────────────────────────────────────────────── --


-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━❰ end configs ❱━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

