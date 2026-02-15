--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: goto-preview
Source: https://github.com/rmagatti/goto-preview
A small Neovim plugin for previewing native LSP's goto definition, type definition,
implementation, declaration and references calls in floating windows.
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
    "rmagatti/goto-preview",
    event = { "LspAttach" },
}

spec.config = function()
    require("goto-preview").setup({
        width = 80,
        height = 15,
        border = { "↖", "─", "┐", "│", "┘", "─", "└", "│" },
        default_mappings = false,
        debug = false,
        opacity = nil,
        resizing_mappings = false,
        post_close_hook = nil,
        references = {},
        focus_on_open = true,
        dismiss_on_move = false,
        force_close = true,
        bufhidden = "wipe",
        stack_floating_preview_windows = true,
        preview_window_title = { enable = true, position = "left" },
        zindex = 1,
    })
    --[[@rs $MAPPING_SET ]]
end

return spec
