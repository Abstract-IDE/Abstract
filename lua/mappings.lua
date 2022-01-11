
--  .______       ______        _______. __    __  .__   __.  __  ____    ____  __  .___  ___.
--  |   _  \     /  __  \      /       ||  |  |  | |  \ |  | |  | \   \  /   / |  | |   \/   |
--  |  |_)  |   |  |  |  |    |   (----`|  |__|  | |   \|  | |  |  \   \/   /  |  | |  \  /  |
--  |      /    |  |  |  |     \   \    |   __   | |  . `  | |  |   \      /   |  | |  |\/|  |
--  |  |\  \----|  `--'  | .----)   |   |  |  |  | |  |\   | |  |    \    /    |  | |  |  |  |
--  | _| `._____|\______/  |_______/    |__|  |__| |__| \__| |__|     \__/     |__| |__|  |__|
--                          Author:     Ali Shahid
--                          Github:     github.com/shaeinst


-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━❰ Plugin-Independent Mapping ❱━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
--[[this config file contains the mapping that don't depends
 on any plugin. mappings for plugins-dependent are in
 lua/plugin" directory. each plugin has it's own config file

To see the current mapping for |<Leader>| type :echo mapleader.
If it reports an undefined variable it means the leader key is
set to the "default of '\'.
i defined leader on very start of init.lua file so that every
keymap would work]]


local keymap	= vim.api.nvim_set_keymap
local cmd		= vim.cmd
local options	= {noremap = true, silent = true}
local silent	= {silent = true}

-- to quit vim
cmd([[ autocmd BufEnter * nmap silent <buffer> <Leader>q :bd<CR> ]])

-- keymap('n', '<Leader>q',':q <CR>',      options)
-- to save file
keymap('i', '<C-s>', '<ESC>ma<ESC>:w <CR>`a', options)
keymap('n', '<C-s>', '<ESC>ma<ESC>:w <CR>`a', options)

-- scroll window up/down
keymap('i', '<C-e>', '<ESC><C-e>', silent)
keymap('i', '<C-y>', '<ESC><C-y>', silent)

-- scroll window horizontally (scroll-horizontal)
-- < reference: https://unix.stackexchange.com/questions/110251/how-to-put-current-line-at-top-center-bottom-of-screen-in-vim
keymap('n', '<C-h>', 'zh', silent) -- left
keymap('n', '<C-l>', 'zl', silent) -- right

-- number line enable
keymap('n', '<leader>n', ':set rnu! <CR>', silent)

-- clear Search Results
keymap('n', '//', ':noh <CR>', silent)

--[[
    on[ly] close all other windows but leave all buffers open.
--]]
keymap('n', '<M-q>', '<C-W>on', silent)

--			Resize splits more quickly
-- ────────────────────────────────────────────────────
-- resize up and down
keymap('n', ';k', ':resize +3 <CR>', options)
keymap('n', ';j', ':resize -3 <CR>', options)
-- resize right and left
keymap('n', ';l', ':vertical resize +3 <CR>', options)
keymap('n', ';h', ':vertical resize -3 <CR>', options)
-- ────────────────────────────────────────────────────

--[[
      easier moving of code blocks
      Try to go into visual mode (v), thenselect several lines of code
      here and then press ``>`` several times.
--]]
keymap('v', '<', '<gv', options)
keymap('v', '>', '>gv', options)

-- going back to normal mode which works even in vim's terminal
-- you will need this if you use floaterm to escape terminal
keymap('t', '<Esc>', '<c-\\><c-n>', options)

-- move selected line(s) up or down
keymap('v', 'J', ":m '>+1<CR>gv=gv", options)
keymap('v', 'K', ":m '<-2<CR>gv=gv", options)

-- map ctl+z to nothing so that it don't suspend terminal
cmd([[ :nnoremap <c-z> <nop><CR> ]])

-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━❰ end of Plugin Mapping ❱━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

