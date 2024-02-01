--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin:    nvim-colorizer.lua
Github:    https://github.com/NvChad/nvim-colorizer.lua

A high-performance color highlighter for Neovim which has
no external dependencies! Written in performant Luajit.
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
	"NvChad/nvim-colorizer.lua",
	event = { "BufReadPre", "BufNewFile", "InsertEnter" },
}

spec.config = function()
	require("colorizer").setup({
		user_default_options = {
			-- RGB = true,       -- #RGB hex codes
			-- RRGGBB = true,    -- #RRGGBB hex codes
			names = false, -- "Name" codes like Blue or blue
			RRGGBBAA = true, -- #RRGGBBAA hex codes
			AARRGGBB = true, -- 0xAARRGGBB hex codes
			rgb_fn = true, -- CSS rgb() and rgba() functions
			hsl_fn = true, -- CSS hsl() and hsla() functions
			-- css = true,          -- Enable all CSS features: rgb_fn, hsl_fn, names, RGB, RRGGBB
			-- css_fn = true,       -- Enable all CSS *functions*: rgb_fn, hsl_fn
			-- Available modes for `mode`: foreground, background,  virtualtext
			mode = "background", -- Set the display mode.
			virtualtext = "■",
		},

		-- all the sub-options of filetypes apply to buftypes
		filetypes = {
			"*", -- Highlight all files, but customize some others.
			css = { rgb_fn = true, names = true },
			-- html = { names = false; }, -- Disable parsing "names" like Blue or Gray
			-- '!vim', -- Exclude vim from highlighting.
			-- 'javascript', -- Highlight for javascript file
			-- html = { mode = 'foreground'; }
		},

		buftypes = {},
	})
end

return spec
