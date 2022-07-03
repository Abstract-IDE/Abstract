
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ───────────────────────────────────────────────── --
--   Plugin:    nvim-lspconfig
--   Github:    github.com/neovim/nvim-lspconfig
-- ───────────────────────────────────────────────── --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --





-- ───────────────────────────────────────────────── --
--[==[
        import required plugins
        NOTE: always call require("lspconfig") after require("nvim-lsp-installer").setup {}, this is the way
              but don't worry,  we alreaed did it in 'packer config
--]==]
local lspconfig_imported, lspconfig = pcall(require, 'lspconfig')
if not lspconfig_imported then return end

local lspinstaller_imported, lspinstaller = pcall(require, 'nvim-lsp-installer')
if not lspinstaller_imported then return end

local installed_servers = lspinstaller.get_installed_servers()
-- don't setup servers if atleast one server is installed, or it will throw an error
if #installed_servers == 0 then return end
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
	buf_set_keymap('n', 'ge',           '<cmd>lua vim.diagnostic.open_float()<CR>', options)
	buf_set_keymap('n', 'gq',           '<cmd>lua vim.diagnostic.set_loclist({})<CR>', options)
	buf_set_keymap('n', 'gn',           '<cmd>lua vim.diagnostic.goto_next()<CR>', options)
	buf_set_keymap('n', 'gb',           '<cmd>lua vim.diagnostic.goto_prev()<CR>', options)

	buf_set_keymap('n', 'gd',           '<Cmd>lua vim.lsp.buf.definition()<CR>', options)
	buf_set_keymap('n', 'gD',           '<Cmd>lua vim.lsp.buf.declaration()<CR>', options)
	buf_set_keymap('n', 'gGG',           '<cmd>lua vim.lsp.buf.type_definition()<CR>', options)
	buf_set_keymap('n', 'gi',           '<cmd>lua vim.lsp.buf.implementation()<CR>', options)
	buf_set_keymap('n', 'gs',           '<cmd>lua vim.lsp.buf.signature_help()<CR>', options)
	buf_set_keymap('n', 'gh',           '<Cmd>lua vim.lsp.buf.hover()<CR>', options)
	buf_set_keymap('n', 'K',            '<Cmd>lua vim.lsp.buf.hover()<CR>', options)
	-- using 'filipdutescu/renamer.nvim' for rename
	-- buf_set_keymap('n', '<space>rn',	'<cmd>lua vim.lsp.buf.rename()<CR>', opts)
	buf_set_keymap('n', 'gr',			'<cmd>Telescope lsp_references<CR>', options)
	buf_set_keymap("n", "gf",           '<cmd>lua vim.lsp.buf.format{ async=true }<CR>', options)

	buf_set_keymap('n', 'ga',           '<cmd>lua vim.lsp.buf.code_action()<CR>',       options)
	buf_set_keymap('x', 'ga',           '<cmd>lua vim.lsp.buf.range_code_action()<CR>', options)

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

local lsp = vim.lsp
local handlers = lsp.handlers

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

-- se LSP diagnostic symbols/signs
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


-- -- change float window border
-- -- [https://github.com/williamboman/nvim-lsp-installer/issues/475#issuecomment-1037222808]
-- -- [NOTE: this setting can consume upto 20ms startup time]
-- local lspconfig_window = require("lspconfig.ui.windows")
-- local old_defaults = lspconfig_window.default_opts
--
-- function lspconfig_window.default_opts(opts)
--     local win_opts = old_defaults(opts)
--     win_opts.border = "rounded"
--     return win_opts
-- end


-- ───────────────────────────────────────────────── --
-- setup LSPs
-- ───────────────────────────────────────────────── --

local capabilities = lsp.protocol.make_client_capabilities()

lsp_options = {
	on_attach = on_attach,
	flags = {
		debounce_text_changes = 150,
	},
	capabilities = require('cmp_nvim_lsp').update_capabilities(capabilities),
}


-- for Flutter and Dart
-- don't put this on loop to set it because dart LSP installed and maintained by akinsho/flutter-tools.nvim
lspconfig["dartls"].setup(lsp_options)

for _, server in ipairs(installed_servers) do

	-- for lua
	if server.name == "sumneko_lua" then
		lsp_options.settings = {
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
		capabilities.offsetEncoding = { "utf-16" }
		lsp_options.capabilities = capabilities
	end

	-- for python
	if server.name == "pyright" then
		lsp_options.filetypes = {"python", }
	end

	-- for html
	if server.name == "html" then
		lsp_options.filetypes = {"html", "htmldjango"}
	end

	-- for css / scss / sass
	if server.name == "cssls" then

		--[==[
				Neovim does not currently include built-in snippets.
				`vscode-css-language-server` only provides completions when snippet support is enabled.
				To enable completion, install a snippet plugin and add the following override to your
				language client capabilities during setup. Enable (broadcasting) snippet capability for completion
				https://github.com/neovim/nvim-lspconfig/blob/master/lua/lspconfig/server_configurations/cssls.lua
		--]==]
		capabilities.textDocument.completion.completionItem.snippetSupport = true
		lsp_options.capabilities = capabilities
	end

	lspconfig[server.name].setup(lsp_options)
end
-- ───────────────────────────────────────────────── --
-- end LSP setup
-- ───────────────────────────────────────────────── --


-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━❰ end configs ❱━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

