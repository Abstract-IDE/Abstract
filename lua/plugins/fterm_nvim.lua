
require'FTerm'.setup({
    border = 'double',
    dimensions  = {
        height = 0.9,
        width = 0.9,
    },
})

-- Example keybindings
local map = vim.api.nvim_set_keymap
local opts = { noremap = true, silent = true }
-- local run = ":w <CR> :lua require('FTerm').run('python ~/.config/nvim/extra/tools/lazy-builder/build.py -o ~/.cache/build_files/ -r 1 %')"

-- map('n', '<Leader>r', run, opts)

