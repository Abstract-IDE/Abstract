--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: markview.nvim
Source: https://github.com/OXY2DEV/markview.nvim
A hackable markdown, Typst, latex, html(inline) & YAML previewer for Neovim
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
    "OXY2DEV/markview.nvim",
    lazy = false,
}

spec.config = function()
    require("markview").setup({
        -- experimental = {
        --     check_rtp = false
        -- },
        -- html = { enable = false },
        -- latex = { enable = false },
        -- markdown = { enable = false },
        -- markdown_inline = { enable = false },
        -- typst = { enable = false },
        -- yaml = { enable = false },
    })
    --[[@rs $MAPPING_SET ]]
end

return spec
