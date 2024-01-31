-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ─────────────────────────────────────────────────--
--      Plugin: nvim-autopairs
--      Github: github.com/windwp/nvim-autopairs
-- ─────────────────────────────────────────────────--
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

return {
	"windwp/nvim-autopairs",
	dependencies = { "hrsh7th/nvim-cmp" },
	event = "InsertEnter",

	config = function()
		require('nvim-autopairs').setup()
		-- this is nvim-cmp Plugin dependent setting
		-- If you want insert `(` after select function or method item
		require('cmp').event:on('confirm_done', require('nvim-autopairs.completion.cmp').on_confirm_done())
	end
}
