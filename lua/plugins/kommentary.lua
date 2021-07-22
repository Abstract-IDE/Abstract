--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--
--─────────────────────────────────────────────────--
--   Plugin:    kommentary
--   Github:    github.com/b3nj5m1n/kommentary
--─────────────────────────────────────────────────--
--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--




--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━━━❰ configs ❱━━━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--

-- For global
require('kommentary.config').configure_language(
  "default",
  {
    prefer_single_line_comments = true,
  }
)

--Configure unsupported language
require('kommentary.config').configure_language(
  "Rust",
  {
    single_line_comment_string = "//",
    multi_line_comment_strings = {"/*", "*/"},
  }
)

require('kommentary.config').configure_language(
  "cpp",
  {
    single_line_comment_string = "//",
    multi_line_comment_strings = {"/*", "*/"},
  }
)



--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━❰ end configs ❱━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--







--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━━❰ Mappings ❱━━━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--
-- If you don't want to use the default mappings,
vim.g.kommentary_create_default_mappings = false


--[[
vim.api.nvim_set_keymap("n", "<leader>cic", "<Plug>kommentary_line_increase", {})
vim.api.nvim_set_keymap("n", "<leader>ci", "<Plug>kommentary_motion_increase", {})
vim.api.nvim_set_keymap("x", "<leader>ci", "<Plug>kommentary_visual_increase", {})
vim.api.nvim_set_keymap("n", "<leader>cdc", "<Plug>kommentary_line_decrease", {})
vim.api.nvim_set_keymap("n", "<leader>cd", "<Plug>kommentary_motion_decrease", {})
vim.api.nvim_set_keymap("x", "<leader>cd", "<Plug>kommentary_visual_decrease", {})
]]

vim.api.nvim_set_keymap("n", "cc", "<Plug>kommentary_line_default", {})
vim.api.nvim_set_keymap("n", "cm", "<Plug>kommentary_motion_default", {})
vim.api.nvim_set_keymap("x", "cmc", "<Plug>kommentary_visual_default", {})


--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━❰ end Mappings ❱━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--

