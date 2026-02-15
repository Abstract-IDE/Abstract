--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: csvview.nvim
Source: https://github.com/hat0uma/csvview.nvim

lightweight CSV file viewer plugin for Neovim.
With this plugin, you can easily view and edit CSV files within Neovim.
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

return {
    'hat0uma/csvview.nvim',
    lazy = true,
    ft = "csv",
    opts = {
        view = {
            ---@type integer
            min_column_width = 5,
            ---@type integer
            spacing = 2,
            ---@type "highlight" | "border"
            display_mode = "highlight",
        },
    }
}
