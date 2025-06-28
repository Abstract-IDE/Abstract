--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: Abstract-cs
Github: https://github.com/Abstract-IDE/Abstract-cs

 Colorscheme for (neo)vim written in lua,
 specially made for roshnivim with Tree-sitter support.
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
	"Abstract-IDE/Abstract-cs",
	branch = "rewrite-2",
}

spec.setup = function(opts)
	opts = opts or {}
	require("abstract_cs").setup(opts)
end

return spec
