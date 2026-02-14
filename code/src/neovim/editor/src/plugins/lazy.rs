use std::path::PathBuf;

use git2::build::RepoBuilder;
use nvim_oxi::{
    self,
    mlua, //
};

use super::{configs, spec::SpecInfo};
use crate::utils::{
    constants,
    trace::{self, NotifyLevel},
};

pub struct PluginManager;

impl PluginManager {
    pub fn new() -> nvim_oxi::Result<Self> {
        Self::setup()?;
        Ok(Self)
    }

    // Setup lazy.nvim with plugins
    fn setup() -> nvim_oxi::Result<()> {
        //
        let lua = mlua::lua();
        Self::bootstrap(lua.clone())?;

        let spec = Self::validated_spec(&lua);
        let nvim_treesitter_home: &str = &constants::NVIM_TREESITTER_HOME;
        let nvim_plugins_home: &str = &constants::NVIM_PM_INSTALL_HOME;
        let nvim_lock_path: &str = &constants::NVIM_PM_LOCK;

        let setup_lazy = format!(
            // language=lua
            r#"
                local ok, err = pcall(function()
                    require("lazy").setup({{
                        spec = {spec},

                        root = {nvim_plugins_home:?}, -- directory where plugins will be installed
                        -- TODO: change it later with proper path
                        lockfile = {nvim_lock_path:?} .. "/plugin-lock.json", -- lockfile generated after running update.

                        performance = {{
                            cache = {{ enabled = true }},
                            reset_packpath = true, -- reset the package path to improve startup time
                            rtp = {{
                                reset = true, -- reset the runtime path to $VIMRUNTIME and your config directory
                                -- add any custom paths here that you want to includes in the rtp
                                ---@type string[]
                                paths = {{
                                    -- ABSTRACT["INSTALL_PATH"],
                                    {nvim_treesitter_home:?}
                                }},
                                ---@type string[] list any plugins you want to disable here
                                disabled_plugins = {{ "tutor" }}, -- "gzip", "matchit", "matchparen", "netrwPlugin", "tarPlugin", "tohtml", "zipPlugin",
                            }},
                        }},

                        install = {{
                            -- install missing plugins on startup. This doesn't increase startup time.
                            missing = true,
                            -- try to load one of these colorschemes when starting an installation during startup
                            colorscheme = {{ "abscs", "default" }},
                        }},

                        ui = {{
                            -- a number <1 is a percentage., >1 is a fixed size
                            size = {{ width = 0.8, height = 0.8 }},
                            wrap = true, -- wrap the lines in the ui
                            border = "rounded", -- The border to use for the UI window. Accepts same border values as |nvim_open_win()|.
                            title_pos = "center", ---@type "center" | "left" | "right"
                            throttle = 20, -- how frequently should the ui process render events
                            backdrop = 100, -- The backdrop opacity. 0 is fully opaque, 100 is fully transparent.
                        }},
                    }})
                end)
                if not ok then
                    vim.notify("[Abstract] lazy.nvim setup failed:\n" .. tostring(err), vim.log.levels.ERROR)
                end
            "#
        );

        lua.load(&setup_lazy).exec()?;

        Ok(())
    }

    fn bootstrap(lua: mlua::Lua) -> nvim_oxi::Result<()> {
        let lazypath: PathBuf = PathBuf::from(&*constants::NVIM_PM_INSTALL_HOME).join("lazy.nvim");

        if !lazypath.exists() {
            let lazy_remote = "https://github.com/folke/lazy.nvim.git";

            tracing::info!("lazy.nvim not found, installing from {lazy_remote}");

            let mut fo = git2::FetchOptions::new();
            fo.depth(1);
            if let Err(e) = RepoBuilder::new().branch("main").fetch_options(fo).clone(lazy_remote, &lazypath) {
                tracing::error!("Failed to clone lazy.nvim: {e}");
            }
        };

        let exec = format!("vim.opt.rtp:prepend({:?})", lazypath);
        lua.load(exec).exec()?;

        Ok(())
    }
}

/// Macro to build the (name, spec_info) list without repeating yourself.
/// Usage: `specs![module_a, module_b, ...]`
/// Expands to: `vec![("module_a", module_a::Plugin::spec()), ...]`
macro_rules! specs {
    ($($module:ident),* $(,)?) => {
        vec![
            $((stringify!($module), $module::Plugin::spec()),)*
        ]
    };
}

