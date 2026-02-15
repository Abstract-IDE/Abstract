--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: grapple-nvim
Source: https://github.com/cbochs/grapple.nvim

Grapple is a plugin that aims to provide immediate navigation
to important files (and their last known cursor location).
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

return {
    "cbochs/grapple.nvim",
    event = { "BufReadPost", "BufNewFile" },
    cmd = "Grapple",
    opts = {
        ---@type boolean
        icons = true,
        ---@type "basename" | "relative"
        style = "basename",
    },
}
