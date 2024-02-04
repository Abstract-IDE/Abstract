--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin:    hop.nvim
Github:    https://github.com/smoka7/hop.nvim
           (forked of: https://github.com/phaazon/hop.nvim)

Hop is an EasyMotion-like plugin allowing you to jump
anywhere in a document with as few keystrokes as possible
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
	"smoka7/hop.nvim",
	version = "*",
	keys = { "f" },
}

spec.config = function()
	local hop = require("hop")
	hop.setup({
		keys = "qwertyuiopasdfghjklzxcvbnm",
		jump_on_sole_occurrence = false,
	})

	require("abstract.utils.map").set_plugin("smoka7/hop.nvim")
end

return spec
