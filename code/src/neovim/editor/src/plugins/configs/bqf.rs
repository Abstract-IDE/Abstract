/*
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
────────────────────────────────────────────────
Plugin: nvim.bqf
Source: https://github.com/kevinhwang91/nvim-bqf

Better quickfix window in Neovim, polish old quickfix window.
────────────────────────────────────────────────
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
*/

pub struct Plugin;

impl Plugin {
    pub fn spec() -> &'static str {
        let opts = Self::opts();
        let spec = format!(
            // language=lua
            r#"{{
                'nvim-treesitter/nvim-treesitter',
                lazy = false,
                build = ':TSUpdate',
                opts = {opts},
            }}"#
        );

        Box::leak(spec.into_boxed_str())
    }
}

impl Plugin {
    pub fn opts() -> &'static str {
        // language=lua
        r#"{
            auto_enable = true,
            auto_resize_height = true, -- highly recommended enable
            preview = {
                delay_syntax = 400,
                show_title = true,
                -- should_preview_cb = function(bufnr, qwinid)
                -- 	local ret = true
                -- 	local bufname = vim.api.nvim_buf_get_name(bufnr)
                -- 	local fsize = vim.fn.getfsize(bufname)
                -- 	if fsize > 100 * 1024 then
                -- 		-- skip file size greater than 100k
                -- 		ret = false
                -- 	elseif bufname:match("^fugitive://") then
                -- 		-- skip fugitive buffer
                -- 		ret = false
                -- 	end
                -- 	return ret
                -- end,
            },
            -- make `drop` and `tab drop` to become preferred
            func_map = {
                drop = "o",
                openc = "O",
                split = "<C-s>",
                tabdrop = "<C-t>",
                -- set to empty string to disable
                tabc = "",
                ptogglemode = "z,",
            },
            filter = {
                fzf = {
                    action_for = { ["ctrl-s"] = "split", ["ctrl-t"] = "tab drop" },
                    extra_opts = { "--bind", "ctrl-o:toggle-all", "--prompt", "> " },
                },
            },
        }"#
    }
}
