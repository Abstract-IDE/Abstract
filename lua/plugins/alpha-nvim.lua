
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ───────────────────────────────────────────────── --
--   Plugin:    alpha-nvim
--   Github:    github.com/goolord/alpha-nvim
-- ───────────────────────────────────────────────── --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --





-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━❰ configs ❱━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

local alpha_ok, alpha = pcall(require, "alpha")
if not alpha_ok then return end

local dashboard_ok, dashboard = pcall(require, "alpha.themes.dashboard")
if not dashboard_ok then return end

local datetime_ok, datetime = pcall(os.date, "%I:%M:%p 🕔 %d-%m-%Y")
local version_ok, nvim_version = pcall(os.capture, "nvim --version | awk 'NR == 1'")


local function footer()
	if version_ok and datetime_ok then
		return nvim_version .. ' | ' .. datetime
	elseif datetime_ok then
		return datetime
	else return ""
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
	-- "",
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

