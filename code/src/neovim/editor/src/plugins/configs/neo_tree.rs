/*
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: neo-tree.nvim
Source: https://github.com/nvim-neo-tree/neo-tree.nvim

Neovim plugin to manage the file system and other tree like structures.
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
*/

use crate::core::keymaps;

pub struct Plugin;

impl Plugin {
    pub fn spec() -> &'static str {
        let opts = Self::opts();
        let keys = keymaps::KEYMAPS.get_map(keymaps::MapKey::NeoTree);

        format!(
            // language=lua
            r#"{{
                "nvim-neo-tree/neo-tree.nvim",
                branch = "v3.x",
                dependencies = {{
                    -- "3rd/image.nvim", -- Optional image support in preview window: See `# Preview Mode` for more information
                }},
                keys = {keys},
                opts = {opts},
            }}"#
        )
        .leak()
    }
}

impl Plugin {
    pub fn opts() -> &'static str {
        keymaps::MapLoader::signal(keymaps::MapKey::NeoTree);

        // language=lua
        r#"{
            source_selector = {
                winbar = true,
                statusline = true,
            },
            enable_modified_markers = true, -- Show markers for files with unsaved changes.
            enable_opened_markers = true, -- Enable tracking of opened files. Required for `components.name.highlight_opened_files`
            enable_refresh_on_write = true, -- Refresh the tree when a file is written. Only used if `use_libuv_file_watcher` is false.
            close_if_last_window = false, -- Close Neo-tree if it is the last window left in the tab
            enable_cursor_hijack = false, -- If enabled neotree will keep the cursor on the first letter of the filename when moving in the tree.
            git_status_async = true,
            -- These options are for people with VERY large git repos
            git_status_async_options = {
                batch_size = 1000, -- how many lines of git status results to process at a time
                batch_delay = 10, -- delay in ms between batches. Spreads out the workload to let other processes run.
                max_lines = 10000, -- How many lines of git status results to process. Anything after this will be dropped.
                -- Anything before this will be used. The last items to be processed are the untracked files.
            },
            popup_border_style = "rounded",
            enable_git_status = true,
            enable_diagnostics = true,
            event_handlers = {
                {
                    event = "neo_tree_popup_input_ready",
                    ---@param input NuiInput
                    handler = function(input)
                        -- enter input popup with normal mode by default.
                        vim.cmd("stopinsert")
                    end,
                },
            },
            open_files_do_not_replace_types = { "terminal", "trouble", "qf" }, -- when opening files, do not use windows containing these filetypes or buftypes
            sort_case_insensitive = false, -- used when sorting files and directories in the tree
            sort_function = nil, -- use a custom function for sorting files and directories in the tree
            -- sort_function = function (a,b)
            --       if a.type == b.type then
            --           return a.path > b.path
            --       else
            --           return a.type > b.type
            --       end
            --   end , -- this sorts files and directories descendantly
            default_component_configs = {
                container = {
                    enable_character_fade = true,
                },
                indent = {
                    indent_size = 2,
                    padding = 1, -- extra padding on left hand side
                    -- indent guides
                    with_markers = true,
                    indent_marker = "│",
                    last_indent_marker = "└",
                    highlight = "NeoTreeIndentMarker",
                    -- expander config, needed for nesting files
                    with_expanders = nil, -- if nil and file nesting is enabled, will enable expanders
                    expander_collapsed = "",
                    expander_expanded = "",
                    expander_highlight = "NeoTreeExpander",
                },
                icon = {
                    folder_closed = "",
                    folder_open = "",
                    folder_empty = "󰜌",
                    -- The next two settings are only a fallback, if you use nvim-web-devicons and configure default icons there
                    -- then these will never be used.
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
                        -- Change type
                        added = "", -- or "✚", but this is redundant info if you use git_status_colors on the name
                        modified = "", -- or "", but this is redundant info if you use git_status_colors on the name
                        deleted = "✖", -- this can only be used in the git_status source
                        renamed = "󰁕", -- this can only be used in the git_status source
                        -- Status type
                        untracked = "",
                        ignored = "",
                        unstaged = "", -- 󰄱
                        staged = "",
                        conflict = "",
                    },
                },
                -- If you don't want to use these columns, you can set `enabled = false` for each of them individually
                file_size = {
                    enabled = true,
                    required_width = 64, -- min width of window required to show this column
                },
                type = {
                    enabled = true,
                    required_width = 122, -- min width of window required to show this column
                },
                last_modified = {
                    enabled = true,
                    required_width = 88, -- min width of window required to show this column
                },
                created = {
                    enabled = true,
                    required_width = 110, -- min width of window required to show this column
                },
                symlink_target = {
                    enabled = false,
                },
            },
            -- A list of functions, each representing a global custom command
            -- that will be available in all sources (if not overridden in `opts[source_name].commands`)
            -- see `:h neo-tree-custom-commands-global`
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
                    visible = false, -- when true, they will just be displayed differently than normal items
                    hide_dotfiles = true,
                    hide_gitignored = true,
                    hide_hidden = true, -- only works on Windows for hidden files/directories
                    hide_by_name = {
                        --"node_modules"
                    },
                    hide_by_pattern = { -- uses glob style patterns
                        --"*.meta",
                        --"*/src/*/tsconfig.json",
                    },
                    always_show = { -- remains visible even if other settings would normally hide it
                        --".gitignored",
                    },
                    never_show = { -- remains hidden even if visible is toggled to true, this overrides always_show
                        --".DS_Store",
                        --"thumbs.db"
                    },
                    never_show_by_pattern = { -- uses glob style patterns
                        --".null-ls_*",
                    },
                },
                follow_current_file = {
                    enabled = true, -- This will find and focus the file in the active buffer every time
                    --               -- the current file is changed while the tree is open.
                    leave_dirs_open = false, -- `false` closes auto expanded dirs, such as with `:Neotree reveal`
                },
                group_empty_dirs = false, -- when true, empty folders will be grouped together
                hijack_netrw_behavior = "open_default", -- netrw disabled, opening a directory opens neo-tree
                -- in whatever position is specified in window.position
                -- "open_current",  -- netrw disabled, opening a directory opens within the
                -- window like netrw would, regardless of window.position
                -- "disabled",    -- netrw left alone, neo-tree does not handle opening dirs
                use_libuv_file_watcher = false, -- This will use the OS level file watchers to detect changes
                commands = {}, -- Add a custom command or override a global one using the same function name
            },
            buffers = {
                follow_current_file = {
                    enabled = true, -- This will find and focus the file in the active buffer every time
                    --              -- the current file is changed while the tree is open.
                    leave_dirs_open = false, -- `false` closes auto expanded dirs, such as with `:Neotree reveal`
                },
                group_empty_dirs = true, -- when true, empty folders will be grouped together
                show_unloaded = true,
            },
            git_status = {
                window = {
                    position = "float",
                },
            },
        }"#
    }
}
