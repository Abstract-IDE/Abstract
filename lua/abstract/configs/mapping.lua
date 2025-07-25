local M = {}
local groups = {
	close         = "Close",
	files         = "Files",
	git           = "Version Control (Git)",
	git_lazygit   = "Lazygit",
	git_more      = "More",
	git_picker    = "Picker",
	http          = "HTTP",
	lsp           = "LSP",
	lsp_workspace = "Workspace",
	terminal      = "Terminal",
	window        = "Window",

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
		-- { "<Leader>f",  "<CMD>lua vim.lsp.buf.format({ timeout_ms = 3000 })<CR>", desc = "Format document" },
		-- using 'patrickpichler/hovercraft.nvim' for hover
		-- { "K",          "<CMD>lua vim.lsp.buf.hover()<CR>",             desc = "Show symbol hover information", },
		-- { "<Leader>rn", "<CMD>lua vim.lsp.buf.rename()<CR>",            desc = "Rename symbol" },
		{ "<Leader>e", function() vim.diagnostic.open_float() end,               desc = "Show diagnostics" },
		{ "<Leader>d", function() vim.lsp.buf.definition() end,                  desc = "Jumps to definition" },
		-- using 'filipdutescu/renamer.nvim' for rename
		{ "<Leader>R", function() require('renamer').rename({}) end,             desc = "Rename symbol" },
		-- using 'rachartier/tiny-code-action.nvim' for code action
		{ "<Leader>a", "<CMD>lua require('tiny-code-action').code_action()<CR>", desc = "Code action" },
		{
			{ "<Leader>l",  group = groups.lsp },
			{ "<Leader>lf", function() vim.lsp.buf.format({ timeout_ms = 3000 }) end,                      desc = "Format document" },
			{ "<Leader>lA", function() vim.lsp.buf.range_code_action() end,                                desc = "Range code action" },
			{ "<Leader>ld", function() vim.lsp.buf.declaration() end,                                      desc = "Jumps to declaration" },
			{ "<Leader>li", function() vim.lsp.buf.implementation() end,                                   desc = "Lists all symbol implementations", },
			{ "<Leader>ls", function() vim.lsp.buf.signature_help() end,                                   desc = "Show symbol signature information", },
			{ "<Leader>lt", function() vim.lsp.buf.type_definition() end,                                  desc = "Jumps to type definition" },
			{ "<Leader>lh", function() vim.lsp.inlay_hint.enable(not vim.lsp.inlay_hint.is_enabled()) end, desc = "Inlay hints (toggle)" },
			{ "<Leader>ln", function() vim.diagnostic.jump({ count = 1, float = true }) end,               desc = "Move to next diagnostic" },
			{ "<Leader>lb", function() vim.diagnostic.jump({ count = -1, float = true }) end,              desc = "Move to previous diagnostic" },
			{ "<Leader>lr", function() require('telescope.builtin').lsp_references() end,                  desc = "Lsp references" },
			{
				{ "<Leader>lw",  group = groups.lsp_workspace },
				{ "<Leader>lwa", function() vim.lsp.buf.add_workspace_folder() end,                       desc = "Add workspace folder" },
				{ "<Leader>lwr", function() vim.lsp.buf.remove_workspace_folder() end,                    desc = "Remove workspace folders" },
				{ "<Leader>lwl", function() print(vim.inspect(vim.lsp.buf.list_workspace_folders())) end, desc = "List workspace folders" },
			}
		}
	},

	["zbirenbaum/copilot.lua"] = {
		mode = { "i" },
		{ "<M-l>", "<CMD>lua require('copilot.suggestion').accept()<CR>",  desc = "Accept suggestion (Copilot)" },
		{ "<M-h>", "<CMD>lua require('copilot.suggestion').dismiss()<CR>", desc = "Dismiss suggestion (Copilot)" },
		{ "<M-j>", "<CMD>lua require('copilot.suggestion').next()<CR>",    desc = "Next suggestion (Copilot)" },
		{ "<M-k>", "<CMD>lua require('copilot.suggestion').prev()<CR>",    desc = "Previous suggestion (Copilot)" },
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

	["ThePrimeagen/harpoon"] = {
		{
			{ "<Leader>g", group = "Harpoon" },
			{ "<Leader>h", "<CMD>lua HarpoonTelescope()<CR>",    desc = "Open harpoon window" },
			{ "<Leader>a", "<CMD>lua Harpoon_List:append()<CR>", desc = "Add current buffer to harpoon" },
			{ "<Leader>r", "<CMD>lua Harpoon_List:remove()<CR>", desc = "Remove current buffer from harpoon" },
			{ "<Leader>p", "<CMD>lua Harpoon_List:prev()<CR>",   desc = "Goto previous in harpoon list" },
			{ "<Leader>n", "<CMD>lua Harpoon_List:next()<CR>",   desc = "Goto next in harpoon list" },
		},
		{ "<C-h>",     "<CMD>lua HarpoonTelescope()<CR>",     desc = "Open harpoon window" },
		{ "<Leader>1", "<CMD>lua Harpoon_List:select(1)<CR>", desc = "Goto 1st file in harpoon" },
		{ "<Leader>2", "<CMD>lua Harpoon_List:select(2)<CR>", desc = "Goto 2nd file in harpoon" },
		{ "<Leader>3", "<CMD>lua Harpoon_List:select(3)<CR>", desc = "Goto 3rd file in harpoon" },
		{ "<Leader>4", "<CMD>lua Harpoon_List:select(4)<CR>", desc = "Goto 4th file in harpoon" },
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
		{ "<leader>T",  group = groups.lsp },
		{ "<leader>t",  "<cmd>Trouble diagnostics toggle filter.buf=0<cr>",           desc = "Buffer Diagnostics (Trouble)" },
		{ "<Leader>Tt", "<CMD>Trouble diagnostics toggle<CR>",                        desc = "Diagnostics (Trouble)" },
		{ "<leader>Ts", "<cmd>Trouble symbols toggle focus=false<cr>",                desc = "Symbols (Trouble)" },
		{ "<leader>Tl", "<cmd>Trouble lsp toggle focus=false win.position=right<cr>", desc = "LSP Definitions / references / ... (Trouble)" },
		{ "<leader>TL", "<cmd>Trouble loclist toggle<cr>",                            desc = "Location List (Trouble)" },
		{ "<leader>Tq", "<cmd>Trouble qflist toggle<cr>",                             desc = "Quickfix List (Trouble)" },
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
		{ "gp",  group = groups.lsp },
		{ "gpd", function() require('goto-preview').goto_preview_definition() end,      desc = "Preview definition" },
		{ "gpt", function() require('goto-preview').goto_preview_type_definition() end, desc = "Preview type definition", },
		{ "gpi", function() require('goto-preview').goto_preview_implementation() end,  desc = "Preview definition" },
		{ "gpD", function() require('goto-preview').goto_preview_declaration() end,     desc = "Preview declaration" },
		{ "gpr", function() require('goto-preview').goto_preview_references() end,      desc = "Preview definition" },
		{ "gpQ", function() require('goto-preview').close_all_win() end,                desc = "Close all window" },
	},

	["anuvyklack/windows.nvim"] = {
		{ ";m", ":WindowsMaximize<CR>", desc = "Window maximizer (toggle)" },
	},

	["stevearc/oil.nvim"] = {
		{ "-", group = groups.files },
		{ "-", "<CMD>Oil<CR>",      desc = "Open parent directory" },
	},

	["lsig/messenger.nvim"] = {
		{ "<Leader>v",  group = groups.git },
		{ "<Leader>vs", "<CMD>lua require('messenger').show()<CR>", desc = "Show commit message" },
	},

	["isakbm/gitgraph.nvim"] = {
		{ "<Leader>v", group = groups.git },
		{
			"<Leader>vg",
			function()
				require("gitgraph").draw({}, { all = true, max_count = 5000 })
			end,
			desc = "New git graph",
		},
	},

	["chrisgrieser/nvim-rip-substitute"] = {
		{ "<Leader>:", "<CMD>lua require('rip-substitute').sub()<CR>", desc = "rip substitute", },
	},

	["CRAG666/code_runner.nvim"] = {
		{ "<Leader>o",  group = "Run Code" },
		{ '<leader>oo', ':RunCode<CR>',     desc = "Runs based on file type (Run Code)" },
		{ '<leader>of', ':RunFile<CR>',     desc = "Execute command from its key in current directory (Run Code)" },
		{ '<leader>ot', ':RunFile tab<CR>', desc = "Run the current file (optionally you can select an opening mode)." },
		{ '<leader>op', ':RunProject<CR>',  desc = "Run the current project(If you are in a project otherwise you will not do anything)." },
		{ '<leader>oq', ':RunClose<CR>',    desc = "Close runner (Run Code)" },
		{ '<leader>oj', ':CRFiletype<CR>',  desc = "Open json with supported files (Run Code)" },
		{ '<leader>oJ', ':CRProjects<CR>',  desc = "Open json with list of projects (Run Code)" },
	},

	["akinsho/toggleterm.nvim"] = {
		{ "<Leader>w",  group = groups.terminal },
		{ "<Leader>w",  ":ToggleTerm<CR>",      desc = "Terminal toggle (ToggleTerm)" },
		{ "<Leader>Wn", ":ToggleTermNew<CR>",   desc = "New terminal" },
		-- { "<Leader>Ws",":ToggleTermSendCurrentLine<CR>",     desc = "Send current line" },
		-- { "<Leader>Wl",":ToggleTermSendVisualLines<CR>",     desc = "Send selected lines" },
		-- { "<Leader>Wv",":ToggleTermSendVisualSelection<CR>", desc = "Send visual selection" },
	},

	["voldikss/vim-floaterm"] = {
		mode = { "n", "t", "v" },
		{ "<Leader>w",  group = groups.terminal },
		{ "<C-t>",      ":FloatermToggle<CR>",  desc = "Terminal toggle (ToggleTerm)" },
		{ "<Leader>wt", ":FloatermToggle<CR>",  desc = "New toggle (ToggleTerm)" },
		{ "<Leader>wn", ":FloatermNew<CR>",     desc = "New terminal (ToggleTerm)" },
		{ "<Leader>wh", ":FloatermPrev<CR>",    desc = "Previous terminal (ToggleTerm)" },
		{ "<Leader>wl", ":FloatermNext<CR>",    desc = "Next terminal (ToggleTerm)" },
	},

	["mistweaverco/kulala.nvim"] = {
		{ "<Leader>r",  group = groups.http },
		{ "<Leader>rr", "<CMD>lua require('kulala').run()<CR>",              desc = "Make HTTP request" },
		{ "<Leader>rh", "<CMD>lua require('kulala').jump_prev()<CR>",        desc = "Jump to the previous request" },
		{ "<Leader>rl", "<CMD>lua require('kulala').jump_next()<CR>",        desc = "Jump to the next request" },
		{ "<Leader>re", "<CMD>lua require('kulala').set_selected_env()<CR>", desc = "Select environment" },
		{ "<Leader>rt", "<CMD>lua require('kulala').toggle_view()<CR>",      desc = "Response view (Toggle )" },

	},

	["rest-nvim/rest.nvim"] = {
		{ "<Leader>r",  group = groups.http },
		{ "<Leader>rr", "<CMD>Rest run<CR>",      desc = "Run request under cursor" },
		{ "<Leader>rl", "<CMD>Rest run last<CR>", desc = "Re-run latest request" },
	},

	["folke/snacks.nvim"] = {
		{ "<Leader>s",  group = "Snacks" },
		{ "<Leader>sn", "<CMD>lua Snacks.notifier.show_history()<CR>", desc = "Notifications history" },
		-- using neo-tree instead
		-- { ";f",          "<CMD>lua Snacks.explorer()<CR>",               desc = "File Explorer(toggle)" },
		-- Terminal
		-- { "<Leader>w",  "<CMD>lua Snacks.terminal.toggle()<CR>",  desc = "Terminal toggle (Snacks)" },

		-- ----------------------------------------
		-- -- find
		-- { "<C-b>",       "<CMD>lua Snacks.picker.buffers()<CR>",                             desc = "Buffers" },
		-- { "<C-p>",       "<CMD>lua Snacks.picker.files()<CR>",                               desc = "Find Files (project dir)" },
		-- { "<C-f>",       "<CMD>lua Snacks.picker.files({cwd = vim.fn.expand('%:p:h')})<CR>", desc = "Find Files (current dir)" },
		-- { "<leader>sp",  "<CMD>lua Snacks.picker.projects()<CR>",                            desc = "Projects" },
		-- { "<leader>sfF", "<CMD>lua Snacks.picker.smart()<CR>",                               desc = "Smart Find Files" },
		-- { "<leader>sfr", "<CMD>lua Snacks.picker.recent()<CR>",                              desc = "Recent files" },
		-- -- Grep
		-- { "<leader>sgb", "<CMD>lua Snacks.picker.lines()<CR>",                               desc = "Buffer Lines" },
		-- { "<leader>sgB", "<CMD>lua Snacks.picker.grep_buffers()<CR>",                        desc = "Grep Open Buffers" },
		-- { "<leader>sgg", "<CMD>lua Snacks.picker.grep()<CR>",                                desc = "Grep" },
		-- { "<leader>sgw", "<CMD>lua Snacks.picker.grep_word()<CR>",                           desc = "Visual selection or word",        mode = { "n", "x" } },
		-- -- search
		-- { "<leader>sh",  "<CMD>lua Snacks.picker.help()<CR>",                                desc = "Help Pages" },
		-- { "<leader>sH",  "<CMD>lua Snacks.picker.highlights()<CR>",                          desc = "Highlights" },
		-- { "<leader>sc",  "<CMD>lua Snacks.picker.commands()<CR>",                            desc = "Commands" },
		-- { "<leader>sC",  "<CMD>lua Snacks.picker.command_history()<CR>",                     desc = "Command History" },

		-- -- git
		-- {
		-- 	{ "<Leader>vp",  group = groups.git_picker },
		-- 	{ "<leader>vpf", "<CMD>lua Snacks.picker.git_files()<CR>",    desc = "Find Git Files" },
		-- 	{ "<leader>vpb", "<CMD>lua Snacks.picker.git_branches()<CR>", desc = "Git Branches" },
		-- 	{ "<leader>vpl", "<CMD>lua Snacks.picker.git_log()<CR>",      desc = "Git Log" },
		-- 	{ "<leader>vpL", "<CMD>lua Snacks.picker.git_log_line()<CR>", desc = "Git Log Line" },
		-- 	{ "<leader>vps", "<CMD>lua Snacks.picker.git_status()<CR>",   desc = "Git Status" },
		-- 	{ "<leader>vpS", "<CMD>lua Snacks.picker.git_stash()<CR>",    desc = "Git Stash" },
		-- 	{ "<leader>vpd", "<CMD>lua Snacks.picker.git_diff()<CR>",     desc = "Git Diff (Hunks)" },
		-- 	{ "<leader>vpF", "<CMD>lua Snacks.picker.git_log_file()<CR>", desc = "Git Log File" },
		-- },
		-- -- { '<leader>s"', function() Snacks.picker.registers() end, desc = "Registers" },
		-- -- { '<leader>s/', function() Snacks.picker.search_history() end, desc = "Search History" },
		-- -- { "<leader>sa", function() Snacks.picker.autocmds() end, desc = "Autocmds" },
		-- -- { "<leader>sb", function() Snacks.picker.lines() end, desc = "Buffer Lines" },
		-- -- { "<leader>sd", function() Snacks.picker.diagnostics() end, desc = "Diagnostics" },
		-- -- { "<leader>sD", function() Snacks.picker.diagnostics_buffer() end, desc = "Buffer Diagnostics" },
		-- -- { "<leader>si", function() Snacks.picker.icons() end, desc = "Icons" },
		-- -- { "<leader>sj", function() Snacks.picker.jumps() end, desc = "Jumps" },
		-- -- { "<leader>sk", function() Snacks.picker.keymaps() end, desc = "Keymaps" },
		-- -- { "<leader>sl", function() Snacks.picker.loclist() end, desc = "Location List" },
		-- -- { "<leader>sm", function() Snacks.picker.marks() end, desc = "Marks" },
		-- -- { "<leader>sM", function() Snacks.picker.man() end, desc = "Man Pages" },
		-- -- { "<leader>sp", function() Snacks.picker.lazy() end, desc = "Search for Plugin Spec" },
		-- -- { "<leader>sq", function() Snacks.picker.qflist() end, desc = "Quickfix List" },
		-- -- { "<leader>sR", function() Snacks.picker.resume() end, desc = "Resume" },
		-- -- { "<leader>su", function() Snacks.picker.undo() end, desc = "Undo History" },
		-- -- { "<leader>uC", function() Snacks.picker.colorschemes() end, desc = "Colorschemes" },
		-- -- -- LSP
		-- -- { "gd", function() Snacks.picker.lsp_definitions() end, desc = "Goto Definition" },
		-- -- { "gD", function() Snacks.picker.lsp_declarations() end, desc = "Goto Declaration" },
		-- -- { "gr", function() Snacks.picker.lsp_references() end, nowait = true, desc = "References" },
		-- -- { "gI", function() Snacks.picker.lsp_implementations() end, desc = "Goto Implementation" },
		-- -- { "gy", function() Snacks.picker.lsp_type_definitions() end, desc = "Goto T[y]pe Definition" },
		-- -- { "<leader>ss", function() Snacks.picker.lsp_symbols() end, desc = "LSP Symbols" },
		-- -- { "<leader>sS", function() Snacks.picker.lsp_workspace_symbols() end, desc = "LSP Workspace Symbols" },

		-- -- { "<leader>n", function() Snacks.picker.notifications() end, desc = "Notification History" },
		-- -- { "<leader>e", function() Snacks.explorer() end, desc = "File Explorer" },
		----------------------------------------
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


	["hakonharnes/img-clip.nvim"] = {
		{ "<Leader>P", group = "img-clip" },
		{ "<Leader>P", "<CMD>PasteImage<CR>", desc = "Paste image from system clipboard" },
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

	["MagicDuck/grug-far.nvim"] = {
		{ "<Leader>f",  group = groups.files },
		{ "<Leader>fr", function() require('grug-far').open() end, desc = "Find and replace: (grug-far)" },
	},

	["A7Lavinraj/fyler.nvim"] = {
		{ "-", "<CMD>Fyler<CR>", desc = "todo: this desc" },
	},
}

return M
