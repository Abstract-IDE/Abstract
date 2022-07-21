
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ───────────────────────────────────────────────── --
--    Plugin:    fidget.nvim
--    Github:    github.com/j-hui/fidget.nvim
-- ───────────────────────────────────────────────── --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --





-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━❰ configs ❱━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

local imported_fidget, fidget = pcall(require, "fidget")
if not imported_fidget then return end

fidget.setup {
	text = {
		spinner = "pipe",         -- animation shown when tasks are ongoing
		done = "✔",               -- character shown when all tasks are complete
		commenced = "Started",    -- message shown when task starts
		completed = "Completed",  -- message shown when task completes
	},
	align = {
		bottom = true,            -- align fidgets along bottom edge of buffer
		right = true,             -- align fidgets along right edge of buffer
	},
	timer = {
		spinner_rate = 125,       -- frame rate of spinner animation, in ms
		fidget_decay = 2000,      -- how long to keep around empty fidget, in ms
		task_decay = 1000,        -- how long to keep around completed task, in ms
	},
	window = {
		relative = "win",         -- where to anchor, either "win" or "editor"
		blend = 0,                -- &winblend for the window
		zindex = nil,             -- the zindex value for the window
	},
	fmt = {
		leftpad = true,           -- right-justify text in fidget box
		stack_upwards = false,     -- list of tasks grows upwards
		max_width = 0,            -- maximum width of the fidget box
		fidget = function(fidget_name, spinner) -- function to format fidget title
			return string.format("%s %s", spinner, fidget_name)
		end,
		task = function(task_name, message, percentage) -- function to format each task line
			return string.format(
				"%s%s [%s]",
				message,
				percentage and string.format(" (%s%%)", percentage) or "",
				task_name
			)
		end,
	},
}

-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━❰ end configs ❱━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

