-- --------------------------------------------------------------------
--   PluginName: lukas-reineke/indent-blankline.nvim
--   Github:     github.com/lukas-reineke/indent-blankline.nvim
-- --------------------------------------------------------------------




-- --------------------------------------------------------------------
--           CONFIGS
-- --------------------------------------------------------------------
local g =vim.g

--  Specifies a list of |filetype| values for which this plugin is enabled. All filetypes are enabled if the value is an empty list.
--g.indent_blankline_filetype = []
-- Specifies a list of |filetype| values for which this plugin is not enabled.  Ignored if the value is an empty list.
--g.indent_blankline_filetype_exclude = ['help']
-- Specifies the character to be used as indent line.
g.indent_blankline_char = '▏'
--     Specifies a list of characters to be used as indent line for each indentation level.
-- g.indent_blankline_char_list = ['|', '¦', '┆', '┊']
-- Specifies the character to be used as the space value in between indent lines.
g.indent_blankline_space_char = ' '
-- Specifies the character to be used as the space value in between indent lines when the line is blank.
g.indent_blankline_space_char_blankline = ' '
-- Specifies the maximum indent level to display.  (Default: 10)
g.indent_blankline_indent_level = 15


-- Highlight of indent character.
vim.cmd(' highlight IndentBlanklineChar guifg=#222111 gui=nocombine ')
-- Highlight of space character.
-- highlight IndentBlanklineSpaceChar guifg=#00FF00 gui=nocombine

--Specifies the list of character highlights for each indentation level.
-- g.indent_blankline_char_highlight_list = ['Error', 'Function']

-- Specifies the list of space character highlights for each indentation level.
-- g.indent_blankline_space_char_highlight_list = ['Error', 'Function']

-- --------------------------------------------------------------------
--           end of CONFIGS
-- --------------------------------------------------------------------


