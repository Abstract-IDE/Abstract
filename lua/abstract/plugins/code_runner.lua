--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: code_runner.nvim
Github: https://github.com/CRAG666/code_runner.nvim

Neovim plugin.The best code runner you could have,
it is like the one in vscode but with super powers,
it manages projects like in intellij but without being slow
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
	"CRAG666/code_runner.nvim",
	lazy = true,
	keys = require("abstract.configs.mapping").plugin["CRAG666/code_runner.nvim"],
}

spec.config = function()
	require("code_runner").setup({
		filetype = {
			java = {
				"cd $dir &&",
				"javac $fileName &&",
				"java $fileNameWithoutExt",
			},
			python = "python3 -u",
			typescript = "deno run",
			rust = {
				"cd $dir &&",
				"rustc $fileName &&",
				"$dir/$fileNameWithoutExt",
			},
			c = function(...)
				c_base = {
					"cd $dir &&",
					"gcc $fileName -o",
					"/tmp/$fileNameWithoutExt",
				}
				local c_exec = {
					"&& /tmp/$fileNameWithoutExt &&",
					"rm /tmp/$fileNameWithoutExt",
				}
				vim.ui.input({ prompt = "Add more args:" }, function(input)
					c_base[4] = input
					vim.print(vim.tbl_extend("force", c_base, c_exec))
					require("code_runner.commands").run_from_fn(vim.list_extend(c_base, c_exec))
				end)
			end,
		},
	})

	require("abstract.utils.map").set_map("CRAG666/code_runner.nvim")
end

return spec
