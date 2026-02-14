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
    lua_file,
    lua_spec,
    utils::constants::NVIM_TREESITTER_HOME, //
};

pub struct Plugin;
impl Plugin {
    pub fn spec() -> crate::plugins::spec::SpecInfo {
        let nvim_treesitter_home: &str = &NVIM_TREESITTER_HOME;
        lua_spec!(
            //
            lua_file!("treesitter.lua"),
            &[("NVIM_TS_HOME", nvim_treesitter_home)]
        )
    }
}
