--[[-------------------------------------------------------------------
   	PluginName: lualine.nvim
   	Github:     github.com/hoob3rt/lualine.nvim
--]]-------------------------------------------------------------------





-- -------------------------------------------------------------------
--       CONFIGS
-- -------------------------------------------------------------------

-- -------- USER DEFINED ---------------
-- Color table for highlights
local colors = {
  bg            = '#202328',
  fg            = '#bbc2cf',
  black         = '#1c1e26',
  white         = '#FFFFFF',
  white1        = '#6C6F93',
  red           = '#F43E5C',
  red_error     = '#800000',
  green         = '#09F7A0',
  blue          = '#25B2BC',
  yellow        = '#F09383',
  gray          = '#E95678',
  darkgray      = '#1A1C23',
  lightgray     = '#2E303E',
  inactivegray  = '#1C1E26',
  yellow        = '#ECBE7B',
  cyan          = '#008080',
  darkblue      = '#081633',
  orange        = '#FF8800',
  violet        = '#a9a1e1',
  magenta       = '#c678dd'
}


-- copied from https://gist.github.com/hoob3rt/b200435a765ca18f09f83580a606b878#file-evil_lualine-lua-L21
local conditions = {
  buffer_not_empty = function() return vim.fn.empty(vim.fn.expand('%:t')) ~= 1 end,
  hide_in_width = function() return vim.fn.winwidth(0) > 80 end,
  check_git_workspace = function()
    local filepath = vim.fn.expand('%:p:h')
    local gitdir = vim.fn.finddir('.git', filepath .. ';')
    return gitdir and #gitdir > 0 and #gitdir < #filepath
  end
}


-- copied and modified from https://github.com/hoob3rt/lualine.nvim/issues/77#issuecomment-765067579
function diagnostics_m()
  local errors              = 0
  local warnings            = 0
  local hints               = 0
  local cur_buf_errors      = 0
  local cur_buf_warnings    = 0
  local cur_buf_hints       = 0

  for _, buffer in ipairs(vim.fn['getbufinfo']()) do  -- Loop through buffers
      if buffer.listed == 1 and buffer.name ~= '' then  -- If the buffer is listed and it is not a no-name buffer
          local bufnr         = buffer.bufnr
          local buf_errors    = vim.lsp.diagnostic.get_count(bufnr, [[Error]])
          local buf_warnings  = vim.lsp.diagnostic.get_count(bufnr, [[Warning]])
          local buf_hints     = vim.lsp.diagnostic.get_count(bufnr, [[Hint]])

          errors = errors + buf_errors  -- Add this buffer's errors to the total errors
          warnings = warnings + buf_warnings  -- Same with warnings
          hints = hints + buf_hints

          if bufnr == vim.fn.bufnr() then  -- If this buffer is the currently open buffer
              cur_buf_errors      = buf_errors
              cur_buf_warnings    = buf_warnings
              cur_buf_hints       = buf_hints
          end
      end
  end
  if errors ~= 0 or warnings ~= 0 or hints~=0 then  -- If there is at least one error or warning or hints
    return '✗:' .. errors .. ' ⚠:' .. warnings .. ' :' .. hints
  else
      return ''
  end
end

 -- Lsp server name .
function lspservername()
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



local lsp_status = require('lsp-status')
lsp_status.register_progress()
local lsp_config = require('lspconfig')


--[[
---------------------------------------------
-- Some arbitrary servers
lsp_config.clangd.setup({
  --handlers = lsp_status.extensions.clangd.setup(),
  --on_attach = lsp_status.on_attach,
  capabilities = lsp_status.capabilities
})

lsp_config.pyright.setup({
  settings = { python = { workspaceSymbols = { enabled = true }}},
  on_attach = lsp_status.on_attach,
  capabilities = lsp_status.capabilities
})

lsp_config.ghcide.setup({
  on_attach = lsp_status.on_attach,
  capabilities = lsp_status.capabilities
})

lsp_config.rust_analyzer.setup({
  on_attach = lsp_status.on_attach,
  capabilities = lsp_status.capabilities
})
---------------------------------------------
]]



-- -------- end of USER DEFINED ---------------





