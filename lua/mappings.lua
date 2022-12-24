
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━❰ Plugin-Independent Mapping ❱━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
--[===[
        this config file contains the mapping that don't depends
        on any plugin. mappings for plugins-dependent are in
        lua/plugin" directory. each plugin has it's own config file

        To see the current mapping for |<Leader>| type :echo mapleader.
        If it reports an undefined variable it means the leader key is
        set to the "default of '\'.
        i defined leader on very start of init.lua file so that every
        keymap would work
--]===]


local keymap  = vim.api.nvim_set_keymap
local cmd     = vim.cmd
local options = {noremap = true, silent = true}
local silent  = {silent = true}


-- TODO: fiqure out to implement <leader>q to quit only one buffer or one window at a time
-- Close buffer
cmd([[ autocmd BufEnter * nmap <silent> <buffer> <leader>q :bd<CR> ]])

vim.api.nvim_create_autocmd(
	"FileType",
	{
		pattern = {
			"man", "help", "lspinfo", "null-ls-info", "lsp-installer"
		},
		command = "nnoremap <silent> <buffer> <leader>q :close<CR>",
	}
)


-- map ctl+z to nothing so that it don't suspend terminal
vim.api.nvim_create_autocmd(
	"BufEnter",
	{
		pattern = "*",
		command = "nnoremap <c-z> <nop>"
	}
)

-- keymap('n', '<Leader>q',':q <CR>',      options)
-- write/save when the buffer has been modified.
keymap('i', '<C-s>', '<ESC>ma<ESC>:update <CR>`a', options)
keymap('n', '<C-s>', '<ESC>ma<ESC>:update <CR>`a', options)

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
keymap('n', '??', ':noh <CR>', silent)

--[[
    on[ly] close all other windows but leave all buffers open.
--]]
keymap('n', '<M-q>', ':only<CR>', silent)

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

-- smart deletion, dd
-- It solves the issue, where you want to delete empty line, but dd will override you last yank.
-- Code above will check if u are deleting empty line, if so - use black hole register.
-- [src: https://www.reddit.com/r/neovim/comments/w0jzzv/comment/igfjx5y/?utm_source=share&utm_medium=web2x&context=3]
local function smart_dd()
	if vim.api.nvim_get_current_line():match("^%s*$") then
		return "\"_dd"
	else return "dd" end
end
vim.keymap.set("n", "dd", smart_dd, { noremap = true, expr = true })

-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━❰ end of Plugin Mapping ❱━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

