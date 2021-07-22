--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--
--─────────────────────────────────────────────────--
--      Plugin: nvim-autopairs
--      Github: github.com/windwp/nvim-autopairs
--─────────────────────────────────────────────────--
--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--






--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━━━❰ Configs ❱━━━━━━━━━━━━━━━━━━━--


require('nvim-autopairs').setup({
    enable_check_bracket_line = true,                   -- Don't add pairs if it already have a close pairs in same line
    disable_filetype = { "TelescopePrompt" , "vim" },   --
    enable_afterquote = false                           -- add bracket pairs after quote

})

-- nvim-compe, because i'm using nvim-compe plugin for auto complete
require("nvim-autopairs.completion.compe").setup({
  map_cr = true,        --  map <CR> on insert mode
  map_complete = true   -- it will auto insert `(` after select function or method item
})

--          -->RULES
local Rule = require('nvim-autopairs.rule')
local npairs = require('nvim-autopairs')

-- before   insert  after
--  (|)     ( |)	( | )
npairs.add_rules({
Rule(' ', ' '):with_pair(function (opts)
  local pair = opts.line:sub(opts.col, opts.col+1)
  return vim.tbl_contains({'()', '[]', '{}'}, pair)
end)
})

--━━━━━━━━━━━━━━━━━❰ end Configs ❱━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--
