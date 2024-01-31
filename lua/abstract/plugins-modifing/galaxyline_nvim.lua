
--[[
--copied and modified from https://github.com/zootedb0t/dotfiles/blob/master/.config/nvim/lua/plugins/nvim-galaxyline.lua
--]]


local gl = require('galaxyline')

local colors = {
    bg          = '#292D38',
    yellow      = '#DCDCAA',
    dark_yellow = '#D7BA7D',
    cyan        = '#4EC9B0',
    green       = '#608B4E',
    light_green = '#B5CEA8',
    string_orange = '#CE9178',
    orange      = '#FF8800',
    purple      = '#C586C0',
    magenta     = '#D16D9E',
    grey        = '#43454F',
    -- blue     = '#569CD6',
    blue        = '#178c94',

    vivid_blue  = '#4FC1FF',
    light_blue  = '#9CDCFE',
    red         = '#D16969',
    error_red   = '#F44747',

    info_yellow = '#FFCC66',
    fg_green    = '#65a380',
    line_bg     = '#202733',
    fg          = '#8FBCBB',

    darkblue    = '#83A598',
}
local condition = require('galaxyline.condition')
local gls = gl.section
gl.short_line_list = {'NvimTree', 'packer', 'undotree'}


 -- Lsp server name .
local function lspservername()
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
end

local mode_color = function()
  local mode_colors = {
    n = colors.blue,
    i = colors.red,
    c = colors.orange,
    V = colors.magenta,
    [''] = colors.magenta,
    v = colors.magenta,
    R = colors.red,
  }

  local color = mode_colors[vim.fn.mode()]

  if color == nil then
    color = colors.red
  end

  return color
end

gls.left[1] = {
  ViMode = {
    provider = function()
      local alias = {
        n = 'NORMAL',
        i = 'INSERT',
        c = 'COMMAND',
        V = 'VISUAL',
        [''] = 'VISUAL',
        v = 'VISUAL',
        Rv   = 'R&V',
        R = 'REPLACE',
          C      = 'C',
          ['r?'] = ':CONFIRM',
          rm     = '--MORE',
          s      = 'S',
          S      = 'S',
          ['r']  = 'HIT-ENTER',
          t      = 'T',
          ['!']  = 'SH',
      }
      vim.api.nvim_command('hi GalaxyViMode gui=bold guibg='..mode_color())
      local alias_mode = alias[vim.fn.mode()]
      if alias_mode == nil then
        alias_mode = vim.fn.mode()
      end
      return '  '..alias_mode..' '
    end,
    separator = '',
    highlight = { colors.bg, colors.section_bg },
    -- separator_highlight = {colors.bg, colors.section_bg },
    separator_highlight = {colors.bg, colors.blue },
  },
}



















---------------------------------------------------------------

gls.left[2] = {
    FileName = {
      provider = function ()
        local file_name = vim.fn.expand('%f')
        local file_lenght = string.len(file_name)
        if file_lenght > 20 then
              return ' ..'..string.sub(file_name, -25)..' '
        end
        return '  '..file_name..' '
      end,
    condition = buffer_not_empty,
        separator = '',
        separator_highlight = {colors.orange, colors.bg},
        highlight = {"#55ffff", colors.grey}
    }
}
gls.left[3] = {
    BufferType = {
        provider = 'FileIcon',
        condition = condition.buffer_not_empty,
        separator = '',
        separator_highlight = {colors.orange, colors.bg},
        highlight = {"#55aaff", colors.grey}
    }
}

gls.left[4] = {
  FileSize = {
    provider = 'FileSize',
    condition = condition.buffer_not_empty,
    highlight = {"#ffaa7f", colors.grey}
  }
}


gls.left[5] = {
  FileEncode = {
    provider = 'FileEncode',
    condition = condition.buffer_not_empty,
    highlight = {"#fff66f", colors.grey},
    separator = ' ',
    separator_highlight = {'NONE', colors.grey},
  }
}




