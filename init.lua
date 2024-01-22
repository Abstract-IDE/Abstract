
--    Author:     Ali Shahid
--    Github:     github.com/shaeinst



-- ───────────────────────────────────────────────── --
-- ────────────────❰ Leader Mapping ❱─────────────── --
-- mapping leader here. it will work for every mapped

vim.g.mapleader = ';'
vim.g.maplocalleader = '|'
-- ───────────────────────────────────────────────── --




-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━❰ Load/Source Configs ❱━━━━━━━━━━━━━ --

require('configs')                  -- plugin independent configs
require('mappings')                 -- plugin independent mappings
require('customs/override_defalut') -- always put this config(override_defalut) at last because it's use to overide the any already defined config

-- load/source PLUGINS CONFIGS
-- loading plugins and its configs are managed by lazy.nvim
-- NOTE: always load plugins at last (if possible)
require('manager')

require('customs/abstractline')     -- status line
require('customs/cursor_line')      -- custom cursor line dynamic color

-- ━━━━━━━━━━━━━━━━━❰ end of Load ❱━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

