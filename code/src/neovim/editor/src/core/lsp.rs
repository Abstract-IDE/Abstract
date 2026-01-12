use nvim_oxi::api::{
    self,
    create_autocmd, //
    opts::{
        CreateAugroupOpts,
        CreateAutocmdOpts, //
    },
};
use wl_utils::neovim::types::events::Events;

use crate::core::keymaps;

pub struct Lsp;

impl Lsp {
    pub fn setup() -> nvim_oxi::Result<&'static str> {
        let setup_diagnostics = Self::setup_diagnostics();
        let lsp_configs = Self::get_configs();

        let keymap = keymaps::MAPPING.set_map_str(keymaps::Key::LspConfig);
        let opts = CreateAutocmdOpts::builder()
            .desc("setup map for lsp when lsp attached")
            .group(api::create_augroup("ABSTRACT_LSP", &CreateAugroupOpts::builder().clear(true).build())?)
            .patterns(["*"])
            .command(format!("lua {keymap}"))
            .build();
        create_autocmd([Events::LspAttach.as_ref()], &opts)?;

        let setup_ = format!(
            r#"
                {setup_diagnostics}

                -- hover and signature help is handled by nvim patrickpichler/hovercraft.nvim
                -- handlers = vim.lsp.handlers
                -- handlers["textDocument/hover"] = vim.lsp.with(handlers.hover, {{ border = "rounded" }})
                -- handlers["textDocument/signatureHelp"] = vim.lsp.with(handlers.signature_help, {{ border = "single" }})

                -- configure LSP clients
                for lsp, config in pairs({lsp_configs}) do
                    vim.lsp.config(lsp, config)
                end
            "#
        );
        Ok(setup_.leak())
    }

    pub fn get_configs() -> &'static str {
        r#"{
            --  Add additional capabilities to all clients
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
        }"#
    }

    pub fn setup_diagnostics() -> &'static str {
        r#"
            -- https://neovim.io/doc/user/diagnostic.html#vim.diagnostic.config()
            local severity = vim.diagnostic.severity

            vim.diagnostic.config({
                underline = true,
                update_in_insert = true,

                -- virtual_lines = {
                -- 	current_line = true, -- Only show virtual line diagnostics for the current cursor line
                -- },

                virtual_text = {
                    prefix = function(dst, index, total)
                        -- not show box if there is just one error
                        -- if total == 1 then
                        -- 	return " "
                        -- end

                        if index == 1 then
                            return "  " .. "■"
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
        "#
    }
}
