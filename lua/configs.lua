
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

 -- neovim backup directory
local backup_dir = vim.fn.stdpath('data').."/.cache"

-- define configs
local configs = {
	guifont = 'DroidSansMono Nerd Font 11',
	termguicolors = true, -- Enable GUI colors for the terminal to get truecolor
	list = true, -- show whitespace
	listchars = {
		nbsp = '⦸', -- CIRCLED REVERSE SOLIDUS (U+29B8, UTF-8: E2 A6 B8)
		extends = '»', -- RIGHT-POINTING DOUBLE ANGLE QUOTATION MARK (U+00BB, UTF-8: C2 BB)
		precedes = '«', -- LEFT-POINTING DOUBLE ANGLE QUOTATION MARK (U+00AB, UTF-8: C2 AB)
		tab = '  ', -- '▷─' WHITE RIGHT-POINTING TRIANGLE (U+25B7, UTF-8: E2 96 B7) + BOX DRAWINGS HEAVY TRIPLE DASH HORIZONTAL (U+2505, UTF-8: E2 94 85)
		trail = '•', -- BULLET (U+2022, UTF-8: E2 80 A2)
		space = ' ',
	},

	fillchars = {
		diff = '∙', -- BULLET OPERATOR (U+2219, UTF-8: E2 88 99)
		eob = ' ', -- NO-BREAK SPACE (U+00A0, UTF-8: C2 A0) to suppress ~ at EndOfBuffer
		fold = '·', -- MIDDLE DOT (U+00B7, UTF-8: C2 B7)
		vert = '│', -- window border when window splits vertically ─ ┴ ┬ ┤ ├ ┼
	},

	-- backup related options
	backup = true, -- make backups before writing
	undofile = false, -- persistent undos - undo after you re-open the file
	writebackup = true, -- Make backup before overwriting the current buffer
	backupcopy = 'yes', -- Overwrite the original backup file
	directory = backup_dir .. '/swap', -- directory to place swap files in
	backupdir = backup_dir .. '/backedUP', -- where to put backup files
	undodir = backup_dir .. '/undos', -- where to put undo files
	viewdir = backup_dir .. '/view', -- where to store files for :mkview
	shada = "'100,<50,f50,n"..backup_dir.."/shada/shada",

	clipboard = vim.opt.clipboard + "unnamedplus", -- copy & paste
	wrap = false, -- don't automatically wrap on load
	showmatch = true, -- show the matching part of the pair for [] {} and ()

	cursorline = true, -- highlight current line
	number = true, -- show line numbers
	relativenumber = false, -- show relative line number

	incsearch = true, -- incremental search
	hlsearch = true, -- highlighted search results
	ignorecase = true, -- ignore case sensetive while searching
	smartcase = true,

	scrolloff = 1, -- when scrolling, keep cursor 1 lines away from screen border
	sidescrolloff = 2, -- keep 30 columns visible left and right of the cursor at all times
	backspace = 'indent,start,eol', -- make backspace behave like normal again
	-- mouse = "a" , -- turn on mouse interaction
	updatetime = 500, -- CursorHold interval

	softtabstop = 4,
	shiftwidth = 4, -- spaces per tab (when shifting), when using the >> or << commands, shift lines by 4 spaces
	tabstop = 4, -- spaces per tab
	smarttab = true, -- <tab>/<BS> indent/dedent in leading whitespace
	autoindent = true, -- maintain indent of current line
	-- expandtab = false, -- don't expand tabs into spaces

	shiftround = true,

	splitbelow = true, -- open horizontal splits below current window
	splitright = true, -- open vertical splits to the right of the current window
	laststatus = 3, -- always show status line. 3 means Global Status Line
	-- colorcolumn = "79", -- vertical word limit line
	cmdheight = 1, -- command height

	hidden = true, -- allows you to hide buffers with unsaved changes without being prompted
	inccommand = 'split', -- live preview of :s results
	shell = 'zsh', -- shell to use for `!`, `:!`, `system()` etc.
	lazyredraw = true, -- faster scrolling

	wildignore = vim.opt.wildignore + '*.o,*.rej,*.so', -- patterns to ignore during file-navigation
	completeopt = 'menuone,noselect,noinsert', -- completion options
}

-- apply colorscheme without throwing any errors
pcall(vim.cmd, 'colorscheme abscs')

-- applying defined configs
for option, value in pairs(configs) do
	vim.opt[option] = value
end

-- -- configs
-- if vim.api.nvim_call_function('has', {'nvim-0.8'}) == 1 then
-- 	vim.opt.cmdheight = 0 -- command height
-- end


-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━❰ Automate ❱━━━━━━━━━━━━━━━━━━━━ --

local group = vim.api.nvim_create_augroup("AbstractAutoGroup", {clear=true})

vim.api.nvim_create_autocmd(
	"TextYankPost",
	{
        desc = "highlight text on yank",
        pattern = "*",
		group = group,
        callback = function()
			vim.highlight.on_yank {
				higroup="Search", timeout=400, on_visual=true
			}
        end,
	}
)

vim.api.nvim_create_autocmd(
	"BufWinEnter",
	{
        desc = "jump to the last position when reopening a file",
        pattern = "*",
		group = group,
		command = [[ if line("'\"") > 0 && line("'\"") <= line("$") | exe "normal! g`\"" | endif ]]
	}
)

vim.api.nvim_create_autocmd(
	"BufWritePre",
	{
		desc = "remove whitespaces on save",
		pattern = "*",
		group = group,
		command = "%s/\\s\\+$//e",
	}
)

vim.api.nvim_create_autocmd(
	"BufEnter",
	{
		desc = "don't auto comment new line",
		pattern = "*",
		group = group,
		command = "setlocal formatoptions-=c formatoptions-=r formatoptions-=o",
	}
)

vim.api.nvim_create_autocmd(
	"VimResized",
	{
		desc = "auto resize splited windows",
		pattern = "*",
		group = group,
		command = "tabdo wincmd =",
	}
)

-- ━━━━━━━━━━━━━━━━❰ end of Automate ❱━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --


-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━❰ end of Plugin-Independent Configs ❱━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

