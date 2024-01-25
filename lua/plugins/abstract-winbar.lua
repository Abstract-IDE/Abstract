--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: abstract-winbar
Github: https://github.com/Abstract-IDE/abstract-winbar

Neovim Winbar: Elevating Awesome. Simple, Dynamic, Navic-Powered.
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]




local _winbar, winbar = pcall(require, "abstract-winbar")
if not _winbar then return end

winbar.setup({
	-- exclude_filetypes = {},
})
