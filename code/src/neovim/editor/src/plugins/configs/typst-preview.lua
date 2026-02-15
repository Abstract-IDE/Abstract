--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: typst-preview.nvim
Source: https://github.com/chomosuke/typst-preview.nvim

The Neovim plugin for tinymist.
Low latency preview with cross jump between code and preview.
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

return {
    "chomosuke/typst-preview.nvim",
    version = "1.*",
    lazy = true,
    ft = { "typst" },
    event = { "LspAttach" },
    opts = {
        debug = false,
        open_cmd = nil,
        port = 36063,
        invert_colors = "never",
        follow_cursor = true,
        dependencies_bin = {
            ['tinymist'] = 'tinymist',
            ["websocat"] = nil,
        },
        extra_args = nil,
        get_root = function(path_of_main_file)
            local root = os.getenv("TYPST_ROOT")
            if root then
                return root
            end
            return vim.fn.fnamemodify(path_of_main_file, ":p:h")
        end,
        get_main_file = function(path_of_buffer)
            return path_of_buffer
        end,
    },
}
