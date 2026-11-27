-- Neovim-side helpers for seek, loaded once from Rust (`nvim.rs`).
local M = {}
local api, fn = vim.api, vim.fn

function M.highlights()
    local links = {
        SeekMode = "Title",
        SeekDir = "Comment",
        SeekLnum = "LineNr",
        SeekCount = "Comment",
        SeekModified = "WarningMsg",
    }
    for name, to in pairs(links) do
        api.nvim_set_hl(0, name, { link = to, default = true })
    end
end

--- Listed buffers, most recently used first; `origin` (the current one) last.
function M.buffers(origin)
    local list = {}
    for _, b in ipairs(fn.getbufinfo({ buflisted = 1 })) do
        local name = b.name ~= "" and fn.fnamemodify(b.name, ":~:.") or "[No Name]"
        list[#list + 1] = { bufnr = b.bufnr, name = name, modified = b.changed == 1, lastused = b.lastused }
    end
    table.sort(list, function(a, b)
        local ao, bo = a.bufnr == origin, b.bufnr == origin
        if ao ~= bo then
            return bo
        end
        return a.lastused > b.lastused
    end)
    return list
end

--- The buffer lines mode searches: `origin` when it holds a file (or a help
--- page), else the most recently used buffer that does; -1 when none.
function M.lines_buffer(origin)
    local function is_text(b)
        local bt = vim.bo[b].buftype
        return bt == "" or bt == "help"
    end
    local function ready(b)
        if not api.nvim_buf_is_loaded(b) then
            pcall(fn.bufload, b)
        end
        return b
    end
    if api.nvim_buf_is_valid(origin) and is_text(origin) then
        return origin
    end
    local best, last = -1, -1
    for _, b in ipairs(fn.getbufinfo({ buflisted = 1 })) do
        if b.bufnr ~= origin and b.name ~= "" and is_text(b.bufnr) and b.lastused > last then
            best, last = b.bufnr, b.lastused
        end
    end
    return best > 0 and ready(best) or -1
end

--- The visual selection (leaving visual mode), or the word under the cursor.
function M.selection_or_word()
    local mode = fn.mode()
    if mode == "v" or mode == "V" or mode == "\22" then
        local ok, lines = pcall(fn.getregion, fn.getpos("v"), fn.getpos("."), { type = mode })
        api.nvim_feedkeys(api.nvim_replace_termcodes("<Esc>", true, false, true), "nx", false)
        if ok and lines then
            return vim.trim(table.concat(lines, " "))
        end
    end
    return fn.expand("<cword>")
end

--- A loaded buffer showing `path`, or -1.
function M.loaded_buffer(path)
    -- Compare full names: `bufnr(path)` is a pattern that also matches
    -- other buffers (`a.js` finds a loaded `a.json`).
    local full = vim.fs.normalize(fn.fnamemodify(path, ":p"))
    for _, b in ipairs(api.nvim_list_bufs()) do
        if api.nvim_buf_is_loaded(b) and vim.fs.normalize(api.nvim_buf_get_name(b)) == full then
            return b
        end
    end
    return -1
end

--- Up to `max` lines of buffer `b`, and its filetype.
function M.buffer_lines(b, max)
    if not api.nvim_buf_is_valid(b) then
        return {}, ""
    end
    if not api.nvim_buf_is_loaded(b) then
        fn.bufload(b)
    end
    return api.nvim_buf_get_lines(b, 0, max, false), vim.bo[b].filetype
end

--- The line the cursor was last on in buffer `b` (1 if unknown).
function M.last_line(b)
    local ok, mark = pcall(api.nvim_buf_get_mark, b, '"')
    if ok and mark[1] > 0 then
        return mark[1]
    end
    return 1
end

function M.filetype(path)
    return vim.filetype.match({ filename = path }) or ""
end

--- Jump to (lnum, byte col) in the current window and center it.
function M.jump(lnum, col)
    local count = api.nvim_buf_line_count(0)
    pcall(api.nvim_win_set_cursor, 0, { math.max(1, math.min(lnum, count)), math.max(col, 0) })
    vim.cmd("normal! zz")
end

function M.quickfix(title, items)
    fn.setqflist({}, " ", { title = title, items = items })
    vim.cmd("botright copen")
end

return M
