
--[[
    NOTE:
         every languages has it's own standard. this config try its best
         to do that. config for each languages are defined on its own seperate
         lua config file. this init.lua file is used source them.
    :end of NOTE:
--]]




-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━❰ Load/Source Configs ❱━━━━━━━━━━━━━ --

local filetype = vim.bo.filetype

if filetype == "c" then
	require('langs/c')                      -- C
elseif filetype == "cpp" then
	require('langs/cpp')                    -- CPP
elseif filetype == "html" then
	require('langs/html')                   -- HTML
elseif filetype == "javascript" or
       filetype == "typescript" then
	require('langs/javascript')             -- JAVASCRIPT, TYPESCRIPT
elseif filetype == "json" then
	require('langs/json')                   -- JSON
elseif filetype == "lua" then
	require('langs/lua')                    -- LUA
elseif filetype == "python" then
	require('langs/python')                 -- PYTHON
else
	require('langs/frameworks')             -- FrameWorks
end

-- ━━━━━━━━━━━━━━━━━❰ end of Load ❱━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

