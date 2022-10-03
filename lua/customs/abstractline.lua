
-- infinite statusline on the earth and they all sucks.

local api = vim.api
local bo  = vim.bo
local cmd = vim.cmd
local fn  = vim.fn


local function vim_mode()
	-- get mode (normal/insert/visual/command)
	local mode = api.nvim_exec('echo mode()', true)
	if mode == "" then return " C " end
	return " " .. string.upper(mode) .. " "
end


local function file_info ()
	local modified_flag = ""
	local readonly_flag = ""
	if bo.modified  then modified_flag = "●" end
	if bo.readonly then readonly_flag = " 🔒" end

    local file_name = api.nvim_buf_get_name(0)
	local file = fn.pathshorten(fn.fnamemodify(file_name, ':~:.'))
	file = readonly_flag .. file .. " " .. modified_flag

	local filetype = bo.filetype
	if filetype == "alpha" then
		return ""
	end
	if filetype ==  "" then
		return file .. " "
	end

	filetype = "[" .. filetype .. "]"

	return file .. filetype .. " "
end


local function git_status()
	-- https://github.com/echasnovski/mini.nvim/blob/793d40f807b3c0f959f19d15cc2fe814dc16938b/lua/mini/statusline.lua#L305
	local head = vim.b.gitsigns_head or '-'
	local signs = (vim.b.gitsigns_status or '')
	local icon = ''

	if signs == '' then
		if head == '-' or head == '' then return '' end
		return string.format(' %s %s ', icon, head)
	end
	return string.format(' %s %s %s ', icon, head, signs)
end


local function get_filesize()
	-- https://github.com/echasnovski/mini.nvim/blob/793d40f807b3c0f959f19d15cc2fe814dc16938b/lua/mini/statusline.lua#L553
	local size = fn.getfsize(fn.getreg('%'))
	if size < 1024 then
		return string.format('%dB ', size)
	elseif size < 1048576 then
		return string.format('%.2fK ', size / 1024)
	else
		return string.format('%.2fM ', size / 1048576)
	end
end


local function get_filetype_icon()
	-- Have this `require()` here to not depend on plugin initialization order
	local has_devicons, devicons = pcall(require, 'nvim-web-devicons')
	if not has_devicons then return ' ' end
	local file_name, file_ext = fn.expand('%:t'), fn.expand('%:e')
	return devicons.get_icon(file_name, file_ext, {default = true}) .. " "
end


local function search_info()
	-- thanks: https://github.com/nvim-lualine/lualine.nvim/issues/186#issuecomment-1170637440
	local hlsearch = api.nvim_get_vvar("hlsearch")
	if hlsearch == nil then goto empty end
	if hlsearch == 1 then
		local result = fn.searchcount({ maxcount = 999, timeout = 1000 })
		local total = result.total
		if total == nil then goto empty end
		if total > 0 then
			local search_string = fn.getreg("/")
			return string.format("%s %d/%d", search_string, result.current, total)
		end
	end
	::empty::
	return ""
end


local function line_info()
	local loc = api.nvim_buf_line_count(0) -- total lines of code in current file
	local line_col = fn.col(".")
    local curr_line_num = api.nvim_win_get_cursor(0)[1]
	if curr_line_num == nil then return end

	local loc_percentage = function ()
		local result = math.ceil((100*curr_line_num)/loc)
		return "(" .. result .. "%%" .. ")"
	end
	return "  " .. curr_line_num .. ":" .. line_col .. " - " .. loc .. loc_percentage()
end


-- local function lsp_provider()
-- 	local msg = ''
-- 	local buf_ft = api.nvim_buf_get_option(0, 'filetype')
-- 	local clients = vim.lsp.get_active_clients()
-- 	if next(clients) == nil then return msg .. " " end
-- 	for _, client in ipairs(clients) do
-- 		local filetypes = client.config.filetypes
-- 		if filetypes and fn.index(filetypes, buf_ft) ~= -1 then
-- 			return "LSP[" .. client.name .. "] "
-- 		end
-- 	end
-- 	return msg
-- end

-- local function lsp_loading() return require('lsp-status').status() end
-------------------------------------------------------


-------------------------------------------------------
local function highlight(group, fg, bg)
    -- api.nvim_set_hl(0, group, { fg=f_g, bg=b_g })
	cmd("highlight " .. group .. " guifg=" .. fg .. " guibg=" .. bg)
end

highlight("StatusLeft", "#000000", "#FFFFFF")
highlight("StatusMid",  "#FFFFFF", "#141414")
highlight("StatusRight","#ff00ff", "#0f0fff")
highlight("ModeInsert", "#FFFFFF", "#000000")
highlight("FileIcon",   "#0000FF", "#FFFFFF")
-------------------------------------------------------


