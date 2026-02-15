--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: neovim-session-manager
Source: https://github.com/Shatur/neovim-session-manager
A simple wrapper around :mksession.
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
    "Shatur/neovim-session-manager",
    event = "BufWinEnter",
    cmd = { "SessionManager" },
}

spec.config = function()
    require("session_manager").setup({
        sessions_dir = vim.fn.stdpath("data") .. "/.cache/sessions",
        path_replacer = "__",
        colon_replacer = "++",
        autoload_mode = require("session_manager.config").AutoloadMode.Disabled,
        autosave_last_session = true,
        autosave_ignore_not_normal = true,
        autosave_ignore_filetypes = {
            "gitcommit",
            "gitrebase",
        },
        autosave_only_in_session = false,
        max_path_length = 80,
    })
    --[[@rs $MAPPING_SET ]]
end

return spec
