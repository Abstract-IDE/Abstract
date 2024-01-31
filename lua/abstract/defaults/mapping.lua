-- Plugin-Independent Mappings

local keymap = vim.api.nvim_set_keymap
local options = { noremap = true, silent = true }
local silent = { silent = true }

-- scroll window up/down
keymap("i", "<C-e>", "<ESC><C-e>", silent)
keymap("i", "<C-y>", "<ESC><C-y>", silent)

-- scroll window horizontally (scroll-horizontal)
-- < reference: https://unix.stackexchange.com/questions/110251/how-to-put-current-line-at-top-center-bottom-of-screen-in-vim
keymap("n", "<C-h>", "zh", silent) -- left
keymap("n", "<C-l>", "zl", silent) -- right

-- number line enable
keymap("n", "<leader>n", ":set rnu! <CR>", silent)

-- clear Search Results
keymap("n", "??", ":noh <CR>", silent)

-- on[ly] close all other windows but leave all buffers open.
keymap("n", "<M-q>", ":only<CR>", silent)

--	Resize splits more quickly
-- resize up and down
keymap("n", ";k", ":resize +3 <CR>", options)
keymap("n", ";j", ":resize -3 <CR>", options)
-- resize right and left
keymap("n", ";l", ":vertical resize +3 <CR>", options)
keymap("n", ";h", ":vertical resize -3 <CR>", options)
