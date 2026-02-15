--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: LuaSnip
Source: https://github.com/L3MON4D3/LuaSnip

Snippet Engine for Neovim written in Lua.
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

return {
    "L3MON4D3/LuaSnip",
    version = "v2.*",
    build = "make install_jsregexp",
    lazy = true,
    dependencies = {
        { "rafamadriz/friendly-snippets", lazy = true },
        { "Neevash/awesome-flutter-snippets", lazy = true, ft = "dart" },
    },
    config = function()
        local luasnip = require("luasnip")

        luasnip.config.set_config({
            history = false,
            update_events = "TextChanged,TextChangedI",
            region_check_events = "CursorMoved",
        })

        luasnip.filetype_extend("javascriptreact", { "html" })
        luasnip.filetype_extend("typescriptreact", { "html" })
        luasnip.filetype_extend("htmldjango", { "html" })

        local from_vscode = require("luasnip.loaders.from_vscode")
        from_vscode.lazy_load()
    end
}
