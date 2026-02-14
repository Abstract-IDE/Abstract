--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: lazy.nvim
Source: https://github.com/folke/lazy.nvim

💤 A modern plugin manager for Neovim
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local setup = function()
   require("lazy").setup({

      --[[@rs spec = $SPEC, ]]

      root = "--[[@rs $NVIM_PLUGINS_HOME ]]",                         -- directory where plugins will be installed
      -- TODO: change it later with proper path
      lockfile = "--[[@rs $NVIM_LOCK_PATH ]]" .. "/plugin-lock.json", -- lockfile generated after running update.

      performance = {
         cache = { enabled = true },
         reset_packpath = true, -- reset the package path to improve startup time
         rtp = {
            reset = true,       -- reset the runtime path to $VIMRUNTIME and your config directory
            -- add any custom paths here that you want to includes in the rtp
            ---@type string[]
            paths = {
               -- ABSTRACT["INSTALL_PATH"],
               "--[[@rs $NVIM_TS_HOME ]]"
            },
            ---@type string[] list any plugins you want to disable here
            disabled_plugins = { "tutor" }, -- "gzip", "matchit", "matchparen", "netrwPlugin", "tarPlugin", "tohtml", "zipPlugin",

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
            wrap = true,        -- wrap the lines in the ui
            border = "rounded", -- The border to use for the UI window. Accepts same border values as |nvim_open_win()|.
            title_pos = "center", ---@type "center" | "left" | "right"
            throttle = 20,      -- how frequently should the ui process render events
            backdrop = 100,     -- The backdrop opacity. 0 is fully opaque, 100 is fully transparent.
         },
      }
   })
end

local ok, err = pcall(setup)

if not ok then
   vim.notify("[Abstract] lazy.nvim setup failed:\n" .. tostring(err), vim.log.levels.ERROR)
end
