/*
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: nvim-treesitter
Source: https://github.com/nvim-treesitter/nvim-treesitter

Nvim Treesitter configurations and abstraction layer
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
*/

use crate::{
    lua_spec,
    utils::constants::NVIM_TREESITTER_HOME, //
};

pub struct Plugin;
impl Plugin {
    pub fn spec() -> crate::plugins::spec::SpecInfo {
        let nvim_treesitter_home: &str = &NVIM_TREESITTER_HOME;
        lua_spec!("treesitter.lua", &[("NVIM_TS_HOME", nvim_treesitter_home)])
    }
}
