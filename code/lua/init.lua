return function(root, data)
    vim.g.ABSTRACT_ROOT = root
    vim.g.ABSTRACT_DATA = data

    local code = root .. "/code"
    local profile = "release"
    local lib_ext = vim.fn.has("mac") == 1 and "dylib" or "so"
    local lib_path = code .. "/target/" .. profile .. "/libabstract." .. lib_ext

    if vim.fn.filereadable(lib_path) == 0 then
        vim.notify("[Abstract] Building (" .. profile .. ")...", vim.log.levels.INFO)
        local cmd = profile == "release" and "cargo build --release" or "cargo build"
        local result = vim.fn.system("cd " .. code .. " && " .. cmd .. " 2>&1")
        if vim.v.shell_error ~= 0 then
            vim.notify("[Abstract] Build failed:\n" .. result, vim.log.levels.ERROR)
            return
        end
    end

    package.cpath = package.cpath .. ";" .. code .. "/target/" .. profile .. "/?." .. lib_ext
    require("libabstract")
end
