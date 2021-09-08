-- Set completeopt to have a better completion experience
vim.o.completeopt = 'menuone,noselect'

local cmp = require'cmp'

cmp.setup({

  completion = {
    --completeopt = 'menu,menuone,noinsert',
  },

  snippet = {
      expand = function(args)
        require('luasnip').lsp_expand(args.body)
      end,
  },

  formatting = {
      format = function(entry, vim_item)
        -- fancy icons and a name of kind
        vim_item.kind = require("lspkind").presets.default[vim_item.kind]

        -- set a name for each source
        vim_item.menu = ({
          buffer = "[Buff]",
          nvim_lsp = "[LSP]",
          luasnip = "[LuaSnip]",
          nvim_lua = "[Lua]",
          latex_symbols = "[Latex]",
        })[entry.source.name]
        return vim_item
      end,
  },

  sources = {
      { name = 'buffer' },
      { name = 'nvim_lsp' },
      { name = 'path' },
      { name = 'luasnip' },
    },

})


----------------------------
-- Require function for tab to work with LUA-SNIP
local check_back_space = function()
  local col = vim.fn.col '.' - 1
  return col == 0 or vim.fn.getline('.'):sub(col, col):match '%s' ~= nil
end
local luasnip = require("luasnip")

local t = function(str)
    return vim.api.nvim_replace_termcodes(str, true, true, true)
end
----------------------------


cmp.setup({
  mapping = {
    ['<C-Space>']   = cmp.mapping.complete(),
    ['<C-e>']       = cmp.mapping.close(),
    ['<C-u>']       = cmp.mapping.scroll_docs(-4),
    ['<C-d>']       = cmp.mapping.scroll_docs(4),
    ['<CR>']        = cmp.mapping.confirm({
              behavior = cmp.ConfirmBehavior.Replace,
              select  = true,
    }),
    ["<Tab>"] = cmp.mapping(function(fallback)
              if vim.fn.pumvisible() == 1 then
                vim.fn.feedkeys(t("<C-n>"), "n")
              elseif luasnip.expand_or_jumpable() then
                vim.fn.feedkeys(t("<Plug>luasnip-expand-or-jump"), "")
              elseif check_back_space() then
                vim.fn.feedkeys(t("<Tab>"), "n")
              else
                fallback()
              end
            end,
            { "i", "s", }
    ),
    ["<S-Tab>"] = cmp.mapping(function(fallback)
              if vim.fn.pumvisible() == 1 then
                vim.fn.feedkeys(t("<C-p>"), "n")
              elseif luasnip.jumpable(-1) then
                vim.fn.feedkeys(t("<Plug>luasnip-jump-prev"), "")
              else
                fallback()
              end
            end,
            { "i", "s", }
    ),


  }
})


