
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ───────────────────────────────────────────────── --
--   Plugin:    nvim-treesitter
--   Github:    github.com/nvim-treesitter/nvim-treesitter
-- ───────────────────────────────────────────────── --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --





-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━❰ configs ❱━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

-- safely import tree-sitter
local treesitter_imported_ok, treesitter =  pcall(require, 'nvim-treesitter.configs')
if not treesitter_imported_ok then return end


local register = vim.treesitter.language.register
register('html', 'htmldjango') -- enable html parser in htmldjango file
register('bash', 'zsh') -- enable bash parser in zsh file


treesitter.setup {
	modules = {}, -- this option is not mentioned in doc. i am providing it to hide warning emmiting when editing this config
	ensure_installed  = {}, -- A list of parser names, or "all" (the five listed parsers should always be installed)
	sync_install = false, -- Install parsers synchronously (only applied to `ensure_installed`)
	auto_install = false, -- Automatically install missing parsers when entering buffer. Recommendation: set to false if you don't have `tree-sitter` CLI installed locally
	ignore_install    = {}, -- List of parsers to ignore installing (or "all")

	-- Indentation based on treesitter for the = operator. NOTE: This is an experimental feature.
	indent = {
		enable = true
	},

	highlight = {
		enable = true, -- {"c", "cpp", "dart", "python", "javascript"}, enable = true (false will disable the whole extension)
		-- disable lighlight if file is too long
		disable = function() -- Disable in large C++ buffers
			-- disable highlight if file has > 6000 LOC
			return vim.api.nvim_buf_line_count(0) > 6000
			-- return lang == "cpp" and vim.api.nvim_buf_line_count(bufnr) > 50000
		end,
		-- disable = { "c", "rust" },  -- list of language that will be disabled
		custom_captures = {
			-- Highlight the @foo.bar capture group with the "Identifier" highlight group.
			["foo.bar"] = "Identifier",
		},
		-- Setting this to true will run `:h syntax` and tree-sitter at the same time.
		-- Set this to `true` if you depend on 'syntax' being enabled (like for indentation).
		-- Using this option may slow down your editor, and you may see some duplicate highlights.
		-- Instead of true it can also be a list of languages
		additional_vim_regex_highlighting = true,
	},

	playground = {
		enable = true,
		disable = {},
		updatetime = 25, -- Debounced time for highlighting nodes in the playground from source code
		persist_queries = false, -- Whether the query persists across vim sessions

-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━❰ end configs ❱━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --




-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━❰ Mappings ❱━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
		-- keybindings = {
		--     toggle_query_editor = 'o',
		--     toggle_hl_groups = 'i',
		--     toggle_injected_languages = 't',
		--     toggle_anonymous_nodes = 'a',
		--     toggle_language_display = 'I',
		--     focus_language = 'f',
		--     unfocus_language = 'F',
		--     update = 'R',
		--     goto_node = '<cr>',
		--     show_help = '?'
		-- }
	},
}

-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━❰ end Mappings ❱━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

