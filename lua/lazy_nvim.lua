
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ───────────────────────────────────────────────── --
--    Plugin:    lazy.nvim
--    Github:    https://github.com/folke/lazy.nvim

-- lazy.nvim is a modern plugin manager for Neovim.
-- ───────────────────────────────────────────────── --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --




-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━❰ configs ❱━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

-- bootstrap lazy.nvim
local root_path = vim.fn.stdpath("data") .. "/lazy"
local install_path = root_path .. "/lazy.nvim"
if not vim.loop.fs_stat(install_path) then
	vim.fn.system({
		"git", "clone", "--filter=blob:none", "https://github.com/folke/lazy.nvim.git", "--branch=stable",
		install_path
	})
end
vim.opt.rtp:prepend(install_path)

-- safely import packer
local _lazy, lazy = pcall(require, "lazy")
if not _lazy then
	return
end

-- lazy.nvim Configurations
local opts = {
	root = root_path, -- directory where plugins will be installed
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
	lockfile = vim.fn.stdpath("config") .. "/.lazy-lock.json", -- lockfile generated after running update.
	concurrency = jit.os:find("Windows") and (vim.loop.available_parallelism() * 2) or nil, ---@type number limit the maximum amount of concurrent tasks
	git = {
		-- defaults for the `Lazy log` command
		-- log = { "-10" }, -- show the last 10 commits
		log = { "-8" }, -- show commits from the last 3 days
		timeout = 120, -- kill processes that take more than 2 minutes
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
		colorscheme = { "habamax" },
	},
	ui = {
		-- a number <1 is a percentage., >1 is a fixed size
		size = { width = 0.8, height = 0.8 },
		wrap = true, -- wrap the lines in the ui
		-- The border to use for the UI window. Accepts same border values as |nvim_open_win()|.
		border = "none",
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
			list = {
				"●",
				"➜",
				"★",
				"‒",
			},
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
		notify = true, -- get a notification when new updates are found
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
			paths = {}, -- add any custom paths here that you want to includes in the rtp
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

-- Plugins Configurations
local plugins = {

	-- the colorscheme should be available when starting Neovim
	{   -- colorscheme for (neo)vim written in lua specially made for Abstract
		'Abstract-IDE/Abstract-cs',
		lazy = false, -- make sure we load this during startup if it is your main colorscheme
		priority = 1000, -- make sure to load this before all the other start plugins
		config = function() require('plugins/Abstract-cs') end,
	},

	{ -- All the lua functions I don't want to write twice.
		'nvim-lua/plenary.nvim',
	},

	{ -- lua `fork` of vim-web-devicons for neovim
		'kyazdani42/nvim-web-devicons',
		config = function () require('plugins/nvim-web-devicons') end
	},

	{ --  A plugin for profiling Vim and Neovim startup time.
		"dstein64/vim-startuptime",
		cmd = "StartupTime", -- lazy-load on a command
		-- init is called during startup. Configuration for vim plugins typically should be set in an init function
		init = function()
			vim.g.startuptime_tries = 10
		end,
	},

	{ -- A collection of common configurations for Neovim's built-in language server client
		'neovim/nvim-lspconfig',
		dependencies = {
			{ -- Companion plugin for nvim-lspconfig that allows you to seamlessly install LSP servers locally (inside :echo stdpath("data")).
				'williamboman/mason.nvim',
				dependencies = {
					{ 'williamboman/mason-lspconfig.nvim', event = 'BufRead' }, -- Extension to mason.nvim that makes it easier to use lspconfig with mason.nvim
					{ 'jose-elias-alvarez/null-ls.nvim', event = 'BufRead' }, -- Use Neovim as a language server to inject LSP diagnostics, code actions, and more via Lua.
				},
			},
			{ -- A pretty diagnostics, references, telescope results, quickfix and location list to help you solve all the trouble your code is causing.
				'folke/trouble.nvim',
				event = 'BufRead',
				config = function () require('plugins/trouble_nvim') end
			},
			{ -- preview native LSP's goto definition calls in floating windows.
				'rmagatti/goto-preview',
				keys = {'gp'},
				config = function () require('plugins/goto-preview') end
			},
			{ -- Standalone UI for nvim-lsp progress
				'j-hui/fidget.nvim',
				event = 'BufRead',
				config = function () require('plugins/fidget_nivm') end
			},
			{ --  Simple winbar/statusline plugin that shows your current code context
				'SmiteshP/nvim-navic',
				event = 'BufRead',
				config = function () require('plugins/nvim_navic') end
			},
			{ --  LSP signature hint as you type
				'ray-x/lsp_signature.nvim',
				event = 'BufRead',
				config = function () require('plugins/lsp_signature_nvim') end
			}
		},
		init = function () require('plugins/nvim-lspconfig') end,
		config = function () require('plugins/null-ls_nvim') end
	},

	{ -- Nvim Treesitter configurations and abstraction layer
		'nvim-treesitter/nvim-treesitter',
		build = ":TSUpdate",
		event = 'BufRead',
		dependencies = {
			{ -- Treesitter playground integrated into Neovim
				'nvim-treesitter/playground',
			},
			{ --  Use treesitter to auto close and auto rename html tag, work with html,tsx,vue,svelte,php.
				"windwp/nvim-ts-autotag",
				ft = { 'html', 'htmldjango', 'tsx', 'jsx', 'javascriptreact', 'typescriptreact', 'vue', 'svelte', 'php' },
				config = function () require('plugins/nvim-ts-autotag') end
			},
			{ --  Neovim treesitter plugin for setting the commentstring based on the cursor location in a file.
				'JoosepAlviste/nvim-ts-context-commentstring',
				config = function () require('plugins/nvim-ts-context-commentstring') end
			}
		},
		config = function () require('plugins/nvim-treesitter') end
	},

	{ -- A completion plugin for neovim coded in Lua.
		'hrsh7th/nvim-cmp',
		event = 'InsertEnter',
		-- these dependencies will only be loaded when cmp loads
		-- dependencies are always lazy-loaded unless specified otherwise
		dependencies = {
			{ -- Snippet Engine for Neovim written in Lua.
				'L3MON4D3/LuaSnip',
				event = 'InsertEnter',
				dependencies = {
					{ 'rafamadriz/friendly-snippets', event = 'InsertEnter' }, -- Snippets collection for a set of different programming languages for faster development.
					{ 'Neevash/awesome-flutter-snippets', ft='dart', event = 'InsertEnter' }, -- collection snippets and shortcuts for commonly used Flutter functions and classes
				},
			},
			{ -- A super powerful autopairs for Neovim. It support multiple character.
				'windwp/nvim-autopairs',
				event = 'InsertEnter',
				config = function() require('plugins/nvim-autopairs') end
			},
			{ 'hrsh7th/cmp-nvim-lsp', event = 'InsertEnter' }, -- nvim-cmp source for neovim builtin LSP client
			{ 'hrsh7th/cmp-nvim-lua', event = 'InsertEnter' }, -- nvim-cmp source for nvim lua
			{ 'hrsh7th/cmp-buffer', event = 'InsertEnter' }, -- nvim-cmp source for buffer words.
			{ 'hrsh7th/cmp-path', event = 'InsertEnter' }, -- nvim-cmp source for filesystem paths.
			{ 'saadparwaiz1/cmp_luasnip', event = 'InsertEnter' }, -- luasnip completion source for nvim-cmp
		},
		config = function()
			require('plugins/LuaSnip')
			require('plugins/nvim-cmp')
		end
	},

	{ --  Add/change/delete surrounding delimiter pairs with ease.
		'kylechui/nvim-surround',
		event = 'InsertEnter',
		keys = {'c'},
		config = function () require('plugins/nvim-surround') end
	},

	{ -- Find, Filter, Preview, Pick. All lua, all the time.
		'nvim-telescope/telescope.nvim',
		event = {'CmdlineEnter', 'CursorHold'},
		keys = { "t", "<C>", "<C-f>", "<C-p>" },
		dependencies = {
			{'nvim-telescope/telescope-fzf-native.nvim', build = 'make'}, -- FZF sorter for telescope written in c
			{'nvim-telescope/telescope-file-browser.nvim'}, -- File Browser extension for telescope.nvim
			{'nvim-telescope/telescope-media-files.nvim'}, -- Telescope extension to preview media files using Ueberzug.
			{'nvim-telescope/telescope-ui-select.nvim'}, -- It sets vim.ui.select to telescope.
			{'nvim-telescope/telescope-symbols.nvim'}, -- telescope-symbols provide its users with the ability of picking symbols and insert them at point.
		},
		config = function () require('plugins/telescope_nvim') end
	},

	{ -- Use (neo)vim terminal in the floating/popup window.
		'voldikss/vim-floaterm',
		keys = { "t" },
		config = function () require('plugins/vim-floaterm') end
	},

	{ -- Maximizes and restores the current window in Vim
		'szw/vim-maximizer',
		keys = { ";" },
		config = function () require('plugins/vim-maximizer') end
	},

	{ -- Smart and powerful comment plugin for neovim. Supports commentstring, dot repeat, left-right/up-down motions, hooks, and more
		'numToStr/Comment.nvim',
		keys = { "cc", "gc", "gb" },
		config = function () require('plugins/Comment_nvim') end
	},

	{ -- The fastest Neovim colorizer.
		'NvChad/nvim-colorizer.lua',
		event = 'BufRead',
		config = function () require('plugins/nvim-colorizer_lua') end
	},

	{ --  Indent guides for Neovim
		'lukas-reineke/indent-blankline.nvim',
		main = "ibl",
		event = "BufWinEnter",
		opts = {},
		config = function () require('plugins/indent-blankline_nvim') end
	},

	{ -- Git signs written in pure lua
		'lewis6991/gitsigns.nvim',
		event = "BufWinEnter",
		config = function () require('plugins/gitsigns_nvim') end
	},

	{ -- A snazzy bufferline for Neovim
		'akinsho/nvim-bufferline.lua',
		event = "BufWinEnter",
		config = function () require('plugins/nvim-bufferline_lua') end
	},

	{ -- Neovim plugin to manage the file system and other tree like structures.
		'nvim-neo-tree/neo-tree.nvim',
		keys=";",
		branch = "v2.x", -- !WARN: update it to v3.x
		dependencies = {
			{ -- UI Component Library for Neovim.
				"MunifTanjim/nui.nvim"
			}
		},
		config = function () require('plugins/neo-tree_nvim') end
	},

	{ -- fast and highly customizable greeter for neovim.
		"goolord/alpha-nvim",
		event = "VimEnter",
		config = function () require('plugins/alpha-nvim') end
	},

	{ --  smart indent and project detector with project based config loader
		'Abstract-IDE/penvim',
		event = "BufWinEnter",
		config = function () require('plugins/penvim') end
	},

	{ --  A simple wrapper around :mksession
		'Shatur/neovim-session-manager',
		event = "BufWinEnter",
		cmd = { "SessionManager" },
		config = function () require('plugins/neovim-session-manager') end
	},

	{ -- VS Code-like renaming UI for Neovim, writen in Lua.
		'filipdutescu/renamer.nvim',
		branch = 'master',
		keys = { "<Space>R" },
		config = function () require('plugins/renamer_nvim') end
	},

	{ --  Neovim motions on speed!
		'phaazon/hop.nvim',
		keys = { "f" },
		config = function () require('plugins/hop_nvim') end
	},

	-- ━━━━━━━━━━━━━━━━━❰ DEVELOPMENT ❱━━━━━━━━━━━━━━━━━ --

	--          [ WEB ]
	{ --  live edit html, css, and javascript in vim
			'turbio/bracey.vim',
			build =  'npm install --prefix server',
			ft = {'html', 'css', 'javascript'}
	},

	--          [ flutter | dart ]
	{ -- Tools to help create flutter apps in neovim using the native lsp
		'akinsho/flutter-tools.nvim',
		ft = {'dart'},
		dependencies = {
			{
				'dart-lang/dart-vim-plugin', -- Syntax highlighting for Dart in Vim
				ft = {'dart'},
				config = function () require('plugins/dart-vim-plugin') end
			}
		},
		config = function () require('plugins/flutter-tools_nvim') end
	}
	-- ━━━━━━━━━━━━━━❰ end of DEVELOPMENT ❱━━━━━━━━━━━━━ --

}

lazy.setup(plugins, opts)

-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━❰ end configs ❱━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

