use std::path::PathBuf;

use git2::build::RepoBuilder;
use nvim_oxi::{
    self,
    mlua, //
};

use super::{configs, spec::SpecInfo};
use crate::{
    core::keymaps::{Key, MAPPING},
    lua_spec,
    utils::{
        constants,
        trace::{self, NotifyLevel},
    },
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

        let lazy_config = lua_spec!(
            "configs/lazy.lua",
            &[
                ("SPEC", &spec),
                ("NVIM_TS_HOME", nvim_treesitter_home),
                ("NVIM_PLUGINS_HOME", nvim_plugins_home),
                ("NVIM_LOCK_PATH", nvim_lock_path)
            ]
        );

        lua.load(lazy_config.spec).exec()?;

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

impl PluginManager {
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

        if info.spec.contains("--@src:") {
            Self::format_file_error(name, info, lua_line, error_msg)
        } else {
            Self::format_inline_error(name, info, lua_line, error_msg)
        }
    }

    /// Format error for `lua_spec!(r#"..."#)` — Lua is inline in Rust file.
    fn format_inline_error(name: &str, info: &SpecInfo, lua_line: Option<usize>, error_msg: &str) -> String {
        let lines: Vec<&str> = info.spec.lines().collect();
        let rust_line = lua_line.map(|l| info.spec_line as usize + l - 1);

        let mut report = format!("[Abstract] Invalid spec '{name}'\n");
        report.push_str(&format!("  --> {}:{}\n", info.file, rust_line.map(|l| l.to_string()).unwrap_or_default()));

        if let Some(lua_ln) = lua_line {
            let start = lua_ln.saturating_sub(2);
            let end = (lua_ln + 1).min(lines.len());

            (start..end).for_each(|i| {
                let display_line = info.spec_line as usize + i;
                let marker = if i + 1 == lua_ln { ">" } else { " " };
                report.push_str(&format!("{marker} {display_line:4} | {}\n", lines[i]));
            });
        }

        report.push_str(&format!("  = {error_msg}"));
        report
    }

    /// Format error for `lua_spec!(lua_file!(...), ...)` — Lua is in external .lua file.
    /// Shows chronological trace: Rust file → Lua file.
    fn format_file_error(name: &str, info: &SpecInfo, lua_line: Option<usize>, error_msg: &str) -> String {
        use crate::plugins::spec::{resolve_full_path, resolve_source};

        let mut report = format!("[Abstract] Invalid spec '{name}'\n");

        // 1. Rust source (where the spec is built)
        report.push_str(&format!("  1 --> {}:{} (spec definition)\n", info.file, info.spec_line));

        if let Some(lua_ln) = lua_line {
            if let Some(resolved) = resolve_source(info.spec, lua_ln) {
                let full_path = resolve_full_path(info.file, &resolved.relative_path);

                // 2. Lua file (where the error is)
                report.push_str(&format!("  2 --> {}:{}\n", full_path, resolved.source_line));

                // Code snippet
                let lines: Vec<&str> = info.spec.lines().collect();
                let start = lua_ln.saturating_sub(2);
                let end = (lua_ln + 1).min(lines.len());

                for (i, line_content) in lines.iter().enumerate().take(end).skip(start) {
                    let display_num = resolved.source_line as isize - (lua_ln as isize - 1 - i as isize);
                    let marker = if i + 1 == lua_ln { ">" } else { " " };
                    if line_content.trim().starts_with("--@src:") {
                        continue;
                    }
                    if display_num > 0 {
                        report.push_str(&format!("{marker} {:4} | {}\n", display_num, line_content));
                    }
                }
            } else {
                // No marker — fallback to composed spec line
                let lines: Vec<&str> = info.spec.lines().collect();
                let start = lua_ln.saturating_sub(2);
                let end = (lua_ln + 1).min(lines.len());

                (start..end).for_each(|i| {
                    let marker = if i + 1 == lua_ln { ">" } else { " " };
                    report.push_str(&format!("{marker} {:4} | {}\n", i + 1, lines[i]));
                });
            }
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

/// Macro to build the (name, spec_info) list without repeating yourself.
/// Usage: `specs![module_a, module_b, ...]`
/// Expands to: `vec![("module_a", module_a::Plugin::spec()), ...]`
macro_rules! specs {
    (@entry $module:ident) => {
        (stringify!($module), $module::Plugin::spec())
    };
    (@entry ($path:literal)) => {{
        let name = $path.rsplit('/').next().unwrap_or($path).trim_end_matches(".lua");
        (name, lua_spec!($path))
    }};
    (@entry ($path:literal, $args:expr)) => {{
        let name = $path.rsplit('/').next().unwrap_or($path).trim_end_matches(".lua");
        (name, lua_spec!($path, $args))
    }};
    ($($entry:tt),* $(,)?) => {
        vec![$(specs!(@entry $entry)),*]
    };
}

impl PluginManager {
    /// Validate each plugin spec individually and return only valid ones.
    /// Invalid specs show a friendly error message with the offending code line.
    fn validated_spec(lua: &mlua::Lua) -> String {
        use configs::*;

        let specs: Vec<(&str, SpecInfo)> = specs![
            // Dependencies
            ("configs/colorful-menu.lua"),
            ("configs/mini-icons.lua"),
            ("configs/nio.lua"),
            ("configs/nui.lua"),
            ("configs/plenary.lua"),
            ("configs/web-devicons.lua"),
            //
            ("configs/abstract-cs.lua"),
            ("configs/abstract-cursor.lua"),
            ("configs/abstract-line.lua"),
            ("configs/abstract-plugs.lua"),
            ("configs/autopairs.lua"),
            ("configs/blink.lua"),
            ("configs/bqf.lua"),
            ("configs/code-runner.lua", &[("MAPPING_SET", MAPPING.set_map_str(Key::CodeRunner))]),
            ("configs/colorizer.lua"),
            ("configs/csvview.lua"),
            ("configs/dap.lua", &[("MAPPING_SET", MAPPING.set_map_str(Key::Dap))]),
            ("configs/dart-vim-plugin.lua"),
            ("configs/fff.lua", &[("MAPPING", MAPPING.get_map(Key::Fff))]),
            ("configs/fidget.lua"),
            ("configs/flutter-tools.lua"),
            ("configs/gitgraph.lua", &[("MAPPING_SET", MAPPING.set_map_str(Key::GitGraph))]),
            ("configs/gitsigns.lua"),
            ("configs/goto-preview.lua", &[("MAPPING_SET", MAPPING.set_map_str(Key::GotoPreview))]),
            ("configs/grapple.lua"),
            ("configs/helpview.lua"),
            ("configs/hop.lua", &[("MAPPING", MAPPING.get_map(Key::Hop))]),
            ("configs/hovercraft.lua", &[("MAPPING", MAPPING.get_map(Key::Hovercraft))]),
            ("configs/java.lua"),
            ("configs/kulala.lua", &[("MAPPING_SET", MAPPING.set_map_str(Key::Kulala))]),
            ("configs/lazydev-nvim.lua"),
            ("configs/luasnip.lua"),
            ("configs/markdown-preview.lua"),
            ("configs/markview.lua", &[("MAPPING_SET", MAPPING.set_map_str(Key::Markview))]),
            ("configs/neo-tree.lua", &[("MAPPING_SET", MAPPING.set_map_str(Key::NeoTree))]),
            ("configs/neotest.lua"),
            ("configs/noice.lua"),
            ("configs/oil.lua", &[("MAPPING", MAPPING.get_map(Key::Oil))]),
            ("configs/penvim.lua"),
            ("configs/renamer.lua"),
            ("configs/rustaceanvim.lua"),
            ("configs/schema-store.lua"),
            ("configs/session-manager.lua", &[("MAPPING_SET", MAPPING.set_map_str(Key::SessionManager))]),
            ("configs/surround.lua"),
            ("configs/tabby.lua"),
            ("configs/tiny-code-action.lua"),
            ("configs/treesitter.lua", &[("NVIM_TS_HOME", &constants::NVIM_TREESITTER_HOME)]),
            ("configs/trouble.lua", &[("MAPPING", MAPPING.get_map(Key::Trouble))]),
            ("configs/ts-autotag.lua"),
            ("configs/typescript-tools.lua"),
            ("configs/typst-preview.lua"),
            ("configs/vim-dadbod.lua"),
            ("configs/which-key.lua", &[("MAPPING", MAPPING.get_map(Key::WhichKey))]),
            comment_nvim,
            mason,
            snack,
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
                    if info.spec.contains("--@src:") {
                        trace::vim_notify_error_report(&report);
                    } else {
                        trace::vim_notify(&report, NotifyLevel::Error);
                    }
                },
            }
        }

        tracing::info!("Validated {}/{} plugin specs", valid_specs.len(), specs.len());
        format!("{{\n{}\n}}", valid_specs.join(",\n"))
    }
}
