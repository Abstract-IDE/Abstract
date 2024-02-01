--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: tabby.nvim
Github: github.com/nanozuki/tabby.nvim

A declarative, highly configurable, and neovim style tabline plugin.
Use your nvim tabs as a workspace multiplexer!
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
	"nanozuki/tabby.nvim",
}

local tab = function()
	local tabs = tostring(#vim.api.nvim_list_tabpages())
	local tabpage = tostring(vim.api.nvim_tabpage_get_number(0))
	return tabpage .. "/" .. tabs
end

-- By default, Tabby counts all windows, resulting in the same name being repeatedly displayed.
-- This function addresses the issue by merging the duplicates.
local function wins_in_tab(line, theme)
	local unique_buffers = {}

	return line.wins_in_tab(line.api.get_current_tab()).foreach(function(win)
		local buf_name = win.buf_name()
		unique_buffers[buf_name] = unique_buffers[buf_name] or { is_current = false, lines = {} }
		unique_buffers[buf_name].is_current = unique_buffers[buf_name].is_current or win.is_current()

		if not vim.tbl_contains(unique_buffers[buf_name].lines, buf_name) then
			table.insert(unique_buffers[buf_name].lines, buf_name)
			local is_same_buff = vim.fn.bufnr(buf_name) == vim.fn.bufnr()

			local icon = " " .. win.file_icon() .. " "
			-- local icon = is_same_buff and "  " or "  "
			local hl = is_same_buff and theme.current_win or theme.win

			return {
				line.sep("", theme.win, theme.fill),
				icon,
				buf_name,
				line.sep("", theme.win, theme.tab),
				hl = hl,
				margin = "",
			}
		end
	end)
end

local theme = {
	fill = "TabLineFill",
	head = "TabLine",
	current_tab = "TabLineSel",
	tab = "TabLine",
	win = "TabLine",
	current_win = "TabLineCurrentWin",
	tail = "TabLine",
}

local view = function(line)
	return {
		{
			{ tab(), hl = theme.head },
			line.sep(" ", theme.head, theme.tab),
		},
		line.tabs().foreach(function(tab)
			local hl = tab.is_current() and theme.current_tab or theme.tab
			return {
				-- line.sep(" ", hl, theme.fill),
				-- tab.is_current() and " " or "󰆣 ",
				-- " " .. tab.number(),
				" ",
				tab.name(),
				" ",
				-- tab.close_btn(" "),
				-- line.sep(" ", hl, theme.tab),
				hl = hl,
				-- margin = " ",
			}
		end),
		line.sep(" ", theme.head, theme.tab),
		line.spacer(),
		wins_in_tab(line, theme),
		-- {
		-- 	line.sep("", theme.tail, theme.fill),
		-- 	{ "  ", hl = theme.tail },
		-- },
		-- hl = theme.fill,
	}
end

local opt = {
	buf_name = {
		mode = "unique", -- 'unique'|'relative'|'tail'|'shorten'
	},
}

spec.config = function()
	-- Save and restore in session
	-- You can save and restore tab layout and tab names in session, by adding word tabpages(for layout)
	-- and globals(for tab names) to vim.opt.sessionoptions. This is a valid sessionoptions:
	vim.opt.sessionoptions = "curdir,folds,globals,help,tabpages,terminal,winsize"
	-- At default, neovim only display tabline when there are at least two tab pages. If you want always display tabline:
	vim.o.showtabline = 1

	-- setup tabby
	require("tabby.tabline").set(view, opt)

	-- Mappings (:h tab)
	local options = { noremap = true, silent = true }
	local keymap = vim.api.nvim_set_keymap

	keymap("n", "<leader>q", ":tabclose<CR>", options)
	keymap("n", "<leader>Q", ":tabonly<CR>", options)
	-- navigate to previous/next tab
	keymap("n", "\\", ":tabn<CR>", options)
	keymap("n", "|", ":tabp<CR>", options)
	-- move current tab to previous/next position
	keymap("n", "<leader>,", ":-tabmove<CR>", options)
	keymap("n", "<leader>.", ":+tabmove<CR>", options)
end

return spec
