
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ───────────────────────────────────────────────── --
--    Plugin:    nvim-navic
--    Github:    github.com/SmiteshP/nvim-navic
-- ───────────────────────────────────────────────── --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --





-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━❰ configs ❱━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

local import_navic, navic = pcall(require, "nvim-navic")
if not import_navic then return end


navic.setup({
	highlight = true,
	separator = " > ",
	depth_limit = 0,
	depth_limit_indicator = "..",

	icons = {
		File = " ",
		Module = " ",
		Namespace = " ",
		Package = " ",
		Class = " ",
		Method = " ",
		Property = " ",
		Field = " ",
		Constructor = " ",
		Enum = " ",
		Interface = " ",
		Function = " ",
		Variable = " ",
		Constant = " ",
		String = " ",
		Number = " ",
		Boolean = " ",
		Array = " ",
		Object = " ",
		Key = " ",
		Null = " ",
		EnumMember = " ",
		Struct = " ",
		Event = " ",
		Operator = " ",
		TypeParameter = " ",
	},
})


local function set_highlight(group, fg, bg, style)
	local options =  { fg=fg, bg=bg }
	if style ~= nil then
		local  italic, bold = false, false
		if style == 'italic' then italic = true end
		if style == 'bold' then bold = true end
		options =  { fg=fg, bg=bg, italic=italic, bold=bold  }
	end
    vim.api.nvim_set_hl(0, group, options)
end


local function init_highlight()
	set_highlight("NavicFileName",          "#acacac", "#080808")
	set_highlight("NavicSeparator",         "#51578f", "#080808", "italic")
	set_highlight("NavicText",              "#666666", "#080808", "italic")

	set_highlight("NavicIconsArray",        "#8b1540", "#080808", "italic")
	set_highlight("NavicIconsBoolean",      "#015c8a", "#080808", "italic")
	set_highlight("NavicIconsClass",        "#8d5c18", "#080808", "italic")
	set_highlight("NavicIconsConstant",     "#FFFADE", "#080808", "italic")
	set_highlight("NavicIconsConstructor",  "#015e8c", "#080808", "italic")
	set_highlight("NavicIconsEnum",         "#168888", "#080808", "italic")
	set_highlight("NavicIconsEnumMember",   "#0c494a", "#080808", "italic")
	set_highlight("NavicIconsEvent",        "#8f476b", "#080808", "italic")
	set_highlight("NavicIconsField",        "#038286", "#080808", "italic")
	-- set_highlight("NavicIconsFile",         "#666666", "#080808", "italic")
	set_highlight("NavicIconsFunction",     "#722587", "#080808", "italic")
	set_highlight("NavicIconsInterface",    "#8b2c27", "#080808", "italic")
	set_highlight("NavicIconsKey",          "#8a0015", "#080808", "italic")
	set_highlight("NavicIconsMethod",       "#73538c", "#080808", "italic")
	set_highlight("NavicIconsModule",       "#8d7138", "#080808", "italic")
	set_highlight("NavicIconsNamespace",    "#FFFFFF", "#080808", "italic")
	set_highlight("NavicIconsNull",         "#8c0015", "#080808", "italic")
	set_highlight("NavicIconsNumber",       "#8a6e37", "#080808", "italic")
	set_highlight("NavicIconsObject",       "#8d008d", "#080808", "italic")
	set_highlight("NavicIconsOperator",     "#8b1540", "#080808", "italic")
	set_highlight("NavicIconsPackage",      "#005a83", "#080808", "italic")
	set_highlight("NavicIconsProperty",     "#008f6b", "#080808", "italic")
	set_highlight("NavicIconsString",       "#8a3c24", "#080808", "italic")
	set_highlight("NavicIconsStruct",       "#8c1717", "#080808", "italic")
	set_highlight("NavicIconsTypeParameter","#4d748f", "#080808", "italic")
	set_highlight("NavicIconsVariable",     "#008470", "#080808", "italic")
end

local function filetype_exclude(filetype)
	-- disable winbar in these filetypes
	local filetype_to_exclude = {
		-- "",
		"alpha",
		"dap-repl",
		"dap-terminal",
		"dapui_breakpoints",
		"dapui_console",
		"dapui_scopes",
		"dapui_stacks",
		"dapui_watches",
		"dashboard",
		"DressingSelect",
		"harpoon",
		"help",
		"Jaq",
		"lab",
		"lir",
		"Markdown",
		"neo-tree",
		"neogitstatus",
		"NvimTree",
		"Outline",
		"packer",
		"spectre_panel",
		"startify",
		"TelescopePrompt",
		"toggleterm",
		"Trouble",
	}
	if vim.tbl_contains(filetype_to_exclude, filetype) then
		return true
	end
	return false
end


local function get_filetype_icon()
	local import_devicons, devicons = pcall(require, 'nvim-web-devicons')
	if not import_devicons then
		return false
	end
	local file_name = vim.fn.expand('%:t')
	local ext = vim.fn.expand "%:e"
    local icon, icon_color = devicons.get_icon_color(file_name, ext, { default = true })

	return {
		icon = icon,
        icon_color = icon_color
	}
end


local function winbar()
	if filetype_exclude(vim.bo.filetype) then
		return
	end
	local filename = vim.fn.expand "%:t"
	local file = ""
	local winbar_data = ""

	if filename ~= "" or filename ~= nil then
		local dev_icon = get_filetype_icon()
		if dev_icon then
			local icon = dev_icon.icon
			local icon_color = dev_icon.icon_color
			if icon == "" or icon == nil then
				icon = " "
			end
			set_highlight("NavicFileIcon", icon_color, "#080808")
			file = " " .. "%#NavicFileIcon#" .. icon .. "%*" .. " " .. "%#NavicFileName#" .. filename .. "%*" .. " "
		end
		winbar_data = file

		if navic.is_available() then
			local status_getlocation, navic_location = pcall(navic.get_location, {})
			if not status_getlocation then
				goto continue_winbar
			end
			if  navic_location == "error" then
				goto continue_winbar
			end
			if not (navic_location == "" or navic_location == nil ) then
				winbar_data =  "%#NavicSeparator#"..">" .. "%*" .. " " .. navic_location
			else
				goto continue_winbar
			end
			winbar_data = file .. winbar_data
		end
	end

	::continue_winbar::
	local status_setwinbar, _ = pcall(vim.api.nvim_set_option_value, "winbar", winbar_data, { scope = "local" })
	if not status_setwinbar then
		return
	end
end


init_highlight()

vim.api.nvim_create_autocmd(
	{ "BufFilePost", "BufWinEnter", "BufWritePost", "CursorHold", "CursorMoved", "InsertEnter", "TabClosed" },
	{
		desc = "don't load winbar for certain filetypes and buffers",
		pattern = "*",
		group = vim.api.nvim_create_augroup("AbstractWinbarAutoGroup", {clear=true}),

		callback = function()
			winbar()
		end,
	}
)

-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━❰ end configs ❱━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

