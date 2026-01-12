/*
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: snacks.nvim
Source: github.com/folke/snacks.nvim

A collection of small QoL plugins for Neovim.
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
*/

use crate::core::keymaps;

pub struct Plugin;

impl Plugin {
    pub fn spec() -> &'static str {
        keymaps::MAPPING.signal(keymaps::Key::Snacks);

        let mut opts_parts = Vec::new();

        opts_parts.push(format!("notifier = {}", Self::config_notifier()));
        opts_parts.push(format!("dashboard = {}", Self::config_dashboard()));
        opts_parts.push(format!("bufdelete = {}", Self::config_bufdelete()));
        // NOTE: using fff for now.
        // opts_parts.push(format!("picker   = {}", Self::config_picker()));

        let opts_parts = opts_parts.join(",\n");

        format!(
            r#"{{
                "folke/snacks.nvim",
                priority = 1000,
                lazy = false,
                config = function() require("snacks").setup({{ {opts_parts} }}) end
            }}"#,
        )
        .leak()
    }
}

// BUFFER DELETE
// https://github.com/folke/snacks.nvim/blob/main/docs/bufdelete.md
impl Plugin {
    pub fn config_bufdelete() -> &'static str {
        keymaps::MAPPING.signal(keymaps::Key::SnacksBufDelete);
        r#"{
            enabled = true,
        }"#
    }
}

// NOTIFIER
// https://github.com/folke/snacks.nvim/blob/main/docs/notifier.md
impl Plugin {
    pub fn config_notifier() -> &'static str {
        r#"{
            enabled = true,
            timeout = 3000, -- default timeout in ms
            width = { min = 40, max = 0.4 },
            height = { min = 1, max = 0.6 },
            -- editor margin to keep free. tabline and statusline are taken into account automatically
            margin = { top = 0, right = 1, bottom = 0 },
            padding = true, -- add 1 cell of left/right padding to the notification window
            sort = { "level", "added" }, -- sort by level and time
            -- minimum log level to display. TRACE is the lowest
            -- all notifications are stored in history
            level = vim.log.levels.TRACE,
            icons = {
                error = " ",
                warn = " ",
                info = " ",
                debug = " ",
                trace = " ",
            },
            keep = function(notif)
                return vim.fn.getcmdpos() > 0
            end,

            ---@type snacks.notifier.style
            -- compact: use border for icon and title
            -- minimal: no border, only icon and message
            -- fancy: similar to the default nvim-notify style
            style = "compact",
            top_down = true, -- place notifications from top to bottom
            date_format = "%R", -- time format for notifications
            -- format for footer when more lines are available
            -- `%d` is replaced with the number of lines.
            -- only works for styles with a border
            ---@type string|boolean
            more_format = " ↓ %d lines ",
            refresh = 50, -- refresh at most every 50ms
        }"#
    }
}

// PICKER
// https://github.com/folke/snacks.nvim/blob/main/docs/picker.md
#[allow(unused)]
impl Plugin {
    pub fn config_picker() -> &'static str {
        keymaps::MAPPING.signal(keymaps::Key::SnacksPicker);

        r##"{
            prompt = "🔎 ", -- 
            sources = {},
            focus = "input",
            layout = {
                cycle = true,
                --- Use the default layout or vertical if the window is too narrow
                preset = function()
                    return vim.o.columns >= 120 and "default" or "vertical"
                end,
            },

            ---@class snacks.picker.matcher.Config
            matcher = {
                fuzzy = true, -- use fuzzy matching
                smartcase = true, -- use smartcase
                ignorecase = true, -- use ignorecase
                sort_empty = false, -- sort results when the search string is empty
                filename_bonus = true, -- give bonus for matching file names (last part of the path)
                file_pos = true, -- support patterns like `file:line:col` and `file:line`
                -- the bonusses below, possibly require string concatenation and path normalization,
                -- so this can have a performance impact for large lists and increase memory usage
                cwd_bonus = false, -- give bonus for matching files in the cwd
                frecency = false, -- frecency bonus
                history_bonus = false, -- give more weight to chronological order
            },

            sort = {
                -- default sort is by score, text length and index
                fields = { "score:desc", "#text", "idx" },
            },

