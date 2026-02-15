--[[
────────────────────────────────────────────────
Plugin: mason-lspconfig.nvim
Source: https://github.com/mason-org/mason-lspconfig.nvim

Extension to mason.nvim that makes it easier to use lspconfig with mason.nvim.
────────────────────────────────────────────────
--]]


local _ = --@spec
{
    "mason-org/mason-lspconfig.nvim",
    lazy = true,
    dependencies = {
        { "neovim/nvim-lspconfig", lazy = true },
    },
}
--@end

--@setup
require("mason-lspconfig").setup({
    automatic_enable = {
        true, -- will automatically enable (vim.lsp.enable()) installed servers
        exclude = {
            "rust_analyzer",
            "ts_ls",
        },
    },

    -- A list of servers to automatically install if they're not already installed. Example: { "rust_analyzer@nightly", "lua_ls" }
    ---@type string[]
    -- ensure_installed = ensure_installed,
})
--@end
