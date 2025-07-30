--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: nvim-dap-ui
Source: https://github.com/rcarriga/nvim-dap-ui

A UI for nvim-dap which provides a good out of the box configuration.
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
	"rcarriga/nvim-dap-ui",
}

spec.config = function()
	local dapui = require("dapui")
	dapui.setup()

	local dap = require("dap")
	-- nvim-dap events to open and close the windows automatically (:help dap-extensions)
	dap.listeners.before.attach.dapui_config = function()
		dapui.open()
	end
	dap.listeners.before.launch.dapui_config = function()
		dapui.open()
	end
	dap.listeners.before.event_terminated.dapui_config = function()
		dapui.close()
	end
	dap.listeners.before.event_exited.dapui_config = function()
		dapui.close()
	end
end


return spec
