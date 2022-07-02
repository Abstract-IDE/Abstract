
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ───────────────────────────────────────────────── --
--   Plugin:    nvim-lspconfig
--   Github:    github.com/neovim/nvim-lspconfig
-- ───────────────────────────────────────────────── --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --




local M = {}

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
	buf_set_keymap('n', '<Space>e', '<cmd>lua vim.diagnostic.open_float()<CR>', options)
	buf_set_keymap('n', '<Space>n', '<cmd>lua vim.diagnostic.goto_next()<CR>', options)
	buf_set_keymap('n', '<Space>b', '<cmd>lua vim.diagnostic.goto_prev()<CR>', options)

	buf_set_keymap('n', '<Space>d', '<Cmd>lua vim.lsp.buf.definition()<CR>', options)
	buf_set_keymap('n', '<Space>D', '<Cmd>lua vim.lsp.buf.declaration()<CR>', options)
	buf_set_keymap('n', '<Space>t', '<cmd>lua vim.lsp.buf.type_definition()<CR>', options)
	buf_set_keymap('n', '<Space>i', '<cmd>lua vim.lsp.buf.implementation()<CR>', options)
	buf_set_keymap('n', '<Space>s', '<cmd>lua vim.lsp.buf.signature_help()<CR>', options)
	buf_set_keymap('n', '<Space>h', '<Cmd>lua vim.lsp.buf.hover()<CR>', options)
	buf_set_keymap('n', 'K',        '<Cmd>lua vim.lsp.buf.hover()<CR>', options)
	-- using 'filipdutescu/renamer.nvim' for renaming
	-- buf_set_keymap('n', '<space>rn',	'<cmd>lua vim.lsp.buf.rename()<CR>', opts)
	buf_set_keymap('n', '<Space>r',	'<cmd>Telescope lsp_references<CR>', options)
	buf_set_keymap("n", "<Space>f", '<cmd>lua vim.lsp.buf.formatting_sync()<CR>', options)

	buf_set_keymap('n', '<Space>a', '<cmd>lua vim.lsp.buf.code_action()<CR>', options)
	buf_set_keymap('x', '<Space>a', '<cmd>lua vim.lsp.buf.range_code_action()<CR>', options)
end

-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━❰ end Mappings ❱━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --




-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━❰ configs ❱━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

local lsp = vim.lsp
local capabilities = lsp.protocol.make_client_capabilities()
local handlers = lsp.handlers

M.options = {
	on_attach = on_attach,
	flags = {
		debounce_text_changes = 150,
	},
	capabilities = require('cmp_nvim_lsp').update_capabilities(capabilities),
}


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

-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━❰ end configs ❱━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

return M

