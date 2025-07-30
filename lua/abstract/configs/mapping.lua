local M = {}
local groups = {
	close         = "Close",
	debug         = "Debug",
	debug_ui      = "UI",
	files         = "Files",
	git           = "Version Control - Git",
	git_lazygit   = "Lazygit",
	git_more      = "More git",
	git_picker    = "Picker",
	http          = "HTTP",
	log           = "Logs",
	lsp           = "LSP",
	lsp_preview   = "Preview",
	lsp_trouble   = "Trouble",
	lsp_workspace = "Workspace",
	terminal      = "Terminal",
	window        = "Window",
	run           = "Run Code"
}

-- Mappings that don't depend on any plugin
M.builtin = {
	-- { "??",         ":let @/ = ''<CR>", desc = "Clear last used search pattern" },

	{ "\\",         ":bnext<CR>",     desc = "Goto next buffer" },
	{ "|",          ":bprevious<CR>", desc = "Goto previous buffer" },
	{ "<M-q><M-w>", ":close <CR>",    desc = "Close current window" },
	{ "<M-q><S-w>", ":only <CR>",     desc = "Close all window except current one" },

	-- TAB (:h tab)
	{ "<M-q>t",     ":tabclose<CR>",  desc = "Close current tab" },
	{ "<M-q>T",     ":tabonly<CR>",   desc = "Close all other tab" },
	{ "<Tab>",      ":tabn<CR>",      desc = "Goto next tab" },
	{ "<S-Tab>",    ":tabp<CR>",      desc = "Goto previous tab" },
	{ "<M-S-,>",    ":-tabmove<CR>",  desc = "Move tab to next position" },
	{ "<M-S-.>",    ":+tabmove<CR>",  desc = "Move tab to previous position" },

	{ "<M-h>",      "<C-w>h",         desc = "Move cursor to left window" },
	{ "<M-l>",      "<C-w>l",         desc = "Move cursor to right window" },
	{ "<M-k>",      "<C-w>k",         desc = "Move cursor to above window" },
	{ "<M-j>",      "<C-w>j",         desc = "Move cursor to below window" },

	-- LOGS
	{ "<Leader>Lm", ":messages<CR>",  desc = "Messages history" },
}

-- Mappings that depends on plugin but reqires to override builtin mappings
M.override = {
	-- example:
	-- { "<Leader>lf", "<CMD>lua vim.lsp.buf.format({ timeout_ms = 3000 })<CR>", desc = "Format document" },
}

