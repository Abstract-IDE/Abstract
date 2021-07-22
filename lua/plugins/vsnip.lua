--"---------------------------------------------------------------------
--"   PluginName: vim-vsnip
--"   Github:     github.com/hrsh7th/vim-vsnip
--
--"---------------------------------------------------------------------




--"---------------------------------------------------------------------
--"       CONFIGS
--"---------------------------------------------------------------------
--" location to store vsnip files
vim.g.vsnip_snippet_dir = vim.fn.expand('~/.config/nvim/extra/snippets/vsnip')

-- How to use LSP snippet?
local capabilities = vim.lsp.protocol.make_client_capabilities()
capabilities.textDocument.completion.completionItem.snippetSupport = true
capabilities.textDocument.completion.completionItem.resolveSupport = {
  properties = {
    'documentation',
    'detail',
    'additionalTextEdits',
  }
}

require'lspconfig'.rust_analyzer.setup {
  capabilities = capabilities,
}
---------------------------------------------------------------------
--       end of CONFIGS
---------------------------------------------------------------------





-----------------------------------------------------------------------
--       MAPPINGS
-----------------------------------------------------------------------

--[[
-- we difine moving in lua/plugin_configs/compe.lua file because vicompie and
-- vsnip could conflict or override one another's mappig so we defined Mapping
-- smartly so that it will work for both without any unexpected result

-- pmenu and vim-vsnip
local keymap = vim.api.nvim_set_keymap
options = {silent=true, expr=true}


-- Jump forward or backward
-- pmenu and vim-vsnip
keymap('i', '<Tab>',        "pumvisible() ? '<C-n>' : vsnip#jumpable(1)     ? '<Plug>(vsnip-jump-next)' : '<Tab>'",        options)
keymap('i', '<S-Tab>',      "pumvisible() ? '<C-n>' : vsnip#jumpable(-1)    ? '<Plug>(vsnip-jump-prev)' : '<S-Tab>'",      options)
-- vim-vsnip
keymap('s', '<Tab>',        "vsnip#jumpable(1)  ? '<Plug>(vsnip-jump-next)' : '<Tab>'",                                     options)
keymap('s', '<S-Tab>',      "vsnip#jumpable(-1)  ? '<Plug>(vsnip-jump-prev)' : '<Tab>'",                                    options)
]]
---------------------------------------------------------------------
--       enf of MAPPINGS
---------------------------------------------------------------------






