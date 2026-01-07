/*
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: neovim-session-manager
Source: https://github.com/Shatur/neovim-session-manager

A simple wrapper around :mksession.
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
*/

use crate::core::keymaps;

pub struct Plugin;

impl Plugin {
    pub fn spec() -> &'static str {
        let config = Self::config();
        let keys = keymaps::KEYMAPS.get_map(keymaps::MapKey::SessionManager);

        format!(
            // language=lua
            r#"{{
                "Shatur/neovim-session-manager",
                event = "BufWinEnter",
                cmd = {{ "SessionManager" }},
                keys = {keys},
                config = {config},
            }}"#
        )
        .leak()
    }
}

impl Plugin {
    pub fn config() -> &'static str {
        keymaps::MapLoader::signal(keymaps::MapKey::SessionManager);

        // language=lua
        r#"function()
            require("session_manager").setup({
                sessions_dir = vim.fn.stdpath("data") .. "/.cache/sessions",       -- The directory where the session files will be saved.
                path_replacer = "__",                                              -- The character to which the path separator will be replaced for session files.
                colon_replacer = "++",                                             -- The character to which the colon symbol will be replaced for session files.
                autoload_mode = require("session_manager.config").AutoloadMode.Disabled, -- Define what to do when Neovim is started without arguments. Possible values: Disabled, CurrentDir, LastSession
                autosave_last_session = true,                                      -- Automatically save last session on exit and on session switch.
                autosave_ignore_not_normal = true,                                 -- Plugin will not save a session when no buffers are opened, or all of them aren't writable or listed.
                autosave_ignore_filetypes = {                                      -- All buffers of these file types will be closed before the session is saved.
                    "gitcommit",
                    "gitrebase",
                },
                autosave_only_in_session = false, -- Always autosaves session. If true, only autosaves after a session is active.
                max_path_length = 80,       -- Shorten the display path if length exceeds this threshold. Use 0 if don't want to shorten the path at all.
            })

            -- -- if you would like to have NvimTree or any other file tree automatically opened after a session load,
            -- local config_group = vim.api.nvim_create_augroup('MyConfigGroup', {}) -- A global group for all your config autocommands
            -- vim.api.nvim_create_autocmd(
            -- 	{ 'SessionLoadPost' }, {
            -- 	group = config_group,
            -- 	callback = function()
            -- 		require('nvim-tree').toggle(false, true)
            -- 	end,
            -- })
        end"#
    }
}
