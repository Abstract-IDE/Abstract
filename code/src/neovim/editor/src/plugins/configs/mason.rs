/*
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: mason.nvim
Source: https://github.com/mason-org/mason.nvim

Git Graph plugin for neovim.
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
*/

use crate::{
    core::lsp::Lsp,
    lua_file,
    lua_spec,
    plugins::spec::extract_section_tracked, //
};

pub struct Plugin;

impl Plugin {
    pub fn spec() -> crate::plugins::spec::SpecInfo {
        let setup_lsp = Lsp::setup().unwrap_or("");

        let none_ls = lua_file!("none_ls.lua");
        let lspconfig = lua_file!("mason_lspconfig.lua");
        let mason_null_ls = lua_file!("mason_null_ls.lua");
        let nvim_dap = lua_file!("mason_nvim_dap.lua");

        lua_spec!(
            lua_file!("mason.lua"),
            &[
                // specs
                ("NONE_LS_SPEC", extract_section_tracked(none_ls, "spec")),
                ("LSPCONFIG_SPEC", extract_section_tracked(lspconfig, "spec")),
                ("NULL_LS_SPEC", extract_section_tracked(mason_null_ls, "spec")),
                ("NVIM_DAP_SPEC", extract_section_tracked(nvim_dap, "spec")),
                // setups
                ("LSPCONFIG_SETUP", extract_section_tracked(lspconfig, "setup")),
                ("LSP_SETUP", setup_lsp),
                ("MASON_NULL_LS_SETUP", extract_section_tracked(mason_null_ls, "setup")),
                ("NONE_LS_SETUP", extract_section_tracked(none_ls, "setup")),
                ("NVIM_DAP_SETUP", extract_section_tracked(nvim_dap, "setup")),
            ]
        )
    }
}
