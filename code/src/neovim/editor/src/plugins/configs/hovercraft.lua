--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: hovercraft.nvim
Source: https://github.com/patrickpichler/hovercraft.nvim

hovercraft.nvim is a plug and play framework for writing custom hover provider.
It brings a few providers out of the box, such as a LSP, as well as a Dictionary.
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
    "patrickpichler/hovercraft.nvim",
    lazy = true,
    --[[@rs keys=$MAPPING, ]]
}

spec.config = function()
    return {
        providers = {
            providers = {
                { "LSP",        require("hovercraft.provider.lsp.hover").new() },
                { "Man",        require("hovercraft.provider.man").new() },
                { "Dictionary", require("hovercraft.provider.dictionary").new() },
            },
        },

        window = {
            border = "rounded",
        },

        keys = {
            { "<C-u>",   function() require("hovercraft").scroll({ delta = -2 }) end },
            { "<C-d>",   function() require("hovercraft").scroll({ delta = 2 }) end },
            { "<TAB>",   function() require("hovercraft").hover_next() end },
            { "<S-TAB>", function() require("hovercraft").hover_next({ step = -1 }) end },
        },
    }
end

return spec
