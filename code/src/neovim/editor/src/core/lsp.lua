--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
LSP Configuration
Core LSP setup: diagnostics, client configs, and keymaps
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

--@setup

-- ── Diagnostics ──
local severity = vim.diagnostic.severity

vim.diagnostic.config({
    underline = true,
    update_in_insert = true,

    virtual_text = {
        prefix = function(_, index, _)
            if index == 1 then
                return "  " .. "■"
            end
            return "■"
        end,
        current_line = true,
        severity = { severity.ERROR, severity.WARN, severity.INFO, severity.HINT },
    },

    float = {
        border = "single",
        focusable = true,
        style = "minimal",
        source = true,
        header = "",
        prefix = "",
    },

    signs = {
        text = {
            [severity.ERROR] = "",
            [severity.WARN] = "",
            [severity.INFO] = "",
            [severity.HINT] = "",
        },
        linehl = {
            [severity.ERROR] = "DiagnosticSignError",
            [severity.WARN] = "DiagnosticSignWarn",
            [severity.INFO] = "DiagnosticSignInfo",
            [severity.HINT] = "DiagnosticSignHint",
        },
        numhl = {
            [severity.ERROR] = "DiagnosticSignError",
            [severity.WARN] = "DiagnosticSignWarn",
            [severity.INFO] = "DiagnosticSignInfo",
            [severity.HINT] = "DiagnosticSignHint",
        },
    },
})

-- ── LSP Keymaps (on attach) ──
vim.api.nvim_create_autocmd("LspAttach", {
    group = vim.api.nvim_create_augroup("ABSTRACT_LSP", { clear = true }),
    pattern = "*",
    desc = "Setup keymaps when LSP attaches",
    callback = function()
        require("which-key").add( --[[@rs $LSP_MAPPING ]])
    end,
})

-- ── LSP Client Configs ──
local configs = {
    ["*"] = {
        root_markers = { ".git" },
        capabilities = {
            textDocument = {
                semanticTokens = {
                    multilineTokenSupport = true,
                },
                completion = {
                    completionItem = {
                        snippetSupport = true,
                    },
                },
            },
        },
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
                    useLibraryCodeForTypes = true,
                },
            },
        },
    },
    ["yamlls"] = {
        settings = {
            yaml = {
                schemaStore = {
                    enable = false,
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

for lsp, config in pairs(configs) do
    vim.lsp.config(lsp, config)
end

--@end
