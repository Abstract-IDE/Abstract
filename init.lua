--  .______       ______        _______. __    __  .__   __.  __  ____    ____  __  .___  ___.
--  |   _  \     /  __  \      /       ||  |  |  | |  \ |  | |  | \   \  /   / |  | |   \/   |
--  |  |_)  |   |  |  |  |    |   (----`|  |__|  | |   \|  | |  |  \   \/   /  |  | |  \  /  |
--  |      /    |  |  |  |     \   \    |   __   | |  . `  | |  |   \      /   |  | |  |\/|  |
--  |  |\  \----|  `--'  | .----)   |   |  |  |  | |  |\   | |  |    \    /    |  | |  |  |  |
--  | _| `._____|\______/  |_______/    |__|  |__| |__| \__| |__|     \__/     |__| |__|  |__|
--                          Author:     Ali Shahid
--                          Github:     github.com/shaeinst




--━━━━━━━━━━━━━━━━❰ Leader Mapping ❱━━━━━━━━━━━━━━━--
-- mapping leader here. it will work for every mapped

vim.g.mapleader = ';'
vim.g.maplocalleader = '|'
--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--


--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━❰ Load/Source Configs ❱━━━━━━━━━━━━━--
-- plugin config to improve start-up time.
-- it should be always on top on init.lua file
require('plugins/impatient_nvim') -- impatient needs to be setup before any other lua plugin is loaded so it is recommended you add the following near the start of your
require('plugins/filetype_nvim')  -- Easily speed up your neovim startup time!

require('configs')
require('mappings')

-- require('customs/roshniline')

-- always put this config(override_defalut) at last because it's use to overide the any already defined config
require('customs/override_defalut')
-- load/source ".__nvim__" configs that are defined in working environment/project
require('customs/project_env')

--━━━━━━━━━━━━━━━━━❰ end of Load ❱━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--



--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━❰ Plugin Manager ❱━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--
--              NOTE:1
--If you want to automatically ensure that packer.nvim is installed on any machine you clone your configuration to,
--add the following snippet (which is due to @Iron-E) somewhere in your config before your first usage of packer:
local execute = vim.api.nvim_command
local fn = vim.fn
local install_path = fn.stdpath('data')..'/site/pack/packer/start/packer.nvim'
if fn.empty(fn.glob(install_path)) > 0 then
  fn.system({'git', 'clone', 'https://github.com/wbthomason/packer.nvim', install_path})
  execute 'packadd packer.nvim'
end


return require('packer').startup{function()
  -- Packer can manage itself
  use 'wbthomason/packer.nvim'

--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━❰ currently Using ❱━━━━━━━━━━━━━━━━--

  --            Improve Start-UP time
  use { -- Speed up loading Lua modules in Neovim to improve startup time.
        'lewis6991/impatient.nvim',
  }

  use { --  Easily speed up your neovim startup time!. A faster version of filetype.vim
        'nathom/filetype.nvim',
  }


  use { --  colorscheme for (neo)vim written in lua specially made for roshnivim
       'shaeinst/roshnivim-cs',
  }

  use { -- A collection of common configurations for Neovim's built-in language server client
        'neovim/nvim-lspconfig',
        config = [[ require('plugins/lspconfig') ]]
  }

  use { -- Companion plugin for nvim-lspconfig that allows you to seamlessly install LSP servers locally (inside :echo stdpath("data")).
        'williamboman/nvim-lsp-installer',
        config = [[ require('plugins/lsp_installer_nvim') ]]
  }

  use { -- vscode-like pictograms for neovim lsp completion items Topics
        'onsails/lspkind-nvim',
        config = [[ require('plugins/lspkind') ]]
  }

  use { -- Nvim Treesitter configurations and abstraction layer
        'nvim-treesitter/nvim-treesitter',
        run = ':TSUpdate',
        config = [[ require('plugins/treesitter') ]]
  }

  use { -- A completion plugin for neovim coded in Lua.
        'hrsh7th/nvim-cmp',
        requires = {
          "hrsh7th/cmp-nvim-lsp",           -- nvim-cmp source for neovim builtin LSP client
          "hrsh7th/cmp-nvim-lua",           -- nvim-cmp source for nvim lua
          "hrsh7th/cmp-buffer",             -- nvim-cmp source for buffer words.
          "hrsh7th/cmp-path",               -- nvim-cmp source for filesystem paths.
          "hrsh7th/cmp-calc",               -- nvim-cmp source for math calculation.
          "saadparwaiz1/cmp_luasnip",       -- luasnip completion source for nvim-cmp
        },
        config = [[ require('plugins/cmp') ]],
  }

  use { -- Snippet Engine for Neovim written in Lua.
        'L3MON4D3/LuaSnip',
        requires = {
          "rafamadriz/friendly-snippets",   -- Snippets collection for a set of different programming languages for faster development.
        },
        config = [[ require('plugins/luasnip') ]],
  }

  use { -- A super powerful autopairs for Neovim. It support multiple character.
        'windwp/nvim-autopairs',
        config = [[ require('plugins/autopairs') ]]
  }

  use { -- Find, Filter, Preview, Pick. All lua, all the time.
        'nvim-telescope/telescope.nvim',
        requires = {
            {'nvim-lua/popup.nvim'},
            {'nvim-lua/plenary.nvim'},
            {'nvim-telescope/telescope-fzf-native.nvim', run = 'make' } -- FZF sorter for telescope written in c
        },
        config = [[ require('plugins/telescope') ]],
  }

  use { -- Use (neo)vim terminal in the floating/popup window.
        'voldikss/vim-floaterm',
        config  = [[ require('plugins/floaterm') ]]
  }

  use { -- lua `fork` of vim-web-devicons for neovim
        'kyazdani42/nvim-web-devicons',
        config = [[ require('plugins/webdevicons_nvim') ]]
  }

  use { -- Maximizes and restores the current window in Vim
        'szw/vim-maximizer',
        config = [[ require('plugins/maximizer') ]]
  }

  use { -- Smart and powerful comment plugin for neovim. Supports commentstring, dot repeat, left-right/up-down motions, hooks, and more
        'numToStr/Comment.nvim',
        config = [[ require('plugins/comment_nvim') ]]
  }

  use {  -- A surround text object plugin for neovim written in lua.
        'blackcauldron7/surround.nvim',
        config = [[ require('plugins/surround_nvim') ]]
  }

  use { -- The fastest Neovim colorizer.
        'norcalli/nvim-colorizer.lua',
        config = [[ require('plugins/colorizer') ]]
  }

  use {
        'lukas-reineke/indent-blankline.nvim',
        config = [[ require('plugins/indent_blankline') ]]
  }

  use { -- to change current working directory to project's root directory.
        'ygm2/rooter.nvim',
        config = [[ require('plugins/rooter_nvim') ]]
  }

  use { -- Git signs written in pure lua
        'lewis6991/gitsigns.nvim',
        requires = {
          'nvim-lua/plenary.nvim',
        },
        config = [[ require('plugins/gitsigns_nvim') ]]
  }

  use { -- A pretty diagnostics, references, telescope results, quickfix and location list to help you solve all the trouble your code is causing.
        'folke/trouble.nvim',
        config = [[ require('plugins/trouble_nvim') ]]
  }

  use { -- A snazzy bufferline for Neovim
        'akinsho/nvim-bufferline.lua',
        requires = 'kyazdani42/nvim-web-devicons',
        config = [[ require('plugins/bufferline_nvim') ]]
  }

  use { -- A File Explorer For Neovim Written In Lua
        'kyazdani42/nvim-tree.lua',
        config = [[ require('plugins/tree_nvim') ]]
  }

  use { -- A minimal, stylish and customizable statusline for Neovim written in Lua
        'Famiu/feline.nvim',
        -- requires = {
        --   'nvim-lua/lsp-status.nvim',
        -- },
        config = [[ require('plugins/feline_nvim') ]],
  }

  use { -- No-nonsense floating terminal plugin for neovim
        "numtostr/FTerm.nvim",
        config = [[ require('plugins/fterm_nvim') ]]
  }

  use { -- No-nonsense floating terminal plugin for neovim
        "mhartington/formatter.nvim",
        config = [[ require('plugins/formatter') ]]
  }

  use { -- fast and highly customizable greeter for neovim.
		"goolord/alpha-nvim",
		requires = { 'kyazdani42/nvim-web-devicons' },
        config = [[ require('plugins/alpha-nvim') ]]
  }

--━━━━━━━━━━━━━━━━━❰ DEVELOPMENT ❱━━━━━━━━━━━━━━━━━--

  ----           for flutter/dart
  use { -- Tools to help create flutter apps in neovim using the native lsp
        'akinsho/flutter-tools.nvim',
        requires = {
              {'nvim-lua/plenary.nvim'},
              {'Neevash/awesome-flutter-snippets'}, -- collection snippets and shortcuts for commonly used Flutter functions and classes
              {
                  'dart-lang/dart-vim-plugin',  -- Syntax highlighting for Dart in Vim
                  config = [[ require('plugins/dart_vim_plugin') ]]
              }
        },
        config = [[ require('plugins/fluttertools') ]],
  }

  --            for Web-Development
  use { --  Use treesitter to auto close and auto rename html tag, work with html,tsx,vue,svelte,php.
        "windwp/nvim-ts-autotag",
        requires = {
              {'nvim-treesitter/nvim-treesitter'},
        },
        config = [[ require('plugins/ts-autotag_nvim') ]],
  }

--━━━━━━━━━━━━━━❰ end of DEVELOPMENT ❱━━━━━━━━━━━━━--


--━━━━━━━━━━━━❰ end currently Using ❱━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--




--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━❰ Not currently Using ❱━━━━━━━━━━━━━━--

--[=======[

  use { -- A light-weight lsp plugin based on neovim built-in lsp with highly a performant UI.
        'glepnir/lspsaga.nvim',
        config = [[ require('plugins/lspsaga') ]]
  }

  use { -- A neovim tabline plugin.
      'romgrk/barbar.nvim',
      config = [[ require('plugins/barbar_nvim') ]]
  }

  use {
        'hoob3rt/lualine.nvim',
        requires = {'kyazdani42/nvim-web-devicons', opt = true},
        config = [[ require('plugins/lualine') ]]
  }

  use { --neovim statusline plugin written in lua
        'glepnir/galaxyline.nvim',
        config = [[ require('plugins/galaxyline_nvim') ]]
  }

  use { -- Neovim commenting plugin, written in lua.
        'b3nj5m1n/kommentary',
        config = [[ require('plugins/kommentary') ]]
  }

  use { --  Github theme for Neovim, kitty, iTerm, Konsole, and Alacritty written in Lua
        "projekt0n/github-nvim-theme",
        config = [[ require('plugins/github-nvim-theme') ]]
  }

  use {
        'lukas-reineke/indent-blankline.nvim',
        config = [[ require('plugins/indent_blankline') ]]
  }

  use { -- Utility functions for getting diagnostic status and progress messages from LSP servers, for use in the Neovim statusline
        'nvim-lua/lsp-status.nvim',
        config = [[ require('plugins/lspstatus') ]]
  }

  use { -- A vim plugin to display the indention levels with thin vertical lines
        'Yggdroot/indentLine',
        config = [[ require('plugins/indentLine') ]]
  }

--]=======]

--━━━━━━━━━━━━━━━❰ end Not Using ❱━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--

end, config = {
  -- Move to lua dir so impatient.nvim can cache it
  compile_path = vim.fn.stdpath('config')..'/plugin/packer_compiled.lua'

  }
}

--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━❰ end of Plugin Manager ❱━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--

