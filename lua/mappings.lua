--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin-Independent Mappings

This config file contains the mappings that don't depend on any plugin.
Mappings for plugin-dependent actions are in the "lua/plugin" directory. Each plugin has its own config file.

To see the current mapping for |<Leader>|, type :echo mapleader.
If it reports an undefined variable, it means the leader key is set to the "default of '\'.
I have defined the leader at the very start of the init.lua file so that every keymap will work.
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local keymap = vim.api.nvim_set_keymap
local options = { noremap = true, silent = true }
local silent = { silent = true }

-- -- TODO: fiqure out to implement <leader>q to quit only one buffer or one window at a time
-- -- Close buffer
-- vim.cmd([[ autocmd BufEnter * nmap <silent> <buffer> <leader>q :bd<CR> ]])

-- vim.api.nvim_create_autocmd(
-- 	"FileType",
-- 	{
-- 		pattern = {
-- 			"man", "help", "lspinfo", "null-ls-info", "lsp-installer"
-- 		},
-- 		command = "nnoremap <silent> <buffer> <leader>q :close<CR>",
-- 	}
-- )

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

