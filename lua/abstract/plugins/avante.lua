--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: avante.nvim
Source: https://github.com/yetone/avante.nvim

Use your Neovim like using Cursor AI IDE!

avante.nvim is a Neovim plugin designed to emulate the behaviour of the Cursor
AI IDE. It provides users with AI-driven code suggestions and the ability to
apply these recommendations directly to their source files with minimal effort.
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
	"yetone/avante.nvim",
	-- if you want to build from source then do `make BUILD_FROM_SOURCE=true`
	-- ⚠️ must add this setting! ! !
	build = function()
		-- conditionally use the correct build system for the current OS
		if vim.fn.has("win32") == 1 then
			return "powershell -ExecutionPolicy Bypass -File Build.ps1 -BuildFromSource false"
		else
			return "make"
		end
	end,
	event = "VeryLazy",
	version = false, -- Never set this value to "*"! Never!
}

---@module 'avante'
---@type avante.Config
spec.opts = {
	---@type "claude" | "openai" | "azure" | "gemini" | "cohere" | "copilot" | string
	provider = "claude", -- The provider used in Aider mode or in the planning phase of Cursor Planning Mode
	---@alias Mode "agentic" | "legacy"
	---@type Mode
	mode = "agentic", -- The default mode for interaction. "agentic" uses tools to automatically generate code, "legacy" uses the old planning method to generate code.
	-- WARNING: Since auto-suggestions are a high-frequency operation and therefore expensive,
	-- currently designating it as `copilot` provider is dangerous because: https://github.com/yetone/avante.nvim/issues/1048
	-- Of course, you can reduce the request frequency by increasing `suggestion.debounce`.
	auto_suggestions_provider = "claude",
	providers = {
		claude = {
			endpoint = "https://api.anthropic.com",
			model = "claude-3-5-sonnet-20241022",
			extra_request_body = {
				temperature = 0.75,
				max_tokens = 4096,
			},
		},
	},
	---Specify the special dual_boost mode
	---1. enabled: Whether to enable dual_boost mode. Default to false.
	---2. first_provider: The first provider to generate response. Default to "openai".
	---3. second_provider: The second provider to generate response. Default to "claude".
	---4. prompt: The prompt to generate response based on the two reference outputs.
	---5. timeout: Timeout in milliseconds. Default to 60000.
	---How it works:
	--- When dual_boost is enabled, avante will generate two responses from the first_provider and second_provider respectively. Then use the response from the first_provider as provider1_output and the response from the second_provider as provider2_output. Finally, avante will generate a response based on the prompt and the two reference outputs, with the default Provider as normal.
	---Note: This is an experimental feature and may not work as expected.
	dual_boost = {
		enabled = false,
		first_provider = "openai",
		second_provider = "claude",
		prompt =
		"Based on the two reference outputs below, generate a response that incorporates elements from both but reflects your own judgment and unique perspective. Do not provide any explanation, just give the response directly. Reference Output 1: [{{provider1_output}}], Reference Output 2: [{{provider2_output}}]",
		timeout = 60000, -- Timeout in milliseconds
	},
	behaviour = {
		auto_suggestions = false, -- Experimental stage
		auto_set_highlight_group = true,
		auto_set_keymaps = true,
		auto_apply_diff_after_generation = false,
		support_paste_from_clipboard = false,
		minimize_diff = true,            -- Whether to remove unchanged lines when applying a code block
		enable_token_counting = true,    -- Whether to enable token counting. Default to true.
		auto_approve_tool_permissions = false, -- Default: show permission prompts for all tools
		-- Examples:
		-- auto_approve_tool_permissions = true,                -- Auto-approve all tools (no prompts)
		-- auto_approve_tool_permissions = {"bash", "replace_in_file"}, -- Auto-approve specific tools only
	},
	prompt_logger = {                                     -- logs prompts to disk (timestamped, for replay/debugging)
		enabled = true,                                   -- toggle logging entirely
		log_dir = vim.fn.stdpath("cache") .. "/avante_prompts", -- directory where logs are saved
		fortune_cookie_on_success = false,                -- shows a random fortune after each logged prompt (requires `fortune` installed)
		next_prompt = {
			normal = "<C-n>",                             -- load the next (newer) prompt log in normal mode
			insert = "<C-n>",
		},
		prev_prompt = {
			normal = "<C-p>", -- load the previous (older) prompt log in normal mode
			insert = "<C-p>",
		},
	},
	mappings = {
		--- @class AvanteConflictMappings
		diff = {
			ours = "co",
			theirs = "ct",
			all_theirs = "ca",
			both = "cb",
			cursor = "cc",
			next = "]x",
			prev = "[x",
		},
		suggestion = {
			accept = "<M-l>",
			next = "<M-]>",
			prev = "<M-[>",
			dismiss = "<C-]>",
		},
		jump = {
			next = "]]",
			prev = "[[",
		},
		submit = {
			normal = "<CR>",
			insert = "<C-s>",
		},
		cancel = {
			normal = { "<C-c>", "<Esc>", "q" },
			insert = { "<C-c>" },
		},
		sidebar = {
			apply_all = "A",
			apply_cursor = "a",
			retry_user_request = "r",
			edit_user_request = "e",
			switch_windows = "<Tab>",
			reverse_switch_windows = "<S-Tab>",
			remove_file = "d",
			add_file = "@",
			close = { "<Esc>", "q" },
			close_from_input = nil, -- e.g., { normal = "<Esc>", insert = "<C-d>" }
		},
	},
	hints = { enabled = true },
	windows = {
		---@type "right" | "left" | "top" | "bottom"
		position = "right", -- the position of the sidebar
		wrap = true,  -- similar to vim.o.wrap
		width = 30,   -- default % based on available width
		sidebar_header = {
			enabled = true, -- true, false to enable/disable the header
			align = "center", -- left, center, right for title
			rounded = true,
		},
		spinner = {
			-- stylua: ignore
			editing = { "⡀", "⠄", "⠂", "⠁", "⠈", "⠐", "⠠", "⢀", "⣀", "⢄", "⢂", "⢁", "⢈", "⢐", "⢠", "⣠", "⢤", "⢢", "⢡", "⢨", "⢰", "⣰", "⢴", "⢲", "⢱", "⢸", "⣸", "⢼", "⢺", "⢹", "⣹", "⢽", "⢻", "⣻", "⢿", "⣿", },
			generating = { "·", "✢", "✳", "∗", "✻", "✽" }, -- Spinner characters for the 'generating' state
			thinking = { "🤯", "🙄" }, -- Spinner characters for the 'thinking' state
		},
		input = {
			prefix = "> ",
			height = 8, -- Height of the input window in vertical layout
		},
		edit = {
			border = "rounded",
			start_insert = true, -- Start insert mode when opening the edit window
		},
		ask = {
			floating = false, -- Open the 'AvanteAsk' prompt in a floating window
			start_insert = true, -- Start insert mode when opening the ask window
			border = "rounded",
			---@type "ours" | "theirs"
			focus_on_apply = "ours", -- which diff to focus after applying
		},
	},
	highlights = {
		---@type AvanteConflictHighlights
		diff = {
			current = "DiffText",
			incoming = "DiffAdd",
		},
	},
	--- @class AvanteConflictUserConfig
	diff = {
		autojump = true,
		---@type string | fun(): any
		list_opener = "copen",
		--- Override the 'timeoutlen' setting while hovering over a diff (see :help timeoutlen).
		--- Helps to avoid entering operator-pending mode with diff mappings starting with `c`.
		--- Disable by setting to -1.
		override_timeoutlen = 500,
	},
	suggestion = {
		debounce = 600,
		throttle = 600,
	},
}

return spec
