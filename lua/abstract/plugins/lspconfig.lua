--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: nvim-lspconfig
Github: https://github.com/neovim/nvim-lspconfig

Quickstart configs for Nvim LSP
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
	"neovim/nvim-lspconfig",
	lazy = true,
}

local function mappings(bufnr)
	local buf_set_keymap = function(...)
		vim.api.nvim_buf_set_keymap(bufnr, ...)
	end
	local buf_set_option = function(...)
		vim.api.nvim_buf_set_option(bufnr, ...)
	end
	local options = { noremap = true, silent = true }

	-- Enable completion triggered by <c-x><c-o>
	buf_set_option("omnifunc", "v:lua.vim.lsp.omnifunc")

	-- See `:help vim.lsp.*` for documentation on any of the below functions
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

local function setup_lsp_config()
	local handlers = vim.lsp.handlers

	-- options for lsp diagnostic
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

	handlers["textDocument/publishDiagnostics"] = vim.lsp.with(vim.lsp.diagnostic.on_publish_diagnostics, {
		underline = true,
		signs = true,
		update_in_insert = true,
		virtual_text = {
			true,
			spacing = 6,
			-- severity_limit='Error'  -- Only show virtual text on error
		},
	})
	handlers["textDocument/hover"] = vim.lsp.with(handlers.hover, { border = "rounded" })
	handlers["textDocument/signatureHelp"] = vim.lsp.with(handlers.signature_help, { border = "single" })

	-- show diagnostic on float window(like auto complete)
	-- vim.api.nvim_command [[ autocmd CursorHold  *.lua,*.sh,*.bash,*.dart,*.py,*.cpp,*.c,js lua vim.lsp.diagnostic.show_line_diagnostics() ]]

	-- set LSP diagnostic symbols/signs ●
	vim.fn.sign_define("DiagnosticSignError", { text = "✗", texthl = "DiagnosticSignError" })
	vim.fn.sign_define("DiagnosticSignWarn", { text = "⚠", texthl = "DiagnosticSignWarn" })
	vim.fn.sign_define("DiagnosticSignInfo", { text = "i", texthl = "DiagnosticSignInfo" })
	vim.fn.sign_define("DiagnosticSignHint", { text = "󰌵", texthl = "DiagnosticSignHint" })

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

		-- Enable Mappings
		mappings(bufnr)
	end,

	capabilities = (function()
		local _capabilities = vim.lsp.protocol.make_client_capabilities()

		-- enable LSP's builtin snippet support
		_capabilities.textDocument.completion.completionItem.snippetSupport = true

		local _cmp_lsp, cmp_lsp = pcall(require, "cmp_nvim_lsp")
		if not _cmp_lsp then
			return _capabilities
		end

		return vim.tbl_deep_extend("force", _capabilities, cmp_lsp.default_capabilities())
	end)(),
}

spec.config = function()
	setup_lsp_config()
	return lsp_options
end

return spec
