--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: telescope-media-files.nvim
Github: https://github.com/nvim-telescope/telescope-media-files.nvim

 Telescope extension to preview media files using Ueberzug.
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

return {
	"nvim-telescope/telescope-media-files.nvim",
	lazy = true,
	dependencies = { "nvim-telescope/telescope.nvim" },
	config = function()
		return {
			-- filetypes whitelist
			filetypes = { "png", "jpg", "mp4", "webm", "pdf" },
			find_cmd = "fd", -- find command (defaults to `fd`)
		}
	end
}
