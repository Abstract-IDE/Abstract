--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: nvim-java
Source: https://github.com/nvim-java/nvim-java

Painless Java in Neovim
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
	"nvim-java/nvim-java",
	lazy = true,
}

spec.config = function()
	require("java").setup()
end

return spec
