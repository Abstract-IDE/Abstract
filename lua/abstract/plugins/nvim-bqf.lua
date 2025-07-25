--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: nvim.bqf
Source: https://github.com/kevinhwang91/nvim-bqf

Better quickfix window in Neovim, polish old quickfix window.
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
	"kevinhwang91/nvim-bqf",
	ft = "qf",
}

spec.opts = {
	auto_enable = true,
	auto_resize_height = true, -- highly recommended enable
	preview = {
		delay_syntax = 400,
		show_title = true,
		-- should_preview_cb = function(bufnr, qwinid)
		-- 	local ret = true
		-- 	local bufname = vim.api.nvim_buf_get_name(bufnr)
		-- 	local fsize = vim.fn.getfsize(bufname)
		-- 	if fsize > 100 * 1024 then
		-- 		-- skip file size greater than 100k
		-- 		ret = false
		-- 	elseif bufname:match("^fugitive://") then
		-- 		-- skip fugitive buffer
		-- 		ret = false
		-- 	end
		-- 	return ret
		-- end,
	},
	-- make `drop` and `tab drop` to become preferred
	func_map = {
		drop = "o",
		openc = "O",
		split = "<C-s>",
		tabdrop = "<C-t>",
		-- set to empty string to disable
		tabc = "",
		ptogglemode = "z,",
	},
	filter = {
		fzf = {
			action_for = { ["ctrl-s"] = "split", ["ctrl-t"] = "tab drop" },
			extra_opts = { "--bind", "ctrl-o:toggle-all", "--prompt", "> " },
		},
	},
}

return spec
