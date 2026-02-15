--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: typescript-tools.nvim
Source: https://github.com/pmizio/typescript-tools.nvim

⚡ TypeScript integration NeoVim deserves ⚡
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

return {
    "pmizio/typescript-tools.nvim",
    ft = { "javascript", "typescript", "javascriptreact", "typescriptreact" },
    lazy = true,
    opts = {
        separate_diagnostic_server = true,
        publish_diagnostic_on = "change",
        expose_as_code_action = {},
        tsserver_path = nil,
        tsserver_plugins = {},
        tsserver_max_memory = "8192",
        tsserver_format_options = {},
        tsserver_file_preferences = {
            includeCompletionsForModuleExports = true,
        },
        tsserver_locale = "en",
        complete_function_calls = false,
        include_completions_with_insert_text = true,
        code_lens = "off",
        disable_member_code_lens = true,
        jsx_close_tag = {
            enable = true,
            filetypes = { "javascriptreact", "typescriptreact" },
        },
    },
}
