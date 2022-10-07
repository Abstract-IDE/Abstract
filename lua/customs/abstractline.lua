
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ───────────────────────────────────────────────── --
-- infinite statusline on the earth and they all sucks.
-- by @Abstract
-- ───────────────────────────────────────────────── --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --





-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━❰ functions ❱━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

local api = vim.api
local bo  = vim.bo
local cmd = vim.cmd
local fn  = vim.fn

local foreground_color = "#b7b7b7"
local background_color = "#072b2c"
local global_color     = "#141414"

local exclude_filetype = {
	-- this block of code is inspired from: https://github.com/cseickel/dotfiles/blob/main/config/nvim/lua/status.lua#L40
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


local function init_highlight()
	vim.api.nvim_set_hl(0, "Abstractline",           {fg=foreground_color, bg=background_color})
	vim.api.nvim_set_hl(0, "AbstractlineFilemodify", {fg="#ff0000",        bg=background_color})
	vim.api.nvim_set_hl(0, "AbstractlineFilename",   {fg=foreground_color, bg=background_color, italic=true})
	vim.api.nvim_set_hl(0, "AbstractlineFilesize",   {fg=foreground_color, bg=global_color})
	vim.api.nvim_set_hl(0, "AbstractlineGit",        {fg="#b44200",        bg=global_color, bold=true})
	vim.api.nvim_set_hl(0, "AbstractlineGitAdded",   {fg="#4c7f33",        bg=global_color, bold=true})
	vim.api.nvim_set_hl(0, "AbstractlineGitChanged", {fg="#985401",        bg=global_color, bold=true})
	vim.api.nvim_set_hl(0, "AbstractlineGitRemoved", {fg="#d10000",        bg=global_color, bold=true})
	vim.api.nvim_set_hl(0, "AbstractlineLsprovider", {fg=foreground_color, bg=global_color})
	vim.api.nvim_set_hl(0, "AbstractlineLsprovidername",{fg="#51A0CF",     bg=global_color})
	vim.api.nvim_set_hl(0, "AbstractlinePsedostring",{fg="#060606", bg="#060606"})
	vim.api.nvim_set_hl(0, "AbstractlineSearch",     {fg="#abab18",        bg=global_color})
	vim.api.nvim_set_hl(0, "AbstractlineSplitter",   {fg=background_color, bg=global_color})

	vim.api.nvim_set_hl(0, "AbstractlineLSPDiagError",{fg="#a81818", bg=global_color})
	vim.api.nvim_set_hl(0, "AbstractlineLSPDiagWarn", {fg="#5d5d00", bg=global_color})
	vim.api.nvim_set_hl(0, "AbstractlineLSPDiagHint", {fg="#336481", bg=global_color})
	vim.api.nvim_set_hl(0, "AbstractlineLSPDiagInfo", {fg="#812900", bg=global_color})
end


local function splitter(icon)
	return "%#AbstractlineSplitter#" ..  icon .. "%*"
end


local function vim_mode()
	-- get mode (normal/insert/visual/command)
	local mode_color
	local mode = api.nvim_exec('echo mode()', true)
	if mode == "" then
		mode =  "C"
		mode_color = "#ffaa00"
		vim.api.nvim_set_hl(0, "AbstractlineMode", {fg=mode_color, bg=global_color, bold=true})
	elseif mode == "n" then
		mode_color = "#60a040"
		vim.api.nvim_set_hl(0, "AbstractlineMode", {fg=mode_color, bg=global_color, bold=true})
	elseif mode == "i" then
		mode_color = "#ff0000"
		vim.api.nvim_set_hl(0, "AbstractlineMode", {fg=mode_color, bg=global_color, bold=true})
	elseif mode == "v" then
		mode_color = "#428cbd"
		vim.api.nvim_set_hl(0, "AbstractlineMode", {fg=mode_color, bg=global_color, bold=true})
	else
		mode_color = "#f12bff"
		vim.api.nvim_set_hl(0, "AbstractlineMode", {fg=mode_color, bg=global_color, bold=true})
	end
	mode = string.upper(mode)
	return " " .. "%#AbstractlineMode#".. mode .. "%*" .. " "
end


local function get_filetype_icon()
	local has_devicons, devicons = pcall(require, 'nvim-web-devicons')
	if not has_devicons then
		return false
	end

	local file_name, file_ext = fn.expand('%:t'), fn.expand('%:e')

    local icon, icon_color = devicons.get_icon_color(file_name, file_ext, { default = true })
	return  {
		icon = icon,
		icon_color = icon_color
	}
end


local function file_info ()
	local modified_flag = ""
	local readonly_flag = ""
	if bo.modified  then modified_flag = "●" end
	if bo.readonly then readonly_flag = " 🔒" end

    local file_name = api.nvim_buf_get_name(0)
	local file = fn.pathshorten(fn.fnamemodify(file_name, ':~:.'))
	file = readonly_flag .. file .. " " ..  "%#AbstractlineFilemodify#" .. modified_flag .. "%*"

	local filetype = bo.filetype
	if filetype == "alpha" then
		file = filetype
		filetype = ""
	end

	local fileicon = get_filetype_icon()
	if fileicon then
		vim.api.nvim_set_hl(0, "AbstractlineFilenameIcon", {fg=fileicon.icon_color, bg=background_color})
		local icon_filetype = "%#AbstractlineFilenameIcon#" .. fileicon.icon .. " " .. filetype .. "%*"
		filetype = "%#Abstractline#" .. "[" ..  "%*".. icon_filetype .. "%#Abstractline#" .. "] " .. "%*"
		file =  "%#AbstractlineFilename#" .. " " .. file .. "%*"
	end

	return file .. filetype
end


local function git_status()
	local head = vim.b.gitsigns_head or ''
	if head == '' then
		return ''
	end
    local git_signs = vim.b.gitsigns_status_dict
	local icon = " "
	local sign_added = ""
	local sign_changed = ""
	local sign_removed = ""

    if git_signs then
		local sa = git_signs["added"] or 0
		local sr = git_signs["removed"] or 0
		local sc = git_signs["changed"] or 0

		if sa > 0 then
			sign_added   = "%#AbstractlineGitAdded#" .. "+" .. tostring(sa) .. "%*"
		end
		if sc > 0 then
			sign_changed = "%#AbstractlineGitChanged#" .. "~" .. tostring(sc) .. "%*"
		end
		if sr > 0 then
			sign_removed = "%#AbstractlineGitRemoved#" .. "-" .. tostring(sr) .. "%*"
		end
    end
	local signs = sign_added .. sign_changed .. sign_removed
	local result = "%#AbstractlineGit#" .. icon .. head .. "%*"
	if signs ~= '' then
		result = result .. "" .. signs
	end
	return " " .. result
end


local function get_filesize()
	-- https://github.com/echasnovski/mini.nvim/blob/793d40f807b3c0f959f19d15cc2fe814dc16938b/lua/mini/statusline.lua#L553
	local size = fn.getfsize(fn.getreg('%'))
	local result
	if size < 1 then
		result =  string.format('%dB ', 0)
	elseif size < 1024 then
		result =  string.format('%dB ', size)
	elseif size < 1048576 then
		result = string.format('%.2fK ', size / 1024)
	else
		result =  string.format('%.2fM ', size / 1048576)
	end
	return " " .. "%#AbstractlineFilesize#" .. result .. "%*"
end


local function search_info()
	-- thanks: https://github.com/nvim-lualine/lualine.nvim/issues/186#issuecomment-1170637440
	local final = ""
	local hlsearch = api.nvim_get_vvar("hlsearch")
	if hlsearch == nil then goto empty end
	if hlsearch == 1 then
		result = fn.searchcount({ maxcount = 999, timeout = 1000 })
		local total = result.total
		if total == nil then goto empty end
		if total > 0 then
			local search_string = fn.getreg("/")
			final = string.format("%s %d/%d", search_string, result.current, total)
		end
	end
	::empty::
	return "%#AbstractlineSearch#" .. final .. "%*" .. " "
end


local function line_info()
	local loc = api.nvim_buf_line_count(0) -- total lines of code in current file
	local line_col = fn.col(".")
    local curr_line_num = api.nvim_win_get_cursor(0)[1]
	if curr_line_num == nil then
		return
	end

	local loc_percentage =  math.ceil((100*curr_line_num)/loc)
	return  string.format("  %2d:%-2d- %d(%2d%%%%)",curr_line_num, line_col, loc, loc_percentage)
end


local function lsp_diagnostics_count()
	local diagnostic = vim.diagnostic

	local error= diagnostic.severity.ERROR
	local warn = diagnostic.severity.WARN
	local info = diagnostic.severity.INFO
	local hint = diagnostic.severity.HINT

	local count_error = vim.tbl_count(diagnostic.get(0, error and { severity = error }))
	local count_warn  = vim.tbl_count(diagnostic.get(0, warn and { severity = warn }))
	local count_info  = vim.tbl_count(diagnostic.get(0, info and { severity = info }))
	local count_hint  = vim.tbl_count(diagnostic.get(0, hint and { severity = hint }))

	return {
		count_error = count_error,
		count_warn = count_warn,
		count_info = count_info,
		count_hint = count_hint
	}
end


local function lsp_provider()
	local msg = ''
	local buf_ft = api.nvim_buf_get_option(0, 'filetype')
	local clients = vim.lsp.get_active_clients()
	if next(clients) == nil then
		return msg
	end

	local lsp_diag = lsp_diagnostics_count()
	local count_error = lsp_diag.count_error
	local count_warn = lsp_diag.count_warn
	local count_info = lsp_diag.count_info
	local count_hint = lsp_diag.count_hint
	local  sign_added, sign_changed, sign_removed, sign_hint = "", "", "", ""

	if count_error > 0 then
		sign_added   = "%#AbstractlineLSPDiagError#" .. "  " .. tostring(count_error) .. "%*"
	end
	if count_warn > 0 then
		sign_changed = "%#AbstractlineLSPDiagWarn#" .. "  " .. tostring(count_warn) .. "%*"
	end
	if count_info > 0 then
		sign_removed = "%#AbstractlineLSPDiagInfo#" .. "  " .. tostring(count_info) .. "%*"
	end
	if count_hint > 0 then
		sign_hint = "%#AbstractlineLSPDiagHint#" .. "  " .. tostring(count_hint) .. "%*"
	end

	for _, client in ipairs(clients) do
		local filetypes = client.config.filetypes
		if filetypes and vim.fn.index(filetypes, buf_ft) ~= -1 then
			if client.name ~= "null-ls" then
				local sign_count = sign_added .. sign_changed .. sign_removed .. sign_hint
				local msg1 = sign_count .. " " .. "%#abstractlineLsprovider#" .. "LSP[" .. "%*"
				local msg2 = "%#abstractlineLsprovider#" .. "]" .. "%*"
				msg = msg1 .. "%#abstractlineLsprovidername#" .. client.name .. "%*" .. msg2
				goto final
			end
		end
	end

	::final::
	return msg
end

-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━❰ end functions ❱━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --


function status_line()
	local filetype = bo.filetype
	if exclude_filetype[filetype] then
		-- a hack to make statusline line invisiable on alpha
		if filetype == "alpha" then
			local c = "----------"
			for _ = 1, 5 do
				c = c .. c
			end
			return "%#AbstractlinePsedostring#" .. c .. "%*"
		end
		return "%f"
	end

	return table.concat {
		-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
		-- ━━━━━━━━━━━━━━━━❰ LEFT ❱━━━━━━━━━━━━━━━━ --
		vim_mode(),
		splitter(""),
		file_info(),
		splitter(""),
		get_filesize(),
		git_status(),
		-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --


		-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
		-- ━━━━━━━━━━━━━━━❰ MIDDLE ❱━━━━━━━━━━━━━━━ --
		--        MIDDLE         --
		"%=",
		lsp_provider(),
		-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --


		-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
		-- ━━━━━━━━━━━━━━━━❰ RIGHT ❱━━━━━━━━━━━━━━━ --
		"%=",
		search_info(),
		"%#Abstractline#",
		line_info(),
		"%*",
		-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
	}
end


init_highlight()
vim.o.statusline = "%!luaeval('status_line()')"

