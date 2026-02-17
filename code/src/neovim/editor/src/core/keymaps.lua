-- ── Keymaps ──
-- All keymap definitions for Abstract.
-- Injected into plugin configs via lua_section!

_G.ABSTRACT_PLUGIN_GROUP = {
    lsp       = "LSP",
    workspace = "Workspace",
    preview   = "Preview",
    debug     = "Debug",
    terminal  = "Terminal",
    files     = "Files",
    logs      = "Logs",
    git       = "Version Control - Git",
    find      = "Find",
    manager   = "Manager",
    session   = "Session",
    http      = "HTTP",
    run_code  = "Run Code",
    close     = "Close",
    ui        = "UI",
    trouble   = "Trouble",
    lazygit   = "Lazygit",
    git_repos = "Git - Repos",
    more_git  = "More git",
    go        = "Go",
}

local M = {}

M.builtin = --@builtin
{
    { "\\",         ":bnext<CR>",     desc = "Goto next buffer" },
    { "|",          ":bprevious<CR>", desc = "Goto previous buffer" },
    -- Window
    { "<M-q><M-w>", ":close <CR>",    desc = "Close current window" },
    { "<M-q><S-w>", ":only <CR>",     desc = "Close all window except current one" },
    -- TAB
    { "<M-q><M-t>", ":tabclose<CR>",  desc = "Close current tab" },
    { "<M-q><S-t>", ":tabonly<CR>",   desc = "Close all other tab" },
    { "<Tab>",      ":tabn<CR>",      desc = "Goto next tab" },
    { "<S-Tab>",    ":tabp<CR>",      desc = "Goto previous tab" },
    { "<M-S-,>",    ":-tabmove<CR>",  desc = "Move tab to next position" },
    { "<M-S-.>",    ":+tabmove<CR>",  desc = "Move tab to previous position" },
    -- LOGS
    { "<Leader>Lm", ":messages<CR>",  desc = "Messages history" },
}
--@end

M.which_key = --@which_key
{
    { "<leader>?", function() require('which-key').show({ global = false }) end, desc = "Buffer Local Keymaps /which-key" },
}
--@end

M.lsp_config = --@lsp_config
{
    { "<Leader>l",  group = _G.ABSTRACT_PLUGIN_GROUP.lsp },
    { "<Leader>lf", function() vim.lsp.buf.format({ timeout_ms = 3000 }) end,                      desc = "Format document" },
    { "<Leader>la", function() require('tiny-code-action').code_action({}) end,                    desc = "Code action" },
    { "<Leader>lA", function() vim.lsp.buf.range_code_action() end,                                desc = "Range code action" },
    { "<Leader>ld", function() vim.lsp.buf.definition() end,                                       desc = "Jumps to definition" },
    { "<Leader>lD", function() vim.lsp.buf.declaration() end,                                      desc = "Jumps to declaration" },
    { "<Leader>le", function() vim.diagnostic.open_float() end,                                    desc = "Show diagnostics" },
    { "<Leader>li", function() vim.lsp.buf.implementation() end,                                   desc = "Lists all symbol implementations" },
    { "<Leader>ls", function() vim.lsp.buf.signature_help() end,                                   desc = "Show symbol signature information" },
    { "<Leader>lT", function() vim.lsp.buf.type_definition() end,                                  desc = "Jumps to type definition" },
    { "<Leader>lh", function() vim.lsp.inlay_hint.enable(not vim.lsp.inlay_hint.is_enabled()) end, desc = "Inlay hints /toggle" },
    { "<Leader>ln", function() vim.diagnostic.jump({ count = 1, float = true }) end,               desc = "Move to next diagnostic" },
    { "<Leader>lb", function() vim.diagnostic.jump({ count = -1, float = true }) end,              desc = "Move to previous diagnostic" },
    { "<Leader>lr", function() require('snacks').picker.lsp_references() end,                      nowait = true,                             desc = "References" },
    { "<Leader>lR", function() require('renamer').rename({}) end,                                  desc = "Rename symbol" },
    {
        { "<Leader>lw",  group = _G.ABSTRACT_PLUGIN_GROUP.workspace },
        { "<Leader>lwa", function() vim.lsp.buf.add_workspace_folder() end,                       desc = "Add workspace folder" },
        { "<Leader>lwr", function() vim.lsp.buf.remove_workspace_folder() end,                    desc = "Remove workspace folders" },
        { "<Leader>lwl", function() print(vim.inspect(vim.lsp.buf.list_workspace_folders())) end, desc = "List workspace folders" },
    },
    { "<Leader>Ll", function() vim.cmd('tabnew ' .. vim.lsp.get_log_path()) end, desc = "LSP logs" },
}
--@end

