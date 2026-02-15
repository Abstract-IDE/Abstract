/*
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: typst-preview.nvim
Source: https://github.com/chomosuke/typst-preview.nvim

The Neovim plugin for https://github.com/Myriad-Dreamin/tinymist.

💪 Features
- Low latency preview: preview your document instantly on type.
  The incremental rendering technique makes the preview latency as low as possible.
- Cross jump between code and preview. You can click on the preview to jump to the
  corresponding code location and have the preview follow your cursor in Neovim.
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
*/

use crate::lua_spec;

pub struct Plugin;

impl Plugin {
    pub fn spec() -> crate::plugins::spec::SpecInfo {
        let opts = Self::opts();

        lua_spec!(raw format!(
            // language=lua
            r#"{{
                "chomosuke/typst-preview.nvim",
                version = "1.*",
                lazy = true,
                ft = {{ "typst" }},
                event = {{ "LspAttach" }},
                opts = {opts},
            }}"#
        )
        .leak())
    }
}

impl Plugin {
    pub fn opts() -> &'static str {
        // language=lua
        r#"{

            -- Setting this true will enable logging debug information to
            -- `vim.fn.stdpath 'data' .. '/typst-preview/log.txt'`
            debug = false,

            -- Custom format string to open the output link provided with %s
            -- Example: open_cmd = 'firefox %s -P typst-preview --class typst-preview'
            open_cmd = nil,

            -- Custom port to open the preview server. Default is random.
            -- Example: port = 8000
            port = 36063,

            -- Setting this to 'always' will invert black and white in the preview
            -- Setting this to 'auto' will invert depending if the browser has enable
            -- dark mode
            -- Setting this to '{"rest": "<option>","image": "<option>"}' will apply
            -- your choice of color inversion to images and everything else
            -- separately.
            invert_colors = "never",

            -- Whether the preview will follow the cursor in the source file
            follow_cursor = true,

            -- Provide the path to binaries for dependencies.
            -- Setting this will skip the download of the binary by the plugin.
            -- Warning: Be aware that your version might be older than the one required.
            dependencies_bin = {
                ['tinymist'] = 'tinymist', -- should point towards the Mason installation of tinymist.
                -- ["tinymist"] = nil,
                ["websocat"] = nil,
            },

            -- A list of extra arguments (or nil) to be passed to previewer.
            -- For example, extra_args = { "--input=ver=draft", "--ignore-system-fonts" }
            extra_args = nil,

            -- This function will be called to determine the root of the typst project
            get_root = function(path_of_main_file)
                local root = os.getenv("TYPST_ROOT")
                if root then
                    return root
                end
                return vim.fn.fnamemodify(path_of_main_file, ":p:h")
            end,

            -- This function will be called to determine the main file of the typst
            -- project.
            get_main_file = function(path_of_buffer)
                return path_of_buffer
            end,
        }"#
    }
}
