/*
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: code_runner.nvim
Source: https://github.com/CRAG666/code_runner.nvim

Neovim plugin.The best code runner you could have,
it is like the one in vscode but with super powers,
it manages projects like in intellij but without being slow
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
*/

use crate::core::keymaps;
use crate::lua_spec;

pub struct Plugin;

impl Plugin {
    pub fn spec() -> crate::plugins::spec::SpecInfo {
        lua_spec!(format!(
            r#"{{
                "CRAG666/code_runner.nvim",
                event = "BufRead",
                keys = {},
                opts = {},
                config = function() {} end,
            }}"#,
            keymaps::MAPPING.get_map(keymaps::Key::CodeRunner),
            Self::opts(),
            keymaps::MAPPING.set_map_str(keymaps::Key::CodeRunner)
        ).leak())
    }
}

impl Plugin {
    pub fn opts() -> &'static str {
        r#"{
            filetype = {
                java = {
                    "cd $dir &&",
                    "javac $fileName &&",
                    "java $fileNameWithoutExt",
                },
                python = "python3 -u",
                typescript = "deno run",
                rust = {
                    "cd $dir &&",
                    "rustc $fileName &&",
                    "$dir/$fileNameWithoutExt",
                },
                c = function(...)
                    c_base = {
                        "cd $dir &&",
                        "gcc $fileName -o",
                        "/tmp/$fileNameWithoutExt",
                    }
                    local c_exec = {
                        "&& /tmp/$fileNameWithoutExt &&",
                        "rm /tmp/$fileNameWithoutExt",
                    }
                    vim.ui.input({ prompt = "Add more args:" }, function(input)
                        c_base[4] = input
                        vim.print(vim.tbl_extend("force", c_base, c_exec))
                        require("code_runner.commands").run_from_fn(vim.list_extend(c_base, c_exec))
                    end)
                end,
            },
        }"#
    }
}
