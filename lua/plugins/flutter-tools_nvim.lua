
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ───────────────────────────────────────────────── --
--   Plugin:    flutter-tools.nvim
--   Github:    github.com/akinsho/flutter-tools.nvim
-- ───────────────────────────────────────────────── --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --





-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━❰ configs ❱━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

local import_flutter, flutter = pcall(require, 'flutter-tools')
if not import_flutter then return end


flutter.setup {

	ui = {
		-- the border type to use for all floating windows, the same options/formats
		-- used for ":h nvim_open_win" e.g. "single" | "shadow" | {<table-of-eight-chars>}
		border = "rounded",
	},

	debugger = { -- integrate with nvim dap + install dart code debugger
		enabled = false,
	},

	widget_guides = {enabled = true},
	closing_tags = {
		prefix = "//", -- character to use for close tag e.g. > Widget
		enabled = true, -- set to false to disable
	},

	dev_log = {
		open_cmd = "tabedit", -- command to use to open the log buffer
	},

	dev_tools = {
		autostart = false, -- autostart devtools server if not detected
		auto_open_browser = false, -- Automatically opens devtools in the browser
	},

	outline = {
		open_cmd = "35vnew", -- command to use to open the outline buffer
	},
}

--- Telescope Integration
---      In order to set this up, you can explicitly load the extension.
local import_telescope, telescope = pcall(require, 'telescope')
if import_telescope then
	telescope.load_extension("flutter")
end

--   --> Highlights/Colors
--                   Widget guides
-- FlutterWidgetGuides       - this changes the highlight of the Widget guides
--                   Notifications
-- The highlights for flutter-tools notifications and popups can be changed by overriding the default highlight groups
-- FlutterNotificationNormal - this changes the highlight of the notification content.
-- FlutterNotificationBorder - this changes the highlight of the notification border.
-- FlutterPopupNormal        - this changes the highlight of the popup content.
-- FlutterPopupBorder        - this changes the highlight of the popup border.
-- FlutterPopupSelected      - this changes the highlight of the popup's selected line.

vim.cmd([[ autocmd VimEnter *.dart highlight FlutterWidgetGuides guifg=#8C8C8C ]])

-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━❰ end configs ❱━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --




-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━❰ Mappings ❱━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

-- to list available commands in flutter-tools plugin
vim.cmd([[ autocmd VimEnter *.dart  nnoremap <Leader>o :Telescope flutter commands <CR> ]])

-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━❰ end Mappings ❱━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

