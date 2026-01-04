use nvim_oxi::mlua;

#[allow(dead_code)]
#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum MapKey {
    Builtin,
    //
    AbstractTerminal,
    AbstractWindow,
    CodeRunner,
    Dap,
    Fff,
    GitGraph,
    GotoPreview,
    Grapple,
    Hop,
    Hovercraft,
    Kulala,
    LspConfig,
    Markview,
    NeoTree,
    Oil,
    SessionManager,
    Snacks,
    SnacksBufDelete,
    SnacksGh,
    SnacksGitBrowse,
    SnacksLazygit,
    SnacksPicker,
    Trouble,
    WhichKey,
}

#[derive(Debug, Clone)]
pub struct Mapping {
    lua: mlua::Lua,
}

impl Mapping {
    pub fn new(lua: mlua::Lua) -> Self {
        Self { lua }
    }

    pub fn set_map(&self, key: MapKey) -> nvim_oxi::Result<()> {
        let which_key = format!(r#"require("which-key").add({})"#, self.get_map(key));
        self.lua.load(which_key).exec().inspect_err(|e| {
            eprintln!("{e}");
        })?;

        Ok(())
    }

    pub fn get_map(&self, key: MapKey) -> &'static str {
        match key {
            MapKey::Builtin => self.builtin(),
            //
            MapKey::AbstractTerminal => self.abstract_terminal(),
            MapKey::AbstractWindow => self.abstract_window(),
            MapKey::CodeRunner => self.code_runner(),
            MapKey::Dap => self.dap(),
            MapKey::Fff => self.fff(),
            MapKey::GitGraph => self.git_graph(),
            MapKey::GotoPreview => self.goto_preview(),
            MapKey::Grapple => self.grapple(),
            MapKey::Hop => self.hop(),
            MapKey::Hovercraft => self.hovercraft(),
            MapKey::Kulala => self.kulala(),
            MapKey::LspConfig => self.lsp_config(),
            MapKey::Markview => self.markview(),
            MapKey::NeoTree => self.neo_tree(),
            MapKey::Oil => self.oil(),
            MapKey::SessionManager => self.session_manager(),
            MapKey::Snacks => self.snacks(),
            MapKey::SnacksBufDelete => self.snacks_bufdelete(),
            MapKey::SnacksGh => self.snacks_gh(),
            MapKey::SnacksGitBrowse => self.snacks_gitbrowse(),
            MapKey::SnacksLazygit => self.snacks_lazygit(),
            MapKey::SnacksPicker => self.snacks_picker(),
            MapKey::Trouble => self.trouble(),
            MapKey::WhichKey => self.which_key(),
        }
    }
}

