--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: renamer.nvim
Source: https://github.com/filipdutescu/renamer.nvim

VS Code-like renaming UI for Neovim, writen in Lua.
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

return {
    "filipdutescu/renamer.nvim",
    branch = "master",
    event = { "LspAttach" },
    config = function()
        local mappings_utils = require("renamer.mappings.utils")
        require("renamer").setup({
            title = "Rename",
            padding = { top = 0, left = 0, bottom = 0, right = 0 },
            min_width = 15,
            max_width = 45,
            border = true,
            border_chars = { "─", "│", "─", "│", "╭", "╮", "╯", "╰" },
            show_refs = true,
            with_qf_list = true,
            with_popup = true,
            handler = nil,
            mappings = {
                ["<c-i>"] = mappings_utils.set_cursor_to_start,
                ["<c-a>"] = mappings_utils.set_cursor_to_end,
                ["<c-e>"] = mappings_utils.set_cursor_to_word_end,
                ["<c-b>"] = mappings_utils.set_cursor_to_word_start,
                ["<c-c>"] = mappings_utils.clear_line,
                ["<c-u>"] = mappings_utils.undo,
                ["<c-r>"] = mappings_utils.redo,
            },
        })
    end,
}
