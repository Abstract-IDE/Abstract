--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: hop.nvim
Source: https://github.com/smoka7/hop.nvim
        (forked of: https://github.com/phaazon/hop.nvim)

Hop is an EasyMotion-like plugin allowing you to jump
anywhere in a document with as few keystrokes as possible
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

return {
    "smoka7/hop.nvim",
    version = "*",
    keys = --[[@rs $MAPPING ]],
    opts = {
        keys = "qwertyuiopasdfghjklzxcvbnm",
        jump_on_sole_occurrence = false,
    },
}
