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
	event = { "CmdlineEnter", "BufRead", "BufNewFile", "InsertEnter" },
}

---@param capabilities lsp.ClientCapabilities
---@return table<string,vim.lsp.Config>
local lsp_configs = function(capabilities)
	return {
		["*"] = {
			capabilities = capabilities,
			flags = {
				debounce_text_changes = 150,
			},
		},
		["swift"] = {
			capabilities = {
				workspace = {
					didChangeWatchedFiles = {
						dynamicRegistration = true,
					},
				},
			},
		},
		["html"] = {
			filetypes = { "html", "htmldjango" },
		},
		["jsonls"] = {
			settings = {
				json = {
					schemas = require("schemastore").json.schemas(),
					validate = { enable = true },
				},
			},
		},
		["pyright"] = {
			settings = {
				python = {
					analysis = {
						autoSearchPaths = true,
						-- diagnosticMode = "workspace", -- options: "workspace" | "openFilesOnly"
						useLibraryCodeForTypes = true,
					},
				},
			},
		},
		["yamlls"] = {
			settings = {
				yaml = {
					schemaStore = {
						-- You must disable built-in schemaStore support if you want to use
						-- this plugin and its advanced options like `ignore`.
						enable = false,
						-- Avoid TypeError: Cannot read properties of undefined (reading 'length')
						url = "",
					},
					schemas = require("schemastore").yaml.schemas(),
				},
			},
		},
		["ccls"] = {
			capabilities = {
				textDocument = {
					completion = {
						completionItem = {
							snippetSupport = true,
						},
					},
				},
			},
		},
	}
end

spec.config = function()
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

	-- Neovim's default capabilities
	local capabilities = vim.lsp.protocol.make_client_capabilities()
	capabilities.textDocument.completion.completionItem.snippetSupport = true

	-- hover and signature help is handled by nvim patrickpichler/hovercraft.nvim
	-- handlers = vim.lsp.handlers
	-- handlers["textDocument/hover"] = vim.lsp.with(handlers.hover, { border = "rounded" })
	-- handlers["textDocument/signatureHelp"] = vim.lsp.with(handlers.signature_help, { border = "single" })

	local lsp_group = require("abstract.configs.autocmd").groups.Lsp
	vim.api.nvim_create_autocmd("LspAttach", {
		group = lsp_group,
		callback = function(ctx)
			local client = vim.lsp.get_client_by_id(ctx.data.client_id)
			if client == nil then
				return
			end

			-- Enable Mappings
			require("abstract.utils.map").set_map("neovim/nvim-lspconfig", true)

			-- if client:supports_method("textDocument/completion") then
			-- 	vim.lsp.completion.enable(true, client.id, ctx.buf, { autotrigger = true })
			-- end
		end,
	})

	local user_lsp = require("override.lsp")
	-- Merge with user defined configs ("~/.config/nvim/lua/override/lsp.lua")
	local configs = vim.tbl_extend("force", lsp_configs(capabilities), user_lsp.configs)

	for lsp, config in pairs(configs) do
		vim.lsp.config(lsp, config)
	end

	require("abstract.plugins.mason").setup()
	require("abstract.plugins.none-ls").setup()
	require("abstract.plugins.mason-lspconfig").setup(user_lsp.ensure_installed) -- Mason-LspConfig
end

return spec
