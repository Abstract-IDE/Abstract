
local filetype = vim.bo.filetype

if filetype == "angular" then
	require('langs/frameworks/angular')    -- Angular
elseif filetype == "htmldjango" then
	require('langs/frameworks/django')     -- Django
elseif filetype == "flutter" then
	require('langs/frameworks/flutter')    -- Flutter
elseif filetype == "javascriptreact" or
   filetype == "typescriptreact" then
	require('langs/frameworks/react')      -- React
elseif filetype == "vue" then
	require('langs/frameworks/vue')        -- Vue
else
	return
end

