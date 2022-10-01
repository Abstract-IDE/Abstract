
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ───────────────────────────────────────────────── --
--   Plugin:    feline.nvim
--   Github:    github.com/feline-nvim/feline.nvim
-- ───────────────────────────────────────────────── --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --





-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━❰ configs ❱━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

local import_feline, feline = pcall(require, 'feline')
if not import_feline then return end

local lsp = require('feline.providers.lsp')
local vi_mode_utils = require('feline.providers.vi_mode')

-- local lsp_status = require('lsp-status')
-- lsp_status.register_progress()

local b = vim.b
local fn = vim.fn
local api = vim.api

local components = {active = {}, inactive = {}}

vim.opt.termguicolors = true -- Enable GUI colors for the terminal to get truecolor

components.active[1] = {
	{provider = ' ', hl = {fg = 'skyblue'}},
	{
		provider = function ()
			local mode = api.nvim_exec('echo mode()', true)
			if mode == "" then
				return "C"
			end
			return string.upper(mode)
		end,
		hl = function()
			return {
				name = vi_mode_utils.get_mode_highlight_name(),
				fg = vi_mode_utils.get_mode_color(),
				style = 'bold',
			}
		end,
		right_sep = '',
	},
	{
		provider = {
			name = "file_info",
			opts = { type = "relative-short" }
		},
		hl = {fg = 'black', bg = 'f_color', style = 'italic'},
		left_sep = {
			' ',
			'slant_left_2',
			{str = ' ', hl = {bg = 'f_color', fg = 'NONE'}},
		},
	},
	{
		provider = function ()
			local filetype = vim.bo.filetype
			if filetype ~= "" and filetype ~= "alpha" then
				return " [" .. filetype .. "]"
			end
			return " "
		end,
		hl = {fg = 'black', bg = 'f_color', style = 'NONE'},
		right_sep = {'slant_right_2', ' '},
	},
	{
		provider = 'file_size',
		enabled = function() return fn.getfsize(fn.expand('%:p')) > 0 end,
		right_sep = {' ', {str = ' ', hl = {fg = 'fg', bg = 'bg'}}},
	},
	{
		provider = function()
			local name_with_path = os.getenv('VIRTUAL_ENV')
			local venv = {};
			for match in (name_with_path .. "/"):gmatch("(.-)" .. "/") do
				table.insert(venv, match);
			end
			return "" .. venv[#venv]
		end,
		enabled = function() return os.getenv('VIRTUAL_ENV') ~= nil end,
		right_sep = " ",
	},
	{
		provider = 'git_branch',
		hl = {fg = 'cyan2', bg = 'bg', style = 'bold'},
		right_sep = function()
			local val = {hl = {fg = 'NONE', bg = 'bg'}}
			if b.gitsigns_status_dict then
				val.str = ' '
			else
				val.str = ''
			end
			return val
		end,
	},
	{
		provider = 'git_diff_added',
		icon = {str = ' +', hl = {fg = 'green', style = 'bold'}},
		hl = {fg = 'green', style = 'bold'},
	},
	{
		provider = 'git_diff_changed',
		icon = {str = ' ~', hl = {fg = 'orange', style = 'bold'}},
		hl = {fg = 'orange', style = 'bold'},
	},
	{
		provider = 'git_diff_removed',
		icon = {str = ' -', hl = {fg = 'red', style = 'bold'}},
		hl = {fg = 'red', style = 'bold'},
		right_sep = function()
			local val = {hl = {fg = 'NONE', bg = 'bg'}}
			if b.gitsigns_status_dict then
				val.str = ' '
			else
				val.str = ''
			end
			return val
		end,
	},
}

components.active[2] = {
	{
		provider = 'diagnostic_errors',
		enabled = function()
			return lsp.diagnostics_exist(vim.diagnostic.severity.ERROR)
		end,
		hl = {fg = 'red'},
	},
	{
		provider = 'diagnostic_warnings',
		enabled = function()
			return lsp.diagnostics_exist(vim.diagnostic.severity.WARN)
		end,
		hl = {fg = 'yellow'},
	},
	{
		provider = 'diagnostic_hints',
		enabled = function()
			return lsp.diagnostics_exist(vim.diagnostic.severity.HINT)
		end,
		hl = {fg = 'cyan'},
	},
	{
		provider = 'diagnostic_info',
		enabled = function()
			return lsp.diagnostics_exist(vim.diagnostic.severity.INFO)
		end,
		hl = {fg = 'skyblue'},
	},
	{
		provider = function()
			local msg = ''
			local buf_ft = api.nvim_buf_get_option(0, 'filetype')
			local clients = vim.lsp.get_active_clients()
			if next(clients) == nil then return msg end
			for _, client in ipairs(clients) do
				local filetypes = client.config.filetypes
				if filetypes and vim.fn.index(filetypes, buf_ft) ~= -1 then
					if client.name ~= "null-ls" then return "LSP[" .. client.name .. "]" end
				end
			end
			return msg
		end,
		-- hl = {style = 'italic'},
		hl = {fg='black'},
		left_sep = {' ', {str = ' ', hl = {fg = 'fg', bg = 'bg'}}},
	},
	-- {
	--   provider = function ()
	--       return lsp_status.status()
	--   end
	-- },

}

components.active[3] = {
	{
		provider = function()
			-- thanks: https://github.com/nvim-lualine/lualine.nvim/issues/186#issuecomment-1170637440
			local hlsearch = api.nvim_get_vvar("hlsearch")
			if hlsearch == nil then goto empty end
			if hlsearch == 1 then
				local result = fn.searchcount({ maxcount = 999, timeout = 1000 })
				local total = result.total
				if total == nil then goto empty end
				if total > 0 then
					local search_string = fn.getreg("/")
					return string.format("%s %d/%d", search_string, result.current, total)
				end
			end
			::empty::
			return ""
		end,
		left_sep = ' ',
		hl = {fg = 'yellow'}
	},
	{
		provider = '  %l:%-2c- %L ',
		left_sep = ' ',
		hl = {fg = 'black', bg = 'f_color'},
	},
	{
		-- provider = 'scroll_bar',
		provider = ' ',
		hl = {
			fg = 'black',
			bg = 'f_color',
			-- style = 'bold',
			left_sep = ' ',
			-- right_sep = ' '
		},
	},
}

-- components.inactive[1] = { }

-- This table is equal to the default colors table
local colors = {
	fg        = '#C8C8C8',
	bg        = '#141414',
	black     = "#b7b7b7",
	black1    = '#1B1B1B',
	skyblue   = '#50B0F0',
	cyan2     = '#006c6c',
	cyan      = '#009090',
	green     = '#60A040',
	oceanblue = '#0066cc',
	magenta   = '#C26BDB',
	orange    = '#FF9000',
	red       = '#D10000',
	violet    = '#9E93E8',
	white     = '#FFFFFF',
	f_color   = '#072b2c',
	yellow    = '#E1E120',
}

-- This table is equal to the default separators table
local separators = {
	vertical_bar       = '┃',
	vertical_bar_thin  = '│',
	left               = '',
	right              = '',
	block              = '█',
	right_filled       = '',
	left_filled        = '',
	slant_left         = '',
	slant_right        = '',
	slant_left_2       = '',
	slant_right_2      = '',
	slant_left_thin    = '',
	slant_right_thin   = '',
	slant_left_2_thin  = '',
	slant_right_2_thin = '',
	left_rounded_thin  = '',
	right_rounded_thin = '',
	left_rounded       = '',
	right_rounded      = '',
	circle             = '●',
}

-- This table is equal to the default vi_mode_colors table
local vi_mode_colors = {
	['NORMAL'] = 'green',
	['OP'] = 'green',
	['INSERT'] = 'red',
	['VISUAL'] = 'skyblue',
	['LINES'] = 'skyblue',
	['BLOCK'] = 'skyblue',
	['REPLACE'] = 'violet',
	['V-REPLACE'] = 'violet',
	['ENTER'] = 'cyan',
	['MORE'] = 'cyan',
	['SELECT'] = 'orange',
	['COMMAND'] = 'green',
	['SHELL'] = 'green',
	['TERM'] = 'green',
	['NONE'] = 'yellow',
}

-- This table is equal to the default force_inactive table
local force_inactive = {
	filetypes = {
		'NvimTree',
		'packer',
		'startify',
		'fugitive',
		'fugitiveblame',
		'qf',
		'help',
	},
	buftypes = {'terminal'},
	bufnames = {},
}

-- This table is equal to the default disable table
local disable = {filetypes = {}, buftypes = {}, bufnames = {}}

-- This table is equal to the default update_triggers table
local update_triggers = {
	'VimEnter',
	'WinEnter',
	'WinClosed',
	'FileChangedShellPost',
}

feline.setup({
	theme = colors,
	separators = separators,
	vi_mode_colors = vi_mode_colors,
	force_inactive = force_inactive,
	disable = disable,
	update_triggers = update_triggers,
	components = components,
})

-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━❰ end configs ❱━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

