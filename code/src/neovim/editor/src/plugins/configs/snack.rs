/*
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: snacks.nvim
Source: github.com/folke/snacks.nvim

A collection of small QoL plugins for Neovim.
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
*/

use crate::{
    core::keymaps::{Key, MAPPING},
    lua_spec,
};

pub struct Plugin;

impl Plugin {
    pub fn spec() -> crate::plugins::spec::SpecInfo {
        MAPPING.signal(Key::Snacks);

        lua_spec!(
            "snack.lua",
            &[
                ("MAPPING_BUFDELETE", MAPPING.get_map(Key::SnacksBufdelete)),
                ("MAPPING_PICKER", MAPPING.get_map(Key::SnacksPicker)),
            ]
        )
    }
}
