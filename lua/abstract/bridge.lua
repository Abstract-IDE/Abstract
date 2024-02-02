local M = {}

local function mapping()
	local maps = require("abstract.configs.mapping")
	require("which-key").register(maps)
end

local function lazy()
	-- bootstrap lazy.nvim
	local opts = require("abstract.plugins.lazy").opts
	local install_path = opts.root .. "/lazy.nvim"
	if not vim.loop.fs_stat(install_path) then
		-- stylua: ignore
		vim.fn.system({ "git", "clone", "--filter=blob:none", "https://github.com/folke/lazy.nvim.git", "--branch=stable",
			install_path })
	end
	vim.opt.rtp:prepend(install_path)

	local plugins = require("abstract.configs.plugin")
	for i, plugin in ipairs(plugins) do
		plugins[i] = { import = "abstract.plugins." .. plugin }
	end
	opts.spec = plugins
	require("lazy").setup(opts)
end

function M.setup()
	-- set Abstract's default config
	require("abstract.configs.vimopt")
	-- Override Abstract's default with user config
	dofile(vim.fn.stdpath("config") .. "/init.lua")
	lazy()
	-- mapping()
end

return M
