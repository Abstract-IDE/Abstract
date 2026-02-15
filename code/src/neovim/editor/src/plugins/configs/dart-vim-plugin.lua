--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: dart-vim-plugin
Source: https://github.com/dart-lang/dart-vim-plugin

dart-vim-plugin provides filetype detection, syntax highlighting,
and indentation for Dart code in Vim.
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

return {
    "dart-lang/dart-vim-plugin",
    lazy = true,
    ft = { "dart" },
    config = function()
        vim.g.dart_html_in_string = "v.true"
        vim.g.dart_style_guide = 2
        vim.g.dart_format_on_save = 0
    end,
}
