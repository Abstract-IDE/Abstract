/*
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: nvim-nio
Github: https://github.com/nvim-neotest/nvim-nio

A library for asynchronous IO in Neovim, inspired by
the asyncio library in Python. The library focuses
on providing both common asynchronous primitives and
asynchronous APIs for Neovim's core.
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
*/

pub struct Plugin;

impl Plugin {
    pub fn spec() -> &'static str {
        r#"{
            "nvim-neotest/nvim-nio",
            version = "*",
            lazy = true
        }"#
    }
}
