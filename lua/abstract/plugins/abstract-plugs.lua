--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: abstract-plugs.nvim
Github: https://github.com/Abstract-IDE/abstract-plugs.nvim

collections of neovim plugins
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
	"Abstract-IDE/abstract-plugs.nvim",
}

spec.config = function()
	local abstract = require('abs')
	local set_map = require("abstract.utils.map").set_map

	require('abstract.plugins.extension.abstract-window').setup(abstract, set_map)
	require("abstract.plugins.extension.abstract-terminal").setup(abstract, set_map)
	abstract.whitespace().setup()
end

return spec
