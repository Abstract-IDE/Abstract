--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: nvim-surround
Source: https://github.com/kylechui/nvim-surround

Add/change/delete surrounding delimiter pairs with ease.
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

return {
    "kylechui/nvim-surround",
    version = "*",
    keys = { "c", "s" },
    opts = {
        keymaps = {
            visual = "S",
            delete = "ds",
            change = "cs",
            insert = "<C-g>s",
        },
        surrounds = {
            ["("] = { add = { "( ", " )" } },
            [")"] = { add = { "(", ")" } },
            ["{"] = { add = { "{ ", " }" } },
            ["}"] = { add = { "{", "}" } },
            ["<"] = { add = { "< ", " >" } },
            [">"] = { add = { "<", ">" } },
            ["["] = { add = { "[ ", " ]" } },
            ["]"] = { add = { "[", "]" } },
            ["'"] = { add = { "'", "'" } },
            ['"'] = { add = { '"', '"' } },
            ["`"] = { add = { "`", "`" } },
        },
        aliases = {
            ["a"] = ">",
            ["b"] = ")",
            ["B"] = "}",
            ["r"] = "]",
            ["q"] = { '"', "'", "`" },
            ["s"] = { ")", "]", "}", ">", "'", '"', "`" },
        },
        highlight = {
            duration = 0,
        },
        move_cursor = "begin",
    },
}
