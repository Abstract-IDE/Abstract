--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: nvim-dap
Source: https://github.com/mfussenegger/nvim-dap

Debug Adapter Protocol client implementation for Neovim
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
	"mfussenegger/nvim-dap",
	dependencies = { "nvim-neotest/nvim-nio" },
}

spec.config = function()
	require("abstract.utils.map").set_map("mfussenegger/nvim-dap")
end


return spec
