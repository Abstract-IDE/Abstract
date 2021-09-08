-- Automatically generated packer.nvim plugin loader code

if vim.api.nvim_call_function('has', {'nvim-0.5'}) ~= 1 then
  vim.api.nvim_command('echohl WarningMsg | echom "Invalid Neovim version for packer.nvim! | echohl None"')
  return
end

vim.api.nvim_command('packadd packer.nvim')

local no_errors, error_msg = pcall(function()

  local time
  local profile_info
  local should_profile = false
  if should_profile then
    local hrtime = vim.loop.hrtime
    profile_info = {}
    time = function(chunk, start)
      if start then
        profile_info[chunk] = hrtime()
      else
        profile_info[chunk] = (hrtime() - profile_info[chunk]) / 1e6
      end
    end
  else
    time = function(chunk, start) end
  end
  
local function save_profiles(threshold)
  local sorted_times = {}
  for chunk_name, time_taken in pairs(profile_info) do
    sorted_times[#sorted_times + 1] = {chunk_name, time_taken}
  end
  table.sort(sorted_times, function(a, b) return a[2] > b[2] end)
  local results = {}
  for i, elem in ipairs(sorted_times) do
    if not threshold or threshold and elem[2] > threshold then
      results[i] = elem[1] .. ' took ' .. elem[2] .. 'ms'
    end
  end

  _G._packer = _G._packer or {}
  _G._packer.profile_output = results
end

time([[Luarocks path setup]], true)
local package_path_str = "/home/lazy/.cache/nvim/packer_hererocks/2.0.5/share/lua/5.1/?.lua;/home/lazy/.cache/nvim/packer_hererocks/2.0.5/share/lua/5.1/?/init.lua;/home/lazy/.cache/nvim/packer_hererocks/2.0.5/lib/luarocks/rocks-5.1/?.lua;/home/lazy/.cache/nvim/packer_hererocks/2.0.5/lib/luarocks/rocks-5.1/?/init.lua"
local install_cpath_pattern = "/home/lazy/.cache/nvim/packer_hererocks/2.0.5/lib/lua/5.1/?.so"
if not string.find(package.path, package_path_str, 1, true) then
  package.path = package.path .. ';' .. package_path_str
end

if not string.find(package.cpath, install_cpath_pattern, 1, true) then
  package.cpath = package.cpath .. ';' .. install_cpath_pattern
end

time([[Luarocks path setup]], false)
time([[try_loadstring definition]], true)
local function try_loadstring(s, component, name)
  local success, result = pcall(loadstring(s))
  if not success then
    vim.schedule(function()
      vim.api.nvim_notify('packer.nvim: Error running ' .. component .. ' for ' .. name .. ': ' .. result, vim.log.levels.ERROR, {})
    end)
  end
  return result
end

time([[try_loadstring definition]], false)
time([[Defining packer_plugins]], true)
_G.packer_plugins = {
  LuaSnip = {
    config = { " require('plugins/luasnip') " },
    loaded = true,
    path = "/home/lazy/.local/share/nvim/site/pack/packer/start/LuaSnip"
  },
  ["awesome-flutter-snippets"] = {
    loaded = true,
    path = "/home/lazy/.local/share/nvim/site/pack/packer/start/awesome-flutter-snippets"
  },
  ["cmp-buffer"] = {
    loaded = true,
    path = "/home/lazy/.local/share/nvim/site/pack/packer/start/cmp-buffer"
  },
  ["cmp-nvim-lsp"] = {
    loaded = true,
    path = "/home/lazy/.local/share/nvim/site/pack/packer/start/cmp-nvim-lsp"
  },
  ["cmp-path"] = {
    loaded = true,
    path = "/home/lazy/.local/share/nvim/site/pack/packer/start/cmp-path"
  },
  cmp_luasnip = {
    loaded = true,
    path = "/home/lazy/.local/share/nvim/site/pack/packer/start/cmp_luasnip"
  },
  ["dart-vim-plugin"] = {
    config = { " require('plugins/dart_vim_plugin') " },
    loaded = true,
    path = "/home/lazy/.local/share/nvim/site/pack/packer/start/dart-vim-plugin"
  },
  ["flutter-tools.nvim"] = {
    config = { " require('plugins/fluttertools') " },
    loaded = true,
    path = "/home/lazy/.local/share/nvim/site/pack/packer/start/flutter-tools.nvim"
  },
  ["friendly-snippets"] = {
    loaded = true,
    path = "/home/lazy/.local/share/nvim/site/pack/packer/start/friendly-snippets"
  },
  ["gitsigns.nvim"] = {
    config = { " require('plugins/gitsigns_nvim') " },
    loaded = true,
    path = "/home/lazy/.local/share/nvim/site/pack/packer/start/gitsigns.nvim"
  },
  indentLine = {
    config = { " require('plugins/indentLine') " },
    loaded = true,
    path = "/home/lazy/.local/share/nvim/site/pack/packer/start/indentLine"
  },
  kommentary = {
    config = { " require('plugins/kommentary') " },
    loaded = true,
    path = "/home/lazy/.local/share/nvim/site/pack/packer/start/kommentary"
  },
  ["lsp-status.nvim"] = {
    config = { " require('plugins/lspstatus') " },
    loaded = true,
    path = "/home/lazy/.local/share/nvim/site/pack/packer/start/lsp-status.nvim"
  },
  ["lspkind-nvim"] = {
    config = { " require('plugins/lspkind') " },
    loaded = true,
    path = "/home/lazy/.local/share/nvim/site/pack/packer/start/lspkind-nvim"
  },
  ["nvim-autopairs"] = {
    config = { " require('plugins/autopairs') " },
    loaded = true,
    path = "/home/lazy/.local/share/nvim/site/pack/packer/start/nvim-autopairs"
  },
  ["nvim-bufferline.lua"] = {
    config = { " require('plugins/bufferline_nvim') " },
    loaded = true,
    path = "/home/lazy/.local/share/nvim/site/pack/packer/start/nvim-bufferline.lua"
  },
  ["nvim-cmp"] = {
    config = { " require('plugins/cmp') " },
    loaded = true,
    path = "/home/lazy/.local/share/nvim/site/pack/packer/start/nvim-cmp"
  },
  ["nvim-colorizer.lua"] = {
    config = { " require('plugins/colorizer') " },
    loaded = true,
    path = "/home/lazy/.local/share/nvim/site/pack/packer/start/nvim-colorizer.lua"
  },
  ["nvim-lspconfig"] = {
    config = { " require('plugins/lspconfig') " },
    loaded = true,
    path = "/home/lazy/.local/share/nvim/site/pack/packer/start/nvim-lspconfig"
  },
  ["nvim-lspinstall"] = {
    config = { " require('plugins/lspinstall') " },
    loaded = true,
    path = "/home/lazy/.local/share/nvim/site/pack/packer/start/nvim-lspinstall"
  },
  ["nvim-tree.lua"] = {
    config = { " require('plugins/tree_nvim') " },
    loaded = true,
    path = "/home/lazy/.local/share/nvim/site/pack/packer/start/nvim-tree.lua"
  },
  ["nvim-web-devicons"] = {
    config = { " require('plugins/webdevicons_nvim') " },
    loaded = true,
    path = "/home/lazy/.local/share/nvim/site/pack/packer/start/nvim-web-devicons"
  },
  ["packer.nvim"] = {
    loaded = true,
    path = "/home/lazy/.local/share/nvim/site/pack/packer/start/packer.nvim"
  },
  ["plenary.nvim"] = {
    loaded = true,
    path = "/home/lazy/.local/share/nvim/site/pack/packer/start/plenary.nvim"
  },
  ["popup.nvim"] = {
    loaded = true,
    path = "/home/lazy/.local/share/nvim/site/pack/packer/start/popup.nvim"
  },
  ["rooter.nvim"] = {
    config = { " require('plugins/rooter_nvim') " },
    loaded = true,
    path = "/home/lazy/.local/share/nvim/site/pack/packer/start/rooter.nvim"
  },
  ["surround.nvim"] = {
    config = { " require('plugins/surround_nvim') " },
    loaded = true,
    path = "/home/lazy/.local/share/nvim/site/pack/packer/start/surround.nvim"
  },
  ["telescope-fzf-native.nvim"] = {
    loaded = true,
    path = "/home/lazy/.local/share/nvim/site/pack/packer/start/telescope-fzf-native.nvim"
  },
  ["telescope.nvim"] = {
    config = { " require('plugins/telescope') " },
    loaded = true,
    path = "/home/lazy/.local/share/nvim/site/pack/packer/start/telescope.nvim"
  },
  ["trouble.nvim"] = {
    config = { " require('plugins/trouble_nvim') " },
    loaded = true,
    path = "/home/lazy/.local/share/nvim/site/pack/packer/start/trouble.nvim"
  },
  ["vim-floaterm"] = {
    config = { " require('plugins/floaterm') " },
    loaded = true,
    path = "/home/lazy/.local/share/nvim/site/pack/packer/start/vim-floaterm"
  },
  ["vim-maximizer"] = {
    config = { " require('plugins/maximizer') " },
    loaded = true,
    path = "/home/lazy/.local/share/nvim/site/pack/packer/start/vim-maximizer"
  }
}

time([[Defining packer_plugins]], false)
-- Config for: surround.nvim
time([[Config for surround.nvim]], true)
 require('plugins/surround_nvim') 
time([[Config for surround.nvim]], false)
-- Config for: nvim-lspconfig
time([[Config for nvim-lspconfig]], true)
 require('plugins/lspconfig') 
time([[Config for nvim-lspconfig]], false)
-- Config for: nvim-cmp
time([[Config for nvim-cmp]], true)
 require('plugins/cmp') 
time([[Config for nvim-cmp]], false)
-- Config for: indentLine
time([[Config for indentLine]], true)
 require('plugins/indentLine') 
time([[Config for indentLine]], false)
-- Config for: trouble.nvim
time([[Config for trouble.nvim]], true)
 require('plugins/trouble_nvim') 
time([[Config for trouble.nvim]], false)
-- Config for: dart-vim-plugin
time([[Config for dart-vim-plugin]], true)
 require('plugins/dart_vim_plugin') 
time([[Config for dart-vim-plugin]], false)
-- Config for: kommentary
time([[Config for kommentary]], true)
 require('plugins/kommentary') 
time([[Config for kommentary]], false)
-- Config for: nvim-autopairs
time([[Config for nvim-autopairs]], true)
 require('plugins/autopairs') 
time([[Config for nvim-autopairs]], false)
-- Config for: flutter-tools.nvim
time([[Config for flutter-tools.nvim]], true)
 require('plugins/fluttertools') 
time([[Config for flutter-tools.nvim]], false)
-- Config for: nvim-bufferline.lua
time([[Config for nvim-bufferline.lua]], true)
 require('plugins/bufferline_nvim') 
time([[Config for nvim-bufferline.lua]], false)
-- Config for: telescope.nvim
time([[Config for telescope.nvim]], true)
 require('plugins/telescope') 
time([[Config for telescope.nvim]], false)
-- Config for: vim-maximizer
time([[Config for vim-maximizer]], true)
 require('plugins/maximizer') 
time([[Config for vim-maximizer]], false)
-- Config for: nvim-colorizer.lua
time([[Config for nvim-colorizer.lua]], true)
 require('plugins/colorizer') 
time([[Config for nvim-colorizer.lua]], false)
-- Config for: lsp-status.nvim
time([[Config for lsp-status.nvim]], true)
 require('plugins/lspstatus') 
time([[Config for lsp-status.nvim]], false)
-- Config for: lspkind-nvim
time([[Config for lspkind-nvim]], true)
 require('plugins/lspkind') 
time([[Config for lspkind-nvim]], false)
-- Config for: rooter.nvim
time([[Config for rooter.nvim]], true)
 require('plugins/rooter_nvim') 
time([[Config for rooter.nvim]], false)
-- Config for: gitsigns.nvim
time([[Config for gitsigns.nvim]], true)
 require('plugins/gitsigns_nvim') 
time([[Config for gitsigns.nvim]], false)
-- Config for: nvim-tree.lua
time([[Config for nvim-tree.lua]], true)
 require('plugins/tree_nvim') 
time([[Config for nvim-tree.lua]], false)
-- Config for: nvim-web-devicons
time([[Config for nvim-web-devicons]], true)
 require('plugins/webdevicons_nvim') 
time([[Config for nvim-web-devicons]], false)
-- Config for: vim-floaterm
time([[Config for vim-floaterm]], true)
 require('plugins/floaterm') 
time([[Config for vim-floaterm]], false)
-- Config for: LuaSnip
time([[Config for LuaSnip]], true)
 require('plugins/luasnip') 
time([[Config for LuaSnip]], false)
-- Config for: nvim-lspinstall
time([[Config for nvim-lspinstall]], true)
 require('plugins/lspinstall') 
time([[Config for nvim-lspinstall]], false)
if should_profile then save_profiles() end

end)

if not no_errors then
  vim.api.nvim_command('echohl ErrorMsg | echom "Error in packer_compiled: '..error_msg..'" | echom "Please check your config for correctness" | echohl None')
end
