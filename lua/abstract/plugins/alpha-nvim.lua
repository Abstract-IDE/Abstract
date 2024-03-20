--[[━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin:    alpha-nvim
Github:    https://github.com/goolord/alpha-nvim

a lua powered greeter like vim-startify / dashboard-nvim
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━]]

local spec = {
	"goolord/alpha-nvim",
	dependencies = {
		"nvim-tree/nvim-web-devicons",
		"nvim-lua/plenary.nvim",
	},
}

local function footer()
	local _datetime, datetime = pcall(os.date, " %I:%M:%p (%d-%m-%Y)")
	local version = vim.version()
	local nvim_verion = string.format("v%d.%d.%d ", version.major, version.minor, version.patch)
	if _datetime then
		return nvim_verion .. " | " .. datetime
	end
	return nvim_verion
end

spec.config = function()
	local dashboard = require("alpha.themes.dashboard")

	vim.api.nvim_set_hl(0, "AlphaDashboard", { fg = "#1D918B" })
	dashboard.section.header.opts.hl = "AlphaDashboard"
	dashboard.section.footer.opts.hl = "AlphaDashboard"

	dashboard.section.header.val = {
		"┃█████     ",
		"┃██ ██    ",
		"┃██  ██   ",
		"┃██ ████████",
		"┃██    ██ ",
		"┃██     ██",
	}
	local options = {}
	dashboard.section.buttons.val = {
		dashboard.button("n", "  New file", ":ene <BAR> startinsert <CR>", options),
		dashboard.button("r", "  Recent", ":Telescope oldfiles<CR>", options),
		dashboard.button("s", "s  Sessions", ":lua require('telescope')<CR>:SessionManager load_session<CR>", options),
		dashboard.button("f", "🔎 Find file", ":Telescope find_files<CR>", options),
		dashboard.button("q", "◼  Quit NVIM", ":qa<CR>", options),
		-- dashboard.button("u", "🔨 Update Plugins", update_cmd, options),
	}
	dashboard.section.footer.val = footer()
	require("alpha").setup(dashboard.opts)
end

return spec
