--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: Abstract-cs
Source: https://github.com/Abstract-IDE/Abstract-cs

Colorscheme for (neo)vim written in lua,
specially made for roshnivim with Tree-sitter support.
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
    "Abstract-IDE/Abstract-cs",
    branch = "rewrite-2",
    lazy = false,
    priority = 1000,
}


spec.config = function()
    require("abstract_cs").setup()
end

return spec
