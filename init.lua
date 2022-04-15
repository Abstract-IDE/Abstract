
--  .______       ______        _______. __    __  .__   __.  __  ____    ____  __  .___  ___.
--  |   _  \     /  __  \      /       ||  |  |  | |  \ |  | |  | \   \  /   / |  | |   \/   |
--  |  |_)  |   |  |  |  |    |   (----`|  |__|  | |   \|  | |  |  \   \/   /  |  | |  \  /  |
--  |      /    |  |  |  |     \   \    |   __   | |  . `  | |  |   \      /   |  | |  |\/|  |
--  |  |\  \----|  `--'  | .----)   |   |  |  |  | |  |\   | |  |    \    /    |  | |  |  |  |
--  | _| `._____|\______/  |_______/    |__|  |__| |__| \__| |__|     \__/     |__| |__|  |__|
--                          Author:     Ali Shahid
--                          Github:     github.com/shaeinst



-- ───────────────────────────────────────────────── --
-- ────────────────❰ Leader Mapping ❱─────────────── --
-- mapping leader here. it will work for every mapped

vim.g.mapleader = ';'
vim.g.maplocalleader = '|'
-- ───────────────────────────────────────────────── --




-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━❰ Load/Source Configs ❱━━━━━━━━━━━━━ --

-- plugin config to improve start-up time.
-- it should be always on top on init.lua file
require('plugins/impatient_nvim') -- impatient needs to be setup before any other lua plugin is loaded so it is recommended you add the following near the start of your
require('plugins/filetype_nvim') -- Easily speed up your neovim startup time!

require('configs') -- plugin independent configs
require('mappings') -- plugin independent mappings
require('customs/override_defalut') -- always put this config(override_defalut) at last because it's use to overide the any already defined config


-- load/source PLUGINS CONFIGS
-- loading plugins and its configs are managed in seperate config file, ~/.config/nvim/lua/plugins/packer_nvim.lua
-- NOTE: laways load plugins at last
require('packer_nvim')


-- ━━━━━━━━━━━━━━━━━❰ end of Load ❱━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

