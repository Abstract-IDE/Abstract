--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: kulala.nvim
Source: https://github.com/mistweaverco/kulala.nvim
A minimal REST-Client Interface for Neovim.
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
    "mistweaverco/kulala.nvim",
    lazy = true,
    ft = "http",
}

spec.config = function()
    require("kulala").setup({
        split_direction = "horizontal",
        default_view = "body",
        default_env = "dev",
        debug = false,
        formatters = {
            json = { "jq", "." },
            xml = { "xmllint", "--format", "-" },
            html = { "xmllint", "--format", "--html", "-" },
        },
        icons = {
            inlay = {
                loading = "⏳",
                done = "✅",
                error = "❌",
            },
            lualine = "🐼",
        },
        additional_curl_options = {},
        winbar = false,
    })
    --[[@rs $MAPPING_SET ]]
end

return spec
