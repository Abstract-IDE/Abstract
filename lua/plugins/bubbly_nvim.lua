vim.g.bubbly_tabline = 1
vim.g.bubbly_statusline = {
  'mode',
  'path',
  'filetype',

  'divisor',
  'progress',
}


vim.g.bubbly_palette = {
  background = "#1B1717",
  foreground = "Black",
  black = "Black",
  red = "Red",
  green = "Green",
  yellow = "Yellow",
  blue = "Blue",
  purple = "Magenta",
  cyan = "Cyan",
  white = "White",
  lightgrey = "LightGrey",
  darkgrey = "Grey",
}


vim.g.bubbly_characters = {
  -- Bubble delimiters
  left = '',
  right = '',
  -- Close character for the tabline
  close = 'x',
  -- Bubble separators
  bubble_separator = '',
}



vim.g.bubbly_symbols = {
  default = 'PANIC!',

  path = {
    readonly = 'RO',
    unmodifiable = '',
    modified = '*',
  },
  signify = {
    added = '+%s', -- requires 1 '%s'
    modified = '~%s', -- requires 1 '%s'
    removed = '-%s', -- requires 1 '%s'
  },
  coc = {
    error = 'E%s', -- requires 1 '%s'
    warning = 'W%s', -- requires 1 '%s'
  },
  builtinlsp = {
    diagnostic_count = {
      error = 'E%s', -- requires 1 '%s'
      warning = 'W%s', --requires 1 '%s'
    },
  },
  branch = ' %s', -- requires 1 '%s'
  total_buffer_number = '﬘ %s', --requires 1 '%d'
  lsp_status = {
    diagnostics = {
      error = 'E%d',
      warning = 'W%d',
      hint = 'H%d',
      info = 'I%d',
    },
  },
}





vim.g.bubbly_colors = {
  default = 'red',

  mode = {
    normal = 'Blue', -- uses by default 'background' as the foreground color.
    insert = 'Red',
    visual = 'White',
    visualblock = 'red',
    command = 'red',
    terminal = 'blue',
    replace = 'yellow',
    default = 'white'
  },
  path = {
    readonly = { background = 'lightgrey', foreground = 'foreground' },
    unmodifiable = { background = 'darkgrey', foreground = 'foreground' },
    path = 'white',
    modified = { background = 'lightgrey', foreground = 'foreground' },
  },
  branch = 'purple',
  signify = {
    added = 'green',
    modified = 'blue',
    removed = 'red',
  },
  paste = 'red',
  builtinlsp = {
    diagnostic_count = {
      error = 'red',
      warning = 'yellow',
    },
    current_function = 'purple',
  },
  filetype = 'blue',
  progress = {
    rowandcol = { background = 'lightgrey', foreground = 'foreground' },
    percentage = { background = 'darkgrey', foreground = 'foreground' },
  },
  tabline = {
    active = 'blue',
    inactive = 'white',
    close = 'darkgrey',
  },
  total_buffer_number = 'cyan',
  lsp_status = {
    messages = 'white',
    diagnostics = {
      error = 'red',
      warning = 'yellow',
      hint = 'white',
      info = 'blue',
    },
  },
}
