/*
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
─────────────────────────────────────────────────
Plugin: csvview.nvim
Source: https://github.com/hat0uma/csvview.nvim

lightweight CSV file viewer plugin for Neovim.
With this plugin, you can easily view and edit CSV files within Neovim.
─────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
*/

pub struct Plugin;

impl Plugin {
    pub fn spec() -> &'static str {
        let opts = Self::opts();

        let spec = format!(
            r#"{{
                'hat0uma/csvview.nvim',
                lazy = true,
                ft = "csv",
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
            view = {
                --- minimum width of a column
                --- @type integer
                min_column_width = 5,

                --- spacing between columns
                --- @type integer
                spacing = 2,

                --- The display method of the delimiter
                --- "highlight" highlights the delimiter
                --- "border" displays the delimiter with `│`
                --- see `Features` section of the README.
                ---@type "highlight" | "border"
                display_mode = "highlight",
            },
        }"##
    }
}