function status_line()

	local exclude_filetype = {
		["NvimTree"] = true,
		["Outline"] = true,
		["TelescopePrompt"] = true,
		["Trouble"] = true,
		["alpha"] = true,
		["dashboard"] = true,
		["lir"] = true,
		["neo-tree"] = true,
		["neogitstatus"] = true,
		["packer"] = true,
		["spectre_panel"] = true,
		["startify"] = true,
		["toggleterm"] = true,
	}
	if exclude_filetype[vim.bo.filetype] then
		return "%f"
	end


	return table.concat {
		---------------------------
		--         LEFT          --
		---------------------------
		vim_mode(),
		get_filetype_icon(),
		file_info(),
		get_filesize(),
		git_status(),
		---------------------------


		---------------------------
		--        MIDDLE         --
		---------------------------
		"%=",
		-- lsp_provider(),
		-- lsp_loading(),
		-- "%{&ff}",		-- file format
		-- "%y",			-- file type
		---------------------------


		---------------------------
		--         RIGHT         --
		---------------------------
		"%=",
		search_info(),
		line_info(),
		---------------------------
	}
end


vim.o.statusline = "%!luaeval('status_line()')"


-- function SpellToggle()
--     if vim.opt.spell:get() then
--         vim.opt_local.spell = false
--         vim.opt_local.spelllang = "en"
--     else
--         vim.opt_local.spell = true
--         vim.opt_local.spelllang = {"en_us", "de"}
--     end
-- end

-- -- statusline
-- local git_branch = function()
--     if vim.g.loaded_fugitive then
--         local branch = fn.FugitiveHead()
--         if branch ~= '' then return string.upper(" " .. branch) end
--     end
--     return ''
-- end

-- local file_path = function()
--     local buf_name = api.nvim_buf_get_name(0)
--     if buf_name == "" then return "[No Name]" end
--     local home = vim.env.HOME
--     local is_term = false
--     local file_dir = ""

--     if buf_name:sub(1, 5):find("term") ~= nil then
--         file_dir = vim.env.PWD
--         is_term = true
--     else
--         file_dir = fn.expand("%:p:h")
--     end

--     if file_dir:find(home, 0, true) ~= nil then
--         file_dir = file_dir:gsub(home, "~", 1)
--     end

--     if api.nvim_win_get_width(0) <= 80 then
--         file_dir = fn.pathshorten(file_dir)
--     end

--     if is_term then
--         return file_dir
--     else
--         return string.format("%s/%s", file_dir, fn.expand("%:t"))
--     end
-- end

-- local word_count = function()
--     if fn.wordcount().visual_words ~= nil then
--         return fn.wordcount().visual_words
--     else
--         return fn.wordcount().words
--     end
-- end

-- local modes = setmetatable({
--     ['n'] = {'NORMAL', 'N'},
--     ['no'] = {'N·OPERATOR', 'N·P'},
--     ['v'] = {'VISUAL', 'V'},
--     ['V'] = {'V·LINE', 'V·L'},
--     [''] = {'V·BLOCK', 'V·B'},
--     [''] = {'V·BLOCK', 'V·B'},
--     ['s'] = {'SELECT', 'S'},
--     ['S'] = {'S·LINE', 'S·L'},
--     [''] = {'S·BLOCK', 'S·B'},
--     ['i'] = {'INSERT', 'I'},
--     ['ic'] = {'INSERT', 'I'},
--     ['R'] = {'REPLACE', 'R'},
--     ['Rv'] = {'V·REPLACE', 'V·R'},
--     ['c'] = {'COMMAND', 'C'},
--     ['cv'] = {'VIM·EX', 'V·E'},
--     ['ce'] = {'EX', 'E'},
--     ['r'] = {'PROMPT', 'P'},
--     ['rm'] = {'MORE', 'M'},
--     ['r?'] = {'CONFIRM', 'C'},
--     ['!'] = {'SHELL', 'S'},
--     ['t'] = {'TERMINAL', 'T'}
-- }, {
--     __index = function()
--         return {'UNKNOWN', 'U'} -- handle edge cases
--     end
-- })

-- local get_current_mode = function()
--     local current_mode = api.nvim_get_mode().mode
--     if api.nvim_win_get_width(0) <= 80 then
--         return string.format('%s ', modes[current_mode][2])
--     else
--         return string.format('%s ', modes[current_mode][1])
--     end
-- end

-- ---@diagnostic disable-next-line: lowercase-global
-- function status_line()
--     return table.concat {
--         get_current_mode(), -- get current mode
--         "%{toupper(&spelllang)}", -- display language and if spell is on
--         git_branch(), -- branch name
--         " %<", -- spacing
--         file_path(), -- smart full path filename
--         "%h%m%r%w", -- help flag, modified, readonly, and preview
--         "%=", -- right align
--         "%{get(b:,'gitsigns_status','')}[", -- gitsigns
--         word_count(), -- word count
--         "][%-3.(%l|%c]", -- line number, column number
--         "[%{strlen(&ft)?&ft[0].&ft[1:]:'None'}]" -- file type
--     }
-- end

-- vim.opt.statusline = "%!v:lua.status_line()"

