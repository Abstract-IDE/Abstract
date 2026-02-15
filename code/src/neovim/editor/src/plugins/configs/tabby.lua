--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: tabby.nvim
Source: https://github.com/nanozuki/tabby.nvim

A declarative, highly configurable, and neovim style tabline plugin.
Use your nvim tabs as a workspace multiplexer!
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

return {
    "nanozuki/tabby.nvim",
    event = { "TabEnter", "TabLeave", "TabNew", "TabClosed" },
    config = function()
        local tab = function()
            local tabs = tostring(#vim.api.nvim_list_tabpages())
            local tabpage = tostring(vim.api.nvim_tabpage_get_number(0))
            return tabpage .. "/" .. tabs
        end

        -- Merge duplicate buffer names across windows in the same tab
        local wins_in_tab = function(line, theme)
            local unique_buffers = {}

            return line.wins_in_tab(line.api.get_current_tab()).foreach(function(win)
                local buf_name = win.buf_name()
                unique_buffers[buf_name] = unique_buffers[buf_name] or { is_current = false, lines = {} }
                unique_buffers[buf_name].is_current = unique_buffers[buf_name].is_current or win.is_current()

                if not vim.tbl_contains(unique_buffers[buf_name].lines, buf_name) then
                    table.insert(unique_buffers[buf_name].lines, buf_name)
                    local is_same_buff = vim.fn.bufnr(buf_name) == vim.fn.bufnr()

                    local icon = " " .. win.file_icon() .. " "
                    local hl = is_same_buff and theme.current_win or theme.win

                    return {
                        line.sep("", theme.win, theme.fill),
                        icon,
                        buf_name,
                        line.sep("", theme.win, theme.tab),
                        hl = hl,
                        margin = "",
                    }
                end
            end)
        end

        local theme = {
            fill = "TabLineFill",
            head = "TabLine",
            current_tab = "TabLineSel",
            tab = "TabLine",
            win = "TabLine",
            current_win = "TabLineCurrentWin",
            tail = "TabLine",
        }

        local view = function(line)
            return {
                {
                    { tab(), hl = theme.head },
                    line.sep(" ", theme.head, theme.tab),
                },
                line.tabs().foreach(function(tab)
                    local hl = tab.is_current() and theme.current_tab or theme.tab
                    return {
                        " ",
                        tab.name(),
                        " ",
                        hl = hl,
                    }
                end),
                line.sep(" ", theme.head, theme.tab),
                line.spacer(),
                wins_in_tab(line, theme),
            }
        end

        local opt = {
            buf_name = {
                mode = "unique",
            },
        }

        -- Save and restore in session
        vim.opt.sessionoptions = "curdir,folds,globals,help,tabpages,terminal,winsize"
        vim.o.showtabline = 1

        require("tabby.tabline").set(view, opt)
    end
}
