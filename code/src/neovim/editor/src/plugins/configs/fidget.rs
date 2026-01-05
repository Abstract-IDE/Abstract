/*
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: fidget.nvim
Source: https://github.com/j-hui/fidget.nvim

💫 Extensible UI for Neovim notifications and LSP progress messages.
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
*/

pub struct Plugin;

impl Plugin {
    pub fn spec() -> &'static str {
        let opts = Self::opts();

        let spec = format!(
            r#"{{
                "j-hui/fidget.nvim",
                lazy = true,
                event = {{ "LspAttach" }},
                opts = {}
            }}"#,
            opts,
        );

        Box::leak(spec.into_boxed_str())
    }
}

impl Plugin {
    pub fn opts() -> &'static str {
        r##"{
            notification = {
                window = {
                    winblend = 100, -- Background color opacity in the notification window
                }
            }
        }"##
    }
}
