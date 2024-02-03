--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin:    nvim-ts-autotag
Github:    github.com/windwp/nvim-ts-autotag

 Use treesitter to auto close and auto rename html tag
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
	"windwp/nvim-ts-autotag",
	event = "InsertEnter",
	dependencies = {
		"nvim-treesitter/nvim-treesitter",
	},
	-- stylua: ignore
	ft = {
		"astro", "glimmer", "handlebars", "hbs", "html", "javascript",
		"javascriptreact", "jsx", "markdown", "php", "rescript",
		"svelte", "tsx", "typescript", "typescriptreact", "vue", "xml",
	},
}

spec.config = function()
	require("nvim-ts-autotag").setup()
end

return spec
