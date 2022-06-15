
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ───────────────────────────────────────────────── --
--   Plugin:    alpha-nvim
--   Github:    github.com/goolord/alpha-nvim
-- ───────────────────────────────────────────────── --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --





-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━❰ configs ❱━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

local alpha = require("alpha")
local dashboard = require("alpha.themes.dashboard")
local datetime = os.date("%I:%M:%p 🕔 %d-%m-%Y")
local ok, nvim_version = pcall(os.capture, "nvim --version | awk 'NR == 1'")


local function footer()
	if ok then
		return nvim_version .. ' | ' .. datetime
	else
		return datetime
	end
end

-- Set header
dashboard.section.header.val = {
	" 																		",
	" ██████╗  ██████╗ ███████╗██╗  ██╗███╗   ██╗██╗██╗   ██╗██╗███╗   ███╗	",
	" ██╔══██╗██╔═══██╗██╔════╝██║  ██║████╗  ██║██║██║   ██║██║████╗ ████║	",
	" ██████╔╝██║   ██║███████╗███████║██╔██╗ ██║██║██║   ██║██║██╔████╔██║	",
	" ██╔══██╗██║   ██║╚════██║██╔══██║██║╚██╗██║██║╚██╗ ██╔╝██║██║╚██╔╝██║	",
	" ██║  ██║╚██████╔╝███████║██║  ██║██║ ╚████║██║ ╚████╔╝ ██║██║ ╚═╝ ██║	",
	" ╚═╝  ╚═╝ ╚═════╝ ╚══════╝╚═╝  ╚═╝╚═╝  ╚═══╝╚═╝  ╚═══╝  ╚═╝╚═╝     ╚═╝	",
	"													",
}

-- Set menu
local options = {}
dashboard.section.buttons.val = {
	dashboard.button("e", "  > New file", ":ene <BAR> startinsert <CR>", options),
	dashboard.button("r", "  > Recent", ":Telescope oldfiles<CR>", options),
	dashboard.button("t", "  > Find text", ":Telescope live_grep <CR>", options),
	dashboard.button("f", "  > Find file", ":Telescope find_files<CR>", options),
	dashboard.button("s", "  > Settings", ":e ~/.config/nvim/init.lua <CR>", options),
	dashboard.button("u", "  > Update Plugins", ":PackerSync <CR>", options),
	dashboard.button("q", "  > Quit NVIM", ":qa<CR>", options),
}

dashboard.section.footer.val = footer()

-- Send config to alpha
alpha.setup(dashboard.opts)

-- Disable folding on alpha buffer
vim.cmd([[
    autocmd FileType alpha setlocal nofoldenable
]])

-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━❰ end configs ❱━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

