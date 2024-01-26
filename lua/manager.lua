--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
We are using Lazy.vim to manage our plugins.
The configurations for each plugin are stored in their respective separate files
located in the "plugins/" directory, and these configurations are loaded by Lazy.nvim.

Repo: https://github.com/folke/lazy.nvim
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]




-- bootstrap lazy.nvim
local root_dir = vim.fn.stdpath("data") .. "/lazy"
local install_path = root_dir .. "/lazy.nvim"
if not vim.loop.fs_stat(install_path) then
	vim.fn.system({
		"git",
		"clone",
		"--filter=blob:none",
		"https://github.com/folke/lazy.nvim.git",
		"--branch=stable",
		install_path,
	})
end
vim.opt.rtp:prepend(install_path)

-- safely import packer
local _lazy, lazy = pcall(require, "lazy")
if not _lazy then
	return
end

-- ───────────────────────────────────────────────── --
-- Plugins Configurations
local plugins = {

	-- the colorscheme should be available when starting Neovim
	{              -- colorscheme for (neo)vim written in lua specially made for Abstract
		"Abstract-IDE/Abstract-cs",
		lazy = false, -- make sure we load this during startup if it is your main colorscheme
		priority = 1000, -- make sure to load this before all the other start plugins
		config = function() require("plugins/Abstract-cs") end
	},

	{ -- All the lua functions I don't want to write twice.
		"nvim-lua/plenary.nvim",
	},

	{ -- lua `fork` of vim-web-devicons for neovim
		"kyazdani42/nvim-web-devicons",
		config = function() require("plugins/nvim-web-devicons") end
	},

	{                  --  A plugin for profiling Vim and Neovim startup time.
		"dstein64/vim-startuptime",
		cmd = "StartupTime", -- lazy-load on a command
		-- init is called during startup. Configuration for vim plugins typically should be set in an init function
		init = function() vim.g.startuptime_tries = 10 end
	},

	{ -- A collection of common configurations for Neovim's built-in language server client
		"neovim/nvim-lspconfig",
		event = { "BufReadPre", "BufNewFile", "InsertEnter" },
		dependencies = {
			{ -- A pretty diagnostics, references, telescope results, quickfix and location list to help you solve all the trouble your code is causing.
				"folke/trouble.nvim",
				config = function() require("plugins/trouble_nvim") end
			},
			{ -- preview native LSP's goto definition calls in floating windows.
				"rmagatti/goto-preview",
				config = function() require("plugins/goto-preview") end
			},
			{ -- Standalone UI for nvim-lsp progress
				"j-hui/fidget.nvim",
				config = function() require("plugins/fidget_nivm") end
			},
			{ -- Neovim Winbar: Elevating Awesome. Simple, Dynamic, Navic-Powered.
				"Abstract-IDE/abstract-winbar",
				dependencies = {
					{ -- Simple winbar/statusline plugin that shows your current code context
						"SmiteshP/nvim-navic",
						config = function () require("plugins/nvim-navic") end
					}
				},
				config = function () require("plugins/abstract-winbar") end
			},
			{ --  LSP signature hint as you type
				"ray-x/lsp_signature.nvim",
				config = function() require("plugins/lsp_signature_nvim") end
			},
			{ "b0o/schemastore.nvim" }  -- A Neovim Lua plugin providing access to the SchemaStore catalog.
		},
		config = function() require("plugins/nvim-lspconfig") end
	},

	{ -- Companion plugin for nvim-lspconfig that allows you to seamlessly install LSP servers locally (inside :echo stdpath("data")).
		"williamboman/mason.nvim",
		cmd = "Mason",
		dependencies = {
			{ "williamboman/mason-lspconfig.nvim" }, -- Extension to mason.nvim that makes it easier to use lspconfig with mason.nvim
			{                               -- Use Neovim as a language server to inject LSP diagnostics, code actions, and more via Lua.
				"nvimtools/none-ls.nvim",
				config = function() require("plugins/none-ls_nvim") end
			},
		},
		config = function() require("plugins/mason_nvim") end
	},

	{ -- Nvim Treesitter configurations and abstraction layer
		"nvim-treesitter/nvim-treesitter",
		build = ":TSUpdate",
		event = { "BufReadPost", "BufNewFile" },
		dependencies = {
			{ -- Treesitter playground integrated into Neovim
				"nvim-treesitter/playground",
			},
			{ -- Use treesitter to auto close and auto rename html tag, work with html,tsx,vue,svelte,php.
				"windwp/nvim-ts-autotag",
				event = "InsertEnter",
				config = function() require("plugins/nvim-ts-autotag") end
			},
			{ -- Neovim treesitter plugin for setting the commentstring based on the cursor location in a file.
				"JoosepAlviste/nvim-ts-context-commentstring",
				keys = { "cc", "gc", "gb", { "cc", mode = "v" }, { "gc", mode = "v" }, { "gb", mode = "v" } },
				config = function() require("plugins/nvim-ts-context-commentstring") end
			},
		},
		config = function() require("plugins/nvim-treesitter") end
	},

	{ -- A completion plugin for neovim coded in Lua.
		"hrsh7th/nvim-cmp",
		event = "InsertEnter",
		-- these dependencies will only be loaded when cmp loads
		-- dependencies are always lazy-loaded unless specified otherwise
		dependencies = {
			{ -- Snippet Engine for Neovim written in Lua.
				"L3MON4D3/LuaSnip",
				dependencies = {
					{ "rafamadriz/friendly-snippets" },   -- Snippets collection for a set of different programming languages for faster development.
					{ "Neevash/awesome-flutter-snippets", ft = "dart" }, -- collection snippets and shortcuts for commonly used Flutter functions and classes
				},
				config = function() require("plugins/LuaSnip") end
			},
			{ -- A super powerful autopairs for Neovim. It support multiple character.
				"windwp/nvim-autopairs",
				config = function() require("plugins/nvim-autopairs") end
			},
			{ -- Simple yet convenient Neovim plugin for tabbing in and out of brackets, parentheses, quotes, and more.
				"kawre/neotab.nvim",
				config = function() require("plugins/neotab") end
			},
			{ "hrsh7th/cmp-nvim-lsp" }, -- nvim-cmp source for neovim builtin LSP client
			{ "hrsh7th/cmp-nvim-lua" }, -- nvim-cmp source for nvim lua
			{ "hrsh7th/cmp-buffer" }, -- nvim-cmp source for buffer words.
			{ "hrsh7th/cmp-path" }, -- nvim-cmp source for filesystem paths.
			{ "saadparwaiz1/cmp_luasnip" }, -- luasnip completion source for nvim-cmp
			{ "ray-x/cmp-treesitter" }, -- nvim-cmp source for treesitter nodes.
		},
		config = function() require("plugins/nvim-cmp") end
	},

	{ -- Add/change/delete surrounding delimiter pairs with ease.
		"kylechui/nvim-surround",
		event = { "BufReadPre", "BufNewFile", "InsertEnter" },
		keys = { "c" },
		config = function() require("plugins/nvim-surround") end
	},

	{ -- Find, Filter, Preview, Pick. All lua, all the time.
		"nvim-telescope/telescope.nvim",
		cmd = { "Telescope", "Mason" },
		keys = { "t", "<C-f>", "<C-p>" },
		dependencies = {
			{ "nvim-telescope/telescope-fzf-native.nvim",  build = "make" }, -- FZF sorter for telescope written in c
			{ "nvim-telescope/telescope-file-browser.nvim" },      -- File Browser extension for telescope.nvim
			{ "nvim-telescope/telescope-media-files.nvim" },       -- Telescope extension to preview media files using Ueberzug.
			{ "nvim-telescope/telescope-ui-select.nvim" },         -- It sets vim.ui.select to telescope.
			{ "nvim-telescope/telescope-symbols.nvim" },           -- telescope-symbols provide its users with the ability of picking symbols and insert them at point.
			{ "nvim-telescope/telescope-project.nvim" },           -- An extension for telescope.nvim that allows you to switch between projects.
		},
		config = function() require("plugins/telescope") end
	},

	{ -- Use (neo)vim terminal in the floating/popup window.
		"voldikss/vim-floaterm",
		keys = { "t", "<C-t>" },
		config = function() require("plugins/vim-floaterm") end
	},

	{ -- Maximizes and restores the current window in Vim
		"szw/vim-maximizer",
		keys = { "<leader>" },
		config = function() require("plugins/vim-maximizer") end
	},

	{ -- Smart and powerful comment plugin for neovim. Supports commentstring, dot repeat, left-right/up-down motions, hooks, and more
		"numToStr/Comment.nvim",
		keys = { "cc", "gc", "gb", { "cc", mode = "v" }, { "gc", mode = "v" }, { "gb", mode = "v" } },
		config = function() require("plugins/Comment_nvim") end
	},

	{ -- The fastest Neovim colorizer.
		"NvChad/nvim-colorizer.lua",
		event = { "BufReadPre", "BufNewFile", "InsertEnter" },
		config = function() require("plugins/nvim-colorizer_lua") end
	},

	{ -- Indent guides for Neovim
		"lukas-reineke/indent-blankline.nvim",
		main = "ibl",
		event = { "BufReadPost", "BufNewFile" },
		opts = {},
		config = function() require("plugins/indent-blankline_nvim") end
	},

	{ -- Git signs written in pure lua
		"lewis6991/gitsigns.nvim",
		event = { "BufReadPre", "BufNewFile", "InsertEnter" },
		config = function() require("plugins/gitsigns_nvim") end
	},

	{ -- A declarative, highly configurable, and neovim style tabline plugin. Use your nvim tabs as a workspace multiplexer!
		"nanozuki/tabby.nvim",
		event = { "BufReadPre", "BufNewFile" },
		config = function() require("plugins/tabby") end
	},

	{ -- Neovim plugin to manage the file system and other tree like structures.
		"nvim-neo-tree/neo-tree.nvim",
		keys = { "<leader>" },
		branch = "v3.x",
		dependencies = {
			{ "MunifTanjim/nui.nvim" }, -- UI Component Library for Neovim.
		},
		config = function() require("plugins/neo-tree_nvim") end
	},

	{ -- fast and highly customizable greeter for neovim.
		"goolord/alpha-nvim",
		event = "VimEnter",
		config = function() require("plugins/alpha-nvim") end
	},

	{ --  smart indent and project detector with project based config loader
		"Abstract-IDE/penvim",
		event = "VimEnter",
		config = function() require("plugins/penvim") end
	},

	{ --  A simple wrapper around :mksession
		"Shatur/neovim-session-manager",
		event = "BufWinEnter",
		cmd = { "SessionManager" },
		config = function() require("plugins/neovim-session-manager") end
	},

	{ -- VS Code-like renaming UI for Neovim, writen in Lua.
		"filipdutescu/renamer.nvim",
		branch = "master",
		keys = { "<Space>R" },
		config = function() require("plugins/renamer_nvim") end
	},

	{ --  Neovim motions on speed!
		"phaazon/hop.nvim",
		keys = { "f" },
		config = function() require("plugins/hop_nvim") end
	},

	{ --  Neovim file explorer: edit your filesystem like a buffer
		"stevearc/oil.nvim",
		keys = { "-" },
		opts = {},
		config = function() require("plugins/oil") end
	},

	-- ━━━━━━━━━━━━━━━━━❰ DEVELOPMENT ❱━━━━━━━━━━━━━━━━━ --

	--          [ WEB ]
	{ --  live edit html, css, and javascript in vim
		"turbio/bracey.vim",
		build = "npm install --prefix server",
		ft = { "html", "css", "javascript" },
	},

	--          [ flutter | dart ]
	{ -- Tools to help create flutter apps in neovim using the native lsp
		"akinsho/flutter-tools.nvim",
		ft = { "dart" },
		dependencies = {
			{
				"dart-lang/dart-vim-plugin", -- Syntax highlighting for Dart in Vim
				config = function() require("plugins/dart-vim-plugin") end
			},
		},
		config = function() require("plugins/flutter-tools_nvim") end
	},
	-- ━━━━━━━━━━━━━━❰ end of DEVELOPMENT ❱━━━━━━━━━━━━━ --
}
-- ───────────────────────────────────────────────── --

