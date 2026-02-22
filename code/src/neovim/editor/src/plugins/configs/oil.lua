--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: oil.nvim
Source: https://github.com/stevearc/oil.nvim

Neovim file explorer: edit your filesystem like a buffer
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
    "stevearc/oil.nvim",
    --[[@rs keys=$MAPPING, ]]
}

spec.opts = {
    default_file_explorer = true,
    columns = {
        "icon",
    },
    buf_options = {
        buflisted = false,
        bufhidden = "hide",
    },
    win_options = {
        wrap = false,
        signcolumn = "no",
        cursorcolumn = false,
        foldcolumn = "0",
        spell = false,
        list = false,
        conceallevel = 3,
        concealcursor = "nvic",
    },
    delete_to_trash = false,
    skip_confirm_for_simple_edits = false,
    prompt_save_on_select_new_entry = true,
    cleanup_delay_ms = 2000,
    lsp_file_methods = {
        timeout_ms = 1000,
        autosave_changes = false,
    },
    constrain_cursor = "editable",
    use_default_keymaps = true,
    view_options = {
        show_hidden = false,
        is_hidden_file = function(name, bufnr)
            return vim.startswith(name, ".")
        end,
        is_always_hidden = function(name, bufnr)
            return false
        end,
        sort = {
            { "type", "asc" },
            { "name", "asc" },
        },
    },
    float = {
        padding = 2,
        max_width = 0,
        max_height = 0,
        border = "rounded",
        win_options = {
            winblend = 0,
        },
        override = function(conf)
            return conf
        end,
    },
    preview = {
        max_width = 0.9,
        min_width = { 40, 0.4 },
        width = nil,
        max_height = 0.9,
        min_height = { 12, 0.4 },
        height = nil,
        border = "rounded",
        win_options = {
            winblend = 0,
        },
        update_on_cursor_moved = true,
    },
    progress = {
        max_width = 0.9,
        min_width = { 40, 0.4 },
        width = nil,
        max_height = { 10, 0.9 },
        min_height = { 5, 0.1 },
        height = nil,
        border = "rounded",
        minimized_border = "none",
        win_options = {
            winblend = 0,
        },
    },
    keymaps = {
        ["g?"] = "actions.show_help",
        ["<CR>"] = "actions.select",
        ["<C-s>"] = "actions.select_vsplit",
        ["<C-h>"] = "actions.select_split",
        ["<C-t>"] = "actions.select_tab",
        ["<C-p>"] = "actions.preview",
        ["<C-c>"] = "actions.close",
        ["<C-l>"] = "actions.refresh",
        ["-"] = "actions.parent",
        ["_"] = "actions.open_cwd",
        ["`"] = "actions.cd",
        ["~"] = "actions.tcd",
        ["gs"] = "actions.change_sort",
        ["gx"] = "actions.open_external",
        ["g."] = "actions.toggle_hidden",
        ["g\\"] = "actions.toggle_trash",
    },
}

return spec
