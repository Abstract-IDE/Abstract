--[[
╔═══════════════════════════════════════════════╗
        Author: Ali Shahid
        Github: github.com/shaeinst
╚═══════════════════════════════════════════════╝
--]]





--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━❰ Plugin-Independent Mapping ❱━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--
 --[[this config file contains the mapping that don't depends
 on any plugin. mappings for plugins-dependent are in
 lua/plugin" directory. each plugin has it's own config file

To see the current mapping for |<Leader>| type :echo mapleader.
If it reports an undefined variable it means the leader key is
set to the "default of '\'.
i defined leader on very start of init.lua file so that every
keymap would work]]


local keymap = vim.api.nvim_set_keymap
local cmd = vim.cmd




-- to quit vim
keymap('n', '<Leader>q',':q <CR>',      { noremap=true, silent=true })
-- to save file
keymap('i', '<C-s>',    '<ESC>:w <CR>', { noremap=true, silent=true })
keymap('n', '<C-s>',    '<ESC>:w <CR>', { noremap=true, silent=true })


-- scroll window up/down
keymap('i', '<C-e>', '<ESC><C-e>', { silent=true })
keymap('i', '<C-y>', '<ESC><C-y>', { silent=true })



-- number line enable
keymap('n', '<leader>n', ':set rnu! <CR>', { silent=true })

-- clear Search Results
keymap('n', '//', ':noh <CR>', { silent=true })


--[[
    on[ly] close all other windows but leave all buffers open.
    you cant map alt in vim, so following is required to bind alt with vim
    execute "set <M-q>=\eq"
--]]
keymap('n', '<M-q>', '<C-W>on', { silent=true })


--					Resize splits more quickly
--────────────────────────────────────────────────────
-- resize up and down
keymap('n', ';k',   ':resize +3 <CR>',          { noremap=true, silent=true })
keymap('n', ';j',   ':resize -3 <CR>',          { noremap=true, silent=true })
-- resize right and left
keymap('n', ';l',   ':vertical resize +3 <CR>', { noremap=true, silent=true })
keymap('n', ';h',   ':vertical resize -3 <CR>', { noremap=true, silent=true })
--────────────────────────────────────────────────────


--[[
      easier moving of code blocks
      Try to go into visual mode (v), thenselect several lines of code
      here and then press ``>`` several times.
--]]
keymap('v', '<',   '<gv', { noremap=true, silent=true })
keymap('v', '>',   '>gv', { noremap=true, silent=true })


-- going back to normal mode which works even in vim's terminal
            -- you will need this if you use floaterm to escape terminal
cmd([[ tmap <Esc> <c-\><c-n> ]])

-- move selected line(s) up or down
keymap('v', 'J', ":m '>+1<CR>gv=gv", {noremap=true, silent=true})
keymap('v', 'K', ":m '<-2<CR>gv=gv", {noremap=true, silent=true})


--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━❰ end of Plugin Mapping ❱━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--



