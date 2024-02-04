local M = {}

function M.set_plugin(name)
	local key = require("abstract.configs.mapping").plugin[name]
	require("which-key").register(key)
end

return M
