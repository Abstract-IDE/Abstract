-- ───────────────────────────────────────────────── --
--    Author:     Ali Shahid
--    Github:     github.com/shaeinst
-- ───────────────────────────────────────────────── --

-- Leader Mapping
-- mapping leader here. it will work for every mapped
vim.g.mapleader = ";"
vim.g.maplocalleader = "|"

-- Load/Source Configs
require("configs")                  -- configs  (plugin independent)
require("autocmds")                 -- autocmds (plugin independent)
require("mappings")                 -- mappings (plugin independent)
require("customs/override_defalut") -- Always put this config (override_default) at last because it's used to override any previously defined config

-- load/source PLUGINS CONFIGS
-- loading plugins and its configs are managed by lazy.nvim
-- NOTE: always load plugins at last (if possible)
require("manager")
