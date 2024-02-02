local M = {}
local plugin = {}

-- -- none-ls
-- ["<Space>"] = {
-- 	f = { "<cmd>lua vim.lsp.buf.format({ timeout_ms = 2000 })<CR>", "Format code", noremap = true, silent = true },
-- },

-- Mappings
local keymap = vim.api.nvim_set_keymap
local opts = { silent = true, noremap = true }

-- Launch Telescope without any argument
keymap("n", "tt", "<cmd>lua require('telescope.builtin').builtin() <CR>", opts)
-- Lists available Commands
keymap("n", "tc", "<cmd>lua require('telescope.builtin').commands() <CR>", opts)
-- Lists available help tags and opens a new window with the relevant help info on
keymap("n", "th", "<cmd>lua require('telescope.builtin').help_tags() <CR>", opts)
-- Show all availabe MAPPING
keymap("n", "tm", "<cmd>lua require('telescope.builtin').keymaps() <CR>", opts)
-- Show buffers/opened files
keymap("n", "<C-b>", "<cmd>lua require('telescope.builtin').buffers() <CR>", opts)
-- Show and grep current buffer
keymap("n", "tw", "<cmd>lua require('telescope.builtin').current_buffer_fuzzy_find() <CR>", opts)
keymap("n", "tg", "<cmd>lua require('telescope.builtin').live_grep() <CR>", opts)

--       --> Find Files
-- NOTE1: to get project root's directory, https://github.com/Abstract-IDE/penvim plugin is used.
-- any config related to project root is in seperate config file (lua/plugin_confs/penvim.lua)
-- to change settings related to working directory, refer to penvim.lua config file

keymap("n", "<C-p>", ":Telescope find_files <cr>", opts)
keymap("n", "<C-f>", "<cmd>lua require('telescope.builtin').find_files( { cwd = vim.fn.expand('%:p:h') }) <CR>", opts)

M.ctrl = {
	-- stylua: ignore
	["<C>"] = {
		-- Find files from current file's project
		p = { "<cmd>Telescope find_files<cr>", "Find File (project)" },
		-- Show all files from current working directory
		f = { "<cmd>lua require('telescope.builtin').find_files( { cwd = vim.fn.expand('%:p:h') }) <CR>", "Find File (current dir)", },
	},
}


return M
