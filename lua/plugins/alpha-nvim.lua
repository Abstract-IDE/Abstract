--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--
--─────────────────────────────────────────────────--
--   Plugin:    alpha-nvim
--   Github:    github.com/goolord/alpha-nvim
--─────────────────────────────────────────────────--
--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--





--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━━━❰ configs ❱━━━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--

local alpha = require("alpha")
local dashboard = require("alpha.themes.dashboard")

local function footer()
	local datetime = os.date("%I:%M:%p | %d-%m-%Y")
	return datetime
end


-- Set header
dashboard.section.header.val = {
"																			",
"	██████╗  ██████╗ ███████╗██╗  ██╗███╗   ██╗██╗██╗   ██╗██╗███╗   ███╗	",
"	██╔══██╗██╔═══██╗██╔════╝██║  ██║████╗  ██║██║██║   ██║██║████╗ ████║	",
"	██████╔╝██║   ██║███████╗███████║██╔██╗ ██║██║██║   ██║██║██╔████╔██║	",
"	██╔══██╗██║   ██║╚════██║██╔══██║██║╚██╗██║██║╚██╗ ██╔╝██║██║╚██╔╝██║	",
"	██║  ██║╚██████╔╝███████║██║  ██║██║ ╚████║██║ ╚████╔╝ ██║██║ ╚═╝ ██║	",
"	╚═╝  ╚═╝ ╚═════╝ ╚══════╝╚═╝  ╚═╝╚═╝  ╚═══╝╚═╝  ╚═══╝  ╚═╝╚═╝     ╚═╝	",
"																			",
}

-- Set menu
dashboard.section.buttons.val = {
    dashboard.button( "e", "  > New file" ,	 ":ene <BAR> startinsert <CR>"),
    dashboard.button( "r", "  > Recent"   ,	 ":Telescope oldfiles<CR>"),
	dashboard.button( "t", "  > Find text",	 ":Telescope live_grep <CR>"),
    dashboard.button( "f", "  > Find file",	 ":Telescope find_files<CR>"),
    dashboard.button( "s", "  > Settings" ,	 ":e ~/.config/nvim/init.lua <CR>"),
    dashboard.button( "u", "  > Update Plugins",":PackerSync <CR>"),
    dashboard.button( "q", "  > Quit NVIM",	 ":qa<CR>"),
}

dashboard.section.footer.val = footer()

-- Send config to alpha
alpha.setup(dashboard.opts)

-- Disable folding on alpha buffer
vim.cmd([[
    autocmd FileType alpha setlocal nofoldenable
]])

--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━❰ end configs ❱━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--

