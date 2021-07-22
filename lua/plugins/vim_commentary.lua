-- --------------------------------------------------------------------
--           tpope/vim-commentary
-- --------------------------------------------------------------------

local keymap = vim.api.nvim_set_keymap


-- for custom file support
vim.cmd([[ autocmd FileType kivy setlocal commentstring=#\ %s ]])

-- Visual mode
keymap('x', 'cm', "<Plug>Commentary", {silent=true})
-- Normal mode
keymap('n', 'cm', "<Plug>Commentary", {silent=true})
-- Operator pending mode (this lets you do e.g. `dmc` to delete a block of comments)
keymap('o', 'cm', "<Plug>Commentary", {silent=true})

-- Normal mode, current line
keymap('n', 'cmc', "<Plug>CommentaryLine", {silent=true})

-- Special case cgc (you can skip this one, but then `cmc` will also delete an extra blank line)
-- nmap cmc <Plug>ChangeCommentary
-- ---------------------------------------------------------------------
--       end of tpope/vim-commentary
-- --------------------------------------------------------------------

