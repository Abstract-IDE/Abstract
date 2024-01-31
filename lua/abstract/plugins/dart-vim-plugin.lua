-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ───────────────────────────────────────────────── --
--   Plugin:    dart-vim-plugin
--   Github:    github.com/dart-lang/dart-vim-plugin
-- ───────────────────────────────────────────────── --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --




return {
	"dart-lang/dart-vim-plugin", -- Syntax highlighting for Dart in Vim
	ft = {"dart"},
	config = function()
		-- Enable HTML syntax highlighting inside Dart strings  (default: false)
		vim.g.dart_html_in_string = "v.true"
		-- Enable Dart style guide syntax (like 2-space indentation)
		vim.g.dart_style_guide = 2
		-- Enable DartFmt execution on buffer save with
		vim.g.dart_format_on_save = 0
	end,
}
