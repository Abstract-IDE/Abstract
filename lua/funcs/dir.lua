

local M = {}

function M.is_dir(path)
    local f = io.open(path, "r")
    local ok, err, code = f:read(1)
    f:close()
    return code == 21
end

return M

