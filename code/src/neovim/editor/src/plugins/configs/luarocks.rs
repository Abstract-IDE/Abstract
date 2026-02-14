#![allow(unused)]

/*
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: plenary.nvim
Source: https://github.com/nvim-lua/plenary.nvim

luarocks.nvim is a Neovim plugin designed to streamline the installation of
luarocks packages directly within Neovim. It simplifies the process of
managing Lua dependencies, ensuring a hassle-free experience for Neovim users.
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
*/

use crate::lua_spec;

pub struct Plugin;

impl Plugin {
    pub fn spec() -> crate::plugins::spec::SpecInfo {
        lua_spec!(r#"{
            "vhyrro/luarocks.nvim",
            -- Very high priority is required,
            -- luarocks.nvim should run as the first plugin in your config.
            priority = 1100,
            config = true,
        }"#)
    }
}
