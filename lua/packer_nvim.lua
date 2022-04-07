
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
local execute = vim.api.nvim_command
local fn = vim.fn
local install_path = fn.stdpath('data') .. '/site/pack/packer/start/packer.nvim'
if fn.empty(fn.glob(install_path)) > 0 then
	fn.system({
		'git', 'clone', 'https://github.com/wbthomason/packer.nvim', install_path
	})
	execute 'packadd packer.nvim'
end

-- don't throw any error on first use by packer
local ok, packer = pcall(require, "packer")
if not ok then return end

-- automatically run :PackerCompile whenever plugins.lua is updated
vim.cmd([[
  augroup packer_user_config
    autocmd!
    autocmd BufWritePost plugins.lua source <afile> | PackerCompile
  augroup end
]])


return packer.startup {
	function()
		-- Packer can manage itself
		use {
			'wbthomason/packer.nvim',
			commit = '4dedd3b'
		}

		-- Improve Start-UP time
		use { -- Speed up loading Lua modules in Neovim to improve startup time.
			'lewis6991/impatient.nvim',
			commit = '2337df7'
		}

		use { -- Easily speed up your neovim startup time!. A faster version of filetype.vim
			'nathom/filetype.nvim',
			commit = '25b5f7e'
		}

		use { -- colorscheme for (neo)vim written in lua specially made for roshnivim
			'shaeinst/roshnivim-cs',
			commit = '761cd55',
			branch = 'v2'
		}

		use { -- A collection of common configurations for Neovim's built-in language server client
			'neovim/nvim-lspconfig',
			commit = 'f183d35',
			config = [[ require('plugins/nvim-lspconfig') ]]
		}

		use { -- Companion plugin for nvim-lspconfig that allows you to seamlessly install LSP servers locally (inside :echo stdpath("data")).
			'williamboman/nvim-lsp-installer',
			commit = 'b1209e9',
			config = [[ require('plugins/nvim-lsp-installer') ]]
		}

		use { -- vscode-like pictograms for neovim lsp completion items Topics
			'onsails/lspkind-nvim',
			commit = '93e98a0',
			config = [[ require('plugins/lspkind-nvim') ]]
		}

		use { -- Nvim Treesitter configurations and abstraction layer
			'nvim-treesitter/nvim-treesitter',
			commit = 'f62fb16',
			requires = {
				'nvim-treesitter/playground', -- Treesitter playground integrated into Neovim
				commit = '7dbcd4d',
				opt = true
			},
			run = ':TSUpdate',
			config = [[ require('plugins/nvim-treesitter') ]]
		}

		use { -- A completion plugin for neovim coded in Lua.
			'hrsh7th/nvim-cmp',
			commit = '27970d8',
			requires = {
				{"hrsh7th/cmp-nvim-lsp", commit="ebdfc20"}, -- nvim-cmp source for neovim builtin LSP client
				{"hrsh7th/cmp-nvim-lua", commit="d276254", ft = 'lua'}, -- nvim-cmp source for nvim lua
				{"hrsh7th/cmp-buffer", commit="d66c4c2"}, -- nvim-cmp source for buffer words.
				{"hrsh7th/cmp-path", commit="466b6b8"}, -- nvim-cmp source for filesystem paths.
				{"saadparwaiz1/cmp_luasnip", commit="b108297"} -- luasnip completion source for nvim-cmp
			},
			config = [[ require('plugins/nvim-cmp') ]]
		}

		use { -- Snippet Engine for Neovim written in Lua.
			'L3MON4D3/LuaSnip',
			commit = '69cb81c',
			requires = {
				"rafamadriz/friendly-snippets", -- Snippets collection for a set of different programming languages for faster development.
				commit = 'e302658',
			},
			config = [[ require('plugins/LuaSnip') ]]
		}

		use { -- Use Neovim as a language server to inject LSP diagnostics, code actions, and more via Lua.
			'jose-elias-alvarez/null-ls.nvim',
			commit = '64d9e2b',
			config = [[ require('plugins/null-ls_nvim') ]]
		}

		use { -- A super powerful autopairs for Neovim. It support multiple character.
			'windwp/nvim-autopairs',
			commit = '06535b1',
			config = [[ require('plugins/nvim-autopairs') ]]
		}

		use { -- Find, Filter, Preview, Pick. All lua, all the time.
			'nvim-telescope/telescope.nvim',
			commit = '6e7ee38',
			requires = {
				{'nvim-lua/popup.nvim', commit='b7404d3'},
				{'nvim-lua/plenary.nvim', commit='470d0e8'},
				{'nvim-telescope/telescope-fzf-native.nvim', commit='8ec164b', run = 'make'}, -- FZF sorter for telescope written in c
				{'nvim-telescope/telescope-file-browser.nvim', commit='c6f5104'}, -- File Browser extension for telescope.nvim
				{'nvim-telescope/telescope-media-files.nvim', commit='513e4ee'}, -- Telescope extension to preview media files using Ueberzug.
			},
			config = [[ require('plugins/telescope_nvim') ]]
		}

		use { -- Use (neo)vim terminal in the floating/popup window.
			'voldikss/vim-floaterm',
			commit = '6244d17',
			config = [[ require('plugins/vim-floaterm') ]]
		}

		use { -- lua `fork` of vim-web-devicons for neovim
			'kyazdani42/nvim-web-devicons',
			commit = '09e6231',
			config = [[ require('plugins/nvim-web-devicons') ]]
		}

		use { -- Maximizes and restores the current window in Vim
			'szw/vim-maximizer',
			commit = '2e54952',
			config = [[ require('plugins/vim-maximizer') ]]
		}

		use { -- Smart and powerful comment plugin for neovim. Supports commentstring, dot repeat, left-right/up-down motions, hooks, and more
			'numToStr/Comment.nvim',
			commit = '0aaea32',
			config = [[ require('plugins/Comment_nvim') ]]
		}

		use { -- The fastest Neovim colorizer.
			'norcalli/nvim-colorizer.lua',
			commit = '36c610a',
			config = [[ require('plugins/nvim-colorizer_lua') ]]
		}

		use {
			'lukas-reineke/indent-blankline.nvim',
			commit = 'cd5b800',
			config = [[ require('plugins/indent-blankline_nvim') ]]
		}

		use { -- Git signs written in pure lua
			'lewis6991/gitsigns.nvim',
			commit = '83ab3ca',
			requires = {
				'nvim-lua/plenary.nvim',
				commit="470d0e8"
			},
			config = [[ require('plugins/gitsigns_nvim') ]]
		}

		use { -- A pretty diagnostics, references, telescope results, quickfix and location list to help you solve all the trouble your code is causing.
			'folke/trouble.nvim',
			commit = '691d490',
			config = [[ require('plugins/trouble_nvim') ]]
		}

		use { -- A snazzy bufferline for Neovim
			'akinsho/nvim-bufferline.lua',
			commit = '874f869',
			requires = {
				'kyazdani42/nvim-web-devicons',
				commit="09e6231"
			},
			config = [[ require('plugins/nvim-bufferline_lua') ]]
		}

		use { -- A File Explorer For Neovim Written In Lua
			'kyazdani42/nvim-tree.lua',
			commit = '618ea25',
			config = [[ require('plugins/nvim-tree_lua') ]]
		}

		use { -- A minimal, stylish and customizable statusline for Neovim written in Lua
			'feline-nvim/feline.nvim',
			commit = '290bea8',
			-- requires = {
			--   'nvim-lua/lsp-status.nvim',
			-- },
			config = [[ require('plugins/feline_nvim') ]]
		}

		use { -- fast and highly customizable greeter for neovim.
			'goolord/alpha-nvim',
			commit = '534a86b',
			requires = {
				'kyazdani42/nvim-web-devicons',
				commit='09e6231'
			},
			config = [[ require('plugins/alpha-nvim') ]]
		}

		use {
			'shaeinst/penvim', --  smart indent and project detector with project based config loader
			commit = '1e8c4c5',
			config = [[ require('plugins/penvim') ]]
		}

		-- ━━━━━━━━━━━━━━━━━❰ DEVELOPMENT ❱━━━━━━━━━━━━━━━━━ --

		----           for flutter/dart
		use { -- Tools to help create flutter apps in neovim using the native lsp
			'akinsho/flutter-tools.nvim',
			commit = '53912f2',
			ft = {'dart'},
			requires = {
				{'nvim-lua/plenary.nvim', commit = '470d0e8'},
				{'Neevash/awesome-flutter-snippets', commit = 'ce19e28'}, -- collection snippets and shortcuts for commonly used Flutter functions and classes
				{
					'dart-lang/dart-vim-plugin', -- Syntax highlighting for Dart in Vim
					commit = '42e6f57',
					config = [[ require('plugins/dart-vim-plugin') ]]
				}
			},
			config = [[ require('plugins/flutter-tools_nvim') ]]
		}

		--            for Web-Development
		use { --  Use treesitter to auto close and auto rename html tag, work with html,tsx,vue,svelte,php.
			"windwp/nvim-ts-autotag",
			commit = '57035b5',
			ft = {'html', 'tsx', 'vue', 'svelte', 'php'},
			requires = {
				'nvim-treesitter/nvim-treesitter',
				commit = 'f62fb16',
			},
			config = [[ require('plugins/nvim-ts-autotag') ]]
		}
		-- ━━━━━━━━━━━━━━❰ end of DEVELOPMENT ❱━━━━━━━━━━━━━ --
	end,


	config = {
		-- Move to lua dir so impatient.nvim can cache it
		compile_path = vim.fn.stdpath('config') .. '/plugin/packer_compiled.lua',

		display = {
			open_fn = function()
				return require('packer.util').float({border = 'single'})
			end
		}
	}
}

-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━❰ end configs ❱━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

