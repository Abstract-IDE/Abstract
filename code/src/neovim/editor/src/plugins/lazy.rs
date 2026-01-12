use std::path::PathBuf;

use git2::build::RepoBuilder;
use nvim_oxi::{
    self,
    mlua, //
};

use crate::utils::constants;

use super::configs;

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

        let spec = Self::spec();
        let nvim_treesitter_home: &str = &constants::NVIM_TREESITTER_HOME;
        let nvim_plugins_home: &str = &constants::NVIM_PM_INSTALL_HOME;
        let nvim_lock_path: &str = &constants::NVIM_PM_LOCK;

        let setup_lazy = format!(
            // language=lua
            r#"
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
            "#
        );

        lua.load(&setup_lazy).exec()?;

        Ok(())
    }

    fn bootstrap(lua: mlua::Lua) -> nvim_oxi::Result<()> {
        let lazypath: PathBuf = PathBuf::from(&*constants::NVIM_PM_INSTALL_HOME).join("lazy.nvim");

        if !lazypath.exists() {
            let lazy_remote = "https://github.com/folke/lazy.nvim.git";

            println!("lazy.nvin not found");
            println!("installing... ");
            println!("cloning: {lazy_remote}");

            let mut fo = git2::FetchOptions::new();
            fo.depth(1);
            if let Err(e) = RepoBuilder::new().branch("main").fetch_options(fo).clone(lazy_remote, &lazypath) {
                println!("failed to clone lazy.nvim\n{}", e);
            }
        };

        let exec = format!("vim.opt.rtp:prepend({:?})", lazypath);
        lua.load(exec).exec()?;

        Ok(())
    }
}

impl PluginManager {
    fn spec() -> String {
        use configs::*;

        let specs = [
            //
            // Dependencies that other plugins depends on
            colorful_menu::Plugin::spec(),
            mini_icons::Plugin::spec(),
            nio::Plugin::spec(),
            nui::Plugin::spec(),
            plenary::Plugin::spec(),
            web_devicons::Plugin::spec(),
            //
            // Plugins
            abstract_cs::Plugin::spec(),
            abstract_cursor::Plugin::spec(),
            abstract_line::Plugin::spec(),
            abstract_plugs::Plugin::spec(),
            autopairs::Plugin::spec(),
            blink::Plugin::spec(),
            bqf::Plugin::spec(),
            code_runner::Plugin::spec(),
            colorizer::Plugin::spec(),
            comment::Plugin::spec(),
            csvview::Plugin::spec(),
            dap::Plugin::spec(),
            dart_vim_plugin::Plugin::spec(),
            fff::Plugin::spec(),
            fidget::Plugin::spec(),
            flutter_tools::Plugin::spec(),
            gitgraph::Plugin::spec(),
            gitsigns::Plugin::spec(),
            goto_preview::Plugin::spec(),
            grapple::Plugin::spec(),
            helpview::Plugin::spec(),
            hop::Plugin::spec(),
            hovercraft::Plugin::spec(),
            java::Plugin::spec(),
            kulala::Plugin::spec(),
            luasnip::Plugin::spec(),
            markdown_preview::Plugin::spec(),
            markview::Plugin::spec(),
            mason::Plugin::spec(),
            neo_tree::Plugin::spec(),
            neotest::Plugin::spec(),
            noice::Plugin::spec(),
            none_ls::Plugin::spec(),
            oil::Plugin::spec(),
            penvim::Plugin::spec(),
            renamer::Plugin::spec(),
            rustaceanvim::Plugin::spec(),
            schema_store::Plugin::spec(),
            session_manager::Plugin::spec(),
            snack::Plugin::spec(),
            surround::Plugin::spec(),
            tabby::Plugin::spec(),
            tiny_code_action::Plugin::spec(),
            treesitter::Plugin::spec(),
            trouble::Plugin::spec(),
            ts_autotag::Plugin::spec(),
            ts_context_commentstring::Plugin::spec(),
            typescript_tools::Plugin::spec(),
            typst_preview::Plugin::spec(),
            vim_dadbod::Plugin::spec(),
            which_key::Plugin::spec(),
        ];

        format!("{{\n{}\n}}", specs.join(",\n"))
    }
}
