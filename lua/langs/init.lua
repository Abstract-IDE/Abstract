
--[[
	 NOTE:
		every languages has it's own standard. this config try its best
		to do that. config for each languages are defined on its own seperate
		lua config file. this init.lua file is used source them.
    :end of NOTE:
--]]




-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━❰ Load/Source Configs ❱━━━━━━━━━━━━━ --

-- get filetype Eg: typescript, python, lua

require('langs/lua')        -- LUA
require('langs/python')     -- PYTHON
require('langs/c')          -- C
require('langs/cpp')        -- CPP
require('langs/html')       -- HTML
require('langs/javascript') -- JAVASCRIPT, TYPESCRIPT
require('langs/json')       -- JSON

-- ━━━━━━━━━━━━━━━━━❰ end of Load ❱━━━━━━━━━━━━━━━━━ --
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ --

