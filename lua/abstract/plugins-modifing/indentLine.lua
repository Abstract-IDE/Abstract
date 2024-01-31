--"---------------------------------------------------------------------
--"       PluginName: indentLine
--"       Github:     github.com/Yggdroot/indentLine
--"---------------------------------------------------------------------
--
--
--
--
--
--"---------------------------------------------------------------------
--"       CONFIGS
--"---------------------------------------------------------------------
--
--" Change Conceal Behaviour
--" This plugin enables the Vim conceal feature which automatically hides stretches of text based on syntax highlighting.
--" For example, users utilizing the built in json.vim syntax file will no longer see quotation marks in their JSON files.
--" so for .json file, disable conceal Behaviour

vim.cmd([[ autocmd BufRead,BufNewFile *.json let g:indentLine_setConceal = 0 ]])

--" enable/disable indent line guide
vim.g.indentLine_enabled = 1

--" Change Indent Char
vim.g.indentLine_char = '▏'
--" to display more beautiful lines.
--" let g:indentLine_char_list = ['|', '¦', '┆', '┊']
--
--" customize conceal color(indent line guide color )
vim.g.indentLine_color_gui = '#222F22'

--"---------------------------------------------------------------------
--"       end of CONFIGS
--"---------------------------------------------------------------------
--
