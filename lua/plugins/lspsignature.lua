--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--
--─────────────────────────────────────────────────--
--   Plugin:    lsp_signature.nvim
--
--   Github:    github.com/rax/lsp_signature.nvim
--─────────────────────────────────────────────────--
--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--





--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━━━❰ configs ❱━━━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--




 cfg = {
  bind = true, -- This is mandatory, otherwise border config won't get registered.
               -- If you want to hook lspsaga or other signature handler, pls set to false
  doc_lines = 2, -- will show two lines of comment/doc(if there are more than two lines in doc, will be truncated);
                 -- set to 0 if you DO NOT want any API comments be shown
                 -- This setting only take effect in insert mode, it does not affect signature help in normal
                 -- mode, 10 by default

  floating_window   = true, -- show hint in a floating window, set to false for virtual text only mode
  fix_pos           = false,  -- set to true, the floating window will not auto-close until finish all parameters
  hint_enable       = true, -- virtual hint enable
  hint_prefix       = "🐼 ",  -- Panda for parameter
  hint_scheme       = "String",
  use_lspsaga       = false,  -- set to true if you want to use lspsaga popup
  hi_parameter      = "Search", -- how your parameter will be highlight
  max_height        = 12, -- max height of signature floating_window, if content is more than max_height, you can scroll down
                       -- to view the hiding contents
  max_width         = 60, -- max_width of signature floating_window, line will be wrapped if exceed max_width
  handler_opts      = {
    border          = "shadow"   -- double, single, shadow, none
  },
  extra_trigger_chars = {} -- Array of extra characters that will trigger signature completion, e.g., {"(", ","}
  -- deprecate !!
  -- decorator = {"`", "`"}  -- this is no longer needed as nvim give me a handler and it allow me to highlight active parameter in floating_window

}

require'lsp_signature'.on_attach(cfg)




--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━❰ end configs ❱━━━━━━━━━━━━━━━━━--
--━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━--

