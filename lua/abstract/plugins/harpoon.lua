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
	Harpoon = require("harpoon")
	Harpoon:setup({})

	-- Telescope integration: to use Telescope as a UI
	function HarpoonTelescope(harpoon_files)
		local conf = require("telescope.config").values
		local file_paths = {}
		for _, item in ipairs(harpoon_files.items) do
			table.insert(file_paths, item.value)
		end

		require("telescope.pickers")
			.new({}, {
				prompt_title = "Harpoon",
				finder = require("telescope.finders").new_table({
					results = file_paths,
				}),
				previewer = conf.file_previewer({}),
				sorter = conf.generic_sorter({}),
			})
			:find()
	end

	require("abstract.utils.map").set_plugin("ThePrimeagen/harpoon")
end

return spec