            ui_select = true, -- replace `vim.ui.select` with the snacks picker
            ---@class snacks.picker.formatters.Config
            formatters = {
                text = {
                    ft = nil, ---@type string? filetype for highlighting
                },
                file = {
                    filename_first = false, -- display filename before the file path
                    truncate = 40, -- truncate the file path to (roughly) this length
                    filename_only = false, -- only show the filename
                    icon_width = 2, -- width of the icon (in characters)
                    git_status_hl = true, -- use the git status highlight group for the filename
                },
                selected = {
                    show_always = false, -- only show the selected column when there are multiple selections
                    unselected = true, -- use the unselected icon for unselected items
                },
                severity = {
                    icons = true, -- show severity icons
                    level = false, -- show severity level
                    ---@type "left"|"right"
                    pos = "left", -- position of the diagnostics
                },
            },

            ---@class snacks.picker.previewers.Config
            previewers = {
                diff = {
                    builtin = true, -- use Neovim for previewing diffs (true) or use an external tool (false)
                    cmd = { "delta" }, -- example to show a diff with delta
                },
                git = {
                    builtin = true, -- use Neovim for previewing git output (true) or use git (false)
                    args = {}, -- additional arguments passed to the git command. Useful to set pager options usin `-c ...`
                },
                file = {
                    max_size = 1024 * 1024, -- 1MB
                    max_line_length = 500, -- max line length
                    ft = nil, ---@type string? filetype for highlighting. Use `nil` for auto detect
                },
                man_pager = nil, ---@type string? MANPAGER env to use for `man` preview
            },
            ---@class snacks.picker.jump.Config
            jump = {
                jumplist = true, -- save the current position in the jumplist
                tagstack = false, -- save the current position in the tagstack
                reuse_win = false, -- reuse an existing window if the buffer is already open
                close = true, -- close the picker when jumping/editing to a location (defaults to true)
                match = false, -- jump to the first match position. (useful for `lines`)
            },
            toggles = {
                follow = "f",
                hidden = "h",
                ignored = "i",
                modified = "m",
                regex = { icon = "R", value = false },
            },

