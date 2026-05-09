--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: fff.nvim
Source: https://github.com/dmtrKovalenko/fff.nvim

Finally a smart fuzzy file picker for neovim.
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
    "dmtrKovalenko/fff.nvim",
    build = function()
        require("fff.download").download_or_build_binary()
    end,
    --[[@rs keys = $MAPPING, ]]
}

spec.opts = {
    base_path = vim.fn.getcwd(),
    prompt = '🔎 ',
    title = 'Find files',
    max_results = 100,
    max_threads = 4,
    lazy_sync = true,
    layout = {
        height = 0.9,
        width = 0.9,
        prompt_position = 'top',
        preview_position = 'right', -- or 'left', 'right', 'top', 'bottom'
        preview_size = 0.5,
        show_scrollbar = false,     -- Show scrollbar for pagination
        flex = {                    -- set to false to disable flex layout
            size = 70,              -- column threshold: if screen width >= size, use preview_position; otherwise use wrap
            wrap = 'bottom',        -- position to use when screen is narrower than size
        },
    },
    preview = {
        enabled = true,
        max_size = 10 * 1024 * 1024,
        chunk_size = 8192,
        binary_file_threshold = 1024,
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
        move_down = { '<Tab>', '<Down>', '<C-n>' },
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
        enabled = false,
        show_scores = false,
    },
    logging = {
        enabled = true,
        log_file = vim.fn.stdpath('log') .. '/fff.log',
        log_level = 'info',
    }
}

return spec
