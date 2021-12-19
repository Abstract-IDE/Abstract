
-- infinite statusline on the earth and they all sucks.
-- will write my own statusline which will called roshniline

-------------------------------------------------------
local fn  = vim.fn
local cmd = vim.cmd

local function highlight(group, fg, bg)
    cmd("highlight " .. group .. " guifg=" .. fg .. " guibg=" .. bg)
end
-------------------------------------------------------


-------------------------------------------------------
highlight("StatusLeft", "#000000", "#FFFFFF")
highlight("StatusMid",	"#FFFFFF", "#141414")
highlight("StatusRight","#000000", "#FFFFFF")
highlight("ModeInsert", "#FFFFFF", "#000000")
highlight("FileIcon",	"#0000FF", "#FFFFFF")
-------------------------------------------------------


-------------------------------------------------------
local function git_status()
    -- https://github.com/echasnovski/mini.nvim/blob/793d40f807b3c0f959f19d15cc2fe814dc16938b/lua/mini/statusline.lua#L305
    local head = vim.b.gitsigns_head or '-'
    local signs = (vim.b.gitsigns_status or '')
    local icon = ''

    if signs == '' then
        if head == '-' or head == '' then return '' end
        return string.format('%s[%s]', icon, head)
    end
    return string.format('%s[%s] %s', icon, head, signs)
end


local function get_filesize()
    -- https://github.com/echasnovski/mini.nvim/blob/793d40f807b3c0f959f19d15cc2fe814dc16938b/lua/mini/statusline.lua#L553
    local size = vim.fn.getfsize(vim.fn.getreg('%'))
    if size < 1024 then
        return string.format('%dB', size)
    elseif size < 1048576 then
        return string.format('%.2fK', size / 1024)
    else
        return string.format('%.2fM', size / 1048576)
    end
end


local function get_filetype_icon()
    -- Have this `require()` here to not depend on plugin initialization order
    local has_devicons, devicons = pcall(require, 'nvim-web-devicons')
    if not has_devicons then return '' end
    local file_name, file_ext = vim.fn.expand('%:t'), vim.fn.expand('%:e')
    return devicons.get_icon(file_name, file_ext, {default = true})
end


local function lsp_provider()
    local msg = ''
    local buf_ft = vim.api.nvim_buf_get_option(0, 'filetype')
    local clients = vim.lsp.get_active_clients()
    if next(clients) == nil then return msg end
    for _, client in ipairs(clients) do
        local filetypes = client.config.filetypes
        if filetypes and vim.fn.index(filetypes, buf_ft) ~= -1 then
            return "LSP[" .. client.name .. "]"
        end
    end
    return msg
end


local function lsp_loading() return require('lsp-status').status() end

-------------------------------------------------------


-------------------------------------------------------
function status_line()
    return table.concat {
        -- LEFT
        "%#ModeInsert#",
		" ",
		vim.fn.mode(),		-- get mode (normal/insert/visual)
        " ",
		"%#StatusLeft#",
		" %f",				-- relative file path with file name
        "%#FileIcon#",
		get_filetype_icon(),
		"%#StatusLeft#",
		"%m",				-- modified flag
        " %r",				-- readonly flag
        "%#StatusMid#",
		" ",
		get_filesize(),
		" ", git_status(),
		"%#StatusMid#",

        -- MIDDLE
        "%=",
		lsp_provider(),
		lsp_loading(),
		-- "%{&ff}",		-- file format
        -- "%y",			-- file type

        -- RIGHT
        "%=",
		"%#StatusRight#",
		" ",
		fn.col("."),
		":%l/%L-%p%%"

    }
end

vim.o.statusline = "%!luaeval('status_line()')"
-------------------------------------------------------

