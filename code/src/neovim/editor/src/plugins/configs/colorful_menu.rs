/*
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: colorful-menu.nvim
Source: https://github.com/xzbdmw/colorful-menu.nvim

Bring enjoyment to your auto completion.
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
*/

pub struct Plugin;

impl Plugin {
    pub fn spec() -> &'static str {
        r#"{
            "xzbdmw/colorful-menu.nvim",
            lazy = true,
            opts = {
                -- If provided, the plugin truncates the final displayed text to
                -- this width (measured in display cells). Any highlights that extend
                -- beyond the truncation point are ignored. When set to a float
                -- between 0 and 1, it'll be treated as percentage of the width of
                -- the window: math.floor(max_width * vim.api.nvim_win_get_width(0))
                -- Default 60.
                max_width = 40,
            }
        }"#
    }
}
