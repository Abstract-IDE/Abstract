
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ───────────────────────────────────────────────── --
--    Plugin:    nvim-dap | nvim-dap-ui | nvim-dap-virtual-text | telescope-dap.nvim
--    Github:    github.com/mfussenegger/nvim-dap
--               github.com/rcarriga/nvim-dap-ui
--               github.com/theHamsta/nvim-dap-virtual-text
--               github.com/nvim-telescope/telescope-dap.nvim
-- ───────────────────────────────────────────────── --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

--[====[

this plugin is not implemented yet. Will do in future


		-- use { --  DAP, Debug Adapter Protocol client implementation for Neovim
		-- 	'mfussenegger/nvim-dap',
		-- 	requires =	{
		-- 		{'rcarriga/nvim-dap-ui'}, -- A UI for nvim-dap
		-- 		{
		-- 			'theHamsta/nvim-dap-virtual-text', -- This plugin adds virtual text support to nvim-dap. nvim-treesitter is used to find variable definitions.
		-- 			requires = {'nvim-treesitter/nvim-treesitter'},
		-- 		},
		-- 		{
		-- 			'nvim-telescope/telescope-dap.nvim',  --  Integration for nvim-dap with telescope.nvim
		-- 			requires = {'nvim-telescope/telescope.nvim'},
		-- 		},
		-- 	},
		-- 	config = [[ require('plugins/nvim-dap') ]]
		-- }

--]====]




-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━❰ configs ❱━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

--------------------------
-- telescope-dap.nvim
pcall(require'telescope'.load_extension, 'dap')
--------------------------


--------------------------
-- nvim-dap-ui
require("dapui").setup({
	icons = {expanded = "▾", collapsed = "▸"},
	-- Expand lines larger than the window
	-- Requires >= 0.7
	expand_lines = vim.fn.has("nvim-0.7"),
	sidebar = {
		-- You can change the order of elements in the sidebar
		elements = {
			-- Provide as ID strings or tables with "id" and "size" keys
			{
				id = "scopes",
				size = 0.25, -- Can be float or integer > 1
			},
			{id = "breakpoints", size = 0.25},
			{id = "stacks", size = 0.25},
			{id = "watches", size = 00.25},
		},
		size = 40,
		position = "left", -- Can be "left", "right", "top", "bottom"
	},
	tray = {
		elements = {"repl", "console"},
		size = 10,
		position = "bottom", -- Can be "left", "right", "top", "bottom"
	},
	floating = {
		max_height = nil, -- These can be integers or a float between 0 and 1.
		max_width = nil, -- Floats will be treated as percentage of your screen.
		border = "single", -- Border style. Can be "single", "double" or "rounded"
		mappings = {close = {"q", "<Esc>"}},
	},
	windows = {indent = 1},
	render = {
		max_type_length = nil, -- Can be integer or nil.
	},
})

-- end nvim-dap-ui
--------------------------

--------------------------
-- nvim-dap-virtual-text
require("nvim-dap-virtual-text").setup {
	enabled = true, -- enable this plugin (the default)
	enabled_commands = true, -- create commands DapVirtualTextEnable, DapVirtualTextDisable, DapVirtualTextToggle, (DapVirtualTextForceRefresh for refreshing when debug adapter did not notify its termination)
	highlight_changed_variables = true, -- highlight changed values with NvimDapVirtualTextChanged, else always NvimDapVirtualText
	highlight_new_as_changed = false, -- highlight new variables in the same way as changed variables (if highlight_changed_variables)
	show_stop_reason = true, -- show stop reason when stopped for exceptions
	commented = false, -- prefix virtual text with comment string
	only_first_definition = true, -- only show virtual text at first definition (if there are multiple)
	all_references = false, -- show virtual text on all all references of the variable (not only definitions)
	filter_references_pattern = '<module', -- filter references (not definitions) pattern when all_references is activated (Lua gmatch pattern, default filters out Python modules)
	-- experimental features:
	virt_text_pos = 'eol', -- position of virtual text, see `:h nvim_buf_set_extmark()`
	all_frames = false, -- show virtual text for all stack frames not only current. Only works for debugpy on my machine.
	virt_lines = false, -- show virtual lines instead of virtual text (will flicker!)
	virt_text_win_col = nil, -- position the virtual text at a fixed window column (starting from the first text column) ,
	-- e.g. 80 to position at column 80, see `:h nvim_buf_set_extmark()`
}
-- end nvim-dap-virtual-text
--------------------------


--------------------------
-- nvim-dap



-- end nvim-dap
--------------------------

-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━❰ end configs ❱━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --




-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━❰ Mappings ❱━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

--------------------------
-- nvim-dap-ui
require("dapui").setup({
	mappings = {
		-- Use a table to apply multiple mappings
		expand = {"<CR>", "<2-LeftMouse>"},
		open = "o",
		remove = "d",
		edit = "e",
		repl = "r",
		toggle = "t",
	},
})
--------------------------

-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━❰ end Mappings ❱━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

