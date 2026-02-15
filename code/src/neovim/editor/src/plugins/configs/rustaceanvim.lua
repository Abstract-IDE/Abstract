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

return {
    "mrcjkb/rustaceanvim",
    version = "^6",
    lazy = false,
    config = function()
        vim.g.rustaceanvim = {
            tools = {
                float_win_config = {
                    border = "rounded",
                },
            },
            dap = {},
        }
    end,
}
