local M = {}

local setup_lazy = function()
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
	local final_plugins = {}

	for _, plugin in ipairs(plugins.plugins) do
		table.insert(final_plugins, { import = "abstract.plugins." .. plugin })
	end
	for _, plugin in ipairs(plugins.deps) do
		table.insert(final_plugins, { import = "abstract.plugins.deps." .. plugin })
	end
	-- user's plugins (~/.config/nvim/lua/plugins/)
	table.insert(final_plugins, { import = "plugins" })

	opts.spec = final_plugins
	require("lazy").setup(opts)
end

local setup_map = function()
	local add = require("which-key").add
	local mappings = require("abstract.configs.mapping")
	for _, map in ipairs(mappings.builtin) do
		add(map)
	end
	for _, map in ipairs(mappings.override) do
		add(map)
	end
	for _, map in ipairs(require("override.mapping")) do
		add(map)
	end
end

function M.setup()
	-- call abstract autocmds
	require("abstract.configs.autocmd").setup()
	-- set Abstract's def
	require("abstract.configs.vimopt")
	-- Override Abstract's default with user config
	dofile(vim.fn.stdpath("config") .. "/init.lua")
	setup_lazy()
	setup_map()
end

return M
