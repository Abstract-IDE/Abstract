--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: neo-tree.nvim
Source: https://github.com/nvim-neo-tree/neo-tree.nvim
Neovim plugin to manage the file system and other tree like structures.
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
    "nvim-neo-tree/neo-tree.nvim",
    branch = "v3.x",
    lazy = false, -- neo-tree will lazily load itself
}

spec.config = function()
    require("neo-tree").setup({
        default_component_configs = {
            icon = {
                folder_closed = "",
                folder_open = "",
                folder_empty = "󰜌",
                default = "*",
                highlight = "NeoTreeFileIcon",
                use_filtered_colors = true, -- Whether to use a different highlight when the file is filtered (hidden, dotfile, etc.).
            },
            modified = {
                symbol = "",
                highlight = "NeoTreeModified",
            },
            git_status = {
                symbols = {
                    -- Change type
                    added = "✚", -- or "✚"
                    modified = "", -- or ""
                    deleted = "✖", -- this can only be used in the git_status source
                    renamed = "󰁕", -- this can only be used in the git_status source
                    -- Status type
                    untracked = "",
                    ignored = "",
                    unstaged = "󰄱",
                    staged = "",
                    conflict = "",
                },
            },
        },
        -- A list of functions, each representing a global custom command
        -- that will be available in all sources (if not overridden in `opts[source_name].commands`)
        -- see `:h neo-tree-custom-commands-global`
        commands = {},
        window = {
            position = "left",
            width = 30,
        },

        filesystem = {
            follow_current_file = {
                enabled = true,                     -- This will find and focus the file in the active buffer every time
            },
            hijack_netrw_behavior = "open_default", -- netrw disabled, opening a directory opens neo-tree
            -- "open_current",  -- netrw disabled, opening a directory opens within the
        },
        buffers = {
            follow_current_file = {
                enabled = true,          -- This will find and focus the file in the active buffer every time
                -- the current file is changed while the tree is open.
                leave_dirs_open = false, -- `false` closes auto expanded dirs, such as with `:Neotree reveal`
            },
            show_unloaded = true,
        },
    })
    --[[@rs $MAPPING_SET ]]
end

return spec
