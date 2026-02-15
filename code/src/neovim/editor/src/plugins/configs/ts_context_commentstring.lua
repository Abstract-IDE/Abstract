--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: nvim-ts-context-commentstring
Source: https://github.com/JoosepAlviste/nvim-ts-context-commentstring

Neovim treesitter plugin for setting the commentstring based on the cursor location in a file.
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]


local _ =
--@spec
{
   "JoosepAlviste/nvim-ts-context-commentstring",
   lazy = true,
}
--@end


--@setup
-- skip backwards compatibility routines and speed up loading.
vim.g.skip_ts_context_commentstring_module = true
require("ts_context_commentstring").setup({
   enable_autocmd = false,
})
--@end