            win = {
                -- input window
                input = {
                    keys = {
                        -- to close the picker on ESC instead of going to normal mode,
                        -- add the following keymap to your config
                        -- ["<Esc>"] = { "close", mode = { "n", "i" } },
                        ["/"] = "toggle_focus",
                        ["<C-Down>"] = { "history_forward", mode = { "i", "n" } },
                        ["<C-Up>"] = { "history_back", mode = { "i", "n" } },
                        ["<C-c>"] = { "cancel", mode = "i" },
                        ["<C-w>"] = { "<c-s-w>", mode = { "i" }, expr = true, desc = "delete word" },
                        ["<CR>"] = { "confirm", mode = { "n", "i" } },
                        ["<Down>"] = { "list_down", mode = { "i", "n" } },
                        ["<Esc>"] = "cancel",
                        ["<S-CR>"] = { { "pick_win", "jump" }, mode = { "n", "i" } },
                        ["<S-Tab>"] = { "select_and_prev", mode = { "i", "n" } },
                        ["<Tab>"] = { "select_and_next", mode = { "i", "n" } },
                        -- Preview
                        ["<a-p>"] = { "toggle_preview", mode = { "i", "n" } },
                        ["<c-u>"] = { "preview_scroll_up", mode = { "i", "n" } },
                        ["<c-d>"] = { "preview_scroll_down", mode = { "i", "n" } },
                        --
                        ["<Up>"] = { "list_up", mode = { "i", "n" } },
                        ["<a-d>"] = { "inspect", mode = { "n", "i" } },
                        ["<a-f>"] = { "toggle_follow", mode = { "i", "n" } },
                        ["<a-h>"] = { "toggle_hidden", mode = { "i", "n" } },
                        ["<a-i>"] = { "toggle_ignored", mode = { "i", "n" } },
                        ["<a-m>"] = { "toggle_maximize", mode = { "i", "n" } },
                        ["<a-w>"] = { "cycle_win", mode = { "i", "n" } },
                        ["<c-a>"] = { "select_all", mode = { "n", "i" } },
                        -- ["<c-d>"] = { "list_scroll_down", mode = { "i", "n" } },
                        -- ["<c-u>"] = { "list_scroll_up", mode = { "i", "n" } },
                        ["<c-g>"] = { "toggle_live", mode = { "i", "n" } },
                        ["<c-j>"] = { "list_down", mode = { "i", "n" } },
                        ["<c-k>"] = { "list_up", mode = { "i", "n" } },
                        ["<c-n>"] = { "list_down", mode = { "i", "n" } },
                        ["<c-p>"] = { "list_up", mode = { "i", "n" } },
                        ["<c-q>"] = { "qflist", mode = { "i", "n" } },
                        ["<c-s>"] = { "edit_split", mode = { "i", "n" } },
                        ["<c-t>"] = { "tab", mode = { "n", "i" } },
                        ["<c-v>"] = { "edit_vsplit", mode = { "i", "n" } },
                        ["<c-r>#"] = { "insert_alt", mode = "i" },
                        ["<c-r>%"] = { "insert_filename", mode = "i" },
                        ["<c-r><c-a>"] = { "insert_cWORD", mode = "i" },
                        ["<c-r><c-f>"] = { "insert_file", mode = "i" },
                        ["<c-r><c-l>"] = { "insert_line", mode = "i" },
                        ["<c-r><c-p>"] = { "insert_file_full", mode = "i" },
                        ["<c-r><c-w>"] = { "insert_cword", mode = "i" },
                        ["<c-w>H"] = "layout_left",
                        ["<c-w>J"] = "layout_bottom",
                        ["<c-w>K"] = "layout_top",
                        ["<c-w>L"] = "layout_right",
                        ["?"] = "toggle_help_input",
                        ["G"] = "list_bottom",
                        ["gg"] = "list_top",
                        ["j"] = "list_down",
                        ["k"] = "list_up",
                        ["q"] = "close",
                    },
                    b = {
                        minipairs_disable = true,
                    },
                },
                -- result list window
                list = {
                    keys = {
                        ["/"] = "toggle_focus",
                        ["<2-LeftMouse>"] = "confirm",
                        ["<CR>"] = "confirm",
                        ["<Down>"] = "list_down",
                        ["<Esc>"] = "cancel",
                        ["<S-CR>"] = { { "pick_win", "jump" } },
                        ["<S-Tab>"] = { "select_and_prev", mode = { "n", "x" } },
                        ["<Tab>"] = { "select_and_next", mode = { "n", "x" } },
                        -- Preview
                        ["<a-p>"] = "toggle_preview",
                        ["<c-u>"] = "preview_scroll_up",
                        ["<c-d>"] = "preview_scroll_down",
                        --
                        ["<Up>"] = "list_up",
                        ["<a-d>"] = "inspect",
                        ["<a-f>"] = "toggle_follow",
                        ["<a-h>"] = "toggle_hidden",
                        ["<a-i>"] = "toggle_ignored",
                        ["<a-m>"] = "toggle_maximize",
                        ["<a-w>"] = "cycle_win",
                        ["<c-a>"] = "select_all",
                        -- ["<c-d>"] = "list_scroll_down",
                        -- ["<c-u>"] = "list_scroll_up",
                        ["<c-j>"] = "list_down",
                        ["<c-k>"] = "list_up",
                        ["<c-n>"] = "list_down",
                        ["<c-p>"] = "list_up",
                        ["<c-q>"] = "qflist",
                        ["<c-s>"] = "edit_split",
                        ["<c-t>"] = "tab",
                        ["<c-v>"] = "edit_vsplit",
                        ["<c-w>H"] = "layout_left",
                        ["<c-w>J"] = "layout_bottom",
                        ["<c-w>K"] = "layout_top",
                        ["<c-w>L"] = "layout_right",
                        ["?"] = "toggle_help_list",
                        ["G"] = "list_bottom",
                        ["gg"] = "list_top",
                        ["i"] = "focus_input",
                        ["j"] = "list_down",
                        ["k"] = "list_up",
                        ["q"] = "close",
                        ["zb"] = "list_scroll_bottom",
                        ["zt"] = "list_scroll_top",
                        ["zz"] = "list_scroll_center",
                    },
                    wo = {
                        conceallevel = 2,
                        concealcursor = "nvc",
                    },
                },
                -- preview window
                preview = {
                    keys = {
                        ["<Esc>"] = "cancel",
                        ["q"] = "close",
                        ["i"] = "focus_input",
                        ["<a-w>"] = "cycle_win",
                    },
                },
            },
            ---@class snacks.picker.icons
            icons = {
                files = {
                    enabled = true, -- show file icons
                    dir = "󰉋 ",
                    dir_open = "󰝰 ",
                    file = "󰈔 ",
                },
                keymaps = {
                    nowait = "󰓅 ",
                },
                tree = {
                    vertical = "│ ",
                    middle = "├╴",
                    last = "└╴",
                },
                undo = {
                    saved = " ",
                },
                ui = {
                    live = "󰐰 ",
                    hidden = "h",
                    ignored = "i",
                    follow = "f",
                    selected = "● ",
                    unselected = "○ ",
                    -- selected = " ",
                },
                git = {
                    enabled = true, -- show git icons
                    commit = "󰜘 ", -- used by git log
                    staged = "●", -- staged changes. always overrides the type icons
                    added = "",
                    deleted = "",
                    ignored = " ",
                    modified = "○",
                    renamed = "",
                    unmerged = " ",
                    untracked = "?",
                },
                diagnostics = {
                    Error = " ",
                    Warn = " ",
                    Hint = " ",
                    Info = " ",
                },
                lsp = {
                    unavailable = "",
                    enabled = " ",
                    disabled = " ",
                    attached = "󰖩 ",
                },
                kinds = {
                    Array = " ",
                    Boolean = "󰨙 ",
                    Class = " ",
                    Color = " ",
                    Control = " ",
                    Collapsed = " ",
                    Constant = "󰏿 ",
                    Constructor = " ",
                    Copilot = " ",
                    Enum = " ",
                    EnumMember = " ",
                    Event = " ",
                    Field = " ",
                    File = " ",
                    Folder = " ",
                    Function = "󰊕 ",
                    Interface = " ",
                    Key = " ",
                    Keyword = " ",
                    Method = "󰊕 ",
                    Module = " ",
                    Namespace = "󰦮 ",
                    Null = " ",
                    Number = "󰎠 ",
                    Object = " ",
                    Operator = " ",
                    Package = " ",
                    Property = " ",
                    Reference = " ",
                    Snippet = "󱄽 ",
                    String = " ",
                    Struct = "󰆼 ",
                    Text = " ",
                    TypeParameter = " ",
                    Unit = " ",
                    Unknown = " ",
                    Value = " ",
                    Variable = "󰀫 ",
                },
            },
            ---@class snacks.picker.db.Config
            db = {
                -- path to the sqlite3 library
                -- If not set, it will try to load the library by name.
                -- On Windows it will download the library from the internet.
                sqlite3_path = nil, ---@type string?
            },
            ---@class snacks.picker.debug
            debug = {
                scores = false, -- show scores in the list
                leaks = false, -- show when pickers don't get garbage collected
                explorer = false, -- show explorer debug info
                files = false, -- show file debug info
                grep = false, -- show file debug info
                proc = false, -- show proc debug info
                extmarks = false, -- show extmarks errors
            },
        }"##
    }
}

