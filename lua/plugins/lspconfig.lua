--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--
--─────────────────────────────────────────────────--
--   Plugin:    nvim-lspconfig
--   Github:    github.com/neovim/nvim-lspconfig
--─────────────────────────────────────────────────--
--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--






--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━━━❰ Mappings ❱━━━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--



-- Use an on_attach function to only map the following keys
-- after the language server attaches to the current buffer
--─────────────────────────────────────────────────--
local on_attach = function(client, bufnr)

  local function buf_set_keymap(...) vim.api.nvim_buf_set_keymap(bufnr, ...) end
  local function buf_set_option(...) vim.api.nvim_buf_set_option(bufnr, ...) end

  -- Enable completion triggered by <c-x><c-o>
  buf_set_option('omnifunc', 'v:lua.vim.lsp.omnifunc')

  -- Mappings.
  local opts = { noremap=true, silent=true }

  -- See `:help vim.lsp.*` for documentation on any of the below functions
--─────────────────────────────────────────────────--
  buf_set_keymap('n', '[ds',    '<cmd>lua vim.lsp.diagnostic.show_line_diagnostics()<CR>',  opts)
  buf_set_keymap('n', '[g',     '<cmd>lua vim.lsp.diagnostic.goto_prev()<CR>',              opts)
  buf_set_keymap('n', ']g',     '<cmd>lua vim.lsp.diagnostic.goto_next()<CR>',              opts)
  buf_set_keymap('n', '[dl',    '<cmd>lua vim.lsp.diagnostic.set_loclist()<CR>',            opts)
  buf_set_keymap('n', '[gD',    '<Cmd>lua vim.lsp.buf.declaration()<CR>',                   opts)
  buf_set_keymap('n', '[gd',    '<Cmd>lua vim.lsp.buf.definition()<CR>',                    opts)
  buf_set_keymap('n', '[lh',    '<Cmd>lua vim.lsp.buf.hover()<CR>',                         opts)
  buf_set_keymap('n', '[gi',    '<cmd>lua vim.lsp.buf.implementation()<CR>',                opts)
  buf_set_keymap('n', '[ls',    '<cmd>lua vim.lsp.buf.signature_help()<CR>',                opts)
  buf_set_keymap('n', '[D',     '<cmd>lua vim.lsp.buf.type_definition()<CR>',         opts)
  buf_set_keymap('n', '[rn',    '<cmd>lua vim.lsp.buf.rename()<CR>',                         opts)
  buf_set_keymap('n', '[gr',    '<cmd>lua vim.lsp.buf.references()<CR>',                    opts)
  buf_set_keymap("n", "[lf",    '<cmd>lua vim.lsp.buf.formatting()<CR>',                    opts)

  -- code action is integrated with telescope, for more see "telescope.nvim.vim" file
  -- buf_set_keymap('n', '<leader>ca',    '<cmd>lua vim.lsp.buf.code_action()<CR>',                   opts)
  -- buf_set_keymap('n', '<leader>wa',    '<cmd>lua vim.lsp.buf.add_workleader_folder()<CR>',          opts)
  -- buf_set_keymap('n', '<leader>wr',    '<cmd>lua vim.lsp.buf.remove_workleader_folder()<CR>',       opts)
  -- buf_set_keymap('n', '<leader>wl',   '<cmd>lua print(vim.inspect(vim.lsp.buf.list_workleader_folders()))<CR>', opts)
end

--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━❰ end Mappings ❱━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--









--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━━━❰ configs ❱━━━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--


-- options for lsp diagnostic
--─────────────────────────────────────────────────--
vim.lsp.handlers["textDocument/publishDiagnostics"] = vim.lsp.with(
  vim.lsp.diagnostic.on_publish_diagnostics, {
    underline = true,
    signs = true,
    update_in_insert = true,
    virtual_text = {
        true,
        spacing = 6,
        --severity_limit='Error'  -- Only show virtual text on error
    },
  }
)


-- show diagnostic on float window(like auto complete)
-- vim.api.nvim_command [[ autocmd CursorHold  *.lua,*.sh,*.bash,*.dart,*.py,*.cpp,*.c,js lua vim.lsp.diagnostic.show_line_diagnostics() ]]

-- se LSP diagnostic symbols/signs
--─────────────────────────────────────────────────--
vim.api.nvim_command [[ sign define LspDiagnosticsSignError         text=✗ texthl=LspDiagnosticsSignError       linehl= numhl= ]]
vim.api.nvim_command [[ sign define LspDiagnosticsSignWarning       text=⚠ texthl=LspDiagnosticsSignWarning     linehl= numhl= ]]
vim.api.nvim_command [[ sign define LspDiagnosticsSignInformation   text= texthl=LspDiagnosticsSignInformation linehl= numhl= ]]
vim.api.nvim_command [[ sign define LspDiagnosticsSignHint          text= texthl=LspDiagnosticsSignHint        linehl= numhl= ]]

-- Auto-format files prior to saving them
--vim.api.nvim_command[[autocmd BufWritePre <buffer> lua vim.lsp.buf.formatting_sync(nil, 1000)]]

--[[
   " to change colors better ot define in color scheme
   " highlight LspDiagnosticsUnderlineError         guifg=#EB4917 gui=undercurl
   " highlight LspDiagnosticsUnderlineWarning       guifg=#EBA217 gui=undercurl
   " highlight LspDiagnosticsUnderlineInformation   guifg=#17D6EB gui=undercurl
   " highlight LspDiagnosticsUnderlineHint          guifg=#17EB7A gui=undercurl
--]]






--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━❰ LSP engines options ❱━━━━━━━━━━━━━--

-- Use a loop to conveniently call 'setup' on multiple servers and
-- map buffer local keybindings when the language server attaches
--─────────────────────────────────────────────────--
--[[ NOTE: you can use
          local servers = require'lspinstall'.installed_servers()
        to get all installed servers but this required an extra
        plugin called "kabouzeid/nvim-lspinstall". though i'm using
        this plugin to install LSPs, i'm not going to use require'lspinstall'.installed_servers()
        because i will manually pass LSP name because there could be more than one LSP for one
        language.

]]--


local function setup_servers(servers)
  for _, server in pairs(servers) do
    require'lspconfig'[server].setup{
      on_attach = on_attach,
      flags = {
        debounce_text_changes = 150,
      }
    }
  end
end

local servers = {
  "lua",            -- for lua
  "cpp",            -- for C/C++
  "python",         -- for Python
  "rust",           -- for Rust
  "json",           -- for Json
  "html",           -- for html
  "css",            -- for css
  "bash",           -- for bash
  "typescript",     -- for typescript / javascript
  --"deno",           -- for Deno, TypeScript/JavaScript
  --"diagnosticls",   -- for diagnosticls
  "cmake",          -- for cmake

}

require'lspinstall'.setup()
setup_servers(servers)

require'lspconfig'.lua.setup{ -- for Lua
  on_attach = on_attach,
  settings = {
    Lua = {
      diagnostics = {
        -- Get the language server to recognize the 'vim', 'use' global
        globals = {'vim', 'use'},
      },
      workspace = {
        -- Make the server aware of Neovim runtime files
        library = vim.api.nvim_get_runtime_file("", true),
      },
      -- Do not send telemetry data containing a randomized but unique identifier
      telemetry = {
        enable = false,
      },
    },
  }
}


--━━━━━━━━━━━━━━━━❰ end LSP options ❱━━━━━━━━━━━━━━--
--═══════════════════════════════════════════════════






--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━❰ end configs ❱━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--

