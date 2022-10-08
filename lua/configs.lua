
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

local opt = vim.opt
local api = vim.api

-- set theme
pcall(require, 'plugins.Abstract_cs')

opt.termguicolors = true -- Enable GUI colors for the terminal to get truecolor
opt.list = true -- show whitespace
opt.listchars = {
	nbsp = '⦸', -- CIRCLED REVERSE SOLIDUS (U+29B8, UTF-8: E2 A6 B8)
	extends = '»', -- RIGHT-POINTING DOUBLE ANGLE QUOTATION MARK (U+00BB, UTF-8: C2 BB)
	precedes = '«', -- LEFT-POINTING DOUBLE ANGLE QUOTATION MARK (U+00AB, UTF-8: C2 AB)
	tab = '  ', -- '▷─' WHITE RIGHT-POINTING TRIANGLE (U+25B7, UTF-8: E2 96 B7) + BOX DRAWINGS HEAVY TRIPLE DASH HORIZONTAL (U+2505, UTF-8: E2 94 85)
	trail = '•', -- BULLET (U+2022, UTF-8: E2 80 A2)
	space = ' ',
}
opt.fillchars = {
	diff = '∙', -- BULLET OPERATOR (U+2219, UTF-8: E2 88 99)
	eob = ' ', -- NO-BREAK SPACE (U+00A0, UTF-8: C2 A0) to suppress ~ at EndOfBuffer
	fold = '·', -- MIDDLE DOT (U+00B7, UTF-8: C2 B7)
	vert = '│', -- window border when window splits vertically ─ ┴ ┬ ┤ ├ ┼
}

-- backup related options
-- neovim backup directory
local backup_dir = vim.fn.stdpath('data').."/.cache"
opt.backup = true -- make backups before writing
opt.undofile = false -- persistent undos - undo after you re-open the file
opt.writebackup = true -- Make backup before overwriting the current buffer
opt.backupcopy = 'yes' -- Overwrite the original backup file
opt.directory = backup_dir .. '/swap' -- directory to place swap files in
opt.backupdir = backup_dir .. '/backedUP' -- where to put backup files
opt.undodir = backup_dir .. '/undos' -- where to put undo files
opt.viewdir = backup_dir .. '/view' -- where to store files for :mkview
opt.shada = "'100,<50,f50,n"..backup_dir.."/shada/shada"

opt.clipboard = vim.opt.clipboard + "unnamedplus" -- copy & paste
opt.wrap = false -- don't automatically wrap on load
opt.showmatch = true -- show the matching part of the pair for [] {} and ()

opt.cursorline = true -- highlight current line
opt.number = true -- show line numbers
opt.relativenumber = false -- show relative line number

opt.incsearch = true -- incremental search
opt.hlsearch = true -- highlighted search results
opt.ignorecase = true -- ignore case sensetive while searching
opt.smartcase = true

opt.scrolloff = 1 -- when scrolling, keep cursor 1 lines away from screen border
opt.sidescrolloff = 2 -- keep 30 columns visible left and right of the cursor at all times
opt.backspace = 'indent,start,eol' -- make backspace behave like normal again
opt.mouse = "a" -- turn on mouse interaction
opt.mousescroll = 'ver:3,hor:2' -- vertical and horizontal scroll speed
opt.updatetime = 500 -- CursorHold interval

opt.softtabstop = 4
opt.shiftwidth = 4 -- spaces per tab (when shifting), when using the >> or << commands, shift lines by 4 spaces
opt.tabstop = 4 -- spaces per tab
opt.smarttab = true -- <tab>/<BS> indent/dedent in leading whitespace
opt.autoindent = true -- maintain indent of current line
-- opt.expandtab = false, -- don't expand tabs into spaces

opt.shiftround = true

opt.splitbelow = true -- open horizontal splits below current window
opt.splitright = true -- open vertical splits to the right of the current window
opt.laststatus = 3 -- always show status line. 3 means Global Status Line
-- opt.colorcolumn = "79", -- vertical word limit line
opt.cmdheight = 1 -- command height

opt.hidden = true -- allows you to hide buffers with unsaved changes without being prompted
opt.inccommand = 'split' -- live preview of :s results
opt.shell = 'zsh' -- shell to use for `!`, `:!`, `system()` etc.
opt.lazyredraw = true -- faster scrolling

opt.wildignore = vim.opt.wildignore + '*.o,*.rej,*.so' -- patterns to ignore during file-navigation
opt.completeopt = 'menuone,noselect,noinsert' -- completion options

-- -- configs
-- if vim.api.nvim_call_function('has', {'nvim-0.8'}) == 1 then
-- 	vim.opt.cmdheight = 0 -- command height
-- end


-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━❰ Automate ❱━━━━━━━━━━━━━━━━━━━━ --

local group = api.nvim_create_augroup("AbstractAutoGroup", {clear=true})

api.nvim_create_autocmd(
	"TextYankPost",
	{
        desc = "highlight text on yank",
        pattern = "*",
		group = group,
        callback = function()
			vim.highlight.on_yank {
				higroup="Search", timeout=150, on_visual=true
			}
        end,
	}
)

api.nvim_create_autocmd(
	"BufWinEnter",
	{
        desc = "jump to the last position when reopening a file",
        pattern = "*",
		group = group,
		command = [[ if line("'\"") > 0 && line("'\"") <= line("$") | exe "normal! g`\"" | endif ]]
	}
)

api.nvim_create_autocmd(
	"BufWritePre",
	{
		desc = "remove whitespaces on save",
		pattern = "*",
		group = group,
		command = "%s/\\s\\+$//e",
	}
)

api.nvim_create_autocmd(
	{"BufEnter", "FileType"},
	{
		desc = "don't auto comment new line",
		pattern = "*",
		group = group,
		command = "setlocal formatoptions-=c formatoptions-=r formatoptions-=o",
	}
)

api.nvim_create_autocmd(
	"VimResized",
	{
		desc = "auto resize splited windows",
		pattern = "*",
		group = group,
		command = "tabdo wincmd =",
	}
)

api.nvim_create_autocmd(
	"BufWinEnter",
	{
		desc = "clear the last used search pattern",
		pattern = "*",
		group = group,
		command = "let @/ = ''",
	}
)

local init_color_fg = vim.api.nvim_get_hl_by_name("CursorLineNr", true).foreground
local init_color_bg = vim.api.nvim_get_hl_by_name("CursorLineNr", true).background
api.nvim_create_autocmd(
	{ 'ModeChanged', "FileType" },
	{
		desc = "change cursor color on mode change",
		group = group,
		callback = function()
			local mode = vim.api.nvim_get_mode().mode
			if mode == "i" then
				api.nvim_set_hl(0, "CursorLineNr", {fg="#000000", bg="#c87255", bold=true})
			elseif mode == "v" or mode == "V" or mode == "" then
				api.nvim_set_hl(0, "CursorLineNr", {fg="#000000", bg="#d1d1d1", bold=true})
			else
				vim.api.nvim_set_hl(0, "CursorLineNr", {fg=init_color_fg, bg=init_color_bg, bold=true})
			end
		end,
	}
)

-- ━━━━━━━━━━━━━━━━❰ end of Automate ❱━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --


-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━❰ end of Plugin-Independent Configs ❱━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

