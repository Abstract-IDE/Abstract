--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: nvim-colorizer.lua
Source: https://github.com/catgoose/nvim-colorizer.lua

A high-performance color highlighter for Neovim which has
no external dependencies! Written in performant Luajit.

NOTE:
Originally NvChad (https://github.com/NvChad/nvim-colorizer.lua)
forked it from https://github.com/norcalli/nvim-colorizer.lua
now it moved to https://github.com/catgoose/nvim-colorizer.lua
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

return {
    "catgoose/nvim-colorizer.lua",
    event = { "BufReadPre", "BufNewFile", "InsertEnter" },
    opts = {
        user_default_options = {
            names = false,
            RRGGBBAA = true,
            AARRGGBB = true,
            rgb_fn = true,
            hsl_fn = true,
            mode = "background",
            virtualtext = "■",
        },

        -- all the sub-options of filetypes apply to buftypes
        filetypes = {
            "*",
            css = { rgb_fn = true, names = true },
        },

        buftypes = {},
    }
}
