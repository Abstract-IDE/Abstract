--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: nvim-ts-context-commentstring
Github: https://github.com/JoosepAlviste/nvim-ts-context-commentstring

Neovim treesitter plugin for setting the commentstring based on the cursor location in a file.
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = { -- Neovim treesitter plugin for setting the commentstring based on the cursor location in a file.
	"JoosepAlviste/nvim-ts-context-commentstring",
	lazy = true,
}

spec.config = function()
	-- skip backwards compatibility routines and speed up loading.
	vim.g.skip_ts_context_commentstring_module = true

	require("ts_context_commentstring").setup({
		enable_autocmd = false,
	})

	-- Integration of nvim-ts-context-commentstring to numToStr/Comment.nvim
	return require("ts_context_commentstring.integrations.comment_nvim").create_pre_hook()
end

return spec
