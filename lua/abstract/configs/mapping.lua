local M = {}
local plugin = {}

plugin["nvim-telescope/telescope.nvim"] = {
	-- Find files from current file's project
	["<C-p>"] = { "<cmd>Telescope find_files<cr>", "Find File (project dir)" },
	-- stylua: ignore
	-- Show all files from current working directory
	["<C-f>"] = { "<cmd>lua require('telescope.builtin').find_files( { cwd = vim.fn.expand('%:p:h') }) <CR>", "Find File (current dir)" },
	["<C-b>"] = { "<cmd>lua require('telescope.builtin').buffers() <CR>", "Opened buffers" },
	tt = { "<cmd>lua require('telescope.builtin').builtin() <CR>", "Telescope builtin" },
	tc = { "<cmd>lua require('telescope.builtin').commands() <CR>", "Commands" },
	th = { "<cmd>lua require('telescope.builtin').help_tags() <CR>", "Help tags" },
	tm = { "<cmd>lua require('telescope.builtin').keymaps() <CR>", "Mappings" },
	tw = { "<cmd>lua require('telescope.builtin').current_buffer_fuzzy_find() <CR>", "Find Word (current file)" },
	tg = { "<cmd>lua require('telescope.builtin').live_grep() <CR>", "Find Word (project wise)" },
	tp = { ":lua require'telescope'.extensions.project.project{}<CR>", "Projects picker" },
}

plugin["neovim/nvim-lspconfig"] = {
	["<Space>e"] = { "<cmd>lua vim.diagnostic.open_float()<CR>", "Show diagnostics" },
	["<Space>n"] = { "<cmd>lua vim.diagnostic.goto_next()<CR>", "Move to the next diagnostic" },
	["<Space>b"] = { "<cmd>lua vim.diagnostic.goto_prev()<CR>", "Move to the previous diagnostic" },
	["<Space>d"] = { "<Cmd>lua vim.lsp.buf.definition()<CR>", "Jumps to definition" },
	["<Space>D"] = { "<Cmd>lua vim.lsp.buf.declaration()<CR>", "Jumps to the declaration" },
	["<Space>T"] = { "<cmd>lua vim.lsp.buf.type_definition()<CR>", "Jumps to the type definition" },
	["<Space>i"] = { "<cmd>lua vim.lsp.buf.implementation()<CR>", "Lists all the symbol implementations" },
	["<Space>s"] = { "<cmd>lua vim.lsp.buf.signature_help()<CR>", "Show symbol signature information" },
	["<Space>h"] = { "<Cmd>lua vim.lsp.buf.hover()<CR>", "Show symbol hover information" },
	["K"] = { "<Cmd>lua vim.lsp.buf.hover()<CR>", "Show symbol hover information" },
	-- using 'filipdutescu/renamer.nvim' for rename instead
	-- ["<Space>rn"] = { "<cmd>lua vim.lsp.buf.rename()<CR>", "" },
	["<Space>f"] = { "<cmd>lua vim.lsp.buf.format({ timeout_ms = 2000 })<CR>", "Format document" },
	["<Space>a"] = { "<cmd>lua vim.lsp.buf.code_action()<CR>", "Code action" },
	-- ["<Space>a"]   = { "<cmd>lua vim.lsp.buf.range_code_action()<CR>", "Range code action" },
	["<Space>r"] = { "<cmd>Telescope lsp_references<CR>", " Lsp references" },

	-- ["<leader>wa"] = { "<cmd>lua vim.lsp.buf.add_workspace_folder()<CR>", "" },
	-- ["<leader>wr"] = { "<cmd>lua vim.lsp.buf.remove_workspace_folder()<CR>", "" },
	-- ["<leader>wl"] = { "<cmd>lua print(vim.inspect(vim.lsp.buf.list_workspace_folders()))<CR>", "" },
}

plugin["folke/trouble.nvim"] = {
	["<Space>t"] = { "<cmd>TroubleToggle<cr>", "Trouble(toggle)" },
}

plugin["nvim-neo-tree/neo-tree.nvim"] = {
	["<Leader>f"] = { ":Neotree toggle<CR>", "File Explorer(toggle)" },
}

plugin["nanozuki/tabby.nvim"] = {
	-- Mappings (:h tab)
	["<leader>q"] = { ":tabclose<CR>", "Close current tab" },
	["<leader>Q"] = { ":tabonly<CR>", "Close all other tab" },
	-- navigate to previous/next tab
	["\\"] = { ":tabn<CR>", "Go to next tab" },
	["|"] = { ":tabp<CR>", "Go to previous tab" },
	-- move current tab to previous/next position
	["<leader>,"] = { ":-tabmove<CR>", "Move tab to next position" },
	["<leader>."] = { ":+tabmove<CR>", "Move tab to previous position" },
}

plugin["filipdutescu/renamer.nvim"] = {
	["<Space>R"] = { "<cmd>lua require('renamer').rename()<cr>", "Rename symbol" },
}

plugin["smoka7/hop.nvim"] = {
	f = { "<cmd>lua require'hop'.hint_words()<cr>", "Jump anywhere" },
	-- local directions = require("hop.hint").HintDirection
	-- vim.keymap.set("", "f", function()
	-- 	hop.hint_char1({ direction = directions.AFTER_CURSOR, current_line_only = true })
	-- end, { remap = true })
	-- vim.keymap.set("", "F", function()
	-- 	hop.hint_char1({ direction = directions.BEFORE_CURSOR, current_line_only = true })
	-- end, { remap = true })
	-- vim.keymap.set("", "t", function()
	-- 	hop.hint_char1({ direction = directions.AFTER_CURSOR, current_line_only = true, hint_offset = -1 })
	-- end, { remap = true })
	-- vim.keymap.set("", "T", function()
	-- 	hop.hint_char1({ direction = directions.BEFORE_CURSOR, current_line_only = true, hint_offset = 1 })
	-- end, { remap = true })
}

M.plugin = plugin

return M
