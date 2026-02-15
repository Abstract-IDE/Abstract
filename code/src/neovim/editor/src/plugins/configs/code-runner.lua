--[[
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: code_runner.nvim
Source: https://github.com/CRAG666/code_runner.nvim

Neovim plugin. The best code runner you could have,
it is like the one in vscode but with super powers,
it manages projects like in intellij but without being slow
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
--]]

local spec = {
    "CRAG666/code_runner.nvim",
    event = "BufRead",
    cmd = "RunCode",
}

spec.config = function()
    --[[@rs $MAPPING_SET ]]
end

return spec
