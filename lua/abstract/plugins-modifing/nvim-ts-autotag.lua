
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ───────────────────────────────────────────────── --
--   Plugin:    nvim-ts-autotag
--   Github:    github.com/windwp/nvim-ts-autotag
-- ───────────────────────────────────────────────── --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --





-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━❰ configs ❱━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

local import_tag, autotag = pcall(require, "nvim-ts-autotag")
if not import_tag then return end

autotag.setup({

	autotag = {
		enable = true,
	},

	filetypes = {
		'html', 'htmldjango',
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

