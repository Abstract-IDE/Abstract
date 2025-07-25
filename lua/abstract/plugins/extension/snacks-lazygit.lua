--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: Lazygit - snacks.nvim
Source: https://github.com/folke/snacks.nvim/blob/main/docs/lazygit.md

Automatically configures lazygit with a theme generated based on your
Neovim colorscheme and integrate edit with the current neovim instance.
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local M = {}


---@return snacks.lazygit.Config: snacks.terminal.Opts
function M.config(keymap)
	keymap("folke/snacks.nvim/lazygit")

	---@type snacks.lazygit.Config: snacks.terminal.Opts
	return {
		-- automatically configure lazygit to use the current colorscheme
		-- and integrate edit with the current neovim instance
		configure = true,
		-- extra configuration for lazygit that will be merged with the default
		-- snacks does NOT have a full yaml parser, so if you need `"test"` to appear with the quotes
		-- you need to double quote it: `"\"test\""`
		config = {
			os = { editPreset = "nvim-remote" },
			gui = {
				-- set to an empty string "" to disable icons
				nerdFontsVersion = "3",
			},
		},
		theme_path = vim.fs.normalize(vim.fn.stdpath("cache") .. "/lazygit-theme.yml"),
		-- Theme for lazygit
		theme = {
			[241] = { fg = "Special" },
			-- activeBorderColor = { fg = "MatchParen", bold = true },
			-- cherryPickedCommitBgColor = { fg = "Identifier" },
			-- cherryPickedCommitFgColor = { fg = "Function" },
			-- defaultFgColor = { fg = "Normal" },
			-- inactiveBorderColor = { fg = "FloatBorder" },
			-- optionsTextColor = { fg = "Function" },
			-- searchingActiveBorderColor = { fg = "MatchParen", bold = true },
			-- selectedLineBgColor = { bg = "Visual" }, -- set to `default` to have no background colour
			-- unstagedChangesColor = { fg = "DiagnosticError" },
		},
		win = {
			style = "lazygit",
		},
	}
end

return M
