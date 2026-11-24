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

--- The rust-analyzer to run for the current buffer's project.
---
--- Prefers the one that ships with the project's own toolchain
--- (`<sysroot>/bin/rust-analyzer`: a Nix dev shell with the `rust-analyzer`
--- extension, or a rustup toolchain with the `rust-analyzer` component) over
--- whatever is first on `$PATH`, which is usually Mason's latest release.
---
--- A rust-analyzer much newer than the toolchain cannot read that toolchain's
--- standard library sources, and then reports std macros (`assert_eq!`,
--- `format!`, `vec!`, ...) as unresolved all over a project that compiles fine.
---@return string
local function rust_analyzer_bin()
    -- Ask from the buffer's directory, so rustup honours `rust-toolchain.toml`.
    local dir = vim.fn.expand("%:p:h")
    local cwd = vim.fn.isdirectory(dir) == 1 and dir or nil
    local ok, result = pcall(function()
        return vim.system({ "rustc", "--print", "sysroot" }, { cwd = cwd, text = true }):wait(5000)
    end)
    if ok and result.code == 0 then
        local bin = vim.trim(result.stdout or "") .. "/bin/rust-analyzer"
        if vim.fn.executable(bin) == 1 then
            return bin
        end
    end
    return "rust-analyzer"
end

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
            cmd = function()
                return { rust_analyzer_bin() }
            end,
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
