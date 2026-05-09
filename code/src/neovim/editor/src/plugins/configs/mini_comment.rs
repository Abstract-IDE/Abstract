/*
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: mini.comment
Source: https://github.com/nvim-mini/mini.comment

Neovim Lua plugin for fast and familiar per-line commenting.
Part of 'mini.nvim' library.
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
*/

use crate::{lua_section, lua_spec};

pub struct Plugin;

impl Plugin {
    pub fn spec() -> crate::plugins::spec::SpecInfo {
        lua_spec!(
            "mini_comment.lua",
            &[
                ("TS_CONTEXT_COMMENTSTRING", lua_section!("ts-context-commentstring.lua", "spec")),
                ("HOOK_SETUP", lua_section!("ts-context-commentstring.lua", "setup")),
            ]
        )
    }
}
