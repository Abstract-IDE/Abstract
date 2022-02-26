
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


return packer.startup {
	function()
		-- Packer can manage itself
		use 'wbthomason/packer.nvim'

		-- Improve Start-UP time
		use { -- Speed up loading Lua modules in Neovim to improve startup time.
			'lewis6991/impatient.nvim'
		}

		use { -- Easily speed up your neovim startup time!. A faster version of filetype.vim
			'nathom/filetype.nvim'
		}

		use { -- colorscheme for (neo)vim written in lua specially made for roshnivim
			'shaeinst/roshnivim-cs',
			branch = 'v2',
		}

		use { -- A collection of common configurations for Neovim's built-in language server client
			'neovim/nvim-lspconfig',
			config = [[ require('plugins/nvim-lspconfig') ]]
		}

		use { -- Companion plugin for nvim-lspconfig that allows you to seamlessly install LSP servers locally (inside :echo stdpath("data")).
			'williamboman/nvim-lsp-installer',
			config = [[ require('plugins/nvim-lsp-installer') ]]
		}

		use { -- vscode-like pictograms for neovim lsp completion items Topics
			'onsails/lspkind-nvim',
			config = [[ require('plugins/lspkind-nvim') ]]
		}

		use { -- Nvim Treesitter configurations and abstraction layer
			'nvim-treesitter/nvim-treesitter',
			requires = {
				'nvim-treesitter/playground', -- Treesitter playground integrated into Neovim
				opt = true
			},
			run = ':TSUpdate',
			config = [[ require('plugins/nvim-treesitter') ]]
		}

		use { -- A completion plugin for neovim coded in Lua.
			'hrsh7th/nvim-cmp',
			requires = {
				"hrsh7th/cmp-nvim-lsp", -- nvim-cmp source for neovim builtin LSP client
				{"hrsh7th/cmp-nvim-lua", ft = 'lua'}, -- nvim-cmp source for nvim lua
				"hrsh7th/cmp-buffer", -- nvim-cmp source for buffer words.
				"hrsh7th/cmp-path", -- nvim-cmp source for filesystem paths.
				-- "hrsh7th/cmp-calc", -- nvim-cmp source for math calculation.
				"saadparwaiz1/cmp_luasnip" -- luasnip completion source for nvim-cmp
			},
			config = [[ require('plugins/nvim-cmp') ]]
		}

		use { -- Snippet Engine for Neovim written in Lua.
			'L3MON4D3/LuaSnip',
			requires = {
				"rafamadriz/friendly-snippets", -- Snippets collection for a set of different programming languages for faster development.
			},
			config = [[ require('plugins/LuaSnip') ]]
		}

		use { -- Use Neovim as a language server to inject LSP diagnostics, code actions, and more via Lua.
			'jose-elias-alvarez/null-ls.nvim',
			config = [[ require('plugins/null-ls_nvim') ]]
		}

		use { -- A super powerful autopairs for Neovim. It support multiple character.
			'windwp/nvim-autopairs',
			config = [[ require('plugins/nvim-autopairs') ]]
		}

		use { -- Find, Filter, Preview, Pick. All lua, all the time.
			'nvim-telescope/telescope.nvim',
			requires = {
				{'nvim-lua/popup.nvim'},
				{'nvim-lua/plenary.nvim'},
				{'nvim-telescope/telescope-fzf-native.nvim', run = 'make'} -- FZF sorter for telescope written in c
			},
			config = [[ require('plugins/telescope_nvim') ]]
		}

		use { -- Use (neo)vim terminal in the floating/popup window.
			'voldikss/vim-floaterm',
			config = [[ require('plugins/vim-floaterm') ]]
		}

		use { -- lua `fork` of vim-web-devicons for neovim
			'kyazdani42/nvim-web-devicons',
			config = [[ require('plugins/nvim-web-devicons') ]]
		}

		use { -- Maximizes and restores the current window in Vim
			'szw/vim-maximizer',
			config = [[ require('plugins/vim-maximizer') ]]
		}

		use { -- Smart and powerful comment plugin for neovim. Supports commentstring, dot repeat, left-right/up-down motions, hooks, and more
			'numToStr/Comment.nvim',
			config = [[ require('plugins/Comment_nvim') ]]
		}

		use { -- The fastest Neovim colorizer.
			'norcalli/nvim-colorizer.lua',
			config = [[ require('plugins/nvim-colorizer_lua') ]]
		}

		use {
			'lukas-reineke/indent-blankline.nvim',
			config = [[ require('plugins/indent-blankline_nvim') ]]
		}

		use { -- to change current working directory to project's root directory.
			'ygm2/rooter.nvim',
			config = [[ require('plugins/rooter_nvim') ]]
		}

		use { -- Git signs written in pure lua
			'lewis6991/gitsigns.nvim',
			requires = {'nvim-lua/plenary.nvim'},
			config = [[ require('plugins/gitsigns_nvim') ]]
		}

		use { -- A pretty diagnostics, references, telescope results, quickfix and location list to help you solve all the trouble your code is causing.
			'folke/trouble.nvim',
			config = [[ require('plugins/trouble_nvim') ]]
		}

		use { -- A snazzy bufferline for Neovim
			'akinsho/nvim-bufferline.lua',
			requires = 'kyazdani42/nvim-web-devicons',
			config = [[ require('plugins/nvim-bufferline_lua') ]]
		}

		use { -- A File Explorer For Neovim Written In Lua
			'kyazdani42/nvim-tree.lua',
			config = [[ require('plugins/nvim-tree_lua') ]]
		}

		use { -- A minimal, stylish and customizable statusline for Neovim written in Lua
			'feline-nvim/feline.nvim',
			-- requires = {
			--   'nvim-lua/lsp-status.nvim',
			-- },
			config = [[ require('plugins/feline_nvim') ]]
		}

		use { -- fast and highly customizable greeter for neovim.
			"goolord/alpha-nvim",
			requires = {'kyazdani42/nvim-web-devicons'},
			config = [[ require('plugins/alpha-nvim') ]]
		}

		-- ━━━━━━━━━━━━━━━━━❰ DEVELOPMENT ❱━━━━━━━━━━━━━━━━━ --

		----           for flutter/dart
		use { -- Tools to help create flutter apps in neovim using the native lsp
			'akinsho/flutter-tools.nvim',
			ft = {'dart'},
			requires = {
				{'nvim-lua/plenary.nvim'},
				{'Neevash/awesome-flutter-snippets'}, -- collection snippets and shortcuts for commonly used Flutter functions and classes
				{
					'dart-lang/dart-vim-plugin', -- Syntax highlighting for Dart in Vim
					config = [[ require('plugins/dart-vim-plugin') ]]
				}
			},
			config = [[ require('plugins/flutter-tools_nvim') ]]
		}

		--            for Web-Development
		use { --  Use treesitter to auto close and auto rename html tag, work with html,tsx,vue,svelte,php.
			"windwp/nvim-ts-autotag",
			ft = {'html', 'tsx', 'vue', 'svelte', 'php'},
			requires = {'nvim-treesitter/nvim-treesitter'},
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

