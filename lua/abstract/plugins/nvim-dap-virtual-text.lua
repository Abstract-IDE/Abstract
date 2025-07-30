--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: nvim-dap-virtual-text
Source: https://github.com/theHamsta/nvim-dap-virtual-text

This plugin adds virtual text support to nvim-dap.
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
	"theHamsta/nvim-dap-virtual-text",
}

spec.opts = {
	virt_text_pos = 'eol',
}

return spec
