--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: mcp-hub.nvim
Source: https://github.com/ravitemer/mcp-hub.nvim

An MCP client for Neovim that seamlessly integrates MCP servers
into your editing workflow with an intuitive interface for managing,
testing, and using MCP servers with your favorite chat plugins.
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
	"ravitemer/mcphub.nvim",
	build = "npm install -g mcp-hub@latest", -- Installs `mcp-hub` node binary globally
	lazy = true,
}

spec.config = function()
	require("mcphub").setup({
		--- `mcp-hub` binary related options-------------------
		config = vim.fn.expand("~/.config/mcphub/servers.json"), -- Absolute path to MCP Servers config file (will create if not exists)
		port = 37373,                                      -- The port `mcp-hub` server listens to
		shutdown_delay = 60 * 10 * 000,                    -- Delay in ms before shutting down the server when last instance closes (default: 10 minutes)
		use_bundled_binary = false,                        -- Use local `mcp-hub` binary (set this to true when using build = "bundled_build.lua")
		mcp_request_timeout = 60000,                       --Max time allowed for a MCP tool or resource to execute in milliseconds, set longer for long running tasks

		---Chat-plugin related options-----------------
		auto_approve = false,     -- Auto approve mcp tool calls
		auto_toggle_mcp_servers = true, -- Let LLMs start and stop MCP servers automatically
		extensions = {
			avante = {
				make_slash_commands = true, -- make /slash commands from MCP server prompts
			},
		},

		--- Plugin specific options-------------------
		native_servers = {}, -- add your custom lua native servers here
		ui = {
			window = {
				width = 0.8, -- 0-1 (ratio); "50%" (percentage); 50 (raw number)
				height = 0.8, -- 0-1 (ratio); "50%" (percentage); 50 (raw number)
				align = "center", -- "center", "top-left", "top-right", "bottom-left", "bottom-right", "top", "bottom", "left", "right"
				relative = "editor",
				zindex = 50,
				border = "rounded", -- "none", "single", "double", "rounded", "solid", "shadow"
			},
			wo = {      -- window-scoped options (vim.wo)
				winhl = "Normal:MCPHubNormal,FloatBorder:MCPHubBorder",
			},
		},
		on_ready = function(hub)
			-- Called when hub is ready
		end,
		on_error = function(err)
			-- Called on errors
		end,
		log = {
			level = vim.log.levels.WARN,
			to_file = false,
			file_path = nil,
			prefix = "MCPHub",
		},
	})
end

return spec
