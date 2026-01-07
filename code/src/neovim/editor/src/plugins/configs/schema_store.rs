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

pub struct Plugin;

impl Plugin {
    pub fn spec() -> &'static str {
        r#"{
            "b0o/schemastore.nvim",
            lazy = true,
        }"#
    }
}