impl PluginManager {
    /// Validate each plugin spec individually and return only valid ones.
    /// Invalid specs show a friendly error message with the offending code line.
    fn validated_spec(lua: &mlua::Lua) -> String {
        use configs::*;

        let specs: Vec<(&str, SpecInfo)> = specs![
            // Dependencies that other plugins depends on
            colorful_menu,
            mini_icons,
            nio,
            nui,
            plenary,
            web_devicons,
            // Plugins
            abstract_cs,
            abstract_cursor,
            abstract_line,
            abstract_plugs,
            autopairs,
            blink,
            bqf,
            code_runner,
            colorizer,
            comment,
            csvview,
            dap,
            dart_vim_plugin,
            fff,
            fidget,
            flutter_tools,
            gitgraph,
            gitsigns,
            goto_preview,
            grapple,
            helpview,
            hop,
            hovercraft,
            java,
            kulala,
            luasnip,
            markdown_preview,
            markview,
            mason,
            neo_tree,
            neotest,
            noice,
            none_ls,
            oil,
            penvim,
            renamer,
            rustaceanvim,
            schema_store,
            session_manager,
            snack,
            surround,
            tabby,
            tiny_code_action,
            treesitter,
            trouble,
            ts_autotag,
            ts_context_commentstring,
            typescript_tools,
            typst_preview,
            vim_dadbod,
            which_key,
        ];

        let mut valid_specs = Vec::with_capacity(specs.len());

        for (name, info) in &specs {
            let check_code = format!("local _ = {}", info.spec);
            let chunk_name = format!("plugin:{name}");

            match lua.load(&check_code).set_name(&chunk_name).exec() {
                Ok(()) => {
                    valid_specs.push(info.spec);
                },
                Err(e) => {
                    let report = Self::format_spec_error(name, info, &e.to_string());
                    tracing::error!("Plugin spec validation failed:\n{report}");
                    trace::vim_notify(&report, NotifyLevel::Error);
                },
            }
        }

        tracing::info!("Validated {}/{} plugin specs", valid_specs.len(), specs.len());
        format!("{{\n{}\n}}", valid_specs.join(",\n"))
    }

    /// Formats a spec error with the actual Rust source file, line number, and offending code.
    ///
    /// Example output:
    /// ```text
    /// [Abstract] Invalid spec 'vim_dadbod'
    ///   --> src/neovim/editor/src/plugins/configs/vim_dadbod.rs:31
    ///   |
    ///    30 |             init = function()
    /// >  31 |       TESTING FOR ERROR
    ///    32 |                 -- Your DBUI configuration
    ///   |
    ///   = '=' expected near 'FOR'
    /// ```
    fn format_spec_error(name: &str, info: &SpecInfo, raw_error: &str) -> String {
        let lua_line = Self::extract_error_line(raw_error);
        let error_msg = Self::extract_error_message(raw_error);
        let lines: Vec<&str> = info.spec.lines().collect();

        // Compute real Rust line: spec_line is where lua_spec! was called,
        // and the spec content starts on that same line (or +1 for the opening `r#"`).
        // lua_line is 1-indexed within the spec string.
        let rust_line = lua_line.map(|l| info.spec_line as usize + l - 1);

        let mut report = format!("[Abstract] Invalid spec '{name}'\n");
        report.push_str(&format!("  --> {}:{}\n", info.file, rust_line.map(|l| l.to_string()).unwrap_or_default()));

        if let Some(lua_ln) = lua_line {
            let start = lua_ln.saturating_sub(2);
            let end = (lua_ln + 1).min(lines.len());

            report.push_str("  |\n");
            (start..end).for_each(|i| {
                let display_line = info.spec_line as usize + i; // real Rust line
                let marker = if i + 1 == lua_ln { ">" } else { " " };
                report.push_str(&format!("{marker} {display_line:4} | {}\n", lines[i]));
            });
            report.push_str("  |\n");
        }

        report.push_str(&format!("  = {error_msg}"));
        report
    }

    /// Parse line number from Lua error like `[string "plugin:foo"]:2: msg`
    fn extract_error_line(error: &str) -> Option<usize> {
        let after_bracket = error.find("]:")?;
        let rest = &error[after_bracket + 2..];
        let colon = rest.find(':')?;
        rest[..colon].trim().parse().ok()
    }

    /// Extract just the error message, stripping the `[string "..."]:N:` prefix
    fn extract_error_message(error: &str) -> &str {
        if let Some(bracket_pos) = error.find("]:") {
            let rest = &error[bracket_pos + 2..];
            if let Some(colon_pos) = rest.find(':') {
                let msg = &rest[colon_pos + 1..];
                return msg.trim();
            }
        }
        error.trim()
    }
}
