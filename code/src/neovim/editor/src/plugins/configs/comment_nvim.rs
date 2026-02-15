/*
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: Comment.nvim
Source: https://github.com/numToStr/Comment.nvim

🧠 💪 // Smart and powerful comment plugin for neovim.
Supports treesitter, dot repeat, left-right/up-down motions, hooks, and more
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
*/

use crate::{lua_section, lua_spec};

pub struct Plugin;

impl Plugin {
    pub fn spec() -> crate::plugins::spec::SpecInfo {
        lua_spec!(
            "comment_nvim.lua",
            &[
                ("TS_CONTEXT_COMMENTSTRING", lua_section!("ts-context-commentstring.lua", "spec")),
                ("HOOK_SETUP", lua_section!("ts-context-commentstring.lua", "setup")),
            ]
        )
    }
}
