--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: markdown-preview.nvim
Source: https://github.com/selimacerbas/mdkite.nvim

Live Markdown preview for Neovim with Mermaid diagrams,
LaTeX math (KaTeX), scroll sync, and syntax highlighting.
Pure Lua, zero npm dependencies.

NOTE:
Before 2.0.0 this plugin was https://github.com/selimacerbas/markdown-preview.nvim
which was itself fork of https://github.com/iamcco/markdown-preview.nvim
actually i am not sure if selimacerbas/markdown-preview.nvim was fork of iamcco/markdown-preview.nvim
but in my note its mentioned that 'old: iamcco/markdown-preview.nvim', but one think i am sure that i
were using iamcco/markdown-preview.nvim before selimacerbas/markdown-preview.nvim
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
    "selimacerbas/mdkite.nvim",
    dependencies = { "selimacerbas/kitehost.nvim" },
}

spec.config = function()
    require("mdkite").setup({
        -- all optional; sane defaults shown
        instance_mode = "takeover", -- "takeover" (one tab) or "multi" (tab per instance)
        port = 0,                   -- 0 = auto (8421 for takeover, OS-assigned for multi)
        open_browser = true,
        default_theme = "dark",     -- "dark" or "light"; initial preview theme
        debounce_ms = 300,
    })
end


return spec
