-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ───────────────────────────────────────────────── --
--    Plugin:       nvim-surround
--    Github:       github.com/kylechui/nvim-surround
--    Description:  Add/change/delete surrounding delimiter pairs with ease.
-- ───────────────────────────────────────────────── --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --





-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━❰ configs ❱━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

local imported_surround, surround = pcall(require, 'nvim-surround')
if not imported_surround then return end


surround.setup({
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
                ["q"] = { '"', "'", "`" }, -- Any quote character
                ["s"] = { ")", "]", "}", ">", "'", '"', "`" }, -- Any surrounding delimiter
        },
	highlight= { -- Highlight before inserting/changing surrounds
		duration = 0,
	},
        move_cursor = "begin",
})

-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━❰ end configs ❱━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

