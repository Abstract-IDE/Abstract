--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: abstract-line
Github: https://github.com/Abstract-IDE/abstract-line

status line for Abstract-IDE
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local _statusline, statusline = pcall(require, "abstract-line")
if not _statusline then
	return
end

statusline.setup()
