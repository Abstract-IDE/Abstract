/*
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: fff.nvim
Source: https://github.com/dmtrKovalenko/fff.nvim

Finally a smart fuzzy file picker for neovim.
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
*/

use crate::core::keymaps;

pub struct Plugin;

impl Plugin {
    pub fn spec() -> &'static str {
        let opts = Self::opts();
        let keys = keymaps::MAPPING.get_map(keymaps::Key::Fff);
        format!(
            // language=lua
            r#"{{
                "dmtrKovalenko/fff.nvim",
                build = function()
                    require("fff.download").download_or_build_binary()
                end,
                keys = {keys},
                opts = {opts},
            }}"#
        )
        .leak()
    }
}

impl Plugin {
    pub fn opts() -> &'static str {
        // language=lua
        r#"{
            base_path = vim.fn.getcwd(),
            prompt = '🔎 ',
            title = 'FFFiles',
            max_results = 100,
            max_threads = 4,
            lazy_sync = true, -- set to false if you want file indexing to start on open
            layout = {
                height = 0.8,
                width = 0.8,
                prompt_position = 'top', -- or 'top'
                preview_position = 'right', -- or 'left', 'right', 'top', 'bottom'
                preview_size = 0.5,
            },
            preview = {
                enabled = true,
                max_size = 10 * 1024 * 1024, -- Do not try to read files larger than 10MB
                chunk_size = 8192, -- Bytes per chunk for dynamic loading (8kb - fits ~100-200 lines)
                binary_file_threshold = 1024, -- amount of bytes to scan for binary content (set 0 to disable)
                imagemagick_info_format_str = '%m: %wx%h, %[colorspace], %q-bit',
                line_numbers = false,
                wrap_lines = false,
                show_file_info = true,
                filetypes = {
                    svg = { wrap_lines = true },
                    markdown = { wrap_lines = true },
                    text = { wrap_lines = true },
                },
            },
            keymaps = {
                close = '<Esc>',
                select = '<CR>',
                select_split = '<C-s>',
                select_vsplit = '<C-v>',
                select_tab = '<C-t>',
                move_up = { '<S-Tab>', '<Up>', '<C-p>' },
                move_down = {'<Tab>', '<Down>', '<C-n>' },
                preview_scroll_up = '<C-u>',
                preview_scroll_down = '<C-d>',
                toggle_debug = '<F2>',
            },
            hl = {
                border = 'FloatBorder',
                normal = 'Normal',
                cursor = 'CursorLine',
                matched = 'IncSearch',
                title = 'Title',
                prompt = 'Question',
                active_file = 'Visual',
                frecency = 'Number',
                debug = 'Comment',
            },
            frecency = {
                enabled = true,
                db_path = vim.fn.stdpath('cache') .. '/fff_nvim',
            },
            debug = {
                enabled = false, -- Set to true to show scores in the UI
                show_scores = false,
            },
            logging = {
                enabled = true,
                log_file = vim.fn.stdpath('log') .. '/fff.log',
                log_level = 'info',
            }
        }"#
    }
}