--[[
Lualine has sections as shown below.
+-------------------------------------------------+
| A | B | C                             X | Y | Z |
+-------------------------------------------------+
]]



require'lualine'.setup {
    --[[ Setting a theme:
            All available themes are listed in  https://github.com/hoob3rt/lualine.nvim/blob/master/THEMES.md
            16color, ayu_dark, ayu_light, ayu_mirage, codedark, dracula, everforest, gruvbox, gruvbox_light, gruvbox_material,
            horizon, iceberg_dark, iceberg_light, jellybeans, material, modus_vivendi, molokai, nightfly, nord, oceanicnext,
            onedark, onelight, palenight, papercolor_dark, papercolor_light, powerline, seoul256, solarized_dark, solarized_light,
            tomorrow, wombat
         Symbols:
                    ║ ⋰    ∫
    ]]
  options = {
    theme                   = 'ayu_mirage',
    section_separators      = {'', ''},       -- separators between sections
    --component_separators    = {'⋰', '⋰'},
    disabled_filetypes      = {},               -- disable lualine for specific filetypes
    icons_enabled           = 1,                -- displays icons in alongside component
    --padding                 = 0,                -- adds padding to the left and right of components
    left_padding            = 0,                -- adds padding to the left of components
    right_padding           = 0,                -- adds padding to the right of components
    upper                   = false,            -- displays components in uppercase
    lower                   = false,            -- displays components in lowercase
    format                  = nil               -- format function, formats component's output
  },

  sections = {

    lualine_a   = {
        {
            'mode',
            right_padding =1,
            left_padding = 1,
            component_separators = ''
        },
        {
            'branch',
            -- icon = '',
            condition       = conditions.check_git_workspace,
            right_padding   =1,
            component_separators = ''
        },
        {
            'diff',
            symbols = {added = '+ ', modified = '~ ', removed = '- '},
            color_added     = colors.green,
            color_modified  = colors.orange,
            color_removed   = colors.red,
            condition       = conditions.hide_in_width,
            right_padding   = 1,
        },
    },


    lualine_b   = {
        {
            'filename',
            file_status     = true,     -- displays file status (readonly status, modified status)
            path            = 1,               -- 0 = just filename, 1 = relative path, 2 = absolute path
            condition       = conditions.buffer_not_empty,
            left_padding    = 1,                -- adds padding to the left of components
            right_padding   = 1,                -- adds padding to the right of components
            component_separators = '',
            icons_enabled   = true,
        },
        {
            'filetype',
            colored         = true, -- displays filetype icon in color if set to `true` ,
            format          = function() return " " end,

        },
        {
            'fileformat',
            upper           = true,
            icons_enabled   = true,
            left_padding    = 1,                -- adds padding to the right of components

            component_separators = ''
        },
        {
            'encoding',
            component_separators = '',
            left_padding    = 1,                -- adds padding to the right of components
            right_padding   = 1,
        },
    },

    lualine_c   = {''},



    lualine_x   = {
        {
            lsp_status.status,
            left_padding    = 1,                -- adds padding to the left of components
            right_padding   = 1,                -- adds padding to the right of components
            component_separators = '',
        },
        {
            diagnostics_m,
            color           = {fg=colors.red, gui='bold'} ,
            left_padding    = 1,                -- adds padding to the left of components
            right_padding   = 1,                -- adds padding to the right of components
            component_separators = '',
        },
        {
            lspservername,
            color           = {fg=colors.white, gui='bold'} ,
            left_padding    = 1,                -- adds padding to the left of components
            right_padding   = 1,                -- adds padding to the right of components
            component_separators = '',
        },

    },



    lualine_y   = {
    },

    lualine_z   = {
        {
            'progress',
            left_padding = 1,
        },
        {
            'location',
            right_padding = 0,
            icon =''
        },
    }
  },


  inactive_sections = {
    lualine_a   = {},
    lualine_b   = {},
    lualine_c   = {'filename'},
    lualine_x   = {'location'},
    lualine_y   = {},
    lualine_z   = {}
  },

  tabline = {},
  extensions = {}
}


















-- -------------------------------------------------------------------
--       end of CONFIGS
-- -------------------------------------------------------------------


