/*
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: nvim-dap
Source: https://github.com/mfussenegger/nvim-dap

Debug Adapter Protocol client implementation for Neovim
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
*/

use crate::{
    core::keymaps,
    lua_spec,
    plugins::spec::SpecInfo, //
};

pub struct Plugin;

impl Plugin {
    pub fn spec() -> SpecInfo {
        keymaps::MAPPING.signal(keymaps::Key::Dap);

        let config_dap_ui = PluginDapUi::spec();
        let config_dap_virtual_text = PluginDapVirtualText::spec();

        // language=lua
        lua_spec!(
            format!(
                r#"{{
            "mfussenegger/nvim-dap",
            dependencies = {{
                {config_dap_ui},
                {config_dap_virtual_text},
            }},
        }}"#
            )
            .leak()
        )
    }
}

/*
────────────────────────────────────────────────
Plugin: nvim-dap-ui
Source: https://github.com/rcarriga/nvim-dap-ui

A UI for nvim-dap which provides a good out of
the box configuration.
────────────────────────────────────────────────
*/
struct PluginDapUi;
impl PluginDapUi {
    pub fn spec() -> &'static str {
        let config = Self::config();
        format!(
            // language=lua
            r#"{{
                "rcarriga/nvim-dap-ui",
                config = {config},
            }}"#
        )
        .leak()
    }

    pub fn config() -> &'static str {
        // language=lua
        r#"function()
            local dapui = require("dapui")
            dapui.setup()

            local dap = require("dap")
            -- nvim-dap events to open and close the windows automatically (:help dap-extensions)
            dap.listeners.before.attach.dapui_config = function()
                dapui.open()
            end
            dap.listeners.before.launch.dapui_config = function()
                dapui.open()
            end
            dap.listeners.before.event_terminated.dapui_config = function()
                dapui.close()
            end
            dap.listeners.before.event_exited.dapui_config = function()
                dapui.close()
            end
        end"#
    }
}

/*
────────────────────────────────────────────────
Plugin: nvim-dap-virtual-text
Source: https://github.com/theHamsta/nvim-dap-virtual-text

This plugin adds virtual text support to nvim-dap.
────────────────────────────────────────────────
*/
struct PluginDapVirtualText;
impl PluginDapVirtualText {
    pub fn spec() -> &'static str {
        r#"{
            "theHamsta/nvim-dap-virtual-text",
            virt_text_pos = 'eol',
        }"#
    }
}
