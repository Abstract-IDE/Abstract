local M = {}
local wk = require("which-key")

function M.set_plugin(name)
	local key = require("abstract.configs.mapping").plugin[name]
	wk.register(key)
end

function M.set_map(map)
	wk.register(map)
end

return M
