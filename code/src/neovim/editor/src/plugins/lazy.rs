use nvim_oxi::{
    self,
    mlua, //
};

use super::configs;

pub struct PluginManager;

impl PluginManager {
    pub fn new(lua: mlua::Lua) -> nvim_oxi::Result<Self> {
        Self::bootstrap(lua.clone())?;
        Self::setup(lua)?;

        Ok(Self)
    }

    // Setup lazy.nvim with plugins
    fn setup(lua: mlua::Lua) -> nvim_oxi::Result<()> {
        //
        let specs = [
            //
            // Dependencies that other plugins depends on
            configs::colorful_menu::Plugin::spec(),
            configs::luarocks::Plugin::spec(),
            configs::mini_icons::Plugin::spec(),
            configs::nio::Plugin::spec(),
            configs::nui::Plugin::spec(),
            configs::plenary::Plugin::spec(),
            configs::web_devicons::Plugin::spec(),
            //
            // Plugins
            configs::abstract_cs::Plugin::spec(),
            configs::abstract_cursor::Plugin::spec(),
            configs::abstract_line::Plugin::spec(),
            configs::abstract_plugs::Plugin::spec(),
            configs::blink::Plugin::spec(),
            configs::code_runner::Plugin::spec(),
            configs::colorizer::Plugin::spec(),
            configs::csvview::Plugin::spec(),
            configs::fidget::Plugin::spec(),
            configs::gitsigns::Plugin::spec(),
            configs::grapple::Plugin::spec(),
            configs::helpview::Plugin::spec(),
            configs::hop::Plugin::spec(),
            configs::luasnip::Plugin::spec(),
            configs::snack::Plugin::spec(),
            configs::treesitter::Plugin::spec(),
            configs::which_key::Plugin::spec(),
        ];

        let spec = format!("{{\n{}\n}}", specs.join(",\n"));
        let setup_code = format!(
            // language=lua
            r#"
                require("lazy").setup({{
                    spec = {spec},
                    install = {{ colorscheme = {{ "habamax" }} }},
                    checker = {{ enabled = true }},
                }})
            "#
        );

        lua.load(&setup_code).exec()?;

        Ok(())
    }

    // Bootstrap lazy.nvim
    fn bootstrap(lua: mlua::Lua) -> nvim_oxi::Result<()> {
        lua.load(
            r#"
                local lazypath = vim.fn.stdpath("data") .. "/rust/lazy/lazy.nvim"
                if not (vim.uv or vim.loop).fs_stat(lazypath) then
                    local lazyrepo = "https://github.com/folke/lazy.nvim.git"
                    local out = vim.fn.system({ "git", "clone", "--filter=blob:none", "--branch=stable", lazyrepo, lazypath })
                    if vim.v.shell_error ~= 0 then
                        vim.api.nvim_echo({
                            { "Failed to clone lazy.nvim:\n", "ErrorMsg" },
                            { out, "WarningMsg" },
                            { "\nPress any key to exit..." },
                        }, true, {})
                        vim.fn.getchar()
                        os.exit(1)
                    end
                end
                vim.opt.rtp:prepend(lazypath)
            "#,
        )
        .exec()?;

        // // Set leaders
        // lua.load(
        //     r#"
        //         vim.g.mapleader = " "
        //         vim.g.maplocalleader = "\\"
        //     "#,
        // )
        // .exec()?;

        Ok(())
    }
}
