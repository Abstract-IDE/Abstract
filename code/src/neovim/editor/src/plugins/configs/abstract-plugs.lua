--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: abstract-plugs.nvim
Source: https://github.com/Abstract-IDE/abstract-plugs.nvim

collections of neovim plugins
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
    "Abstract-IDE/abstract-plugs.nvim",
}

spec.config = function()
    local abstract = require('abs')
    abstract.window().setup()
    abstract.terminal().setup({
        height = 0.4,
        width = 0.6,
        offset_row = 0.9,
        offset_col = 0.5,
        border = "rounded",
    })
    abstract.whitespace().setup()

    --[[@rs $TERMINAL_MAPPING_SET ]]
end

return spec
