--[[━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: nvim-lspconfig
Github: https://github.com/neovim/nvim-lspconfig

Quickstart configs for Nvim LSP
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━]]

local spec = {
	"neovim/nvim-lspconfig",
	lazy = true,
}

local function lsp_config()
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
	vim.fn.sign_define("DiagnosticSignError", { text = "", texthl = "DiagnosticSignError" })
	vim.fn.sign_define("DiagnosticSignWarn", { text = "", texthl = "DiagnosticSignWarn" })
	vim.fn.sign_define("DiagnosticSignInfo", { text = "", texthl = "DiagnosticSignInfo" })
	vim.fn.sign_define("DiagnosticSignHint", { text = "", texthl = "DiagnosticSignHint" })

	-- Auto-format files prior to saving them
	-- vim.api.nvim_command[[autocmd BufWritePre <buffer> lua vim.lsp.buf.formatting_sync(nil, 1000)]]
end

local lsp_options = {
	flags = { debounce_text_changes = 150 },
	on_attach = function(client, bufnr)
		-- invoke custom user cmd so that LSP dependent lsp can be lazy loaded
		-- only after LSP is attached to buffer
		vim.api.nvim_command("doautocmd User AbstractLSPLoaded")
		---------------------
		-- NOTE: integrate with none-ls | null -ls
		-- Avoiding LSP formatting conflicts
		-- ref: https://github.com/jose-elias-alvarez/null-ls.nvim/wiki/Avoiding-LSP-formatting-conflicts
		-- 2nd red: https://github.com/neovim/nvim-lspconfig/issues/1891#issuecomment-1157964108
		-- client.server_capabilities.documentFormattingProvider = false
		-- client.server_capabilities.documentRangeFormattingProvider = false
		--------------------------
		-- lsp support on winbar
		local _navic, navic = pcall(require, "nvim-navic")
		if _navic then
			if client.server_capabilities.documentSymbolProvider then
				navic.attach(client, bufnr)
			end
		end
		-- Enable Mappings
		require("abstract.utils.map").set_plugin("neovim/nvim-lspconfig")
	end,
	capabilities = (function()
		local _capabilities = vim.lsp.protocol.make_client_capabilities()
		-- enable LSP's builtin snippet support
		_capabilities.textDocument.completion.completionItem.snippetSupport = true
		local _cmp_lsp, cmp_lsp = pcall(require, "cmp_nvim_lsp")
		if _cmp_lsp then
			return vim.tbl_deep_extend("force", _capabilities, cmp_lsp.default_capabilities())
		end
		return _capabilities
	end)(),
}

spec.config = function()
	lsp_config()
	return lsp_options
end

return spec
