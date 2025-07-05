--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: Bigfile - snacks.nvim
Source: https://github.com/folke/snacks.nvim/blob/main/docs/bigfile.md

bigfile adds a new filetype bigfile to Neovim that triggers when the
file is larger than the configured size. This automatically prevents
things like LSP and Treesitter attaching to the buffer.
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

---@class snacks.bigfile.Config
---@field enabled? boolean
local config = {
	enabled = true,
	notify = true,       -- show notification when big file detected
	size = 5 * 1024 * 1024, -- 5MB
	line_length = 1000,  -- average line length (useful for minified files)
	-- Enable or disable features when big file detected
	---@param ctx {buf: number, ft:string}
	setup = function(ctx)
		if vim.fn.exists(":NoMatchParen") ~= 0 then
			vim.cmd([[NoMatchParen]])
		end
		Snacks.util.wo(0, { foldmethod = "manual", statuscolumn = "", conceallevel = 0 })
		vim.b.minianimate_disable = true
		vim.schedule(function()
			if vim.api.nvim_buf_is_valid(ctx.buf) then
				vim.bo[ctx.buf].syntax = ctx.ft
			end
		end)
	end,
}

return config
