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
    core::keymaps::{Key, MAPPING},
    lua_section, lua_spec,
};

pub struct Plugin;
impl Plugin {
    pub fn spec() -> crate::plugins::spec::SpecInfo {
        lua_spec!(
            "mason.lua",
            &[
                // specs
                ("NONE_LS_SPEC", lua_section!("none_ls.lua", "spec")),
                ("LSPCONFIG_SPEC", lua_section!("mason_lspconfig.lua", "spec")),
                ("NULL_LS_SPEC", lua_section!("mason_null_ls.lua", "spec")),
                ("NVIM_DAP_SPEC", lua_section!("mason_nvim_dap.lua", "spec")),
                // setups
                (
                    "LSP_SETUP",
                    lua_section!("../../core/lsp.lua", "setup", &[("LSP_MAPPING", MAPPING.get_map(Key::LspConfig))])
                ),
                ("LSPCONFIG_SETUP", lua_section!("mason_lspconfig.lua", "setup")),
                ("MASON_NULL_LS_SETUP", lua_section!("mason_null_ls.lua", "setup")),
                ("NONE_LS_SETUP", lua_section!("none_ls.lua", "setup")),
                ("NVIM_DAP_SETUP", lua_section!("mason_nvim_dap.lua", "setup")),
            ]
        )
    }
}
