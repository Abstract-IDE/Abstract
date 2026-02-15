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

use crate::{lua_file, lua_spec, plugins::spec::extract_section_tracked};

pub struct Plugin;

impl Plugin {
    pub fn spec() -> crate::plugins::spec::SpecInfo {
        let ts_cc = lua_file!("ts_context_commentstring.lua");
        lua_spec!(
            lua_file!("comment_nvim.lua"),
            &[
                ("TS_CONTEXT_COMMENTSTRING", extract_section_tracked(ts_cc, "spec")),
                ("HOOK_SETUP", extract_section_tracked(ts_cc, "setup")),
            ]
        )
    }
}
