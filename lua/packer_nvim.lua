
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ───────────────────────────────────────────────── --
--    Plugin:    packer.nvim
--    Github:    github.com/wbthomason/packer.nvim
-- ───────────────────────────────────────────────── --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --





-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━❰ configs ❱━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

--              NOTE:1
-- If you want to automatically ensure that packer.nvim is installed on any machine you clone your configuration to,
-- add the following snippet (which is due to @Iron-E) somewhere in your config before your first usage of packer:
local fn = vim.fn
local install_path = fn.stdpath('data')..'/site/pack/packer/start/packer.nvim'
if fn.empty(fn.glob(install_path)) > 0 then
	Packer_bootstrap = fn.system({
		'git', 'clone', '--depth', '1', 'https://github.com/wbthomason/packer.nvim',
		install_path
	})
end

-- safely import packer
local ok, packer = pcall(require, "packer")
if not ok then return end

local commits = require("plugins.commits")


return packer.startup {

	config = {
		-- Move to lua dir so impatient.nvim can cache it
		compile_path = vim.fn.stdpath('config') .. '/plugin/packer_compiled.lua',

		display = {
			open_fn = function()
				return require('packer.util').float({border = 'single'})
			end
		}
	},


	function()

		-- ━━━━━━━━━━━━━━━━━❰ Required plugins ❱━━━━━━━━━━━━━━━━━ --
		use { -- All the lua functions I don't want to write twice.
			'nvim-lua/plenary.nvim',
			commit=commits.plenary_nvim,
		}

		use { -- lua `fork` of vim-web-devicons for neovim
			'kyazdani42/nvim-web-devicons',
			commit = commits.nvim_web_devicons,
			config = [[ require('plugins/nvim-web-devicons') ]]
		}
		-- ━━━━━━━━━━━━━━━━❰ end Required Plugin ❱━━━━━━━━━━━━━━━━ --

		use { -- Packer can manage itself
			'wbthomason/packer.nvim',
			commit = commits.packer_nvim,
		}

		-- Improve Start-UP time
		use { -- Speed up loading Lua modules in Neovim to improve startup time.
			'lewis6991/impatient.nvim',
			commit = commits.impatient_nvim,
		}

		use { -- Easily speed up your neovim startup time!. A faster version of filetype.vim
			'nathom/filetype.nvim',
			commit = commits.filetype_nvim,
		}

		use { -- colorscheme for (neo)vim written in lua specially made for Abstract
			'Abstract-IDE/Abstract-cs',
			-- commit = commits.Abstract_cs,
			branch = 'v2',
		}

		use { -- A collection of common configurations for Neovim's built-in language server client
			'neovim/nvim-lspconfig',
			commit = commits.nvim_lspconfig,
			event = 'BufRead',
			requires = {
				{ -- Companion plugin for nvim-lspconfig that allows you to seamlessly install LSP servers locally (inside :echo stdpath("data")).
					'williamboman/mason.nvim',
					commit = commits.mason_nvim,
					requires = {
						{ 'williamboman/mason-lspconfig.nvim', commit=commits.mason_lspconfig_nvim }, -- Extension to mason.nvim that makes it easier to use lspconfig with mason.nvim
						{ -- Use Neovim as a language server to inject LSP diagnostics, code actions, and more via Lua.
							'jose-elias-alvarez/null-ls.nvim',
							commit = commits.null_ls_nvim,
							after = "mason.nvim"
						}
					},
				},
				{ -- A pretty diagnostics, references, telescope results, quickfix and location list to help you solve all the trouble your code is causing.
					'folke/trouble.nvim',
					commit = commits.trouble_nvim,
					after = "nvim-lspconfig"
				},
				{ -- preview native LSP's goto definition calls in floating windows.
					'rmagatti/goto-preview',
					commit = commits.goto_preview,
					after = "nvim-lspconfig"
				},
				{ -- Standalone UI for nvim-lsp progress
					'j-hui/fidget.nvim',
					commit = commits.fidget_nvim,
					event = 'BufRead',
					config = [[ require('plugins/fidget_nivm') ]]
				},
			},
			config = [[
				require('plugins/nvim-lspconfig')
				require('plugins/null-ls_nvim')
				require('plugins/trouble_nvim')
				require('plugins/goto-preview')
			]]
		}

		use { -- Nvim Treesitter configurations and abstraction layer
			'nvim-treesitter/nvim-treesitter',
			commit = commits.nvim_treesitter,
			run = function() vim.cmd([[TSUpdate]]) end,
			requires = {
				{ -- Treesitter playground integrated into Neovim
					'nvim-treesitter/playground', commit=commits.playground, after='nvim-treesitter'
				},
				{ --  Use treesitter to auto close and auto rename html tag, work with html,tsx,vue,svelte,php.
					"windwp/nvim-ts-autotag",
					commit = commits.nvim_ts_autotag,
					ft = {'html', 'htmldjango', 'tsx', 'jsx', 'vue', 'svelte', 'php'},
				},
				{ --  Neovim treesitter plugin for setting the commentstring based on the cursor location in a file.
					'JoosepAlviste/nvim-ts-context-commentstring',
					commit = commits.nvim_ts_context_commentstring,
					after = 'nvim-treesitter',
				}
			},
			config = [[
				require('plugins/nvim-treesitter')
				require('plugins/nvim-ts-autotag')
				require('plugins/nvim-ts-context-commentstring')
			]]
		}

		use { -- A completion plugin for neovim coded in Lua.
			'hrsh7th/nvim-cmp',
			commit = commits.nvim_cmp,
			event = 'InsertEnter',
			requires = {
				{ -- vscode-like pictograms for neovim lsp completion items Topics
					'onsails/lspkind-nvim',
					after = "nvim-cmp",
					commit = commits.lspkind_nvim,
				},
				{ -- Snippet Engine for Neovim written in Lua.
					'L3MON4D3/LuaSnip',
					commit = commits.LuaSnip,
					module = "luasnip",
					requires = {
						{ 'rafamadriz/friendly-snippets', after="LuaSnip", commit=commits.friendly_snippets }, -- Snippets collection for a set of different programming languages for faster development.
						{ 'Neevash/awesome-flutter-snippets', ft='dart', commit=commits.awesome_flutter_snippets }, -- collection snippets and shortcuts for commonly used Flutter functions and classes
					},
				},
				{ 'windwp/nvim-autopairs', after='nvim-cmp', commit=commits.nvim_autopairs }, -- A super powerful autopairs for Neovim. It support multiple character.
				{ 'hrsh7th/cmp-nvim-lsp', after='nvim-cmp', commit=commits.cmp_nvim_lsp }, -- nvim-cmp source for neovim builtin LSP client
				{ 'hrsh7th/cmp-nvim-lua', after='nvim-cmp', commit=commits.cmp_nvim_lua}, -- nvim-cmp source for nvim lua
				{ 'hrsh7th/cmp-buffer', after='nvim-cmp', commit=commits.cmp_buffer }, -- nvim-cmp source for buffer words.
				{ 'hrsh7th/cmp-path', after='nvim-cmp', commit=commits.cmp_path}, -- nvim-cmp source for filesystem paths.
				{ 'hrsh7th/cmp-nvim-lsp-signature-help', after='nvim-cmp', commit=commits.cmp_nvim_lsp_signature_help}, -- nvim-cmp source for displaying function signatures with the current parameter emphasized:
				{ 'saadparwaiz1/cmp_luasnip', after='nvim-cmp', commit=commits.cmp_luasnip}, -- luasnip completion source for nvim-cmp
			},
			config = [[
				require('plugins/nvim-cmp')
				require('plugins/LuaSnip')
				require('plugins/nvim-autopairs')
			]]
		}

		use { --  Add/change/delete surrounding delimiter pairs with ease.
			'kylechui/nvim-surround',
			commit = commits.nvim_surround,
			event = 'InsertEnter',
			keys = {'c'},
			config = [[ require('plugins/nvim-surround') ]]
		}

		use { -- Find, Filter, Preview, Pick. All lua, all the time.
			'nvim-telescope/telescope.nvim',
			commit = commits.telescope_nvim,
			event = {'CmdlineEnter', 'CursorHold'},
			keys = { "t", "<C>", "<C-f>", "<C-p>" },
			requires = {
				{'nvim-lua/popup.nvim', commit=commits.popup_nvim},
				{'nvim-telescope/telescope-fzf-native.nvim', commit=commits.telescope_fzf_native_nvim, run = 'make'}, -- FZF sorter for telescope written in c
				{'nvim-telescope/telescope-file-browser.nvim', commit=commits.telescope_file_browser_nvim}, -- File Browser extension for telescope.nvim
				{'nvim-telescope/telescope-media-files.nvim', commit=commits.telescope_media_files_nvim}, -- Telescope extension to preview media files using Ueberzug.
				{'nvim-telescope/telescope-ui-select.nvim', commit=commits.telescope_ui_select_nvim}, -- It sets vim.ui.select to telescope.
			},
			config = [[ require('plugins/telescope_nvim') ]]
		}

		use { -- Use (neo)vim terminal in the floating/popup window.
			'voldikss/vim-floaterm',
			commit = commits.vim_floaterm,
			config = [[ require('plugins/vim-floaterm') ]]
		}

		use { -- Maximizes and restores the current window in Vim
			'szw/vim-maximizer',
			commit = commits.vim_maximizer,
			config = [[ require('plugins/vim-maximizer') ]]
		}

		use { -- Smart and powerful comment plugin for neovim. Supports commentstring, dot repeat, left-right/up-down motions, hooks, and more
			'numToStr/Comment.nvim',
			commit = commits.Comment_nvim,
			keys = { "cc", "gc", "gb" },
			config = [[ require('plugins/Comment_nvim') ]]
		}

		use { -- The fastest Neovim colorizer.
			'norcalli/nvim-colorizer.lua',
			commit = commits.nvim_colorizer_lua,
			config = [[ require('plugins/nvim-colorizer_lua') ]]
		}

		use { --  Indent guides for Neovim
			'lukas-reineke/indent-blankline.nvim',
			commit = commits.indent_blankline_nvim,
			config = [[ require('plugins/indent-blankline_nvim') ]]
		}

		use { -- Git signs written in pure lua
			'lewis6991/gitsigns.nvim',
			commit = commits.gitsigns_nvim,
			config = [[ require('plugins/gitsigns_nvim') ]]
		}

		use { -- A snazzy bufferline for Neovim
			'akinsho/nvim-bufferline.lua',
			commit = commits.nvim_bufferline_lua,
			event = "BufWinEnter",
			config = [[ require('plugins/nvim-bufferline_lua') ]]
		}

		use { -- A File Explorer For Neovim Written In Lua
			'kyazdani42/nvim-tree.lua',
			commit = commits.nvim_tree_lua,
			config = [[ require('plugins/nvim-tree_lua') ]]
		}

		use { -- A minimal, stylish and customizable statusline for Neovim written in Lua
			'feline-nvim/feline.nvim',
			commit = commits.feline_nvim,
			-- requires = {
			--   'nvim-lua/lsp-status.nvim',
			-- },
			config = [[ require('plugins/feline_nvim') ]]
		}

		use { -- fast and highly customizable greeter for neovim.
			"goolord/alpha-nvim",
			commit = commits.alpha_nvim,
			config = [[ require('plugins/alpha-nvim') ]]
		}

		use { --  smart indent and project detector with project based config loader
			'Abstract-IDE/penvim',
			commit = commits.penvim,
			config = [[ require('plugins/penvim') ]]
		}

		use { --  A simple wrapper around :mksession
			'Shatur/neovim-session-manager',
			commit = commits.neovim_session_manager,
			config = [[ require('plugins/neovim-session-manager') ]]
		}

		use { -- VS Code-like renaming UI for Neovim, writen in Lua.
			'filipdutescu/renamer.nvim',
			commit = commits.renamer_nvim,
			branch = 'master',
			config = [[ require('plugins/renamer_nvim') ]]
		}

		use { -- EditorConfig plugin for Neovim
			'gpanders/editorconfig.nvim',
			commit = commits.editorconfig_nvim,
		}

		use { --  Neovim motions on speed!
			'phaazon/hop.nvim',
			commit = commits.hop_nvim,
			config = [[ require('plugins/hop_nvim') ]]
		}

		-- ━━━━━━━━━━━━━━━━━❰ DEVELOPMENT ❱━━━━━━━━━━━━━━━━━ --

		----           for flutter/dart
		use { -- Tools to help create flutter apps in neovim using the native lsp
			'akinsho/flutter-tools.nvim',
			commit = commits.flutter_tools_nvim,
			ft = {'dart'},
			requires = {
				{
					'dart-lang/dart-vim-plugin', -- Syntax highlighting for Dart in Vim
					commit = commits.dart_vim_plugin,
					ft = {'dart'},
					config = [[ require('plugins/dart-vim-plugin') ]]
				}
			},
			config = [[ require('plugins/flutter-tools_nvim') ]]
		}
		-- ━━━━━━━━━━━━━━❰ end of DEVELOPMENT ❱━━━━━━━━━━━━━ --


		-- Automatically set up your configuration after cloning packer.nvim
		-- Put this at the end after all plugins
		if Packer_bootstrap then
			packer.sync()
		end
	end
}

-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━❰ end configs ❱━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

