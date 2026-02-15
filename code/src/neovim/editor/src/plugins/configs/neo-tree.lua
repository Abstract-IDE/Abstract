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
    dependencies = {},
}

spec.config = function()
    require("neo-tree").setup({
        source_selector = {
            winbar = true,
            statusline = true,
        },
        enable_modified_markers = true,
        enable_opened_markers = true,
        enable_refresh_on_write = true,
        close_if_last_window = false,
        enable_cursor_hijack = false,
        git_status_async = true,
        git_status_async_options = {
            batch_size = 1000,
            batch_delay = 10,
            max_lines = 10000,
        },
        popup_border_style = "rounded",
        enable_git_status = true,
        enable_diagnostics = true,
        event_handlers = {
            {
                event = "neo_tree_popup_input_ready",
                ---@param input NuiInput
                handler = function(input)
                    vim.cmd("stopinsert")
                end,
            },
        },
        open_files_do_not_replace_types = { "terminal", "trouble", "qf" },
        sort_case_insensitive = false,
        sort_function = nil,
        default_component_configs = {
            container = {
                enable_character_fade = true,
            },
            indent = {
                indent_size = 2,
                padding = 1,
                with_markers = true,
                indent_marker = "│",
                last_indent_marker = "└",
                highlight = "NeoTreeIndentMarker",
                with_expanders = nil,
                expander_collapsed = "",
                expander_expanded = "",
                expander_highlight = "NeoTreeExpander",
            },
            icon = {
                folder_closed = "",
                folder_open = "",
                folder_empty = "󰜌",
                default = "*",
                highlight = "NeoTreeFileIcon",
            },
            modified = {
                symbol = "●",
                highlight = "NeoTreeModified",
            },
            name = {
                trailing_slash = false,
                use_git_status_colors = true,
                highlight = "NeoTreeFileName",
            },
            git_status = {
                symbols = {
                    added = "",
                    modified = "",
                    deleted = "✖",
                    renamed = "󰁕",
                    untracked = "",
                    ignored = "",
                    unstaged = "",
                    staged = "",
                    conflict = "",
                },
            },
            file_size = {
                enabled = true,
                required_width = 64,
            },
            type = {
                enabled = true,
                required_width = 122,
            },
            last_modified = {
                enabled = true,
                required_width = 88,
            },
            created = {
                enabled = true,
                required_width = 110,
            },
            symlink_target = {
                enabled = false,
            },
        },
        commands = {},
        window = {
            position = "left",
            width = 30,
            mapping_options = {
                noremap = true,
                nowait = true,
            },
        },
        nesting_rules = {},
        filesystem = {
            filtered_items = {
                visible = false,
                hide_dotfiles = true,
                hide_gitignored = true,
                hide_hidden = true,
                hide_by_name = {},
                hide_by_pattern = {},
                always_show = {},
                never_show = {},
                never_show_by_pattern = {},
            },
            follow_current_file = {
                enabled = true,
                leave_dirs_open = false,
            },
            group_empty_dirs = false,
            hijack_netrw_behavior = "open_default",
            use_libuv_file_watcher = false,
            commands = {},
        },
        buffers = {
            follow_current_file = {
                enabled = true,
                leave_dirs_open = false,
            },
            group_empty_dirs = true,
            show_unloaded = true,
        },
        git_status = {
            window = {
                position = "float",
            },
        },
    })
    --[[@rs $MAPPING_SET ]]
end

return spec
