--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: nvim-dap
Source: https://github.com/mfussenegger/nvim-dap
Debug Adapter Protocol client implementation for Neovim
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
    "mfussenegger/nvim-dap",
    dependencies = {
        --[[
        ────────────────────────────────────────────────
        Plugin: nvim-dap-ui
        Source: https://github.com/rcarriga/nvim-dap-ui
        A UI for nvim-dap which provides a good out of the box configuration.
        ────────────────────────────────────────────────
        --]]
        {
            "rcarriga/nvim-dap-ui",
            config = function()
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
            end,
        },
        --[[
        ────────────────────────────────────────────────
        Plugin: nvim-dap-virtual-text
        Source: https://github.com/theHamsta/nvim-dap-virtual-text
        This plugin adds virtual text support to nvim-dap.
        ────────────────────────────────────────────────
        --]]
        {
            "theHamsta/nvim-dap-virtual-text",
            virt_text_pos = 'eol',
        },
    },
}

spec.config = function()
    --[[@rs $MAPPING_SET ]]
end

return spec
