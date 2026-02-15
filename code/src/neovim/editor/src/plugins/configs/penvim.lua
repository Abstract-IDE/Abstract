--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: penvim
Source: https://github.com/Abstract-IDE/penvim

project's root directory and documents indentation detector with project based config loader
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

return {
    "Abstract-IDE/penvim",
    opts = {
        project_env = {
            enable = true,
            config_name = ".__nvim__.lua",
        },
        rooter = {
            enable = true,
            patterns = { ".__nvim__.lua" },
        },
        indentor = {
            enable = true,
            indent_length = 4,
            indent_type = "auto",
        },
    },
}
