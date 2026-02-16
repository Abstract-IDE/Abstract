--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: nvim-ts-autotag
Source: github.com/windwp/nvim-ts-autotag

Use treesitter to auto close and auto rename html tag
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
    "windwp/nvim-ts-autotag",
    event = { "InsertEnter", "LspAttach" },
    -- stylua: ignore
    ft = {
        "astro", "glimmer", "handlebars", "hbs", "html", "javascript",
        "javascriptreact", "jsx", "markdown", "php", "rescript",
        "svelte", "tsx", "typescript", "typescriptreact", "vue", "xml",
    },
}

spec.opts = {
    opts = {
        enable_close = true,      -- Auto close tags
        enable_rename = true,     -- Auto rename pairs of tags
        enable_close_on_slash = false -- Auto close on trailing </
    },

    -- Also override individual filetype configs, these take priority.
    -- Empty by default, useful if one of the "opts" global settings
    -- doesn't work well in a specific filetype
    per_filetype = {
        ["html"] = {
            enable_close = false
        }
    }
}


return spec
