--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: mason.nvim
Source: https://github.com/mason-org/mason.nvim

Git Graph plugin for neovim.
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local M = {
    "mason-org/mason.nvim",
    lazy = true,
    event = { "CmdlineEnter", "BufRead", "BufNewFile", "InsertEnter" },
    dependencies = {
        --[[@rs $NONE_LS_SPEC, ]]
        --[[@rs $LSPCONFIG_SPEC, ]]
        --[[@rs $NULL_LS_SPEC, ]]
        --[[@rs $NVIM_DAP_SPEC, ]]
    },
}

M.config = function()
    require("mason").setup({
        log_level = vim.log.levels.INFO,
        max_concurrent_installers = 4,
        ui = {
            check_outdated_packages_on_open = true,
            border = "rounded",
            width = 0.8,
            height = 0.9,
            icons = {
                package_installed = "✓",
                package_pending = "➜",
                package_uninstalled = "✗",
            },
        },
    })

    -- === must be loaded after mason ===
    -- WARN! order matters
    --[[@rs $LSP_SETUP ]]
    --[[@rs $LSPCONFIG_SETUP ]]
    --[[@rs $MASON_NULL_LS_SETUP ]]
    --[[@rs $NVIM_DAP_SETUP ]]
    --[[@rs $NONE_LS_SETUP ]]
end

return M
