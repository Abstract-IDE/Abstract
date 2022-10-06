
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


local function footer()
	local datetime_ok, datetime = pcall(os.date, " %I:%M:%p (%d-%m-%Y)")
	local version_ok, nvim_version = pcall(os.capture, "nvim --version | awk 'NR == 1'")
	if datetime_ok then
		if version_ok then
			return  nvim_version .. ' | ' .. datetime
		end
		return datetime
	end
	return ""
end


-- Set header
dashboard.section.header.val = {
	"┃█████     ",
	"┃██ ██    ",
	"┃██  ██   ",
	"┃██ ████████",
	"┃██    ██ ",
	"┃██     ██",
}


-- Set menu
local options = {}
local update_cmd = ":FloatermNew cd ~/.config/nvim/ && "
update_cmd = update_cmd .. "echo updating configs... && "
update_cmd = update_cmd .. "git pull && "
update_cmd = update_cmd .. "echo updating plugins... && "
update_cmd = update_cmd .. "nvim --headless -c 'autocmd User PackerComplete quitall' -c PackerSync && "
update_cmd = update_cmd .. "echo updated && "
update_cmd = update_cmd .. " <CR><CR>"
dashboard.section.buttons.val = {
	dashboard.button("n", "  New file", ":ene <BAR> startinsert <CR>", options),
	dashboard.button("r", "  Recent", ":Telescope oldfiles<CR>", options),
	dashboard.button("s", "s  Sessions", ":SessionManager load_session<CR>", options),
	dashboard.button("f", "🔎 Find file", ":Telescope find_files<CR>", options),
	dashboard.button("u", "🔨 Update Plugins", update_cmd, options),
	dashboard.button("q", "  Quit NVIM", ":qa<CR>", options),
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

