
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ───────────────────────────────────────────────── --
--   Plugin:    nvim-ts-autotag
--   Github:    github.com/windwp/nvim-ts-autotag
-- ───────────────────────────────────────────────── --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --





-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━❰ configs ❱━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

require('nvim-ts-autotag').setup({

	autotag = {
		enable = true,
	}
	filetypes = {
		'html',
		'javascript', 'javascriptreact', 'jsx',
		'typescript', 'typescriptreact', 'tsx',
		'rescript',
		'svelte',
		'vue',
		'xml',
		'php',
		'markdown',
		'glimmer','handlebars','hbs'
	},

	skip_tags = {
	'area', 'base', 'br', 'col', 'command', 'embed', 'hr', 'img', 'slot',
	'input', 'keygen', 'link', 'meta', 'param', 'source', 'track', 'wbr', 'menuitem'
	}
})

-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━❰ end configs ❱━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

