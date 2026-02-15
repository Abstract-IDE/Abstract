--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: vim-dadbod
Source: https://github.com/tpope/vim-dadbod

Dadbod is a Vim plugin for interacting with databases
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

return {
    "tpope/vim-dadbod",
    dependencies = {
        { "kristijanhusak/vim-dadbod-ui", lazy = true },
        {
            "kristijanhusak/vim-dadbod-completion",
            ft = { "sql", "mysql", "plsql" },
            lazy = true,
        },
    },
    lazy = true,
    cmd = { "DBUI", "DBUIToggle", "DBUIAddConnection", "DBUIFindBuffer" },
    init = function()
        vim.g.db_ui_use_nerd_fonts = 1
    end,
}