-- Mappings that depends on plugin
M.plugin = {
	["folke/which-key.nvim"] = {
		{ "<leader>?", "<CMD> lua require('which-key').show({ global = false }) <CR>", desc = "Buffer Local Keymaps (which-key)" },
	},

	["nvim-telescope/telescope.nvim"] = {
		{ "t",     group = "Telescope" },
		{ "tt",    function() require('telescope.builtin').builtin() end,                                    desc = "Telescope builtin" },
		{ "tc",    function() require('telescope.builtin').commands() end,                                   desc = "Commands" },
		{ "th",    function() require('telescope.builtin').help_tags() end,                                  desc = "Help tags" },
		{ "tm",    function() require('telescope.builtin').keymaps() end,                                    desc = "Mappings" },
		{ "tw",    function() require('telescope.builtin').current_buffer_fuzzy_find() end,                  desc = "Find word (current file)" },
		{ "tg",    function() require('telescope').extensions.live_grep_args.live_grep_args() end,           desc = "Find word (project wise)" },
		{ "tG",    function() require('telescope-live-grep-args.shortcuts').grep_word_under_cursor() end,    desc = "Find word under cursor (project wise)" },
		-- Find files from current file's project
		{ "tp",    function() require('telescope').extensions.project.project {} end,                        desc = "Projects picker" },
		{ "<C-p>", function() require('telescope.builtin').find_files() end,                                 desc = "Find File (project dir)", },
		-- Show all files from current working directory
		{ "<C-b>", function() require('telescope.builtin').buffers() end,                                    desc = "Opened buffers", },
		{ "<C-f>", function() require('telescope.builtin').find_files({ cwd = vim.fn.expand('%:p:h') }) end, desc = "Find File (current dir)", },
	},

	["neovim/nvim-lspconfig"] = {
		-- using 'patrickpichler/hovercraft.nvim' for hover
		-- { "K",          "<CMD>lua vim.lsp.buf.hover()<CR>",             desc = "Show symbol hover information", },
		{ "<Leader>l",  group = groups.lsp },
		{ "<Leader>lf", function() vim.lsp.buf.format({ timeout_ms = 3000 }) end,                      desc = "Format document" },
		-- using 'rachartier/tiny-code-action.nvim' for code action
		{ "<Leader>la", function() require('tiny-code-action').code_action({}) end,                    desc = "Code action" },
		{ "<Leader>lA", function() vim.lsp.buf.range_code_action() end,                                desc = "Range code action" },
		{ "<Leader>ld", function() vim.lsp.buf.definition() end,                                       desc = "Jumps to definition" },
		{ "<Leader>lD", function() vim.lsp.buf.declaration() end,                                      desc = "Jumps to declaration" },
		{ "<Leader>le", function() vim.diagnostic.open_float() end,                                    desc = "Show diagnostics" },
		{ "<Leader>li", function() vim.lsp.buf.implementation() end,                                   desc = "Lists all symbol implementations", },
		{ "<Leader>ls", function() vim.lsp.buf.signature_help() end,                                   desc = "Show symbol signature information", },
		{ "<Leader>lT", function() vim.lsp.buf.type_definition() end,                                  desc = "Jumps to type definition" },
		{ "<Leader>lh", function() vim.lsp.inlay_hint.enable(not vim.lsp.inlay_hint.is_enabled()) end, desc = "Inlay hints (toggle)" },
		{ "<Leader>ln", function() vim.diagnostic.jump({ count = 1, float = true }) end,               desc = "Move to next diagnostic" },
		{ "<Leader>lb", function() vim.diagnostic.jump({ count = -1, float = true }) end,              desc = "Move to previous diagnostic" },
		{ "<Leader>lr", function() require('telescope.builtin').lsp_references() end,                  desc = "Lsp references" },
		-- using 'filipdutescu/renamer.nvim' for rename
		{ "<Leader>lR", function() require('renamer').rename({}) end,                                  desc = "Rename symbol" },
		{
			{ "<Leader>lw",  group = groups.lsp_workspace },
			{ "<Leader>lwa", function() vim.lsp.buf.add_workspace_folder() end,                       desc = "Add workspace folder" },
			{ "<Leader>lwr", function() vim.lsp.buf.remove_workspace_folder() end,                    desc = "Remove workspace folders" },
			{ "<Leader>lwl", function() print(vim.inspect(vim.lsp.buf.list_workspace_folders())) end, desc = "List workspace folders" },
		},
		-- others
		{ "<Leader>Ll", function() vim.cmd('tabnew ' .. vim.lsp.get_log_path()) end, desc = "LSP logs" },
	},

	["patrickpichler/hovercraft.nvim"] = {
		{
			"K",
			function()
				local hovercraft = require("hovercraft")
				if hovercraft.is_visible() then hovercraft.enter_popup() else hovercraft.hover() end
			end,
			desc = "Hover"
		},
	},

	["cbochs/grapple.nvim"] = {
		{
			{ "<Leader>g",  group = "Grapple" },
			{ "<Leader>gt", "<CMD>Grapple open_tags<CR>",      desc = "Show tags" },
			{ "<Leader>gl", "<CMD>Grapple open_loaded<CR>",    desc = "Show loaded" },
			{ "<Leader>gs", "<CMD>Grapple open_scopes<CR>",    desc = "Show scopes" },
			{ "<Leader>ga", "<CMD>Grapple toggle<CR>",         desc = "Tag (toggle)" },
			{ "<Leader>gk", "<CMD>Grapple toggle_tags<CR>",    desc = "Tags (toggle)" },
			{ "<Leader>gK", "<CMD>Grapple toggle_scopes<CR>",  desc = "Scopes (toggle)" },
			{ "<Leader>gn", "<CMD>Grapple cycle forward<CR>",  desc = "Goto next tag" },
			{ "<Leader>gp", "<CMD>Grapple cycle backward<CR>", desc = "Goto previous tag" },
		},
		{ "<M-1>", "<CMD>Grapple select index=1<CR>", desc = "Grapple select 1" },
		{ "<M-2>", "<CMD>Grapple select index=2<CR>", desc = "Grapple select 2" },
		{ "<M-3>", "<CMD>Grapple select index=3<CR>", desc = "Grapple select 3" },
		{ "<M-4>", "<CMD>Grapple select index=4<CR>", desc = "Grapple select 4" },
		{ "<M-5>", "<CMD>Grapple select index=5<CR>", desc = "Grapple select 5" },
	},

	["folke/trouble.nvim"] = {
		{ "<leader>lt",  group = groups.lsp_trouble },
		{ "<Leader>ltt", "<CMD>Trouble diagnostics toggle<CR>",                        desc = "Diagnostics - project" },
		{ "<leader>ltT", "<cmd>Trouble diagnostics toggle filter.buf=0<cr>",           desc = "Diagnostics - buffer" },
		{ "<leader>lts", "<cmd>Trouble symbols toggle focus=false<cr>",                desc = "Symbols" },
		{ "<leader>ltl", "<cmd>Trouble lsp toggle focus=false win.position=right<cr>", desc = "LSP Definitions / references / ..." },
		{ "<leader>ltL", "<cmd>Trouble loclist toggle<cr>",                            desc = "Location List" },
		{ "<leader>ltq", "<cmd>Trouble qflist toggle<cr>",                             desc = "Quickfix List" },
	},

	["nvim-neo-tree/neo-tree.nvim"] = {
		{ ";f", ":Neotree toggle<CR>", desc = "File Explorer(toggle)" },
	},

	["smoka7/hop.nvim"] = {
		{ "f", "<CMD>lua require'hop'.hint_words()<CR>", desc = "Jump anywhere" },
	},

	["Shatur/neovim-session-manager"] = {
		{ ";s",  group = "Session Manager" },
		{ ";sl", ":SessionManager load_session<CR>",   desc = "Load sessions" },
		{ ";sd", ":SessionManager delete_session<CR>", desc = "Delete sessions" },
	},

	["rmagatti/goto-preview"] = {
		{ "<Leader>lp",  group = groups.lsp_preview },
		{ "<Leader>lpd", function() require('goto-preview').goto_preview_definition({}) end,      desc = "Definition preview" },
		{ "<Leader>lpt", function() require('goto-preview').goto_preview_type_definition({}) end, desc = "Type definition preview", },
		{ "<Leader>lpi", function() require('goto-preview').goto_preview_implementation({}) end,  desc = "Implementation preview" },
		{ "<Leader>lpD", function() require('goto-preview').goto_preview_declaration({}) end,     desc = "Declaration preview" },
		{ "<Leader>lpr", function() require('goto-preview').goto_preview_references() end,        desc = "References preview" },
		{ "<Leader>lpQ", function() require('goto-preview').close_all_win() end,                  desc = "Close all window preview" },
	},

	["stevearc/oil.nvim"] = {
		{ "-", group = groups.files },
		{ "-", "<CMD>Oil<CR>",      desc = "Open parent directory" },
	},

	["isakbm/gitgraph.nvim"] = {
		{ "<Leader>v", group = groups.git },
		{
			"<Leader>vg",
			function()
				require("gitgraph").draw({}, { all = true, max_count = 5000 })
			end,
			desc = "Git graph",
		},
	},

	["CRAG666/code_runner.nvim"] = {
		{ "<Leader>o",  group = groups.run },
		{ "<leader>oo", ":RunCode<CR>",     desc = "Runs based on file type (Run Code)" },
		{ "<leader>of", ":RunFile<CR>",     desc = "Execute command from its key in current directory (Run Code)" },
		{ "<leader>ot", ":RunFile tab<CR>", desc = "Run the current file (optionally you can select an opening mode)." },
		{ "<leader>op", ":RunProject<CR>",  desc = "Run the current project(If you are in a project otherwise you will not do anything)." },
		{ "<leader>oq", ":RunClose<CR>",    desc = "Close runner (Run Code)" },
		{ "<leader>oj", ":CRFiletype<CR>",  desc = "Open json with supported files (Run Code)" },
		{ "<leader>oJ", ":CRProjects<CR>",  desc = "Open json with list of projects (Run Code)" },
	},

	["mistweaverco/kulala.nvim"] = {
		{ "<Leader>r",  group = groups.http },
		{ "<Leader>rr", "<CMD>lua require('kulala').run()<CR>",              desc = "Make HTTP request" },
		{ "<Leader>rh", "<CMD>lua require('kulala').jump_prev()<CR>",        desc = "Jump to the previous request" },
		{ "<Leader>rl", "<CMD>lua require('kulala').jump_next()<CR>",        desc = "Jump to the next request" },
		{ "<Leader>re", "<CMD>lua require('kulala').set_selected_env()<CR>", desc = "Select environment" },
		{ "<Leader>rt", "<CMD>lua require('kulala').toggle_view()<CR>",      desc = "Response view (Toggle )" },

	},

	["folke/snacks.nvim"] = {
		{ "<Leader>L",  group = groups.log },
		{ "<Leader>Ln", "<CMD>lua Snacks.notifier.show_history()<CR>", desc = "Notifications history - snacks" },
	},
	["folke/snacks.nvim/bufdelete"] = {
		{ "<M-q>",      group = groups.close },
		{ "<M-q><M-q>", "<CMD>lua Snacks.bufdelete()<CR>",       desc = "Delete current buffer", },
		{ "<M-q><S-q>", "<CMD>lua Snacks.bufdelete.other()<CR>", desc = "Delete all buffers except the current one", },
	},
	["folke/snacks.nvim/lazygit"] = {
		{ "<Leader>vL",  group = groups.git_lazygit },
		{ "<Leader>vl",  "<CMD>lua Snacks.lazygit()<CR>",          desc = "open lazygit" },
		{ "<Leader>vLl", "<CMD>lua Snacks.lazygit.log()<CR>",      desc = "log view" },
		{ "<Leader>vLf", "<CMD>lua Snacks.lazygit.log_file()<CR>", desc = "log of the current file" },
	},
	["folke/snacks.nvim/gitbrowse"] = {
		{ "<Leader>vm",  group = groups.git_more },
		{ "<Leader>vmb", "<CMD>lua Snacks.gitbrowse()<CR>", desc = "Git Browse" },
	},

	["OXY2DEV/markview.nvim"] = {
		-- TODO
	},

	["Abstract-IDE/abstract-plugs.nvim/window"] = {
		{ "<Leader>m",  group = groups.window },
		{ "<Leader>mm", function() require("abs").window().toggle.maximize() end, desc = "Window maximizer (toggle)" },
		{
			mode = { "i", "n", "t" },
			{ "<M-m>", function() require("abs").window().toggle.maximize() end, desc = "Window maximizer (toggle)" },
		}
	},
	["Abstract-IDE/abstract-plugs.nvim/terminal"] = {
		mode = { "i", "n", "t" },
		{ "<C>",   group = groups.terminal },
		{ "<C-t>", function() require("abs").terminal().toggle() end, desc = "New toggle (ToggleTerm)" },
		{
			mode = { "t" },
			{ "<C-n>", function() require("abs").terminal().new() end,  desc = "Open new terminal" },
			{ "<C-h>", function() require("abs").terminal().prev() end, desc = "Goto previous terminal" },
			{ "<C-l>", function() require("abs").terminal().next() end, desc = "Goto next terminal" },
		}
	},

	["mfussenegger/nvim-dap"] = {
		{ "<Leader>d",  group = groups.debug },
		{ "<Leader>dc", function() require('dap').continue() end,          desc = "Continue debug" },
		{ "<Leader>ds", function() require('dap').step_over() end,         desc = "Step over" },
		{ "<Leader>di", function() require('dap').step_into() end,         desc = "Step into" },
		{ "<Leader>do", function() require('dap').step_out() end,          desc = "Step out" },
		--
		{ "<Leader>db", function() require('dap').toggle_breakpoint() end, desc = "Breakpoint - toggle" },
		{ "<Leader>dr", function() require('dap').repl.open() end,         desc = "Open REPL" },
		{ "<Leader>dl", function() require('dap').run_last() end,          desc = "Run last" },
		{
			{ "<Leader>du",  group = groups.debug_ui },
			{ "<Leader>dup", function() require('dap.ui.widgets').preview() end, desc = " Preview widget" },
			{
				"<Leader>duf",
				function()
					local widgets = require('dap.ui.widgets')
					widgets.centered_float(widgets.frames)
				end,
				desc = "Frames widget"
			},
			{
				"<Leader>duc",
				function()
					local widgets = require('dap.ui.widgets')
					widgets.centered_float(widgets.scopes)
				end,
				desc = "Scopes widget"
			},
		}
	},
}

return M
