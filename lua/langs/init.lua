
--[[
	 NOTE:
		every languages has it's own standard. this config try its best
		to do that. config for each languages are defined on its own seperate
		lua config file. this init.lua file is used source them.
    :end of NOTE:
--]]




-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━❰ Load/Source Configs ❱━━━━━━━━━━━━━ --

 -- get bufffer number
local bufnr = vim.api.nvim_get_current_buf()
 -- get absolute path (with filename) Eg: /home/user/filename.ext
local absolute_path = vim.api.nvim_buf_get_name(bufnr)
-- get filename name Eg: filename.ext
local filename = absolute_path:match(".*[\\/](.*)")
-- don't load anything if file don't have an extension
if filename == nil then
	return
end
-- get extension of file Eg: ext
local ext = filename:match(".+%.(%w+)")



if ext=='lua' then
	require('langs/lua')        -- LUA
	return
end
if ext=='py' then
	require('langs/python')     -- PYTHON
	return
end
if ext=='c' then
	require('langs/c')          -- C
	return
end
if ext=='cpp' then
	require('langs/cpp')        -- CPP
	return
end
if ext=='html' or ext=='htm' then
	require('langs/html')       -- HTML
	return
end
if ext=='js' or ext=='ts' then
	require('langs/javascript') -- JAVASCRIPT, TYPESCRIPT
	return
end
if ext=='json' then
	require('langs/json')       -- JSON
	return
end


-- ━━━━━━━━━━━━━━━━━❰ end of Load ❱━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

