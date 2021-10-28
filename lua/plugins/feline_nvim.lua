--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--
--─────────────────────────────────────────────────--
--   Plugin:    feline.nvim
--   Github:    github.com/Famiu/feline.nvim
--─────────────────────────────────────────────────--
--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--





--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━━━❰ configs ❱━━━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--


local lsp = require('feline.providers.lsp')
local vi_mode_utils = require('feline.providers.vi_mode')
local lsp_status = require('lsp-status')

local b = vim.b
local fn = vim.fn

local components = {
    active = {},
    inactive = {}
}

components.active[1] = {
    {
        provider = '▊ ',
        hl = {
            fg = 'skyblue'
        }
    },
    {
        provider = 'vi_mode',
        hl = function()
            return {
                name = vi_mode_utils.get_mode_highlight_name(),
                fg = vi_mode_utils.get_mode_color(),
                style = 'bold'
            }
        end,
        right_sep = ' '
    },
    {
        provider = 'file_info',
        hl = {
            fg = 'black',
            bg = 'fg',
            style = 'bold'
        },
        left_sep = {
            ' ', 'slant_left_2',
            {str = ' ', hl = {bg = 'fg', fg = 'NONE'}}
        },
        right_sep = {'slant_right_2', ' '}
    },
    {
        provider = 'file_size',
        enabled = function() return fn.getfsize(fn.expand('%:p')) > 0 end,
        right_sep = {
            ' ',
            {
                str = ' ',
                hl = {
                    fg = 'fg',
                    bg = 'bg'
                }
            },
        }
    },
    {
        provider = function()
            local name_with_path = os.getenv('VIRTUAL_ENV')
            local venv = {};
            for match in (name_with_path.."/"):gmatch("(.-)".."/") do
                table.insert(venv, match);
            end
            return  "" .. venv[#venv]
        end,
        enabled = function() return os.getenv('VIRTUAL_ENV') ~= nil end,
        right_sep = " ",
    },
    {
        provider = 'git_branch',
        hl = {
            fg = 'white',
            bg = 'black1',
            style = 'bold'
        },
        right_sep = function()
            local val = {hl = {fg = 'NONE', bg = 'black1'}}
            if b.gitsigns_status_dict then val.str = ' ' else val.str = '' end
            return val
        end
    },
    {
        provider = 'git_diff_added',
        icon = {
            str = ' +',
            hl = {
              fg = 'green' ,
              style = 'bold',
            },
        },
        hl = {
          fg = 'green' ,
          style = 'bold',
        },
    },
    {
        provider = 'git_diff_changed',
        icon = {
            str = ' ~',
            hl = {
              fg = 'orange' ,
              style = 'bold',
            },
        },
        hl = {
          fg = 'orange' ,
          style = 'bold',
        },
    },
    {
        provider = 'git_diff_removed',
        icon = {
            str = ' -',
            hl = {
              fg = 'red' ,
              style = 'bold',
            },
        },
        hl = {
          fg = 'red' ,
          style = 'bold',
        },
        right_sep = function()
            local val = {hl = {fg = 'NONE', bg = 'black1'}}
            if b.gitsigns_status_dict then val.str = ' ' else val.str = '' end
            return val
        end
    },
}

components.active[2] = {
    {
        provider = 'diagnostic_errors',
        enabled = function() return lsp.diagnostics_exist('Error') end,
        hl = { fg = 'red' }
    },
    {
        provider = 'diagnostic_warnings',
        enabled = function() return lsp.diagnostics_exist('Warning') end,
        hl = { fg = 'yellow' }
    },
    {
        provider = 'diagnostic_hints',
        enabled = function() return lsp.diagnostics_exist('Hint') end,
        hl = { fg = 'cyan' }
    },
    {
        provider = 'diagnostic_info',
        enabled = function() return lsp.diagnostics_exist('Information') end,
        hl = { fg = 'skyblue' }
    },
    {
      provider = function()
            local msg = ''
            local buf_ft = vim.api.nvim_buf_get_option(0, 'filetype')
            local clients = vim.lsp.get_active_clients()
            if next(clients) == nil then return msg end
            for _, client in ipairs(clients) do
              local filetypes = client.config.filetypes
              if filetypes and vim.fn.index(filetypes, buf_ft) ~= -1 then
                return "LSP[" .. client.name .. "]"
              end
            end
            return msg
      end,
      left_sep = {
            ' ',
            {
                str = ' ',
                hl = {
                    fg = 'fg',
                    bg = 'bg'
                }
            }
      }

    },
    {
      provider = function ()
          lsp_status.register_progress()
          return lsp_status.status()
      end
    },

}

components.active[3] = {
    {
        provider = '  %l:%-2c- %L ',
        left_sep = ' ',
        hl = {
            fg = 'black',
            bg = 'fg',
            style = 'bold'
        },
    },
    {
        provider = 'scroll_bar',
        hl = {
            fg = 'black',
            bg = 'fg',
            style = 'bold',
            left_sep = ' ',
            right_sep = ' '
        }
    }
}

components.inactive[1] = {
    {
        provider = 'file_info',
        hl = {
            fg = 'black',
            bg = 'white1',
            style = 'bold'
        },
        -- left_sep = {
        --     str = ' ',
        --     hl = {
        --         fg = 'black',
        --         bg = 'white',
        --     }
        -- },
        right_sep = {
            {
                str = ' ',
                hl = {
                  fg = 'black',
                  bg = 'bg',
                }
            },
            -- 'slant_right'
        }
    }
}

-- This table is equal to the default colors table
local colors = {
    fg          = '#C8C8C8',
    bg          = '#1F1F23',
    black       = "#000000",
    black1      = '#1B1B1B',
    skyblue     = '#50B0F0',
    cyan        = '#009090',
    green       = '#60A040',
    oceanblue   = '#0066cc',
    magenta     = '#C26BDB',
    orange      = '#FF9000',
    red         = '#D10000',
    violet      = '#9E93E8',
    white       = '#FFFFFF',
    white1      = '#808080',
    yellow      = '#E1E120'
}

-- This table is equal to the default separators table
local separators        = {
    vertical_bar        = '┃',
    vertical_bar_thin   = '│',
    left                = '',
    right               = '',
    block               = '█',
    left_filled         = '',
    right_filled        = '',
    slant_left          = '',
    slant_left_thin     = '',
    slant_right         = '',
    slant_right_thin    = '',
    slant_left_2        = '',
    slant_left_2_thin   = '',
    slant_right_2       = '',
    slant_right_2_thin  = '',
    left_rounded        = '',
    left_rounded_thin   = '',
    right_rounded       = '',
    right_rounded_thin  = '',
    circle              = '●'
}

-- This table is equal to the default vi_mode_colors table
local vi_mode_colors = {
    ['NORMAL']      = 'green',
    ['OP']          = 'green',
    ['INSERT']      = 'red',
    ['VISUAL']      = 'skyblue',
    ['LINES']       = 'skyblue',
    ['BLOCK']       = 'skyblue',
    ['REPLACE']     = 'violet',
    ['V-REPLACE']   = 'violet',
    ['ENTER']       = 'cyan',
    ['MORE']        = 'cyan',
    ['SELECT']      = 'orange',
    ['COMMAND']     = 'green',
    ['SHELL']       = 'green',
    ['TERM']        = 'green',
    ['NONE']        = 'yellow'
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
        'help'
    },
    buftypes = {
        'terminal'
    },
    bufnames = {}
}

-- This table is equal to the default disable table
local disable = {
    filetypes = {},
    buftypes = {},
    bufnames = {}
}

-- This table is equal to the default update_triggers table
local update_triggers = {
    'VimEnter',
    'WinEnter',
    'WinClosed',
    'FileChangedShellPost'
}

require('feline').setup({
    colors = colors,
    separators = separators,
    vi_mode_colors = vi_mode_colors,
    force_inactive = force_inactive,
    disable = disable,
    update_triggers = update_triggers,
    components = components
})



--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━❰ end configs ❱━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--


