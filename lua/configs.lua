
--  .______       ______        _______. __    __  .__   __.  __  ____    ____  __  .___  ___.
--  |   _  \     /  __  \      /       ||  |  |  | |  \ |  | |  | \   \  /   / |  | |   \/   |
--  |  |_)  |   |  |  |  |    |   (----`|  |__|  | |   \|  | |  |  \   \/   /  |  | |  \  /  |
--  |      /    |  |  |  |     \   \    |   __   | |  . `  | |  |   \      /   |  | |  |\/|  |
--  |  |\  \----|  `--'  | .----)   |   |  |  |  | |  |\   | |  |    \    /    |  | |  |  |  |
--  | _| `._____|\______/  |_______/    |__|  |__| |__| \__| |__|     \__/     |__| |__|  |__|
--                          Author:     Ali Shahid
--                          Github:     github.com/shaeinst


-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━❰ Plugin-Independent Configs ❱━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

--[[
	 NOTE:
		every configs in this file are independent of any plugin
		configs for plugins are in "lua/plugins" directory and each plugin has
		it's own config file. some settings are already default in neovim so you
		don't need to define explicitly but it won't make any difference
    :end of NOTE
--]]

local exec = vim.api.nvim_exec -- execute Vimscript
local set = vim.opt -- global options
local cmd = vim.cmd -- execute Vim commands
-- local fn    = vim.fn            -- call Vim functions
-- local g     = vim.g             -- global variables
-- local b     = vim.bo            -- buffer-scoped options
-- local w     = vim.wo            -- windows-scoped options

cmd('colorscheme rvcs')
set.guifont = 'DroidSansMono Nerd Font 11'
set.termguicolors = true -- Enable GUI colors for the terminal to get truecolor
set.list = true -- show whitespace
set.listchars = {
	nbsp = '⦸', -- CIRCLED REVERSE SOLIDUS (U+29B8, UTF-8: E2 A6 B8)
	extends = '»', -- RIGHT-POINTING DOUBLE ANGLE QUOTATION MARK (U+00BB, UTF-8: C2 BB)
	precedes = '«', -- LEFT-POINTING DOUBLE ANGLE QUOTATION MARK (U+00AB, UTF-8: C2 AB)
	tab = '  ', -- '▷─' WHITE RIGHT-POINTING TRIANGLE (U+25B7, UTF-8: E2 96 B7) + BOX DRAWINGS HEAVY TRIPLE DASH HORIZONTAL (U+2505, UTF-8: E2 94 85)
	trail = '•', -- BULLET (U+2022, UTF-8: E2 80 A2)
	space = ' ',
}

set.fillchars = {
	diff = '∙', -- BULLET OPERATOR (U+2219, UTF-8: E2 88 99)
	eob = ' ', -- NO-BREAK SPACE (U+00A0, UTF-8: C2 A0) to suppress ~ at EndOfBuffer
	fold = '·', -- MIDDLE DOT (U+00B7, UTF-8: C2 B7)
	vert = ' ', -- remove ugly vertical lines on window division
}

if root then
	set.shada = '' -- Don't create root-owned files.
	set.shadafile = 'NONE'
else
	local backup_dir = vim.fn.expand('~/.cache/nvim')
	set.backup = true -- make backups before writing
	set.undofile = false -- persistent undos - undo after you re-open the file
	set.writebackup = true -- Make backup before overwriting the current buffer
	set.backupcopy = 'yes' -- Overwrite the original backup file
	set.directory = backup_dir .. '/swap' -- directory to place swap files in
	set.backupdir = backup_dir .. '/backedUP' -- where to put backup files
	set.undodir = backup_dir .. '/undos' -- where to put undo files
	set.viewdir = backup_dir .. '/view' -- where to store files for :mkview
	set.shada = "'100,<50,f50,n~/.cache/nvim/shada/shada"
end

set.clipboard = set.clipboard + "unnamedplus" -- copy & paste
set.wrap = false -- don't automatically wrap on load
set.showmatch = true -- show the matching part of the pair for [] {} and ()

set.cursorline = true -- highlight current line
set.number = true -- show line numbers
set.relativenumber = false -- show relative line number

set.incsearch = true -- incremental search
set.hlsearch = true -- highlighted search results
set.ignorecase = true -- ignore case sensetive while searching
set.smartcase = true

set.scrolloff = 1 -- when scrolling, keep cursor 1 lines away from screen border
set.sidescrolloff = 2 -- keep 30 columns visible left and right of the cursor at all times
set.backspace = 'indent,start,eol' -- make backspace behave like normal again
-- set.mouse		= "a"  		-- turn on mouse interaction
set.updatetime = 500 -- CursorHold interval

set.softtabstop = 4
set.shiftwidth = 4 -- spaces per tab (when shifting), when using the >> or << commands, shift lines by 4 spaces
set.tabstop = 4 -- spaces per tab
set.smarttab = true -- <tab>/<BS> indent/dedent in leading whitespace
set.autoindent = true -- maintain indent of current line
set.expandtab = false -- don't expand tabs into spaces
set.shiftround = true

set.splitbelow = true -- open horizontal splits below current window
set.splitright = true -- open vertical splits to the right of the current window
set.laststatus = 2 -- always show status line
-- set.colorcolumn	= "79"			-- vertical word limit line

set.hidden = true -- allows you to hide buffers with unsaved changes without being prompted
set.inccommand = 'split' -- live preview of :s results
set.shell = 'zsh' -- shell to use for `!`, `:!`, `system()` etc.
set.lazyredraw = true -- faster scrolling

set.wildignore = set.wildignore + '*.o,*.rej,*.so' -- patterns to ignore during file-navigation
set.completeopt = 'menuone,noselect,noinsert' -- completion options


-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━❰ Automate ❱━━━━━━━━━━━━━━━━━━━━ --

-- highlight on yank
exec([[
	augroup YankHighlight
		autocmd!
    	autocmd TextYankPost * silent! lua vim.highlight.on_yank{higroup="IncSearch", timeout=500, on_visual=true}
	augroup end
]], false)

-- jump to the last position when reopening a file
cmd([[
	if has("autocmd")
		au BufReadPost * if line("'\"") > 0 && line("'\"") <= line("$") | exe "normal! g`\"" | endif
	endif
]])

-- remove whitespace on save
cmd([[au BufWritePre * :%s/\s\+$//e]])

-- don't auto commenting new lines
cmd([[au BufEnter * set fo-=c fo-=r fo-=o]])

-- ━━━━━━━━━━━━━━━━❰ end of Automate ❱━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --


-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━❰ end of Plugin Configs ❱━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

