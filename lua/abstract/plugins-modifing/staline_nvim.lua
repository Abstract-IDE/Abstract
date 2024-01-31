
 -- Lsp server name .
function lspservername()
    local msg = ''
    local buf_ft = vim.api.nvim_buf_get_option(0, 'filetype')
    local clients = vim.lsp.get_active_clients()
    if next(clients) == nil then return msg end
    for _, client in ipairs(clients) do
      local filetypes = client.config.filetypes
      if filetypes and vim.fn.index(filetypes, buf_ft) ~= -1 then
        return "LSP[" .. client.name .. "]"
      end
    end
    return msg
end






require('staline').setup {
    defaults = {
        left_separator   = "",
        right_separator  = "",
        line_column      = "[%l:%c /%L] 並%p%%", -- `:h stl` to see all flags.
        fg               = "#000000",  -- Foreground text color.
        bg               = "#031118",     -- Default background is transparent.
        cool_symbol      = " ",       -- Change this to override defult OS icon.
        full_path        = false,
        filename_section = '%{luaeval("lspservername()")}',

    },
    mode_colors = {
        n = "#25B2BC",
        i = "#859f96",
        c = "#e27d60",
        v = "#4799eb",
    },
    mode_icons = {
        n = " ",
        i = " ",
        c = " ",
        v = " ",
    },
}
