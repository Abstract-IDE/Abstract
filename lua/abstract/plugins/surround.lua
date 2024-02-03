--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: nvim-surround
Github: https://github.com/kylechui/nvim-surround

Add/change/delete surrounding delimiter pairs with ease.
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

return {
	"kylechui/nvim-surround",
	version = "*", -- Use for stability; omit to use `main` branch for the latest features
	event = { "InsertEnter" },
	keys = { "c", "s" },

	config = function()
		require("nvim-surround").setup({
			keymaps = { -- vim-surround style keymaps
				visual = "S",
				delete = "ds",
				change = "cs",
				insert = "<C-g>s",
				-- insert_line = "<C-g>S",
				-- normal = "ys",
				-- normal_cur = "yss",
				-- normal_line = "yS",
				-- normal_cur_line = "ySS",
				-- visual_line = "gS",
			},
			surrounds = {
				["("] = { add = { "( ", " )" }, },
				[")"] = { add = { "(", ")" }, },
				["{"] = { add = { "{ ", " }" }, },
				["}"] = { add = { "{", "}" }, },
				["<"] = { add = { "< ", " >" }, },
				[">"] = { add = { "<", ">" }, },
				["["] = { add = { "[ ", " ]" }, },
				["]"] = { add = { "[", "]" }, },
				["'"] = { add = { "'", "'" }, },
				['"'] = { add = { '"', '"' }, },
				["`"] = { add = { "`", "`" }, },
			},
			aliases = {
				["a"] = ">", -- Single character aliases apply everywhere
				["b"] = ")",
				["B"] = "}",
				["r"] = "]",
				-- Table aliases only apply for changes/deletions
				["q"] = { '"', "'", "`" },         -- Any quote character
				["s"] = { ")", "]", "}", ">", "'", '"', "`" }, -- Any surrounding delimiter
			},
			highlight = {                          -- Highlight before inserting/changing surrounds
				duration = 0,
			},
			move_cursor = "begin",
		})
	end
}
