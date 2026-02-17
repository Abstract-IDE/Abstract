--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: rustaceanvim
Source: https://github.com/mrcjkb/rustaceanvim

Supercharge your Rust experience in Neovim!
A heavily modified fork of rust-tools.nvim
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
    "mrcjkb/rustaceanvim",
    version = "^6",
    lazy = false,
}

spec.config = function()
    vim.g.rustaceanvim = {
        -- Plugin configuration
        tools = {
            float_win_config = {
                border = "rounded",
            },
        },

        -- LSP configuration
        server = {
            on_attach = function(client, bufnr)
                -- you can also put keymaps in here
            end,
            default_settings = {
                -- rust-analyzer language server configuration
                ['rust-analyzer'] = {
                    procMacro = {
                        ignored = {
                            -- Leptos
                            -- https://book.leptos.dev/getting_started/leptos_dx.html#2-editor-autocompletion-inside-component-and-server
                            leptos_macro = {
                                -- optional: --
                                -- "component",
                                "server",
                            },
                        },
                    },
                    cargo = {
                        -- Leptos
                        -- https://book.leptos.dev/getting_started/leptos_dx.html#3-enable-features-in-rust-analyzer-for-your-editor-optional
                        features = "all", -- Enable all features
                    },
                },
            },
        },
        -- DAP configuration
        dap = {},
    }
end

return spec
