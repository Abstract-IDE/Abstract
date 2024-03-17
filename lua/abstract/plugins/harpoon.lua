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
	-- lazy = true,
	branch = "harpoon2",
	dependencies = {
		"nvim-lua/plenary.nvim",
		"nvim-telescope/telescope.nvim",
	},
}

spec.config = function()
	-- Maping Harpoon and HarpoonTelescope globle so that is can be accessed globally
	local harpoon = require("harpoon")
	Harpoon_List = harpoon:list()
	harpoon:setup({})

	-- Telescope integration: to use Telescope as a UI
	function HarpoonTelescope()
		-- local conf = require("telescope.config").values
		-- local file_paths = {}
		-- for _, item in ipairs(Harpoon_List.items) do
		-- 	table.insert(file_paths, item.value)
		-- end
		-- require("telescope.pickers")
		-- 	.new({}, {
		-- 		prompt_title = "Harpoon",
		-- 		finder = require("telescope.finders").new_table({
		-- 			results = file_paths,
		-- 		}),
		-- 		previewer = conf.file_previewer({}),
		-- 		sorter = conf.generic_sorter({}),
		-- 	})
		-- 	:find()

		-- NOTE: not using telescope for harpoon
		harpoon.ui:toggle_quick_menu(Harpoon_List)
	end

	require("abstract.utils.map").set_plugin("ThePrimeagen/harpoon")
end

return spec
