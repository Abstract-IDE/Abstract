--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: abstract-cursor
Github: https://github.com/Abstract-IDE/abstract-autocmds

auto cmds
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
	"Abstract-IDE/abstract-autocmds",
	lazy = false,
}

spec.config = function()
	require("abstract-autocmds").setup({
		auto_resize_splited_window = true,
		remove_whitespace_on_save = true,
		no_autocomment_newline = true,
		clear_last_used_search = true,
		open_file_last_position = true,
		highlight_on_yank = {
			enable = true,
			opts = {
				timeout = 150,
			},
		},
		give_border = {
			enable = true,
			opts = {
				pattern = { "null-ls-info", "lspinfo" },
			},
		},
		smart_dd = true,
		visually_codeblock_shift = true,
		move_selected_upndown = true,
		go_back_normal_in_terminal = true,
		ctrl_backspace_delete = false,
		smart_visual_paste = true,
		dont_suspend_with_cz = true,
		smart_save_in_insert_mode = true,
	})
end

return spec
