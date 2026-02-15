--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: nvim-bqf
Source: https://github.com/kevinhwang91/nvim-bqf

Better quickfix window in Neovim, polish old quickfix window.
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

return {
    'kevinhwang91/nvim-bqf',
    lazy = false,
    opts = {
        auto_enable = true,
        auto_resize_height = true,
        preview = {
            delay_syntax = 400,
            show_title = true,
        },
        -- make `drop` and `tab drop` to become preferred
        func_map = {
            drop = "o",
            openc = "O",
            split = "<C-s>",
            tabdrop = "<C-t>",
            tabc = "",
            ptogglemode = "z,",
        },
        filter = {
            fzf = {
                action_for = { ["ctrl-s"] = "split", ["ctrl-t"] = "tab drop" },
                extra_opts = { "--bind", "ctrl-o:toggle-all", "--prompt", "> " },
            },
        },
    },
}
