--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin:    penvim
Github:    https://github.com/Abstract-IDE/penvim

 project's root directory and documents indentation detector with project based config loader
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
	"Abstract-IDE/penvim",
}

spec.config = function()
	require("penvim").setup({

		project_env = {
			enable = true,
			config_name = ".__nvim__.lua",
		},

		rooter = {
			enable = true,
			patterns = { ".__nvim__.lua" },
		},

		indentor = {
			enable = true,
			indent_length = 4,
			indent_type = "auto", -- auto|tab|space
		},

		-- langs = {
		-- 	enable = true,
		-- }
	})
end

return spec
