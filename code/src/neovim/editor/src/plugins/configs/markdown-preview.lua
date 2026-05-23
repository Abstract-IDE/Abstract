--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: markdown-preview.nvim
Source: https://github.com/selimacerbas/markdown-preview.nvim

OLD: https://github.com/iamcco/markdown-preview.nvim

Live Markdown preview for Neovim with Mermaid diagrams,
LaTeX math (KaTeX), scroll sync, and syntax highlighting
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
    "selimacerbas/markdown-preview.nvim",
    dependencies = { "selimacerbas/live-server.nvim" },
}

spec.config = function()
    require("markdown_preview").setup({
        -- all optional; sane defaults shown
        instance_mode = "takeover", -- "takeover" (one tab) or "multi" (tab per instance)
        port = 9696,                -- 0 = auto (8421 for takeover, OS-assigned for multi)
        open_browser = true,
        debounce_ms = 300,
    })
end


return spec