M.hovercraft = --@hovercraft
{
    {
        "K",
        function()
            local hovercraft = require("hovercraft")
            if hovercraft.is_visible() then hovercraft.enter_popup() else hovercraft.hover() end
        end,
        desc = "Hover"
    },
}
--@end

M.trouble = --@trouble
{
    { "<leader>lt",  group = _G.ABSTRACT_PLUGIN_GROUP.trouble },
    { "<Leader>ltt", "<CMD>Trouble diagnostics toggle<CR>",                        desc = "Diagnostics /project" },
    { "<leader>ltL", "<cmd>Trouble loclist toggle<cr>",                            desc = "Location List" },
    { "<leader>ltT", "<cmd>Trouble diagnostics toggle filter.buf=0<cr>",           desc = "Diagnostics /buffer" },
    { "<leader>ltl", "<cmd>Trouble lsp toggle focus=false win.position=right<cr>", desc = "LSP Definitions/references/..." },
    { "<leader>ltq", "<cmd>Trouble qflist toggle<cr>",                             desc = "Quickfix List" },
    { "<leader>lts", "<cmd>Trouble symbols toggle focus=false<cr>",                desc = "Symbols" },
}
--@end

M.neo_tree = --@neo_tree
{
    { ";f", ":Neotree toggle<CR>", desc = "File Explorer(toggle)" },
}
--@end

M.hop = --@hop
{
    { "f", "<CMD>lua require'hop'.hint_words()<CR>", desc = "Jump anywhere" },
}
--@end

M.goto_preview = --@goto_preview
{
    { "<Leader>lp",  group = _G.ABSTRACT_PLUGIN_GROUP.preview },
    { "<Leader>lpd", function() require('goto-preview').goto_preview_definition({}) end,      desc = "Definition preview" },
    { "<Leader>lpt", function() require('goto-preview').goto_preview_type_definition({}) end, desc = "Type definition preview" },
    { "<Leader>lpi", function() require('goto-preview').goto_preview_implementation({}) end,  desc = "Implementation preview" },
    { "<Leader>lpD", function() require('goto-preview').goto_preview_declaration({}) end,     desc = "Declaration preview" },
    { "<Leader>lpr", function() require('goto-preview').goto_preview_references() end,        desc = "References preview" },
    { "<Leader>lpQ", function() require('goto-preview').close_all_win() end,                  desc = "Close all window preview" },
}
--@end

M.oil = --@oil
{
    { "-", group = _G.ABSTRACT_PLUGIN_GROUP.files },
    { "-", "<CMD>Oil<CR>",                        desc = "Open parent directory" },
}
--@end

M.git_graph = --@git_graph
{
    { "<Leader>v",  group = _G.ABSTRACT_PLUGIN_GROUP.git },
    { "<Leader>vg", function() require("gitgraph").draw({}, { all = true, max_count = 5000 }) end, desc = "Git graph" },
}
--@end

M.code_runner = --@code_runner
{
    { "<Leader>o",  group = _G.ABSTRACT_PLUGIN_GROUP.run_code },
    { "<leader>oo", ":RunCode<CR>",                           desc = "Runs based on file type (Run Code)" },
    { "<leader>of", ":RunFile<CR>",                           desc = "Execute command from its key in current directory (Run Code)" },
    { "<leader>ot", ":RunFile tab<CR>",                       desc = "Run the current file (optionally you can select an opening mode)." },
    { "<leader>op", ":RunProject<CR>",                        desc = "Run the current project(If you are in a project otherwise you will not do anything)." },
    { "<leader>oq", ":RunClose<CR>",                          desc = "Close runner (Run Code)" },
    { "<leader>oj", ":CRFiletype<CR>",                        desc = "Open json with supported files (Run Code)" },
    { "<leader>oJ", ":CRProjects<CR>",                        desc = "Open json with list of projects (Run Code)" },
}
--@end

M.kulala = --@kulala
{
    { "<Leader>r",  group = _G.ABSTRACT_PLUGIN_GROUP.http },
    { "<Leader>rr", "<CMD>lua require('kulala').run()<CR>",              desc = "Make HTTP request" },
    { "<Leader>rh", "<CMD>lua require('kulala').jump_prev()<CR>",        desc = "Jump to the previous request" },
    { "<Leader>rl", "<CMD>lua require('kulala').jump_next()<CR>",        desc = "Jump to the next request" },
    { "<Leader>re", "<CMD>lua require('kulala').set_selected_env()<CR>", desc = "Select environment" },
    { "<Leader>rt", "<CMD>lua require('kulala').toggle_view()<CR>",      desc = "Response view /toggle" },
}
--@end

M.snacks = --@snacks
{
    { "<Leader>L",  group = _G.ABSTRACT_PLUGIN_GROUP.logs },
    { "<Leader>Ln", "<CMD>lua Snacks.notifier.show_history()<CR>", desc = "Notification history /snacks" },
}
--@end

