--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--
--─────────────────────────────────────────────────--
--   Plugin:    nvim-compe
--   Github:    github.com/hrsh7th/nvim-compe
--─────────────────────────────────────────────────--
--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--




--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━━━❰ configs ❱━━━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--

--─────────────────────────────────────────────────--
vim.opt.completeopt    = 'menuone'-- show menu even if there is only one candidate
vim.opt.completeopt    = vim.opt.completeopt + 'noselect'   -- don't automatically select canditate
--─────────────────────────────────────────────────--

require'compe'.setup {
  enabled           = true;
  autocomplete      = true;
  debug             = false;
  min_length        = 1;
  preselect         = 'enable';
  throttle_time     = 80;
  source_timeout    = 200;
  resolve_timeout   = 800;
  incomplete_delay  = 400;
  max_abbr_width    = 100;
  max_kind_width    = 100;
  max_menu_width    = 100;
  documentation     = {
    --border = { "╔", "═" ,"╗", "║", "╝", "═", "╚", "║" },  -- the border option is the same as `|help nvim_open_win|`
    border = { "╭", "─" ,"╮", "│", "╯", "─", "╰", "│" },
    winhighlight = "NormalFloat:CompeDocumentation,FloatBorder:CompeDocumentationBorder",
    max_width      = math.floor(vim.o.columns * 0.5),
    min_width       = 30,
    max_height      = math.floor(vim.o.lines * 0.3),
    min_height      = 1,
  };

  source = {
    path        = true;
    buffer      = true;
    calc        = true;
    nvim_lsp    = true;
    nvim_lua    = true;
    vsnip       = true;
    ultisnips   = true;
    luasnip     = true;
  };
}

--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━❰ end configs ❱━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--




--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━━━❰ Mappings ❱━━━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--


local keymap = vim.api.nvim_set_keymap
local options = { noremap=true, expr=true, silent=true }

--keymap('i', '<CR>',     "compe#confirm('<CR>')",        options) --disabled because i'm using windwp/nvim-autopairs
keymap('i', '<C-Space>',"compe#complete()",               options)
keymap('i', '<C-e>',    "compe#close('<C-e>')",           options)
keymap('i', '<C-d>',    "compe#scroll({ 'delta': +4 })",  options)
keymap('i', '<C-u>',    "compe#scroll({ 'delta': -4 })",  options)





--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--
-- How to use tab to navigate completion menu?
--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--
--[[
Tab and S-Tab keys need to be mapped to <C-n> and <C-p>
when completion menu is visible. Following example will
use Tab and S-Tab (shift+tab) to navigate completion menu
and jump between vim-vsnip placeholders when possible:
--]]

local t = function(str)
  return vim.api.nvim_replace_termcodes(str, true, true, true)
end

local check_back_space = function()
    local col = vim.fn.col('.') - 1
    return col == 0 or vim.fn.getline('.'):sub(col, col):match('%s') ~= nil
end

-- Use (s-)tab to:
--- move to prev/next item in completion menuone
--- jump to prev/next snippet's placeholder
_G.tab_complete = function()
  if vim.fn.pumvisible() == 1 then
    return t "<C-n>"
  elseif vim.fn['vsnip#available'](1) == 1 then
    return t "<Plug>(vsnip-expand-or-jump)"
  elseif check_back_space() then
    return t "<Tab>"
  else
    -- return vim.fn['compe#complete']()
    return t "<Tab>"        -- so that tab don't autocomplete( auto complete is job of <C-Space>). OR comment this line and uncomment above one
  end
end
_G.s_tab_complete = function()
  if vim.fn.pumvisible() == 1 then
    return t "<C-p>"
  elseif vim.fn['vsnip#jumpable'](-1) == 1 then
    return t "<Plug>(vsnip-jump-prev)"
  else
    -- If <S-Tab> is not working in your terminal, change it to <C-h>
    return t "<S-Tab>"
  end
end

vim.api.nvim_set_keymap("i", "<Tab>",   "v:lua.tab_complete()",   {expr = true})
vim.api.nvim_set_keymap("s", "<Tab>",   "v:lua.tab_complete()",   {expr = true})
vim.api.nvim_set_keymap("i", "<S-Tab>", "v:lua.s_tab_complete()", {expr = true})
vim.api.nvim_set_keymap("s", "<S-Tab>", "v:lua.s_tab_complete()", {expr = true})

--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--
--  end tab to navigate
--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--








--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━❰ end Mappings ❱━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--

