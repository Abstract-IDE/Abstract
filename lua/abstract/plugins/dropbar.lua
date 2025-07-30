--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: dropbar.nvim
Source: https://github.com/Bekaboo/dropbar.nvim

IDE-like breadcrumbs, out of the box
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
	"Bekaboo/dropbar.nvim",
	lazy = true,
}

spec.setup = function()
	local sources = require('dropbar.sources')
	local custom_path = {
		get_symbols = function(buff, win, cursor)
			local symbols = sources.path.get_symbols(buff, win, cursor)
			symbols[#symbols].name_hl = 'DropBarFileName'
			if vim.bo[buff].modified then
				symbols[#symbols].name = symbols[#symbols].name .. '●'
				symbols[#symbols].name_hl = 'DiffChange'
			end
			-- from filename (don't show dir)
			return { symbols[#symbols] }
		end,
	}

	local config = {
		bar = {
			sources = function(buf, _)
				if vim.bo[buf].ft == 'markdown' then
					return {
						custom_path,
						sources.markdown,
					}
				end
				if vim.bo[buf].buftype == 'terminal' then
					return {
						sources.terminal,
					}
				end
				return {
					custom_path,
					require('dropbar.utils').source.fallback {
						sources.lsp,
						sources.treesitter,
					},
				}
			end,
		},
	}


	return {
		config = config,
		apply = function(cfg)
			require("dropbar").setup(cfg)
		end,
	}
end

return spec
