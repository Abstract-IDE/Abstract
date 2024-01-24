--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin-Independent Configs

NOTE:
	every configs in this file are independent of any plugin
	configs for plugins are in "lua/plugins" directory and each plugin has
	it's own config file. some settings are already default in neovim so you
	don't need to define explicitly but it won't make any difference
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]




-- Enables the experimental Lua module loader:
--    • overrides loadfile
--    • adds the lua loader using the byte-compilation cache
--    • adds the libs loader
--    • removes the default Neovim loader
vim.loader.enable()

vim.opt.termguicolors = true -- Enable GUI colors for the terminal to get truecolor
vim.opt.list = true -- show whitespace
vim.opt.listchars = {
	nbsp = "⦸", -- CIRCLED REVERSE SOLIDUS (U+29B8, UTF-8: E2 A6 B8)
	extends = "»", -- RIGHT-POINTING DOUBLE ANGLE QUOTATION MARK (U+00BB, UTF-8: C2 BB)
	precedes = "«", -- LEFT-POINTING DOUBLE ANGLE QUOTATION MARK (U+00AB, UTF-8: C2 AB)
	tab = "  ", -- '▷─' WHITE RIGHT-POINTING TRIANGLE (U+25B7, UTF-8: E2 96 B7) + BOX DRAWINGS HEAVY TRIPLE DASH HORIZONTAL (U+2505, UTF-8: E2 94 85)
	trail = "•", -- BULLET (U+2022, UTF-8: E2 80 A2)
	space = " ",
}
vim.opt.fillchars = {
	diff = "∙", -- BULLET OPERATOR (U+2219, UTF-8: E2 88 99)
	eob = " ", -- NO-BREAK SPACE (U+00A0, UTF-8: C2 A0) to suppress ~ at EndOfBuffer
	fold = "·", -- MIDDLE DOT (U+00B7, UTF-8: C2 B7)
	vert = "│", -- window border when window splits vertically ─ ┴ ┬ ┤ ├ ┼
}

-- backup related options
-- neovim backup directory
local backup_dir = vim.fn.stdpath("data") .. "/.cache"
vim.opt.backup = true                         -- make backups before writing
vim.opt.undofile = false                      -- persistent undos - undo after you re-open the file
vim.opt.writebackup = true                    -- Make backup before overwriting the current buffer
vim.opt.backupcopy = "yes"                    -- Overwrite the original backup file
vim.opt.directory = backup_dir .. "/swap"     -- directory to place swap files in
vim.opt.backupdir = backup_dir .. "/backedUP" -- where to put backup files
vim.opt.undodir = backup_dir .. "/undos"      -- where to put undo files
vim.opt.viewdir = backup_dir .. "/view"       -- where to store files for :mkview
vim.opt.shada = "'100,<50,f50,n" .. backup_dir .. "/shada/shada"

vim.opt.clipboard = vim.opt.clipboard + "unnamedplus" -- copy & paste
vim.opt.wrap = false                                  -- don't automatically wrap on load
vim.opt.showmatch = true                              -- show the matching part of the pair for [] {} and ()

vim.opt.cursorline = true                             -- highlight current line
vim.opt.number = true                                 -- show line numbers
vim.opt.relativenumber = true                         -- show relative line number

vim.opt.incsearch = true                              -- incremental search
vim.opt.hlsearch = true                               -- highlighted search results
vim.opt.ignorecase = true                             -- ignore case sensetive while searching
vim.opt.smartcase = true

vim.opt.scrolloff = 1                  -- when scrolling, keep cursor 1 lines away from screen border
vim.opt.sidescrolloff = 2              -- keep 30 columns visible left and right of the cursor at all times
vim.opt.backspace = "indent,start,eol" -- make backspace behave like normal again
vim.opt.mouse = "a"                    -- turn on mouse interaction
vim.opt.mousescroll = "ver:3,hor:2"    -- vertical and horizontal scroll speed
vim.opt.updatetime = 500               -- CursorHold interval

vim.opt.softtabstop = 4
vim.opt.shiftwidth = 4    -- spaces per tab (when shifting), when using the >> or << commands, shift lines by 4 spaces
vim.opt.tabstop = 4       -- spaces per tab
vim.opt.smarttab = true   -- <tab>/<BS> indent/dedent in leading whitespace
vim.opt.autoindent = true -- maintain indent of current line
-- opt.expandtab = false, -- don't expand tabs into spaces

vim.opt.shiftround = true

vim.opt.splitbelow = true                                  -- open horizontal splits below current window
vim.opt.splitright = true                                  -- open vertical splits to the right of the current window
vim.opt.laststatus = 3                                     -- always show status line. 3 means Global Status Line
-- opt.colorcolumn = "79", -- vertical word limit line
vim.opt.cmdheight = 1                                      -- command height

vim.opt.hidden = true                                      -- allows you to hide buffers with unsaved changes without being prompted
vim.opt.inccommand = "split"                               -- live preview of :s results
vim.opt.shell = "zsh"                                      -- shell to use for `!`, `:!`, `system()` etc.
vim.opt.lazyredraw = true                                  -- faster scrolling

vim.opt.wildignore = vim.opt.wildignore + "*.o,*.rej,*.so" -- patterns to ignore during file-navigation
vim.opt.completeopt = "menuone,noselect,noinsert"          -- completion options

vim.opt.showmode = false                                   -- If in Insert, Replace or Visual mode put a message on the last line.

-- vim.opt.cmdheight = 0                                  -- command height
