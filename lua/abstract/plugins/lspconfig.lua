--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: nvim-lspconfig
Source: https://github.com/neovim/nvim-lspconfig

Quickstart configs for Nvim LSP
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
	"neovim/nvim-lspconfig",
	lazy = true,
}

local lsp_config = function()
	-- NOTE: previously mapping was done in on_attach function but its no longer working.
	-- Enable Mappings
	vim.api.nvim_create_autocmd("LspAttach", {
		callback = function(_args)
			require("abstract.utils.map").set_map("neovim/nvim-lspconfig", true)
		end,
	})

	-- https://neovim.io/doc/user/diagnostic.html#vim.diagnostic.config()
	local severity = vim.diagnostic.severity

	vim.diagnostic.config({
		underline = true,
		update_in_insert = true, -- Update diagnostics in Insert mode

		-- virtual_lines = {
		-- 	current_line = true, -- Only show virtual line diagnostics for the current cursor line
		-- },

		virtual_text = {
			prefix = function(_, index, total)
				if total == 1 then
					return " "
				end

				if index ~= 1 then
					return "■"
				end

				local symbols = ""
				for _ = 2, total do
					symbols = symbols .. "■"
				end

				return " " .. symbols
			end,
			current_line = true,
			severity = { severity.ERROR, severity.WARN, severity.INFO, severity.HINT },
		},

		float = {
			border = "single",
			focusable = true,
			style = "minimal",
			source = true, --- Include the diagnostic source in the message.
			header = "",
			prefix = "",
		},

		-- ●       
		signs = {
			text = { [severity.ERROR] = "", [severity.WARN] = "", [severity.INFO] = "", [severity.HINT] = "" },
			-- Highlight entire line for errors
			linehl = {
				[severity.ERROR] = "DiagnosticSignError",
				[severity.WARN] = "DiagnosticSignWarn",
				[severity.INFO] = "DiagnosticSignInfo",
				[severity.HINT] = "DiagnosticSignHint",
			},
			-- Highlight the line number for warnings
			numhl = {
				[severity.ERROR] = "DiagnosticSignError",
				[severity.WARN] = "DiagnosticSignWarn",
				[severity.INFO] = "DiagnosticSignInfo",
				[severity.HINT] = "DiagnosticSignHint",
			},
		},
	})

	-- hover and signature help is handled by nvim patrickpichler/hovercraft.nvim
	-- handlers = vim.lsp.handlers
	-- handlers["textDocument/hover"] = vim.lsp.with(handlers.hover, { border = "rounded" })
	-- handlers["textDocument/signatureHelp"] = vim.lsp.with(handlers.signature_help, { border = "single" })
	-- show diagnostic on float window(like auto complete)
	-- vim.api.nvim_command [[ autocmd CursorHold  *.lua,*.sh,*.bash,*.dart,*.py,*.cpp,*.c,js lua vim.lsp.diagnostic.show_line_diagnostics() ]]

	-- Auto-format files prior to saving them
	-- vim.api.nvim_command[[autocmd BufWritePre <buffer> lua vim.lsp.buf.formatting_sync(nil, 1000)]]
end

local hook = {
	flags = { debounce_text_changes = 150 },
	on_attach = function(client, bufnr)
		--[[
		NOTE: integrate with none-ls | null -ls
		Avoiding LSP formatting conflicts
		ref: https://github.com/jose-elias-alvarez/null-ls.nvim/wiki/Avoiding-LSP-formatting-conflicts
		     https://github.com/neovim/nvim-lspconfig/issues/1891#issuecomment-1157964108
		client.server_capabilities.documentFormattingProvider = false
		client.server_capabilities.documentRangeFormattingProvider = false
		--]]

		-- lsp support on winbar with nvim-navic
		if ABSTRACT.PLUGINS["SmiteshP/nvim-navic"].enabled then
			local _navic, navic = pcall(require, "nvim-navic")
			if _navic and client.server_capabilities.documentSymbolProvider then
				navic.attach(client, bufnr)
			end
		end
	end,
	capabilities = (function()
		local _capabilities = vim.lsp.protocol.make_client_capabilities()
		-- enable LSP's builtin snippet support
		_capabilities.textDocument.completion.completionItem.snippetSupport = true

		-- for blink.cmp
		_capabilities = require("blink.cmp").get_lsp_capabilities(_capabilities)

		-- -- for nvim.cmp
		-- local _cmp_lsp, cmp_lsp = pcall(require, "cmp_nvim_lsp")
		-- if _cmp_lsp then
		-- 	return vim.tbl_deep_extend("force", _capabilities, cmp_lsp.default_capabilities())
		-- end

		return _capabilities
	end)(),
}

spec.setup = function()
	lsp_config()
	return hook
end

return spec
