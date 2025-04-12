--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: blink.cmp
Source: github.com/Saghen/blink.cmp

Performant, batteries-included completion plugin for Neovim
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
	"saghen/blink.cmp",
	lazy = false, -- lazy loading handled internally
	dependencies = {},
	-- use a release tag to download pre-built binaries
	-- OR build from source, requires nightly-https://rust-lang.github.io/rustup/concepts/channels.html#working-with-nightly-rust
	-- build = 'cargo build --release',
	version = "1.*",
	-- allows extending the providers array elsewhere in your config without having to redefine it
	opts_extend = { "sources.default" },
}

---@module 'blink.cmp'
---@type blink.cmp.Config
spec.opts = {
	enabled = function()
		if vim.b.completion == false then
			return false
		end
		local buftype = vim.bo.buftype
		if
			buftype == "prompt"
			or buftype == "alpha"
			or buftype == "neo-tree"
			or buftype == "neo-tree-popup"
			or buftype == "nofile"
			or buftype == "TelescopePrompt"
		then
			return false
		end
		return true
	end,

	-- https://cmp.saghen.dev/configuration/sources.html
	sources = {
		-- default = { "lsp", "path", "snippets", "buffer" },
		default = function()
			local success, node = pcall(vim.treesitter.get_node)
			if success and node and vim.tbl_contains({ "comment", "line_comment", "block_comment" }, node:type()) then
				return { "path", "buffer" }
			end
			return { "lsp", "path", "snippets", "buffer" }
		end,
		providers = {
			path = {
				opts = {
					-- Path completion from cwd instead of current buffer's directory
					get_cwd = function(_)
						return vim.fn.getcwd()
					end,
				},
			},
		},
	},

	-- https://cmp.saghen.dev/configuration/snippets.html
	snippets = {
		--NOTE: luasnip needs to be properly setup with blink
		preset = "luasnip",
		-- expand = function(snippet)
		-- 	local luasnip = require("abstract.plugins.LuaSnip").setup()
		-- 	luasnip.lsp_expand(snippet)
		-- end,
		-- active = function(filter)
		-- 	local luasnip = require("abstract.plugins.LuaSnip").setup()
		-- 	if filter and filter.direction then
		-- 		return luasnip.jumpable(filter.direction)
		-- 	end
		-- 	return luasnip.in_snippet()
		-- end,
		-- jump = function(direction)
		-- 	local luasnip = require("abstract.plugins.LuaSnip").setup()
		-- 	luasnip.jump(direction)
		-- end,
	},

	signature = {
		enabled = true,
		window = { border = "single" },
	},

	-- https://cmp.saghen.dev/configuration/fuzzy
	fuzzy = {
		implementation = "prefer_rust_with_warning",
		sorts = {
			"exact",
			-- default sorts
			"score",
			"sort_text",
		},
	},

	completion = {
		-- https://cmp.saghen.dev/configuration/completion.html#keyword
		-- 'prefix' will fuzzy match on the text before the cursor
		-- 'full' will fuzzy match on the text before _and_ after the cursor
		-- example: 'foo_|_bar' will match 'foo_' for 'prefix' and 'foo__bar' for 'full'
		keyword = { range = "full" },

		trigger = {
			-- SRC: https://cmp.saghen.dev/configuration/completion.html#trigger
			show_in_snippet = true, -- When false, will not show the completion window automatically when in a snippet
			show_on_keyword = true, -- Shows after typing a keyword, typically an alphanumeric character or _
			show_on_trigger_character = true, -- Shows after typing a trigger character, defined by the sources. For example for Lua or Rust, the LSP will define . as a trigger character.
			show_on_insert_on_trigger_character = true, -- Shows after entering insert mode on top of a trigger character.
		},

		-- https://cmp.saghen.dev/configuration/completion.html#list-go-to-default-configuration
		list = {
			-- Maximum number of items to display
			max_items = 200,
			selection = {
				preselect = false,
				auto_insert = true,
			},

			cycle = {
				-- When `true`, calling `select_next` at the _bottom_ of the completion list will select the _first_ completion item.
				-- from_bottom = true,
				-- When `true`, calling `select_prev` at the _top_ of the completion list will select the _last_ completion item.
				-- from_top = true,
			},
		},
		accept = {
			-- Create an undo point when accepting a completion item
			create_undo_point = true,
		},
		-- Displays a preview of the selected item on the current line
		ghost_text = {
			enabled = false,
		},
		menu = {
			enabled = true,
			min_width = 15,
			max_height = 12,
			border = "single",
			scrolloff = 0, -- keep the cursor X lines away from the top/bottom of the window

			draw = {
				-- padding = 1, -- Left and right padding, optionally { left, right } for different padding on each side
				-- gap = 1, -- Gap between columns

				align_to = "none", -- Aligns the keyword you've typed to a component in the menu. 'label' or 'none' to disable, or 'cursor' to align to the cursor
				treesitter = { "lsp" }, -- Use treesitter to highlight the label text for the given list of sources
				columns = { { "kind_icon", "label", "label_description", "kind" } },
				components = {
					kind_icon = {
						text = function(ctx)
							return ctx.kind_icon .. " "
						end,
					},
					label = {
						width = { max = 30, fill = false },
					},
					label_description = {
						width = { max = 14, fill = true },
						text = function(ctx)
							return " " .. ctx.label_description
						end,
					},
					kind = {
						ellipsis = false,
						width = { fill = false },
					},
				},
			},
		},
		documentation = {
			auto_show = true,
			auto_show_delay_ms = 500, -- Delay before showing the documentation window
			update_delay_ms = 150, -- Delay before updating the documentation window when selecting a new item, while an existing item is still visible
			-- Whether to use treesitter highlighting, disable if you run into performance issues
			treesitter_highlighting = true,
			window = {
				max_width = 60,
				max_height = 20,
				border = "single",
			},
		},
	},
	appearance = {
		highlight_ns = vim.api.nvim_create_namespace("blink_cmp"),
		-- Sets the fallback highlight groups to nvim-cmp's highlight groups
		-- Useful for when your theme doesn't support blink.cmp
		-- Will be removed in a future release
		use_nvim_cmp_as_default = true,
		-- Set to 'mono' for 'Nerd Font Mono' or 'normal' for 'Nerd Font'
		-- Adjusts spacing to ensure icons are aligned
		nerd_font_variant = "mono",

		kind_icons = {
			Text = "",
			Method = "",
			Function = "󰊕",
			Constructor = "󰒓",

			Field = "󰜢",
			Variable = "󰆦",
			Property = "󰖷",

			Class = "",
			Interface = "",
			Struct = "",
			Module = "󰅩",

			Unit = "",
			Value = "󰦨",
			Enum = "",
			EnumMember = "",

			Keyword = "󰻾",
			Constant = "󰏿",

			Snippet = "",
			Color = "",
			File = "",
			Reference = "",
			Folder = "󰉋",
			Event = "",
			Operator = "",
			TypeParameter = " ",
		},
	},

	-- When specifying 'preset' in the keymap table, the custom key mappings are merged with the preset,
	-- and any conflicting keys will overwrite the preset mappings.
	-- The "fallback" command will run the next non blink keymap.
	keymap = {
		["<C-space>"] = { "show", "show_documentation", "hide_documentation" },
		["<C-e>"] = { "hide", "fallback" },
		["<Enter>"] = { "accept", "fallback" },

		["<Up>"] = { "select_prev", "fallback" },
		["<Down>"] = { "select_next", "fallback" },

		["<Tab>"] = { "select_next", "snippet_forward", "fallback" },
		["<S-Tab>"] = { "select_prev", "snippet_backward", "fallback" },

		["<C-u>"] = { "scroll_documentation_up", "fallback" },
		["<C-d>"] = { "scroll_documentation_down", "fallback" },
	},
}

return spec