-- ───────────────────────────────────────────────── --
-- lazy.nvim Configurations
local opts = {
	root = root_dir, -- directory where plugins will be installed
	defaults = {
		lazy = true, -- should plugins be lazy-loaded?
		version = nil,
		-- default `cond` you can use to globally disable a lot of plugins
		-- when running inside vscode for example
		cond = nil, ---@type boolean|fun(self:LazyPlugin):boolean|nil
		-- version = "*", -- enable this to try installing the latest stable versions of plugins
	},
	-- leave nil when passing the spec as the first argument to setup()
	spec = nil, ---@type LazySpec
	lockfile = root_dir .. "/lazy-lock.json", -- lockfile generated after running update.
	concurrency = jit.os:find("Windows") and (vim.loop.available_parallelism() * 2) or nil, ---@type number limit the maximum amount of concurrent tasks
	git = {
		-- defaults for the `Lazy log` command
		-- log = { "-10" }, -- show the last 10 commits
		log = { "-8" }, -- show commits from the last 3 days
		timeout = 300, -- kill processes that take more than 5 minutes
		url_format = "https://github.com/%s.git",
		-- lazy.nvim requires git >=2.19.0. If you really want to use lazy with an older version,
		-- then set the below to false. This should work, but is NOT supported and will
		-- increase downloads a lot.
		filter = true,
	},
	dev = {
		-- directory where you store your local plugin projects
		path = "~/projects",
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
		wrap = true, -- wrap the lines in the ui
		-- The border to use for the UI window. Accepts same border values as |nvim_open_win()|.
		border = "rounded",
		title = nil, ---@type string only works when border is not "none"
		title_pos = "center", ---@type "center" | "left" | "right"
		-- Show pills on top of the Lazy window
		pills = true, ---@type boolean
		icons = {
			cmd = " ",
			config = "",
			event = "",
			ft = " ",
			init = " ",
			import = " ",
			keys = " ",
			lazy = "󰒲 ",
			loaded = "●",
			not_loaded = "○",
			plugin = " ",
			runtime = " ",
			require = "󰢱 ",
			source = " ",
			start = "",
			task = "✔ ",
			list = { "●", "➜", "★", "‒" },
		},
		-- leave nil, to automatically select a browser depending on your OS.
		-- If you want to use a specific browser, you can define it here
		browser = nil, ---@type string?
		throttle = 20, -- how frequently should the ui process render events
		custom_keys = {
			-- You can define custom key maps here. If present, the description will
			-- be shown in the help menu.
			-- To disable one of the defaults, set it to false.

			["<localleader>l"] = {
				function(plugin)
					require("lazy.util").float_term({ "lazygit", "log" }, {
						cwd = plugin.dir,
					})
				end,
				desc = "Open lazygit log",
			},

			["<localleader>t"] = {
				function(plugin)
					require("lazy.util").float_term(nil, {
						cwd = plugin.dir,
					})
				end,
				desc = "Open terminal in plugin dir",
			},
		},
	},
	diff = {
		-- diff command <d> can be one of:
		-- * browser: opens the github compare view. Note that this is always mapped to <K> as well,
		--   so you can have a different command for diff <d>
		-- * git: will run git diff and open a buffer with filetype git
		-- * terminal_git: will open a pseudo terminal with git diff
		-- * diffview.nvim: will open Diffview to show the diff
		cmd = "git",
	},
	checker = {
		-- automatically check for plugin updates
		enabled = false,
		concurrency = nil, ---@type number? set to 1 to check for updates very slowly
		notify = true,  -- get a notification when new updates are found
		frequency = 3600, -- check for updates every hour
		check_pinned = false, -- check for pinned packages that can't be updated
	},
	change_detection = {
		-- automatically check for config file changes and reload the ui
		enabled = true,
		notify = true, -- get a notification when changes are found
	},
	performance = {
		cache = {
			enabled = true,
		},
		reset_packpath = true, -- reset the package path to improve startup time
		rtp = {
			reset = true, -- reset the runtime path to $VIMRUNTIME and your config directory
			---@type string[]
			paths = {},  -- add any custom paths here that you want to includes in the rtp
			---@type string[] list any plugins you want to disable here
			disabled_plugins = {
				-- "gzip",
				-- "matchit",
				-- "matchparen",
				-- "netrwPlugin",
				-- "tarPlugin",
				-- "tohtml",
				-- "tutor",
				-- "zipPlugin",
			},
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
	build = {
		-- Plugins can provide a `build.lua` file that will be executed when the plugin is installed
		-- or updated. When the plugin spec also has a `build` command, the plugin's `build.lua` not be
		-- executed. In this case, a warning message will be shown.
		warn_on_override = true,
	},
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
-- ───────────────────────────────────────────────── --

lazy.setup(plugins, opts)