-- This function gives space on the right side
gls.left[6] = {
    Space = {
        provider = function() return ' ' end,
        separator = ' ',
        separator_highlight = {'NONE', colors.bg},
        highlight = {colors.orange, colors.bg}
    }
}
---------------------------------------------------------------
gls.left[7] = {
    GitIcon = {
        provider = function() return '' end,
        condition = condition.check_git_workspace,
        separator = ' ',
        separator_highlight = {'NONE', colors.bg},
        highlight = {colors.orange, colors.bg}
    }
}

gls.left[8] = {
    GitBranch = {
        provider = 'GitBranch',
        condition = condition.check_git_workspace,
        separator = ' ',
        separator_highlight = {'NONE', colors.bg},
        highlight = {"#55aaff", colors.bg}
    }
}
gls.left[9] = {
    DiffAdd = {
        provider = 'DiffAdd',
        condition = condition.hide_in_width,
        icon = ' +',
        highlight = {colors.green, colors.bg}
    }
}
gls.left[10] = {
    DiffModified = {
        provider = 'DiffModified',
        condition = condition.hide_in_width,
        icon = ' ~',
        highlight = {colors.yellow, colors.bg}
    }
}

gls.left[11] = {
    DiffRemove = {
        provider = 'DiffRemove',
        condition = condition.hide_in_width,
        icon = ' -',
        highlight = {colors.red, colors.bg}
    }
}
----------------------------------------------------







local lsp_status = require('lsp-status')
lsp_status.register_progress()
LspStat = lsp_status.status

gls.mid[1] = {
    LspStat = {
     provider = LspStat,
     highlight = {colors.green,colors.bg},
    }
}




----------------------------------------------------







----------------------------------------------------

gls.right[1] = {
    DiagnosticError = {
        provider = 'DiagnosticError',
        icon = ' ✗ ',
        highlight = {colors.error_red, colors.bg}
    }
}
gls.right[2] = {
    DiagnosticWarn = {
        provider = 'DiagnosticWarn',
        icon = ' ⚠  ',
        highlight = {colors.orange, colors.bg}
    }
}

gls.right[3] = {
    DiagnosticHint = {
        provider = 'DiagnosticHint',
        icon = '  ',
        highlight = {colors.info_yellow, colors.bg}
    }
}

gls.right[4] = {
    DiagnosticInfo = {
        provider = 'DiagnosticInfo',
        icon = '  ',
        highlight = {colors.info_yellow, colors.bg}
    }
}






gls.right[5] = {
    ShowLspClient = {
      provider = function()
          local msg = ''
          local buf_ft = vim.api.nvim_buf_get_option(0, 'filetype')
          local clients = vim.lsp.get_active_clients()
          if next(clients) == nil then return msg end
          for _, client in ipairs(clients) do
            local filetypes = client.config.filetypes
            if filetypes and vim.fn.index(filetypes, buf_ft) ~= -1 then
              return "LSP[" .. client.name .. "]"
            end
          end
          return msg end,
      highlight = {colors.cyan, colors.bg},
    }
}


---------------------------------------------------


-- for line position and numbers
gls.right[6] = {
  LineColumn = {
      provider = function ()
        local max_lines = vim.fn.line('$')
        local line = vim.fn.line('.')
        local column = vim.fn.col('.')
        local ld = " "
        if string.len(max_lines) > 3 then
          ld = string.format("  %5d/%2d :%6d ", line, column, max_lines)
        end
        ld =string.format("  %3d/%2d :%2d ", line, column, max_lines)
        return ld
      end,
      highlight = {"#000000", colors.blue},
    }
}



---------------------------------------------------

-- for inactive/focused window
gls.short_line_left[1] = {
  FileName = {
    provider = 'FileName',
    separator =' ',
    highlight = { colors.fg, colors.bg_inactive },
    separator_highlight = { colors.fg, colors.bg_inactive },
  }
}

