/*
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: hop.nvim
Source: https://github.com/smoka7/hop.nvim
        (forked of: https://github.com/phaazon/hop.nvim)

Hop is an EasyMotion-like plugin allowing you to jump
anywhere in a document with as few keystrokes as possible
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
*/

use crate::core::keymaps::{
    MAPPING, //
    Key,
};

pub struct Plugin;

impl Plugin {
    pub fn spec() -> &'static str {
        let spec = format!(
            r#"{{
                "smoka7/hop.nvim",
                version = "*",
                keys={},
                opts = {{
                    keys = "qwertyuiopasdfghjklzxcvbnm",
                    jump_on_sole_occurrence = false,
                }},
            }}"#,
            MAPPING.get_map(Key::Hop)
        );

        Box::leak(spec.into_boxed_str())
    }
}
