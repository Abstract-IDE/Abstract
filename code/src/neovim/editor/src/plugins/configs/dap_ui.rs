/*
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: nvim-dap-ui
Source: https://github.com/rcarriga/nvim-dap-ui

A UI for nvim-dap which provides a good out of the box configuration.
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
*/

pub struct Plugin;

impl Plugin {
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
}

impl Plugin {
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