// Plugin keymaps
impl Mapping {
    pub fn builtin(&self) -> &'static str {
        r##"{
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
        }"##
    }

    fn which_key(&self) -> &'static str {
        r#"{
            { "<leader>?", function() require('which-key').show({ global = false }) end, desc = "Buffer Local Keymaps /which-key" },
        }"#
    }

    fn lsp_config(&self) -> &'static str {
        r##"{
            { "<Leader>l",  group = "LSP" },
            { "<Leader>lf", function() vim.lsp.buf.format({ timeout_ms = 3000 }) end, desc = "Format document" },
            { "<Leader>la", function() require('tiny-code-action').code_action({}) end, desc = "Code action" },
            { "<Leader>lA", function() vim.lsp.buf.range_code_action() end, desc = "Range code action" },
            { "<Leader>ld", function() vim.lsp.buf.definition() end, desc = "Jumps to definition" },
            { "<Leader>lD", function() vim.lsp.buf.declaration() end, desc = "Jumps to declaration" },
            { "<Leader>le", function() vim.diagnostic.open_float() end, desc = "Show diagnostics" },
            { "<Leader>li", function() vim.lsp.buf.implementation() end, desc = "Lists all symbol implementations" },
            { "<Leader>ls", function() vim.lsp.buf.signature_help() end, desc = "Show symbol signature information" },
            { "<Leader>lT", function() vim.lsp.buf.type_definition() end, desc = "Jumps to type definition" },
            { "<Leader>lh", function() vim.lsp.inlay_hint.enable(not vim.lsp.inlay_hint.is_enabled()) end, desc = "Inlay hints /toggle" },
            { "<Leader>ln", function() vim.diagnostic.jump({ count = 1, float = true }) end, desc = "Move to next diagnostic" },
            { "<Leader>lb", function() vim.diagnostic.jump({ count = -1, float = true }) end, desc = "Move to previous diagnostic" },
            { "<Leader>lr", function() require('snacks').picker.lsp_references() end, nowait = true, desc = "References" },
            { "<Leader>lR", function() require('renamer').rename({}) end, desc = "Rename symbol" },
            {
                { "<Leader>lw",  group = "Workspace" },
                { "<Leader>lwa", function() vim.lsp.buf.add_workspace_folder() end, desc = "Add workspace folder" },
                { "<Leader>lwr", function() vim.lsp.buf.remove_workspace_folder() end, desc = "Remove workspace folders" },
                { "<Leader>lwl", function() print(vim.inspect(vim.lsp.buf.list_workspace_folders())) end, desc = "List workspace folders" },
            },
            { "<Leader>Ll", function() vim.cmd('tabnew ' .. vim.lsp.get_log_path()) end, desc = "LSP logs" },
        }"##
    }

    fn hovercraft(&self) -> &'static str {
        r#"{
            {
                "K",
                function()
                    local hovercraft = require("hovercraft")
                    if hovercraft.is_visible() then hovercraft.enter_popup() else hovercraft.hover() end
                end,
                desc = "Hover"
            },
        }"#
    }

    fn trouble(&self) -> &'static str {
        r#"{
            { "<leader>lt",  group = "Trouble" },
            { "<Leader>ltt", "<CMD>Trouble diagnostics toggle<CR>", desc = "Diagnostics /project" },
            { "<leader>ltT", "<cmd>Trouble diagnostics toggle filter.buf=0<cr>", desc = "Diagnostics /buffer" },
            { "<leader>lts", "<cmd>Trouble symbols toggle focus=false<cr>", desc = "Symbols" },
            { "<leader>ltl", "<cmd>Trouble lsp toggle focus=false win.position=right<cr>", desc = "LSP Definitions/references/..." },
            { "<leader>ltL", "<cmd>Trouble loclist toggle<cr>", desc = "Location List" },
            { "<leader>ltq", "<cmd>Trouble qflist toggle<cr>", desc = "Quickfix List" },
        }"#
    }

    fn neo_tree(&self) -> &'static str {
        r#"{
            { ";f", ":Neotree toggle<CR>", desc = "File Explorer(toggle)" },
        }"#
    }

    fn hop(&self) -> &'static str {
        r#"{
            { "f", "<CMD>lua require'hop'.hint_words()<CR>", desc = "Jump anywhere" },
        }"#
    }

    fn goto_preview(&self) -> &'static str {
        r#"{
            { "<Leader>lp",  group = "Preview" },
            { "<Leader>lpd", function() require('goto-preview').goto_preview_definition({}) end, desc = "Definition preview" },
            { "<Leader>lpt", function() require('goto-preview').goto_preview_type_definition({}) end, desc = "Type definition preview" },
            { "<Leader>lpi", function() require('goto-preview').goto_preview_implementation({}) end, desc = "Implementation preview" },
            { "<Leader>lpD", function() require('goto-preview').goto_preview_declaration({}) end, desc = "Declaration preview" },
            { "<Leader>lpr", function() require('goto-preview').goto_preview_references() end, desc = "References preview" },
            { "<Leader>lpQ", function() require('goto-preview').close_all_win() end, desc = "Close all window preview" },
        }"#
    }

    fn oil(&self) -> &'static str {
        r#"{
            { "-", group = "Files" },
            { "-", "<CMD>Oil<CR>", desc = "Open parent directory" },
        }"#
    }

    fn git_graph(&self) -> &'static str {
        r#"{
            { "<Leader>v", group = "Version Control - Git" },
            {
                "<Leader>vg",
                function()
                    require("gitgraph").draw({}, { all = true, max_count = 5000 })
                end,
                desc = "Git graph",
            },
        }"#
    }

    fn code_runner(&self) -> &'static str {
        r#"{
            { "<Leader>o",  group = "Run Code" },
            { "<leader>oo", ":RunCode<CR>", desc = "Runs based on file type (Run Code)" },
            { "<leader>of", ":RunFile<CR>", desc = "Execute command from its key in current directory (Run Code)" },
            { "<leader>ot", ":RunFile tab<CR>", desc = "Run the current file (optionally you can select an opening mode)." },
            { "<leader>op", ":RunProject<CR>", desc = "Run the current project(If you are in a project otherwise you will not do anything)." },
            { "<leader>oq", ":RunClose<CR>", desc = "Close runner (Run Code)" },
            { "<leader>oj", ":CRFiletype<CR>", desc = "Open json with supported files (Run Code)" },
            { "<leader>oJ", ":CRProjects<CR>", desc = "Open json with list of projects (Run Code)" },
        }"#
    }

    fn kulala(&self) -> &'static str {
        r#"{
            { "<Leader>r",  group = "HTTP" },
            { "<Leader>rr", "<CMD>lua require('kulala').run()<CR>", desc = "Make HTTP request" },
            { "<Leader>rh", "<CMD>lua require('kulala').jump_prev()<CR>", desc = "Jump to the previous request" },
            { "<Leader>rl", "<CMD>lua require('kulala').jump_next()<CR>", desc = "Jump to the next request" },
            { "<Leader>re", "<CMD>lua require('kulala').set_selected_env()<CR>", desc = "Select environment" },
            { "<Leader>rt", "<CMD>lua require('kulala').toggle_view()<CR>", desc = "Response view /toggle" },
        }"#
    }

    fn snacks(&self) -> &'static str {
        r##"{
            { "<Leader>L",  group = "Logs" },
            { "<Leader>Ln", "<CMD>lua Snacks.notifier.show_history()<CR>", desc = "Notification history /snacks" },
        }"##
    }

    fn snacks_bufdelete(&self) -> &'static str {
        r##"{
            { "<M-q>",      group = "Close" },
            { "<M-q><M-q>", "<CMD>lua Snacks.bufdelete()<CR>", desc = "Delete current buffer" },
            { "<M-q><S-q>", "<CMD>lua Snacks.bufdelete.other()<CR>", desc = "Delete all buffers except the current one" },
        }"##
    }

    fn snacks_lazygit(&self) -> &'static str {
        r##"{
            { "<Leader>vL",  group = "Lazygit" },
            { "<Leader>vl",  "<CMD>lua Snacks.lazygit()<CR>", desc = "open lazygit" },
            { "<Leader>vLl", "<CMD>lua Snacks.lazygit.log()<CR>", desc = "log view" },
            { "<Leader>vLf", "<CMD>lua Snacks.lazygit.log_file()<CR>", desc = "log of the current file" },
        }"##
    }

    fn snacks_gh(&self) -> &'static str {
        r###"{
            { "<Leader>vg", group = "Git - Repos" },
            { "<Leader>vgp", "<CMD>lua Snacks.picker.gh_pr()<CR>", desc = "Browse open pull requests" },
            { "<Leader>vgP", "<CMD>lua Snacks.picker.gh_pr({state='all'})<CR>", desc = "Browse open pull requests (All)" },
            { "<Leader>vgi", "<CMD>lua Snacks.picker.gh_issue()<CR>", desc = "Browse open issues" },
            { "<Leader>vgI", "<CMD>lua Snacks.picker.gh_issue({ state = 'all' })<CR>", desc = "Browse open issues (All)" },
        }"###
    }

    fn snacks_gitbrowse(&self) -> &'static str {
        r##"{
            { "<Leader>vm",  group = "More git" },
            { "<Leader>vmb", "<CMD>lua Snacks.gitbrowse()<CR>", desc = "Git Browse" },
        }"##
    }

    fn snacks_picker(&self) -> &'static str {
        r###"{
            {
                { "<M-b>",  "<CMD>lua Snacks.picker.buffers()<CR>", desc = "Buffers" },
                { "<M-f>",  "<CMD>lua Snacks.picker.files()<CR>" , desc = "Find Files /project" },
                { "<M-F>",  "<CMD>lua Snacks.picker.find_files({ cwd = vim.fn.expand('%:p:h') })<CR>", desc = "Find Files /current", mode = { "n", "x" } },
            },
            {
                { "<M-g>",  group = "Find" },
                { "<M-g>g", "<CMD>lua Snacks.picker.grep()<CR>", desc = "Find word /project" },
                { "<M-g>w", "<CMD>lua Snacks.picker.grep_word()<CR>", desc = "Find under Visual selection or word" },
                { "<M-g>l", "<CMD>lua Snacks.picker.lines()<CR>", desc = "Search Buffer Lines" },
                { "<M-g>c", "<CMD>lua Snacks.picker.commands()<CR>", desc = "Commands" },
                { "<M-g>m", "<CMD>lua Snacks.picker.keymaps()<CR>", desc = "Mappings" },
                { "<M-g>h", "<CMD>lua Snacks.picker.help()<CR>", desc = "Help" },
                { "<M-g>p", "<CMD>lua Snacks.picker.pick()<CR>", desc = "Snacks builtin cmds" },
            },
            {
                { "<Leader>m",  group = "Manager" },
                { "<Leader>mp", "<CMD>lua Snacks.picker.projects()<CR>", desc = "Projects" },
            },
        }"###
    }

    fn fff(&self) -> &'static str {
        r###"{
            { "<M-g>",  group = "Find" },
            { "<M-b>",  "<CMD>lua Snacks.picker.buffers()<CR>", desc = "Buffers" },
            { "<M-f>",  "<CMD>lua require('fff').find_in_git_root()<CR>" , desc = "Find Files /project" },
            { "<M-F>",  "<CMD>lua require('fff').find_files()<CR>", desc = "Find Files /current", mode = { "n", "x" } },
        }"###
    }

    fn markview(&self) -> &'static str {
        r#"{}"#
    }

    fn abstract_window(&self) -> &'static str {
        r#"{
            mode = { "i", "n", "t" },
            { "<M-m>", function() require("abs").window().toggle.maximize() end, desc = "Window maximizer /toggle" },
        }"#
    }

    fn abstract_terminal(&self) -> &'static str {
        r#"{
            mode = { "i", "n", "t" },
            { "<M-t>",      group = "Terminal" },
            { "<M-t><M-t>", function() require("abs").terminal().toggle() end, desc = "Terminal /toggle" },
            {
                mode = { "t" },
                { "<M-t>n",     function() require("abs").terminal().new() end, desc = "Open new terminal" },
                { "<M-t><M-h>", function() require("abs").terminal().prev() end, desc = "Goto previous terminal" },
                { "<M-t><M-l>", function() require("abs").terminal().next() end, desc = "Goto next terminal" },
            }
        }"#
    }

    fn dap(&self) -> &'static str {
        r#"{
            { "<Leader>d",  group = "Debug" },
            { "<Leader>dc", function() require('dap').continue() end, desc = "Continue debug" },
            { "<Leader>ds", function() require('dap').step_over() end, desc = "Step over" },
            { "<Leader>di", function() require('dap').step_into() end, desc = "Step into" },
            { "<Leader>do", function() require('dap').step_out() end, desc = "Step out" },
            { "<Leader>db", function() require('dap').toggle_breakpoint() end, desc = "Breakpoint /toggle" },
            { "<Leader>dr", function() require('dap').repl.open() end, desc = "Open REPL" },
            { "<Leader>dl", function() require('dap').run_last() end, desc = "Run last" },
            {
                { "<Leader>du",  group = "UI" },
                { "<Leader>dup", function() require('dap.ui.widgets').preview() end, desc = " Preview widget" },
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
            }
        }"#
    }

    fn grapple(&self) -> &'static str {
        r#"{
            { "<Leader>g",  group = "Go" },
            { "<Leader>gt", ":Grapple open_tags<CR>", desc = "Show tags" },
            { "<Leader>gl", ":Grapple open_loaded<CR>", desc = "Show loaded" },
            { "<Leader>gs", ":Grapple open_scopes<CR>", desc = "Show scopes" },
            { "<Leader>ga", ":Grapple toggle<CR>", desc = "Tag /toggle" },
            { "<Leader>gk", ":Grapple toggle_tags<CR>", desc = "Tags /toggle" },
            { "<Leader>gK", ":Grapple toggle_scopes<CR>", desc = "Scopes /toggle" },
            { "<Leader>gn", ":Grapple cycle forward<CR>", desc = "Goto next tag" },
            { "<Leader>gp", ":Grapple cycle backward<CR>", desc = "Goto previous tag" },
            { "<M-1>", ":Grapple select index=1<CR>", desc = "Grapple select 1" },
            { "<M-2>", ":Grapple select index=2<CR>", desc = "Grapple select 2" },
            { "<M-3>", ":Grapple select index=3<CR>", desc = "Grapple select 3" },
            { "<M-4>", ":Grapple select index=4<CR>", desc = "Grapple select 4" },
            { "<M-5>", ":Grapple select index=5<CR>", desc = "Grapple select 5" },
        }"#
    }

    fn session_manager(&self) -> &'static str {
        r#"{
            { "<Leader>ms",  group = "Session" },
            { "<Leader>mS",  ":SessionManager available_commands<CR>", desc = "Session commands" },
            { "<Leader>msc", ":SessionManager available_commands<CR>", desc = "Session commands" },
            { "<Leader>msl", ":SessionManager load_session<CR>", desc = "Load sessions" },
            { "<Leader>msL", ":SessionManager load_current_dir_session<CR>", desc = "Load current dir session" },
            { "<Leader>mss", ":SessionManager save_current_session<CR>", desc = "Save current session" },
            { "<Leader>msd", ":SessionManager delete_session<CR>", desc = "Delete sessions" },
            { "<Leader>msD", ":SessionManager delete_current_dir_session<CR>", desc = "Delete current dir sessions" },
        }"#
    }
}
