local spec = {
    'nvim-treesitter/nvim-treesitter',
    lazy = false,
    build = ':TSUpdate',
}

spec.init = function()
    local register = vim.treesitter.language.register
    register("html", { "htmldjango" })
    register("bash", { "zsh" })
    register('xml', { 'svg', 'xslt' })
end

spec.config = function()
    require('nvim-treesitter').setup({
        -- Directory to install parsers and queries to (prepended to `runtimepath` to have priority)
        -- NOTE!: Remember to run vim.opt.runtimepath:append("/some/path/to/store/parsers")
        -- NOTE!: we are adding to rtp using lazy.nvim
        install_dir = "--[[@rs = $NVIM_TS_HOME ]]"
    })
end

return spec
