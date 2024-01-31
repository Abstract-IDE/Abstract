local M = {}

local function abstract_default()
	require("abstract.defaults.config")
	require("abstract.defaults.mapping")
end

local function plugin()
	require("abstract.lazy").setup()
end

function M.setup()
	abstract_default()
	plugin()
end

return M
