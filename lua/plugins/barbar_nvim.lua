--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--
--─────────────────────────────────────────────────--
--   Plugin:    barbar.nvim
--   Github:    github.com/romgrk/barbar.nvim
--─────────────────────────────────────────────────--
--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--





--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━━━❰ configs ❱━━━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━❰ end configs ❱━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--









--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━━━❰ Mappings ❱━━━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--

local keymap = vim.api.nvim_set_keymap
local options = { noremap=true, silent=true }


-- Move to previous/next
keymap('n', '<C-Tab>',      ':BufferNext<CR>',          options)
keymap('n', '<S-C-Tab>',    ':BufferPrevious<CR>',      options)
-- Goto buffer in position...
keymap('n', '<Leader>1',    ':BufferGoto 1<CR>',        options)
keymap('n', '<Leader>2',    ':BufferGoto 2<CR>',        options)
keymap('n', '<Leader>3',    ':BufferGoto 3<CR>',        options)
keymap('n', '<Leader>4',    ':BufferGoto 4<CR>',        options)
keymap('n', '<Leader>5',    ':BufferGoto 5<CR>',        options)
keymap('n', '<Leader>6',    ':BufferGoto 6<CR>',        options)
keymap('n', '<Leader>7',    ':BufferGoto 7<CR>',        options)
keymap('n', '<Leader>8',    ':BufferGoto 8<CR>',        options)
keymap('n', '<Leader>9',    ':BufferGoto 9<CR>',        options)

-- Re-order to previous/next
keymap('n', '<Leader>,',    ':BufferMovePrevious 9<CR>',        options)
keymap('n', '<Leader>.',    ':BufferMoveNext 9<CR>',        options)


-- Close buffer
-- nnoremap <silent>    <A-c> :BufferClose<CR>
keymap('n', '<Leader>q',    ':BufferClose<CR>',        options)

-- Magic buffer-picking mode
keymap('n', '<Leader>m',    ':BufferPick<CR>',        options)


--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━❰ end Mappings ❱━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--

