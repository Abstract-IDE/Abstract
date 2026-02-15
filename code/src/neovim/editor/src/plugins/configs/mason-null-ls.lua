--[[
────────────────────────────────────────────────
Plugin: mason-null-ls.nvim
Source: https://github.com/jay-babu/mason-null-ls.nvim

mason-null-ls bridges mason.nvim with the null-ls plugin
 - making it easier to use both plugins together.
────────────────────────────────────────────────
--]]

local _ = --@spec
{
    "jay-babu/mason-null-ls.nvim",
    lazy = true,
    event = { "BufReadPre", "BufNewFile" },
}
--@end

--@setup
require("mason-null-ls").setup({
    -- A list of sources to install if they're not already installed.
    ensure_installed = {},
    -- Enable or disable null-ls methods to get set up
    -- This setting is useful if some functionality is handled by other plugins such as `conform` and `nvim-lint`
    methods = {
        formatting = true,
        code_actions = true,
    },
    automatic_installation = false,
    handlers = {},
})
--@end
