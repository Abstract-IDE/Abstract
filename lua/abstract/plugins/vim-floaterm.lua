--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: vim-floaterm
Source: https://github.com/voldikss/vim-floaterm

💻 Terminal manager for (neo)vim
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
	"voldikss/vim-floaterm",
	lazy = true,
	cmd = {
		"FloatermFirst",
		"FloatermHide",
		"FloatermKill",
		"FloatermLast",
		"FloatermNext",
		"FloatermPrev",
		"FloatermSend",
		"FloatermShow",
		"FloatermToggle",
		"FloatermUpdate",
	},
	keys = require("abstract.configs.mapping").plugin["voldikss/vim-floaterm"],
}

spec.init = function()
	-- Mappings
	-- set mapping with which-key
	require("abstract.utils.map").set_map("voldikss/vim-floaterm")

	-- floaterms provide to set map through global variable.
	-- NOTE: when we set key mapping with globale, it works for all mode.
	--       i tried with which-key mapping, but it only works for normal mode and not terminal mode
	vim.g.floaterm_keymap_toggle = "<C-t>"

	-- Type String. Show floaterm info(e.g., 'floaterm: 1/3' implies there are 3 floaterms in total and the current is the first one) at the top left corner of floaterm window.
	-- Default: 'floaterm: $1/$2'($1 and $2 will be substituted by 'the index of the current floaterm' and 'the count of all floaterms' respectively)
	-- Example: 'floaterm($1|$2)'
	vim.g.floaterm_title = "$1/$2:Terminal"

	-- Type String. Default: &shell
	-- vim.g.floaterm_shell='&shell'

	-- Type String. 'float'(nvim's floating or vim's popup) by default. Set it to 'split' or 'vsplit' if you don't want to use floating or popup window.
	-- vim.g.floaterm_wintype='float'

	-- Type Number (number of columns) or Float (between 0 and 1). If Float, the width is relative to &columns.
	-- Default: 0.6
	vim.g.floaterm_width = 0.6

	-- If Float, the height is relative to &lines.
	-- Type Number (number of lines) or Float (between 0 and 1). Default: 0.6
	vim.g.floaterm_height = 0.4

	-- The position of the floating window. Available values:
	-- Type String. Default: 'botright'
	--     If wintype is split/vsplit: 'leftabove', 'aboveleft', 'rightbelow', 'belowright', 'topleft', 'botright'.
	--     It's recommended to have a look at those options meanings, e.g. :help :leftabove.
	--     If wintype is float: 'top', 'bottom', 'left', 'right', 'topleft', 'topright', 'bottomleft', 'bottomright', 'center', 'auto'(at the cursor place). Default: 'center'
	-- In addition, there is another option 'random' which allows to pick a random position from above when (re)opening a floaterm window.
	vim.g.floaterm_position = "bottom"

	-- 8 characters of the floating window border (top, right, bottom, left, topleft, topright, botright, botleft).
	-- Type String. Default:
	-- vim.g.floaterm_borderchars="─│─│┌┐┘└"

	-- Markers used to detect the project root directory for --cwd=<root> or --cwd=<buffer-root>.
	-- Type List of String. Default: ['.project', '.git', '.hg', '.svn', '.root']
	vim.g.floaterm_rootmarkers = "['.project', '.git', '.hg', '.svn', '.root']"

	-- Whether to override $GIT_EDITOR in floaterm terminals so git commands can open open an editor in the same neovim instance. See git for details. This flag also overrides $HGEDITOR for Mercurial.
	-- Type Boolean. Default: v:true
	-- g:floaterm_giteditor

	-- Command used for opening a file in the outside nvim from within :terminal.
	-- Type String. Default: 'split'
	-- Available: 'edit', 'split', 'vsplit', 'tabe', 'drop' or user-defined commands
	-- g:floaterm_opener

	-- Whether to close floaterm window once the job gets finished.
	--     0: Always do NOT close floaterm window
	--     1: Close window if the job exits normally, otherwise stay it with messages like [Process exited 101]
	--     2: Always close floaterm window
	-- Type Number. Default: 1
	-- vim.g.floaterm_autoclose=1

	-- Whether to hide previous floaterms before switching to or opening a another one.
	-- Type Number. Default: 1
	--     0: Always do NOT hide previous floaterm windows
	--     1: Only hide those whose position (b:floaterm_position) is identical to that of the floaterm which will be opened
	--     2: Always hide them
	-- vim.g.floaterm_autohide=1

	-- Whether to enter Terminal-mode after opening a floaterm.
	-- Type Boolean. Default: v:true
	vim.g.floaterm_autoinsert = false

	-- The position of the floaterm title.
	-- Type String. Default: 'left'
	-- Available: 'left', 'center', 'right'.
	vim.g.floaterm_titleposition = "right"
end

return spec
