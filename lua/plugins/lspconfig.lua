-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ───────────────────────────────────────────────── --
--   Plugin:    nvim-lspconfig
--   Github:    github.com/neovim/nvim-lspconfig
-- ───────────────────────────────────────────────── --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --




-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━❰ configs ❱━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

-- options for lsp diagnostic
-- ───────────────────────────────────────────────── --
vim.lsp.handlers["textDocument/publishDiagnostics"] =
    vim.lsp.with(vim.lsp.diagnostic.on_publish_diagnostics, {
        underline = true,
        signs = true,
        update_in_insert = true,
        virtual_text = {
            true,
            spacing = 6
            -- severity_limit='Error'  -- Only show virtual text on error
        }
    })
-- show diagnostic on float window(like auto complete)
-- vim.api.nvim_command [[ autocmd CursorHold  *.lua,*.sh,*.bash,*.dart,*.py,*.cpp,*.c,js lua vim.lsp.diagnostic.show_line_diagnostics() ]]

-- se LSP diagnostic symbols/signs
-- ─────────────────────────────────────────────────--
vim.api.nvim_command [[ sign define DiagnosticSignError text=✗ texthl=DiagnosticSignError linehl= numhl= ]]
vim.api.nvim_command [[ sign define DiagnosticSignWarn  text=⚠ texthl=DiagnosticSignWarn  linehl= numhl= ]]
vim.api.nvim_command [[ sign define DiagnosticSignInfo  text= texthl=DiagnosticSignInfo  linehl= numhl= ]]
vim.api.nvim_command [[ sign define DiagnosticSignHint  text= texthl=DiagnosticSignHint  linehl= numhl= ]]

vim.api.nvim_command [[ hi DiagnosticUnderlineError cterm=underline gui=underline guisp=#840000]]
vim.api.nvim_command [[ hi DiagnosticUnderlineHint cterm=underline  gui=underline guisp=#17D6EB ]]
vim.api.nvim_command [[ hi DiagnosticUnderlineWarn cterm=underline  gui=underline guisp=#2f2905 ]]

-- Auto-format files prior to saving them
-- vim.api.nvim_command[[autocmd BufWritePre <buffer> lua vim.lsp.buf.formatting_sync(nil, 1000)]]

--[[
   " to change colors, it's better to define in color scheme
   " highlight LspDiagnosticsUnderlineError         guifg=#EB4917 gui=undercurl
   " highlight LspDiagnosticsUnderlineWarning       guifg=#EBA217 gui=undercurl
   " highlight LspDiagnosticsUnderlineInformation   guifg=#17D6EB gui=undercurl
   " highlight LspDiagnosticsUnderlineHint          guifg=#17EB7A gui=undercurl
--]]

-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━❰ end configs ❱━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --




-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━❰ Mappings ❱━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

-- Use an on_attach function to only map the following keys
-- after the language server attaches to the current buffer
-- ───────────────────────────────────────────────── --
On_attach = function(client, bufnr)

    local function buf_set_keymap(...)
        vim.api.nvim_buf_set_keymap(bufnr, ...)
    end
    local function buf_set_option(...)
        vim.api.nvim_buf_set_option(bufnr, ...)
    end

    -- Enable completion triggered by <c-x><c-o>
    buf_set_option('omnifunc', 'v:lua.vim.lsp.omnifunc')

    -- Mappings.
    local opts = {noremap = true, silent = true}

    -- See `:help vim.lsp.*` for documentation on any of the below functions
    -- ───────────────────────────────────────────────── --
    buf_set_keymap('n', '<space>e', '<cmd>lua vim.lsp.diagnostic.show_line_diagnostics()<CR>', opts)
    buf_set_keymap('n', '[d',		'<cmd>lua vim.lsp.diagnostic.goto_prev()<CR>',	opts)
    buf_set_keymap('n', ']d',		'<cmd>lua vim.lsp.diagnostic.goto_next()<CR>',	opts)
    buf_set_keymap('n', '<space>q', '<cmd>lua vim.lsp.diagnostic.set_loclist()<CR>',opts)

    buf_set_keymap('n', 'gD',		'<Cmd>lua vim.lsp.buf.declaration()<CR>',		opts)
    buf_set_keymap('n', 'gd',		'<Cmd>lua vim.lsp.buf.definition()<CR>',		opts)
    buf_set_keymap('n', 'K',  		'<Cmd>lua vim.lsp.buf.hover()<CR>',				opts)
    buf_set_keymap('n', 'gi', 		'<cmd>lua vim.lsp.buf.implementation()<CR>',	opts)
    buf_set_keymap('n', '[ls',		'<cmd>lua vim.lsp.buf.signature_help()<CR>',	opts)
    buf_set_keymap('n', '<space>D', '<cmd>lua vim.lsp.buf.type_definition()<CR>',	opts)
    buf_set_keymap('n', '<space>rn','<cmd>lua vim.lsp.buf.rename()<CR>',			opts)
    buf_set_keymap('n', 'gr',		'<cmd>lua vim.lsp.buf.references()<CR>',		opts)
    buf_set_keymap("n", "<space>f", '<cmd>lua vim.lsp.buf.formatting()<CR>',		opts)

    -- code action is integrated with telescope, for more see "telescope.lua" file
    -- buf_set_keymap('n', '<space>ca',    '<cmd>lua vim.lsp.buf.code_action()<CR>',                   opts)
    -- buf_set_keymap('n', '<leader>wa',    '<cmd>lua vim.lsp.buf.add_workleader_folder()<CR>',          opts)
    -- buf_set_keymap('n', '<leader>wr',    '<cmd>lua vim.lsp.buf.remove_workleader_folder()<CR>',       opts)
    -- buf_set_keymap('n', '<leader>wl',   '<cmd>lua print(vim.inspect(vim.lsp.buf.list_workleader_folders()))<CR>', opts)
end

-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━❰ end Mappings ❱━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

