
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ───────────────────────────────────────────────── --
--   Plugin:    nvim-colorizer.lua
--   Github:    github.com/NvChad/nvim-colorizer.lua
-- ───────────────────────────────────────────────── --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --





-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━❰ configs ❱━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

-- safely import colorizer
local colorizer_imported_ok, colorizer = pcall(require, 'colorizer')
if not colorizer_imported_ok then return end

colorizer.setup {

	user_default_options = {
		-- RGB = true,       -- #RGB hex codes
		-- RRGGBB = true,    -- #RRGGBB hex codes
		names = false,    -- "Name" codes like Blue or blue
		-- RRGGBBAA = false, -- #RRGGBBAA hex codes
		-- AARRGGBB = false, -- 0xAARRGGBB hex codes
		-- rgb_fn = false,   -- CSS rgb() and rgba() functions
		-- hsl_fn = false,   -- CSS hsl() and hsla() functions
		css = true,          -- Enable all CSS features: rgb_fn, hsl_fn, names, RGB, RRGGBB
		css_fn = true,       -- Enable all CSS *functions*: rgb_fn, hsl_fn
		-- Available modes for `mode`: foreground, background,  virtualtext
		mode = "background", -- Set the display mode.
		virtualtext = "■",
	},

	-- all the sub-options of filetypes apply to buftypes
	filetypes = {
		'*', -- Highlight all files, but customize some others.
		css = { rgb_fn = true, names = true },
		-- html = { names = false; }, -- Disable parsing "names" like Blue or Gray
		-- '!vim', -- Exclude vim from highlighting.
		-- 'javascript', -- Highlight for javascript file
		-- html = { mode = 'foreground'; }
	},

	buftypes = {},
}

-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━❰ end configs ❱━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

