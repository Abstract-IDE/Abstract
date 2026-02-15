--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: fidget.nvim
Source: https://github.com/j-hui/fidget.nvim

💫 Extensible UI for Neovim notifications and LSP progress messages.
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

return {
    "j-hui/fidget.nvim",
    lazy = true,
    event = { "LspAttach" },
    opts = {
        notification = {
            window = {
                winblend = 100,
            }
        }
    }
}