M.snacks_bufdelete = --@snacks_bufdelete
{
    { "<M-q>",      group = _G.ABSTRACT_PLUGIN_GROUP.close },
    { "<M-q><M-q>", "<CMD>lua Snacks.bufdelete()<CR>",       desc = "Delete current buffer" },
    { "<M-q><S-q>", "<CMD>lua Snacks.bufdelete.other()<CR>", desc = "Delete all buffers except the current one" },
}
--@end

M.snacks_lazygit = --@snacks_lazygit
{
    { "<Leader>vL",  group = _G.ABSTRACT_PLUGIN_GROUP.lazygit },
    { "<Leader>vl",  "<CMD>lua Snacks.lazygit()<CR>",          desc = "open lazygit" },
    { "<Leader>vLl", "<CMD>lua Snacks.lazygit.log()<CR>",      desc = "log view" },
    { "<Leader>vLf", "<CMD>lua Snacks.lazygit.log_file()<CR>", desc = "log of the current file" },
}
--@end

M.snacks_gh = --@snacks_gh
{
    { "<Leader>vg",  group = _G.ABSTRACT_PLUGIN_GROUP.git_repos },
    { "<Leader>vgp", "<CMD>lua Snacks.picker.gh_pr()<CR>",                     desc = "Browse open pull requests" },
    { "<Leader>vgP", "<CMD>lua Snacks.picker.gh_pr({state='all'})<CR>",        desc = "Browse open pull requests (All)" },
    { "<Leader>vgi", "<CMD>lua Snacks.picker.gh_issue()<CR>",                  desc = "Browse open issues" },
    { "<Leader>vgI", "<CMD>lua Snacks.picker.gh_issue({ state = 'all' })<CR>", desc = "Browse open issues (All)" },
}
--@end

M.snacks_gitbrowse = --@snacks_gitbrowse
{
    { "<Leader>vm",  group = _G.ABSTRACT_PLUGIN_GROUP.more_git },
    { "<Leader>vmb", "<CMD>lua Snacks.gitbrowse()<CR>",        desc = "Git Browse" },
}
--@end

M.snacks_picker = --@snacks_picker
{
    {
        { "<M-b>", "<CMD>lua Snacks.picker.buffers()<CR>",                               desc = "Buffers" },
        { "<M-f>", "<CMD>lua Snacks.picker.files()<CR>",                                 desc = "Find Files /project" },
        { "<M-F>", "<CMD>lua Snacks.picker.files({ cwd = vim.fn.expand('%:p:h') })<CR>", desc = "Find Files /current", mode = { "n", "x" } },
    },
    {
        { "<M-g>",  group = _G.ABSTRACT_PLUGIN_GROUP.find },
        { "<M-g>g", "<CMD>lua Snacks.picker.grep()<CR>",      desc = "Find word /project" },
        { "<M-g>w", "<CMD>lua Snacks.picker.grep_word()<CR>", desc = "Find under Visual selection or word" },
        { "<M-g>l", "<CMD>lua Snacks.picker.lines()<CR>",     desc = "Search Buffer Lines" },
        { "<M-g>c", "<CMD>lua Snacks.picker.commands()<CR>",  desc = "Commands" },
        { "<M-g>m", "<CMD>lua Snacks.picker.keymaps()<CR>",   desc = "Mappings" },
        { "<M-g>h", "<CMD>lua Snacks.picker.help()<CR>",      desc = "Help" },
        { "<M-g>p", "<CMD>lua Snacks.picker.pick()<CR>",      desc = "Snacks builtin cmds" },
    },
    {
        { "<Leader>m",  group = _G.ABSTRACT_PLUGIN_GROUP.manager },
        { "<Leader>mp", "<CMD>lua Snacks.picker.projects()<CR>", desc = "Projects" },
    },
}
--@end

M.fff = --@fff
{
    { "<M-g>",      group = _G.ABSTRACT_PLUGIN_GROUP.find },
    { "<M-f>",      function() require('fff').find_files() end,                              desc = "Find Files /project" },
    { "<M-F>",      function() require('fff').find_files_in_dir(vim.fn.expand('%:p:h')) end, desc = "Find Files /current",                mode = { "n", "x" } },
    { "<M-b>",      "<CMD>lua Snacks.picker.buffers()<CR>",                                  desc = "Buffers" },
    { "<M-g><M-g>", "<CMD>lua Snacks.picker.grep()<CR>",                                     desc = "Find word /project" },
    { "<M-g><M-w>", "<CMD>lua Snacks.picker.grep_word()<CR>",                                desc = "Find under Visual selection or word" },
    -- { "<M-g><M-g>", function() require('fff').live_grep() end,                               desc = "Find word /project" },
}
--@end

M.markview = --@markview
{}
--@end

