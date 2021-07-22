--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--
--─────────────────────────────────────────────────--
--   Plugin:    nvim-bufferline.lua
--   Github:    github.com/akinsho/nvim-bufferline.lua
--─────────────────────────────────────────────────--
--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--





--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━━━❰ configs ❱━━━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--


require('bufferline').setup {
  options = {
    numbers                 = "ordinal", -- optionas -> "none" | "ordinal" | "buffer_id" | "both",
    number_style            = "ssubscript",  -- options -> "superscript" | "" | { "none", "subscript" }, buffer_id at index 1, ordinal at index 2
    mappings = true,
    -- NOTE: this plugin is designed with this icon in mind,
    -- and so changing this is NOT recommended, this is intended
    -- as an escape hatch for people who cannot bear it for whatever reason
    indicator_icon      = '',
    buffer_close_icon   = '',
    modified_icon       = '●',
    close_icon          = '',
    left_trunc_marker   = '',
    right_trunc_marker  = '',

    max_name_length = 18,
    max_prefix_length = 15, -- prefix used when a buffer is de-duplicate
    tab_size = 18,

    show_close_icon = false,
    show_buffer_icons = true,   -- disable filetype icons for buffers
    show_buffer_close_icons = false,
    show_tab_indicators = false,

    view = "multiwindow",
    separator_style = {"", ""},
    offsets = {{filetype = "NvimTree", text = "Explorer", text_align = "left"}},
    always_show_bufferline = false


  },

  highlights = {

    fill = {
        guibg = "#21252d",
    },
    background  = {
        guifg = '#FFFFFF',
        guibg = "#21252d",
    },
    separator_selected = {
        guifg = "#060606"
    },
    separator = {
        guifg = "#141414"
    },
    close_button_selected = {
        guibg = "#21252d",
        guifg = "#F1252d",
    },

  },
}


--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━❰ end configs ❱━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--




--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━━━❰ Mappings ❱━━━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--

local keymap = vim.api.nvim_set_keymap
local options = { noremap=true, silent=true }


-- Move to previous/next
keymap('n', '}}',   ':BufferLineCycleNext<CR>',      options)
keymap('n', '{{',   ':BufferLineCyclePrev<CR>',      options)

-- Re-order to previous/next
keymap('n', '<Leader>.',    ':BufferLineMoveNext<CR>',        options)
keymap('n', '<Leader>,',    ':BufferLineMovePrev<CR>',        options)


-- Close buffer
-- nnoremap <silent>    <A-c> :BufferClose<CR>
keymap('n', '<Leader>q',    ':bd<CR>',        options)

-- Magic buffer-picking mode
keymap('n', '<Leader>?',    ':BufferLinePick<CR>',        options)

--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━❰ end Mappings ❱━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--

