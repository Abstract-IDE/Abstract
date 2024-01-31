-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ───────────────────────────────────────────────── --
--    Plugin:    Abstract-cs
--    Github:    github.com/Abstract-IDE/Abstract-cs
-- ───────────────────────────────────────────────── --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --





-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━❰ configs ❱━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --


return {           -- colorscheme for (neo)vim written in lua specially made for Abstract
	"Abstract-IDE/Abstract-cs",
	lazy = false,  -- make sure we load this during startup if it is your main colorscheme
	priority = 1000, -- make sure to load this before all the other start plugins
	config = function()
		-- apply colorscheme without throwing any errors
		local apply_cs, _ = pcall(vim.cmd, 'colorscheme abscs')
		if not apply_cs then return end

		-- theme name
		vim.g.abscs_theme_name = 'aqua'
	end,
}
