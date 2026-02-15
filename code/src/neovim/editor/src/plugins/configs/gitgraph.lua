--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: gitgraph.nvim
Source: https://github.com/isakbm/gitgraph.nvim
Git Graph plugin for neovim.
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
    "isakbm/gitgraph.nvim",
    dependencies = { "sindrets/diffview.nvim" },
    event = "BufEnter",
}

spec.config = function()
    require('gitgraph').setup({
        symbols = {
            merge_commit = "M",
            commit = "*",
        },
        format = {
            timestamp = "%H:%M:%S %d-%m-%Y",
            fields = { "hash", "timestamp", "author", "branch_name", "tag" },
        },
    })
    --[[@rs $MAPPING_SET ]]
end

return spec
