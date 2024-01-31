local M = {}

-- local plugins = {
-- 	"alpha-nvim",
-- 	"Abstract-cs",
-- 	-- ─────────────────────────────────────────────────
-- 	-- CORE:
-- 	-- ─────────────────────────────────────────────────
-- 	"abstract-autocmds",
-- 	"nvim-web-devicons",
-- 	-- "nvim-cmp",
-- 	-- "nvim-lspconfig",
-- 	-- "mason",
-- 	-- "none-ls",
-- 	-- "indent-blankline",
-- 	-- "Comment",
-- 	-- "nvim-ts-context-commentstring",
-- 	"gitsigns",
-- }

local NVIM_DATA_DIR = vim.fn.stdpath("data")
-- WARN: only for dev
local ABSTRACT_DIR = "/home/lazy/codeDNA/dev/Projects/neovim/Abstract"
-- local ABSTRACT_DIR = NVIM_DATA_DIR .. "/Abstract"

M.opts = {
	root = NVIM_DATA_DIR .. "/lazy/plugins", -- directory where plugins will be installed
	-- leave nil when passing the spec as the first argument to setup()
	spec = nil, ---@type LazySpec
	lockfile = NVIM_DATA_DIR .. "/lazy/lazy-lock.json", -- lockfile generated after running update.
	git = {
		log = { "-3" },                              -- show commits from the last 3 days
		timeout = 300,                               -- kill processes that take more than 5 minutes
	},
	dev = {
		-- directory where you store your local plugin projects
		path = "~/codeDNA/dev/Projects/neovim",
		---@type string[] plugins that match these patterns will use your local versions instead of being fetched from GitHub
		patterns = {}, -- For example {"folke"}
		fallback = false, -- Fallback to git when local plugin doesn't exist
	},
	install = {
		-- install missing plugins on startup. This doesn't increase startup time.
		missing = true,
		-- try to load one of these colorschemes when starting an installation during startup
		colorscheme = { "abscs", "default" },
	},
	ui = {
		-- a number <1 is a percentage., >1 is a fixed size
		size = { width = 0.8, height = 0.8 },
		wrap = true,  -- wrap the lines in the ui
		border = "rounded", -- The border to use for the UI window. Accepts same border values as |nvim_open_win()|.
		title_pos = "center", ---@type "center" | "left" | "right"
		throttle = 20, -- how frequently should the ui process render events
	},
	checker = {
		-- automatically check for plugin updates
		enabled = false,
		concurrency = nil, ---@type number? set to 1 to check for updates very slowly
		notify = true,  -- get a notification when new updates are found
		frequency = 3600, -- check for updates every hour
		check_pinned = false, -- check for pinned packages that can't be updated
	},
	performance = {
		cache = { enabled = true },
		reset_packpath = true, -- reset the package path to improve startup time
		rtp = {
			reset = true, -- reset the runtime path to $VIMRUNTIME and your config directory
			-- add any custom paths here that you want to includes in the rtp
			---@type string[]
			paths = {
				ABSTRACT_DIR,
			},
			---@type string[] list any plugins you want to disable here
			disabled_plugins = { "tutor" }, -- "gzip", "matchit", "matchparen", "netrwPlugin", "tarPlugin", "tohtml", "zipPlugin",
		},
	},
	-- lazy can generate helptags from the headings in markdown readme files,
	-- so :help works even for plugins that don't have vim docs.
	-- when the readme opens with :help it will be correctly displayed as markdown
	readme = {
		enabled = true,
		root = vim.fn.stdpath("state") .. "/lazy/readme",
		files = { "README.md", "lua/**/README.md" },
		-- only generate markdown helptags for plugins that dont have docs
		skip_if_doc_exists = true,
	},
	state = vim.fn.stdpath("state") .. "/lazy/state.json", -- state info for checker and other things
	-- Enable profiling of lazy.nvim. This will add some overhead,
	-- so only enable this when you are debugging lazy.nvim
	profiling = {
		-- Enables extra stats on the debug tab related to the loader cache.
		-- Additionally gathers stats about all package.loaders
		loader = false,
		-- Track each new require in the Lazy profiling tab
		require = false,
	},
}

return M
