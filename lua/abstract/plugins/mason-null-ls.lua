--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: mason-null-ls.nvim
Source: https://github.com/jay-babu/mason-null-ls.nvim

mason-null-ls bridges mason.nvim with the null-ls plugin
 - making it easier to use both plugins together.
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
	"jay-babu/mason-null-ls.nvim",
	lazy = true,
}

spec.setup = function()
	require("mason-null-ls").setup({
		ensure_installed = {
			-- Opt to list sources here, when available in mason.
		},
		automatic_installation = false,
		handlers = {},
	})
end

return spec
