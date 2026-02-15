--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: nvim-ts-autotag
Source: github.com/windwp/nvim-ts-autotag

Use treesitter to auto close and auto rename html tag
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

return {
    "windwp/nvim-ts-autotag",
    event = { "InsertEnter", "LspAttach" },
    -- stylua: ignore
    ft = {
        "astro", "glimmer", "handlebars", "hbs", "html", "javascript",
        "javascriptreact", "jsx", "markdown", "php", "rescript",
        "svelte", "tsx", "typescript", "typescriptreact", "vue", "xml",
    },
    opts = {},
}
