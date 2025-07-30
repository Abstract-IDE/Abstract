--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: mason-nvim-dap.nvim
Source: https://github.com/jay-babu/mason-nvim-dap.nvim

mason-nvim-dap bridges mason.nvim with the nvim-dap plugin
- making it easier to use both plugins together.
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
	"jay-babu/mason-nvim-dap.nvim",
}

spec.setup = function()
	require("mason-nvim-dap").setup({
		automatic_installation = false,
		ensure_installed = { "python", "delve" },
		handlers = {
			function(cnf)
				-- all sources with no handler get passed here

				-- Keep original functionality
				require('mason-nvim-dap').default_setup(cnf)
			end,
			-- python = function(cnf)
			-- 	config.adapters = {
			-- 		type = "executable",
			-- 		command = "/usr/bin/python3",
			-- 		args = {
			-- 			"-m",
			-- 			"debugpy.adapter",
			-- 		},
			-- 	}
			-- 	require('mason-nvim-dap').default_setup(cnf) -- don't forget this!
			-- end,
		},

	})
end

return spec
