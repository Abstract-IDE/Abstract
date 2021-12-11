--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--
--─────────────────────────────────────────────────--
-- create .__nvim__.lua file in your project root directory and
-- define options/configs in that file.

------------ planning ---------------
-- 1. check if .__nvim__.lua file exist in the project
-- 2. if exist:
--      load  .__nvim__.lua config
-------------------------------------
--─────────────────────────────────────────────────--
--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--


----------------------------------------------
local bufnr = vim.api.nvim_get_current_buf() -- get bufffer number
local curr_file = vim.api.nvim_buf_get_name(bufnr) -- get buffer/filename with location
----------------------------------------------


----------------------------------------------
local function dirsplit (inputstr, sep)
  -- https://stackoverflow.com/a/7615129/11812032
  if sep == nil then
          sep = "%s"
  end
  local t={}
  for str in string.gmatch(inputstr, "([^"..sep.."]+)") do
      table.insert(t, str)
  end
  return t
end
----------------------------------------------


----------------------------------------------
-- get file directory (i dont know if api exist for this)
-- /home/user/document/file.txt --> /home/user/document/
local function filedir(file)
  local max_len = #(dirsplit(file, "/"))
  local count = 1
  local dir = "/"
  for _, d in ipairs(dirsplit(file, "/")) do
    if count < max_len then
      dir = dir..d.."/"
    end
    count = count + 1
  end
  return dir
end
----------------------------------------------


----------------------------------------------
-- check if file exist
local function file_exists(fname)
   local f=io.open(fname,"r")
   if f~=nil then io.close(f) return true else return false end
end
----------------------------------------------


----------------------------------------------
function os.capture(cmd, raw)
  -- https://gist.github.com/dukeofgaming/453cf950abd99c3dc8fc
    local handle = assert(io.popen(cmd, 'r'))
    local output = assert(handle:read('*a'))
    handle:close()
    if raw then
        return output
    end
    output = string.gsub(
        string.gsub(
            string.gsub(output, '^%s+', ''),
            '%s+$',
            ''
        ),
        '[\n\r]+',
        ' '
    )
   return output
end
----------------------------------------------


----------------------------------------------
local function osuser()
    return os.capture("whoami")
end
----------------------------------------------


----------------------------------------------
--  normalize dir
--  /home/user/../user/download/../ --> /home/user
local function normalize_dir(dir)
  local pwd = os.capture("cd "..dir.." && pwd")
  return pwd .. "/"
end
----------------------------------------------


----------------------------------------------
-- if .__nvim__.lua file exist, return it with full
-- location else return false
local function conf_file(cfdir)
  local conff
  for _, _ in ipairs(dirsplit(cfdir, "/")) do
    conff = cfdir .. ".__nvim__.lua"

    if file_exists(conff) then
      return  cfdir .. ".__nvim__.lua"
    else
      if cfdir == "/" or cfdir == "/home/"..osuser().."/" then
        return false
      else
        cfdir = cfdir.."../"
        cfdir = normalize_dir(cfdir)
      end
    end
  end
end
----------------------------------------------


----------------------------------------------
local curr_file_dir = filedir(curr_file)

if curr_file_dir ~= "/" then
  local conff = conf_file(curr_file_dir)

  if conff ~= false then
    -- source ".__nvim__.lua" file
    dofile(conff)
  end
end
----------------------------------------------

