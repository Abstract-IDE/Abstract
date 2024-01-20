
local group = vim.api.nvim_create_augroup("AbstractLineGroup", {clear=true})
local init_color_fg = vim.api.nvim_get_hl_by_name("CursorLineNr", true).foreground
local init_color_bg = vim.api.nvim_get_hl_by_name("CursorLineNr", true).background

vim.api.nvim_create_autocmd(
	{ 'ModeChanged', 'InsertLeave'},
	{
		desc = "change cursor color on mode change",
		group = group,
		callback = function()
			local mode = vim.api.nvim_get_mode().mode
			if mode == "i" then
				vim.api.nvim_set_hl(0, "CursorLineNr", {fg="#000000", bg="#ac3131", bold=true})
			elseif mode == "v" or mode == "V" or mode == "" then
				vim.api.nvim_set_hl(0, "CursorLineNr", {fg="#000000", bg="#d1d1d1", bold=true})
			else
				vim.api.nvim_set_hl(0, "CursorLineNr", {fg=init_color_fg, bg=init_color_bg, bold=true})
			end
		end,
	}
)

