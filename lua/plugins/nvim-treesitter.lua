
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ───────────────────────────────────────────────── --
--   Plugin:    nvim-treesitter
--   Github:    github.com/nvim-treesitter/nvim-treesitter
-- ───────────────────────────────────────────────── --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --





-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━❰ configs ❱━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

require'nvim-treesitter.configs'.setup {

	-- ensure_installed  = "maintained", -- one of "all", "maintained" (parsers with maintainers), or a list of languages
	-- ignore_install    = { "javascript" }, -- List of parsers to ignore installing

	autotag = {
		enable = true,
		filetypes = {
			"html" , "xml",
			'html',
			'javascript', 'javascriptreact', 'jsx',
			'typescript', 'typescriptreact', 'tsx',
			'rescript',
			'svelte',
			'vue',
			'php',
			'markdown',
			'glimmer','handlebars','hbs'
		},
	},

	highlight = {
		enable = true, -- {"c", "cpp", "dart", "python", "javascript"}, enable = true (false will disable the whole extension)
		-- disable lighlight if file is too long
		disable = function(lang, bufnr) -- Disable in large C++ buffers
			-- disable highlight if file has > 6000 LOC
			return vim.api.nvim_buf_line_count(bufnr) > 6000
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