// DASHBOARD
// https://github.com/folke/snacks.nvim/blob/main/docs/dashboard.md
impl Plugin {
    pub fn config_dashboard() -> &'static str {
        r#"{
            enabled = true,
            width = 40,
            sections = function()
                local header = function()
                    local _header = ""
                    _header = _header .. "┃█████       " .. "\n"
                    _header = _header .. "┃██ ██      " .. "\n"
                    _header = _header .. "┃██  ██     " .. "\n"
                    _header = _header .. "┃██ ████████  " .. "\n"
                    _header = _header .. "┃██    ██   " .. "\n"
                    _header = _header .. "┃██     ██  " .. "\n"
                    return _header
                end

                local info = function()
                    local _datetime, datetime = pcall(os.date, " %I:%M:%p (%d-%m-%Y)")
                    local version = vim.version()
                    local nvim_verion = string.format("v%d.%d.%d", version.major, version.minor, version.patch)
                    if _datetime then
                        return nvim_verion .. " | " .. datetime
                    end
                    return nvim_verion
                end

                local session = function()
                    vim.cmd([[SessionManager load_session]])
                end

                -- stylua: ignore
                return {
                    { align = "center", text = { header() } },
                    {
                        gap = 0,
                        indent = 0,
                        padding = 4,
                        { icon = " ", key = "n", desc = "New File", action = ":ene | startinsert" },
                        { icon = " ", key = "r", desc = "Recent Files", action = ":lua Snacks.dashboard.pick('oldfiles')" },
                        { icon = " ", key = "s", desc = "Sessions", action = session },
                        { icon = " ", key = "f", desc = "Find File", action = ":lua Snacks.dashboard.pick('files')" },
                        { icon = " ", key = "g", desc = "Find Text", action = ":lua Snacks.dashboard.pick('live_grep')" },
                        { icon = "󰒲 ", key = "L", desc = "Lazy", action = ":Lazy", enabled = package.loaded.lazy ~= nil },
                        { icon = " ", key = "q", desc = "Quit", action = ":qa" },
                    },
                    {
                        align = "center",
                        gap = 0,
                        { section = "startup" },
                        { text = { info() } },
                    }
                }
            end,
        }"#
    }
}