M.abstract_window = --@abstract_window
{
    mode = { "i", "n", "t" },
    { "<M-m>", function() require("abs").window().toggle.maximize() end, desc = "Window maximizer /toggle" },
}
--@end

M.abstract_terminal = --@abstract_terminal
{
    mode = { "i", "n", "t" },
    { "<M-t>",      group = _G.ABSTRACT_PLUGIN_GROUP.terminal },
    { "<M-t><M-t>", function() require("abs").terminal().toggle() end, desc = "Terminal /toggle" },
    {
        mode = { "t" },
        { "<M-t>n",     function() require("abs").terminal().new() end,  desc = "Open new terminal" },
        { "<M-t><M-h>", function() require("abs").terminal().prev() end, desc = "Goto previous terminal" },
        { "<M-t><M-l>", function() require("abs").terminal().next() end, desc = "Goto next terminal" },
    },
}
--@end

M.dap = --@dap
{
    { "<Leader>d",  group = _G.ABSTRACT_PLUGIN_GROUP.debug },
    { "<Leader>db", function() require('dap').toggle_breakpoint() end, desc = "Breakpoint /toggle" },
    { "<Leader>dc", function() require('dap').continue() end,          desc = "Continue debug" },
    { "<Leader>di", function() require('dap').step_into() end,         desc = "Step into" },
    { "<Leader>dl", function() require('dap').run_last() end,          desc = "Run last" },
    { "<Leader>do", function() require('dap').step_out() end,          desc = "Step out" },
    { "<Leader>dr", function() require('dap').repl.open() end,         desc = "Open REPL" },
    { "<Leader>ds", function() require('dap').step_over() end,         desc = "Step over" },
    {
        { "<Leader>du",  group = _G.ABSTRACT_PLUGIN_GROUP.ui },
        { "<Leader>dup", function() require('dap.ui.widgets').preview() end, desc = "Preview widget" },
        {
            "<Leader>duf",
            function()
                local widgets = require('dap.ui.widgets')
                widgets.centered_float(widgets.frames)
            end,
            desc = "Frames widget"
        },
        {
            "<Leader>duc",
            function()
                local widgets = require('dap.ui.widgets')
                widgets.centered_float(widgets.scopes)
            end,
            desc = "Scopes widget"
        },
    },
}
--@end

M.grapple = --@grapple
{
    { "<Leader>g",  group = _G.ABSTRACT_PLUGIN_GROUP.go },
    { "<Leader>gt", "<CMD> Grapple open_tags<CR>",      desc = "Show tags" },
    { "<Leader>gl", "<CMD> Grapple open_loaded<CR>",    desc = "Show loaded" },
    { "<Leader>gs", "<CMD> Grapple open_scopes<CR>",    desc = "Show scopes" },
    { "<Leader>ga", "<CMD> Grapple toggle<CR>",         desc = "Tag /toggle" },
    { "<Leader>gk", "<CMD> Grapple toggle_tags<CR>",    desc = "Tags /toggle" },
    { "<Leader>gK", "<CMD> Grapple toggle_scopes<CR>",  desc = "Scopes /toggle" },
    { "<Leader>gn", "<CMD> Grapple cycle forward<CR>",  desc = "Goto next tag" },
    { "<Leader>gp", "<CMD> Grapple cycle backward<CR>", desc = "Goto previous tag" },
    { "<M-1>",      "<CMD> Grapple select index=1<CR>", desc = "Grapple select 1" },
    { "<M-2>",      "<CMD> Grapple select index=2<CR>", desc = "Grapple select 2" },
    { "<M-3>",      "<CMD> Grapple select index=3<CR>", desc = "Grapple select 3" },
    { "<M-4>",      "<CMD> Grapple select index=4<CR>", desc = "Grapple select 4" },
    { "<M-5>",      "<CMD> Grapple select index=5<CR>", desc = "Grapple select 5" },
}
--@end

M.session_manager = --@session_manager
{
    { "<Leader>ms",  group = _G.ABSTRACT_PLUGIN_GROUP.session },
    { "<Leader>mS",  ":SessionManager available_commands<CR>",         desc = "Session commands" },
    { "<Leader>msc", ":SessionManager available_commands<CR>",         desc = "Session commands" },
    { "<Leader>msl", ":SessionManager load_session<CR>",               desc = "Load sessions" },
    { "<Leader>msL", ":SessionManager load_current_dir_session<CR>",   desc = "Load current dir session" },
    { "<Leader>mss", ":SessionManager save_current_session<CR>",       desc = "Save current session" },
    { "<Leader>msd", ":SessionManager delete_session<CR>",             desc = "Delete sessions" },
    { "<Leader>msD", ":SessionManager delete_current_dir_session<CR>", desc = "Delete current dir sessions" },
}
--@end

return M
