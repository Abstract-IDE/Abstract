/*
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: SchemaStore.nvim
Source: https://github.com/b0o/SchemaStore.nvim

A Neovim plugin that provides the SchemaStore
catalog for use with jsonls and yamlls.
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
*/

use crate::lua_spec;

pub struct Plugin;

impl Plugin {
    pub fn spec() -> crate::plugins::spec::SpecInfo {
        lua_spec!(raw r#"{
            "b0o/schemastore.nvim",
            lazy = true,
        }"#)
    }
}
