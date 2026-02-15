--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: noice.nvim
Source: https://github.com/folke/noice.nvim

💥 Highly experimental plugin that completely replaces
the UI for messages, cmdline and the popupmenu.
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

return {
    "folke/noice.nvim",
    event = "VeryLazy",
    opts = {
        lsp = {
            signature = { enabled = false },
            hover = { enabled = false },
        },
        presets = {
            bottom_search = false,
            command_palette = true,
            long_message_to_split = true,
            inc_rename = false,
            lsp_doc_border = true,
        },
        health = {
            checker = false,
        },
        cmdline = {
            enabled = true,
            view = "cmdline",
        },
    },
}
