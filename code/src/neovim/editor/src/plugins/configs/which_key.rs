/*
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: which-key
Source: https://github.com/folke/which-key.nvim

WhichKey is a lua plugin for Neovim 0.5 that displays a popup
with possible key bindings of the command you started typing.
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
*/

use crate::core::keymaps::{
    KEYMAPS,
    MapKey, //
};

pub struct Plugin;

impl Plugin {
    pub fn spec() -> &'static str {
        let opts = Self::opts();
        let spec = format!(
            r#"{{
                "folke/which-key.nvim",
                event = "VeryLazy",
                keys = {},
                opts = {},
            }}"#,
            KEYMAPS.get_map(MapKey::WhichKey),
            opts,
        );

        Box::leak(spec.into_boxed_str())
    }
}

impl Plugin {
    pub fn opts() -> &'static str {
        // language=lua
        r#"{
            ---@type false | "classic" | "modern" | "helix"
            preset = "modern",
            -- Delay before showing the popup. Can be a number or a function that returns a number.
            ---@type number | fun(ctx: { keys: string, mode: string, plugin?: string }):number
            delay = function(ctx)
                return ctx.plugin and 0 or 200
            end,
            ---@param mapping wk.Mapping
            filter = function(mapping)
                -- example to exclude mappings without a description
                -- return mapping.desc and mapping.desc ~= ""
                return true
            end,
            --- You can add any mappings here, or use `require('which-key').add()` later
            ---@type wk.Spec
            spec = {},
            -- show a warning when issues were detected with your mappings
            notify = true,
            -- Which-key automatically sets up triggers for your mappings.
            -- But you can disable this and setup the triggers manually.
            -- Check the docs for more info.
            ---@type wk.Spec
            triggers = {
                { "<auto>", mode = "nixsotc" },
            },
            -- Start hidden and wait for a key to be pressed before showing the popup
            -- Only used by enabled xo mapping modes.
            ---@param ctx { mode: string, operator: string }
            defer = function(ctx)
                return ctx.mode == "V" or ctx.mode == "<C-V>"
            end,
            plugins = {
                marks = true, -- shows a list of your marks on ' and `
                registers = true, -- shows your registers on " in NORMAL or <C-r> in INSERT mode
                -- the presets plugin, adds help for a bunch of default keybindings in Neovim
                -- No actual key bindings are created
                spelling = {
                    enabled = true, -- enabling this will show WhichKey when pressing z= to select spelling suggestions
                    suggestions = 20, -- how many suggestions should be shown in the list?
                },
                presets = {
                    operators = true, -- adds help for operators like d, y, ...
                    motions = true, -- adds help for motions
                    text_objects = true, -- help for text objects triggered after entering an operator
                    windows = true, -- default bindings on <c-w>
                    nav = true, -- misc bindings to work with windows
                    z = true,   -- bindings for folds, spelling and others prefixed with z
                    g = true,   -- bindings for prefixed with g
                },
            },
            ---@type wk.Win.opts
            win = {
                -- don't allow the popup to overlap with the cursor
                no_overlap = true,
                -- width = 1,
                -- height = { min = 4, max = 25 },
                -- col = 0,
                -- row = math.huge,
                -- border = "none",
                padding = { 1, 2 }, -- extra window padding [top/bottom, right/left]
                title = true,
                title_pos = "center",
                zindex = 1000,
                -- Additional vim.wo and vim.bo options
                bo = {},
                wo = {
                    -- winblend = 10, -- value between 0-100 0 for fully opaque and 100 for fully transparent
                },
            },
            layout = {
                width = { min = 20 }, -- min and max width of the columns
                spacing = 3,    -- spacing between columns
            },
            keys = {
                scroll_down = "<c-d>", -- binding to scroll down inside the popup
                scroll_up = "<c-u>", -- binding to scroll up inside the popup
            },
            ---@type (string|wk.Sorter)[]
            --- Mappings are sorted using configured sorters and natural sort of the keys
            --- Available sorters:
            --- * local: buffer-local mappings first
            --- * order: order of the items (Used by plugins like marks / registers)
            --- * group: groups last
            --- * alphanum: alpha-numerical first
            --- * mod: special modifier keys last
            --- * manual: the order the mappings were added
            --- * case: lower-case first
            sort = { "local", "order", "group", "alphanum", "mod" },
            ---@type number|fun(node: wk.Node):boolean?
            expand = 1, -- expand groups when <= n mappings
            -- expand = function(node)
            --   return not node.desc -- expand all nodes without a description
            -- end,
            -- Functions/Lua Patterns for formatting the labels
            ---@type table<string, ({[1]:string, [2]:string}|fun(str:string):string)[]>
            replace = {
                key = {
                    function(key)
                        return require("which-key.view").format(key)
                    end,
                    -- { "<Space>", "SPC" },
                },
                desc = {
                    { "<Plug>%(?(.*)%)?", "%1" },
                    { "^%+",              "" },
                    { "<[cC]md>",         "" },
                    { "<[cC][rR]>",       "" },
                    { "<[sS]ilent>",      "" },
                    { "^lua%s+",          "" },
                    { "^call%s+",         "" },
                    { "^:%s*",            "" },
                },
            },
            icons = {
                breadcrumb = "»", -- symbol used in the command line area that shows your active key combo
                separator = "➜", -- symbol used between a key and it's label
                group = "+", -- symbol prepended to a group
                ellipsis = "…",
                -- set to false to disable all mapping icons,
                -- both those explicitely added in a mapping
                -- and those from rules
                mappings = false,
                --- See `lua/which-key/icons.lua` for more details
                --- Set to `false` to disable keymap icons from rules
                ---@type wk.IconRule[]|false
                rules = {},
                -- use the highlights from mini.icons
                -- When `false`, it will use `WhichKeyIcon` instead
                colors = true,
                -- used by key format
                keys = {
                    Up = " ",
                    Down = " ",
                    Left = " ",
                    Right = " ",
                    C = "󰘴 ",
                    M = "󰘵 ",
                    D = "󰘳 ",
                    S = "󰘶 ",
                    CR = "󰌑 ",
                    Esc = "󱊷 ",
                    ScrollWheelDown = "󱕐 ",
                    ScrollWheelUp = "󱕑 ",
                    NL = "󰌑 ",
                    BS = "󰁮",
                    Space = "󱁐 ",
                    Tab = "󰌒 ",
                    F1 = "󱊫",
                    F2 = "󱊬",
                    F3 = "󱊭",
                    F4 = "󱊮",
                    F5 = "󱊯",
                    F6 = "󱊰",
                    F7 = "󱊱",
                    F8 = "󱊲",
                    F9 = "󱊳",
                    F10 = "󱊴",
                    F11 = "󱊵",
                    F12 = "󱊶",
                },
            },
            show_help = true, -- show a help message in the command line for using WhichKey
            show_keys = true, -- show the currently pressed key and its label as a message in the command line
            -- disable WhichKey for certain buf types and file types.
            disable = {
                ft = {},
                bt = {},
            },
            debug = false, -- enable wk.log in the current directory
        }"#
    }
}
