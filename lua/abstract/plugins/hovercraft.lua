--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: hovercraft.nvim
Source: https://github.com/patrickpichler/hovercraft.nvim

hovercraft.nvim is a plug and play framework for writing custom hover provider.
It brings a few providers out of the box, such as a LSP, as well as a Dictionary.
It allows for basic customizations of the hover window
(such as defining custom borders, set the max width and so on).
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
	"patrickpichler/hovercraft.nvim",
	lazy = true,
	keys = require("abstract.configs.mapping").plugin["patrickpichler/hovercraft.nvim"],
}

spec.opts = function()
	return {
		providers = {
			providers = {
				{ "LSP", require("hovercraft.provider.lsp.hover").new() },
				{ "Man", require("hovercraft.provider.man").new() },
				{ "Dictionary", require("hovercraft.provider.dictionary").new() },
				-- { "Git Blame", require("hovercraft.provider.git.blame").new() },
				-- { 'Diagnostics',  Provider.Diagnostics.new(), },
				-- { 'LSP',          Provider.Lsp.Hover.new(), },
				-- { 'Man',          Provider.Man.new(), },
				-- { 'Github Issue', Provider.Github.Issue.new(), },
				-- { 'Github Repo',  Provider.Github.Repo.new(), },
				-- { 'Github User',  Provider.Github.User.new(), },
				-- { 'Diagnostics',  Provider.Diagnostics.new(), },
			},
		},

		window = {
			border = "rounded",
		},

		keys = {
			{
				"<C-u>",
				function()
					require("hovercraft").scroll({ delta = -2 })
				end,
			},
			{
				"<C-d>",
				function()
					require("hovercraft").scroll({ delta = 2 })
				end,
			},
			{
				"<TAB>",
				function()
					require("hovercraft").hover_next()
				end,
			},
			{
				"<S-TAB>",
				function()
					require("hovercraft").hover_next({ step = -1 })
				end,
			},
		},
	}
end

return spec
