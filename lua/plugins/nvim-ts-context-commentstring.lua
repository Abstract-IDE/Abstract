
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ───────────────────────────────────────────────── --
--    Plugin:    nvim-ts-context-commentstring
--    Github:    github.com/JoosepAlviste/nvim-ts-context-commentstring
-- ───────────────────────────────────────────────── --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --





-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━❰ configs ❱━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

-- safely import tree-sitter
local treesitter_imported_ok, treesitter =  pcall(require, 'nvim-treesitter.configs')
if not treesitter_imported_ok then return end

treesitter.setup {
	context_commentstring = {
		enable = true,
		enable_autocmd = false,

		config = {
			javascript = {
				__default = '// %s',
				jsx_element = '{/* %s */}',
				jsx_fragment = '{/* %s */}',
				jsx_attribute = '// %s',
				comment = '// %s',
				__parent = {
					-- if a node has this as the parent, use the `//` commentstring
					jsx_expression = '// %s',
				},
			},
			css = '// %s',
		}
	},
}

-- safely import Comment
local comment_imported_ok, comment = pcall(require, 'Comment')
if not comment_imported_ok then return end

comment.setup({

	-- Integration of nvim-ts-context-commentstring to numToStr/Comment.nvim
	---Pre-hook, called before commenting the line
	-- NOTE: Comment.nvim already supports treesitter out-of-the-box except for tsx/jsx.
	pre_hook = function(ctx)
        -- Only calculate commentstring for jsx/tsx filetypes
		local filetype = vim.bo.filetype
        if filetype == 'typescriptreact' or filetype == "javascriptreact" then
            local import_U, U = pcall(require, 'Comment.utils')
			if not import_U then return end
            -- Determine whether to use linewise or blockwise commentstring
            local type = ctx.ctype == U.ctype.linewise and '__default' or '__multiline'
            -- Determine the location where to calculate commentstring from
            local location = nil
            if ctx.ctype == U.ctype.blockwise then
                location = require('ts_context_commentstring.utils').get_cursor_location()
            elseif ctx.cmotion == U.cmotion.v or ctx.cmotion == U.cmotion.V then
                location = require('ts_context_commentstring.utils').get_visual_start_location()
            end
            return require('ts_context_commentstring.internal').calculate_commentstring({
                key = type,
                location = location,
            })
        end
	end,
})

-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━❰ end configs ❱━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

