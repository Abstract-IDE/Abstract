--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin:    harpoon
Github:    https://github.com/ThePrimeagen/harpoon

Getting you where you want with the fewest keystrokes.
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
	"ThePrimeagen/harpoon",
	lazy = true,
	branch = "harpoon2",
	dependencies = {
		"nvim-lua/plenary.nvim",
		"nvim-telescope/telescope.nvim",
	},
}

spec.config = function()
	local harpoon = require("harpoon")
	harpoon:setup({})

	-- -- basic telescope configuration
	-- local conf = require("telescope.config").values
	-- local function toggle_telescope(harpoon_files)
	-- 	local file_paths = {}
	-- 	for _, item in ipairs(harpoon_files.items) do
	-- 		table.insert(file_paths, item.value)
	-- 	end

	-- 	require("telescope.pickers")
	-- 		.new({}, {
	-- 			prompt_title = "Harpoon",
	-- 			finder = require("telescope.finders").new_table({
	-- 				results = file_paths,
	-- 			}),
	-- 			previewer = conf.file_previewer({}),
	-- 			sorter = conf.generic_sorter({}),
	-- 		})
	-- 		:find()
	-- end

	-- vim.keymap.set("n", "<C-e>", function()
	-- 	toggle_telescope(harpoon:list())
	-- end, { desc = "Open harpoon window" })

	-- vim.keymap.set("n", "<leader>a", function()
	-- 	harpoon:list():append()
	-- end)

	-- vim.keymap.set("n", "<C-h>", function()
	-- 	harpoon:list():select(1)
	-- end)

	-- vim.keymap.set("n", "<C-t>", function()
	-- 	harpoon:list():select(2)
	-- end)

	-- vim.keymap.set("n", "<C-n>", function()
	-- 	harpoon:list():select(3)
	-- end)

	-- vim.keymap.set("n", "<C-s>", function()
	-- 	harpoon:list():select(4)
	-- end)

	-- -- Toggle previous & next buffers stored within Harpoon list
	-- vim.keymap.set("n", "<C-S-P>", function()
	-- 	harpoon:list():prev()
	-- end)

	-- vim.keymap.set("n", "<C-S-N>", function()
	-- 	harpoon:list():next()
	-- end)
end

return spec
