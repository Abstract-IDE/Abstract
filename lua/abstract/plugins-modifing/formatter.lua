
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ───────────────────────────────────────────────── --
--   Plugin:    formatter.nvim
--   Github:    github.com/mhartington/formatter.nvim
-- ───────────────────────────────────────────────── --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --





-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━❰ configs ❱━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

require('formatter').setup({
	logging = false,

	filetype = {
		-- C++
		cpp = {
			function()
				return {
					exe = "clang-format",
					args = {
						'-assume-filename=',
						vim.fn.shellescape(vim.api.nvim_buf_get_name(0)),
						'-style="{BasedOnStyle: Microsoft, UseTab: Always}"'
					},
					stdin = true
				}
			end
		},

		-- C
		c = {
			function()
				return {
					exe = "clang-format",
					args = {
						'-assume-filename=',
						vim.fn.shellescape(vim.api.nvim_buf_get_name(0)),
						'-style="{BasedOnStyle: Microsoft, UseTab: Always}"'
					},
					stdin = true
				}
			end
		},

		-- Python
		python = {function() return {exe = "yapf", stdin = true} end},

		-- Javascript
		javascript = {
			function()
				return {
					exe = "js-beautify",
					args = {'--indent-size 4', '--max-preserve-newlines 3'},
					stdin = true
				}
			end
		},

		-- Lua
		lua = {
			function()
				return {
					exe = "lua-format",
					args = {'--indent-width 1', '--tab-width 4', '--use-tab'},
					stdin = true
				}
			end
		}
	}

})

-- Format on save
--      To enable format on save, you can create a autocmd to trigger the formater using FormatWrite,
--              which will format and write to the current saved file.
-- vim.api.nvim_exec([[
-- augroup FormatAutogroup
--   autocmd!
--   autocmd BufWritePost *.py,*.c,*.cpp,*.js FormatWrite
-- augroup END
-- ]], true)

-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━❰ end configs ❱━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --





-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━❰ Mappings ❱━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

-- local keymap = vim.api.nvim_set_keymap
-- keymap('n', '<Space>fm', '<ESC>:Format<CR>', {noremap = true, silent = true})

-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━❰ end Mappings ❱━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

