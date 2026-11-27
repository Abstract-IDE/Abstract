# Seek

A search hub for Abstract, built on ab-nui (`wp_ui`): project files, file
contents, open buffers and the lines of the current buffer — in one window,
one mode at a time.

Searching runs on background threads (ripgrep's `.gitignore`-aware directory
walker, fzf-style scoring in parallel), so typing stays responsive in large
projects. The preview is a real buffer, so it gets treesitter (or regex)
syntax highlighting.

## Opening

| Key           | Opens                                   |
| ------------- | --------------------------------------- |
| `<M-f>`       | files in the project (cwd)              |
| `<M-F>`       | files in the current file's directory   |
| `<M-b>`       | buffers                                 |
| `<M-g>g` / `<M-g><M-g>` | grep                          |
| `<M-g>w` / `<M-g><M-w>` | grep the selection / word under the cursor |
| `<M-g>l`      | lines of the current buffer             |

Or `:AbstractSeek [files|grep|buffers|lines] [query]`, or from Lua:

```lua
require("seek").open({ mode = "grep", query = "TODO", cwd = "~/src/project" })
require("seek").toggle({ mode = "files" })   -- open if closed, close if open
require("seek").grep_word()
require("seek").close()
```

Opening Seek **while it is open**:

- the same mode again (e.g. `<M-f>` twice — the keys work from the prompt's
  insert mode too): closes it, or with `on_reopen = "keep"` leaves it open;
- another mode or a query (e.g. `<M-g>g` while browsing files): switches the
  open window to it;
- another directory: starts afresh there.

`:AbstractSeekToggle [mode] [query]` / `toggle()` always open-or-close,
whatever `on_reopen` says.

## The window

```text
╭───────────────────────────────────────────────────────────────╮
│ ❯ query                    󰢱 src/engine.rs                    │  prompt · preview's file
├──────────────────────────┬────────────────────────────────────┤
│▌ src/engine.rs           ┃  1 //! The search engine…          │  results · preview
│  src/fuzzy.rs            │  2 …                               │  (scrollbar on the divider)
├──────────────────────────┴────────────────────────────────────┤
│ 12/200/1475                rust · 12.3 KiB · 245 lines · 3d ago│  selected/loaded/total · facts
╰───────────────────────────────────────────────────────────────╯
```

- The count is **selected / loaded / total**: results load a page at a time
  (`page_size`) as you move towards the end of what is loaded; `G` loads up to
  the last one.
- The empty prompt's placeholder names the mode ("Search files…").
- Seek is **modal**: the editor behind is dimmed and doesn't react to clicks
  or scrolling, and the cursor stays in the prompt.
- The touchpad/wheel moves through the results (over the list) and scrolls
  the preview (over it).
- Long paths lose characters on the left (`…/src/nvim/buffer.rs`) so the file
  name stays visible; on the selected row the matched characters and the
  file icon keep their colours.

## In the window

The prompt is a real one-line buffer: it opens in insert mode and supports
everything Vim does to text (`<Esc>` to normal mode, `dw`, `ciw`, `0`, `$`,
`u`…). Your own mappings apply in it too. The results and preview never take
the cursor; the touchpad/wheel scrolls the preview.

Insert **and** normal mode:

| Key                                | Does                                   |
| ---------------------------------- | -------------------------------------- |
| `<Tab>` `<Down>` `<C-j>` `<C-n>`   | next result                            |
| `<S-Tab>` `<Up>` `<C-k>`           | previous result                        |
| `<CR>`                             | open                                   |
| `<C-s>` / `<C-v>` / `<C-t>`        | open in a split / vsplit / tab         |
| `<C-q>`                            | send all results to the quickfix list  |
| `<C-d>` / `<C-u>`                  | scroll the preview                     |
| `<C-p>`                            | toggle the preview                     |
| `<C-z>`                            | maximize the preview                   |
| `<C-f>` `<C-g>` `<C-b>` `<C-l>`    | switch to files / grep / buffers / lines |
| `<C-c>`                            | close                                  |

Normal mode only:

| Key            | Does                    |
| -------------- | ----------------------- |
| `j` / `k`      | next / previous result  |
| `gg` / `G`     | first / last result     |
| `<Esc>` `q`    | close                   |

So `<Esc>` twice closes from insert mode, `<C-c>` closes at once.

Typing a **prefix** as the first character switches mode: `>` grep, `#`
buffers, `/` lines; **Backspace on the empty prompt goes back** to the default
mode (files). The prompt shows the mode's icon, and its placeholder shows the
way in (`> grep · # buffers · / lines`) or back (`⌫ files`). The query is kept
when switching.

Grep and line results open at the exact line and column; buffers switch to
the buffer.

**Lines** searches the current buffer when it holds a file (or a help page).
From a special buffer — a dashboard, a terminal, a file tree — it searches
the file buffer you used last instead (starting on the line you were on);
results show lines without their indentation.

## Query syntax (files, buffers, lines)

Words separated by spaces must all match.

| Term     | Matches                |
| -------- | ---------------------- |
| `abc`    | fuzzy: a…b…c in order  |
| `'abc`   | exact substring        |
| `^abc`   | starts with            |
| `abc$`   | ends with              |
| `!abc`   | does not contain       |

Smart case: a term with an uppercase letter is case-sensitive. Grep is a
literal, smart-case search by default (set `grep.regex = true` for regular
expressions).

## Configuration

Every option is optional:

```lua
require("seek").setup({
    layout = {
        width = 0.8,                 -- of the editor
        height = 0.8,
        preview_ratio = 0.6,         -- width share of the preview
        min_width_for_preview = 100, -- hide the preview in narrower editors
        preview = true,              -- show it initially
    },
    modes = { "files", "grep", "buffers", "lines" }, -- enabled, in this order
    default_mode = "files",
    prefixes = { grep = ">", buffers = "#", lines = "/" }, -- "" disables one
    page_size = 200,                 -- results loaded at a time; more load as you scroll
    files = {
        hidden = false,              -- include dotfiles
        follow = false,              -- follow symlinks
        gitignore = true,            -- honour .gitignore / .ignore / git excludes
        exclude = {},                -- extra globs, e.g. { "*.lock", "target/" }
    },
    grep = {
        regex = false,
        hidden = false, follow = false, gitignore = true, exclude = {},
        max_results = 10000,
        max_filesize = 1048576,      -- bytes; bigger files are skipped
        debounce_ms = 80,
    },
    preview = {
        max_filesize = 1048576, max_lines = 10000,
        line_numbers = false,
        -- Images (png, jpg, gif, webp, bmp, tiff, ico), video frames (with
        -- ffmpeg) and PDF pages (with pdftoppm or mutool) shown as pictures, in
        -- kitty / Ghostty (not tmux) with termguicolors. Abstract turns it on.
        media = false,
    },
    start_insert = true,             -- open the prompt in insert mode
    backspace_to_default = true,     -- <BS> on an empty prompt: back to default_mode
    on_reopen = "toggle",            -- opening it again while open: "toggle" (close) | "keep"
    -- Each action takes a key or a list of keys; `false` unbinds it.
    -- Actions: next prev first last open split vsplit tab quickfix close
    -- preview_down preview_up toggle_preview maximize files grep buffers lines
    keys = {                         -- insert and normal mode
        next = { "<Tab>", "<Down>", "<C-j>", "<C-n>" },
        prev = { "<S-Tab>", "<Up>", "<C-k>" },
        open = "<CR>", split = "<C-s>", vsplit = "<C-v>", tab = "<C-t>",
        quickfix = "<C-q>", close = "<C-c>",
        preview_down = "<C-d>", preview_up = "<C-u>",
        toggle_preview = "<C-p>", maximize = "<C-z>",
        files = "<C-f>", grep = "<C-g>", buffers = "<C-b>", lines = "<C-l>",
    },
    normal_keys = {                  -- normal mode only
        next = "j", prev = "k", first = "gg", last = "G", close = { "<Esc>", "q" },
    },
})
```

Options live in `vim.g.abstract_seek_opts`, so setting that before Abstract
starts works too.

Keys are buffer-local mappings on the prompt, so any `lhs` works (`<M-x>`,
`<leader>…` in normal mode). In most terminals `<C-m>`/`<C-i>`/`<C-[>` are the
same keys as `<CR>`/`<Tab>`/`<Esc>`.

## Highlights

All link to standard groups by default (`default = true`, so yours win):
`SeekMode` (Title), `SeekDir` (Comment), `SeekLnum`
(LineNr), `SeekCount` (Comment), `SeekModified` (WarningMsg). The preview's
line and matches use ab-nui's `AbNuiCodeLine` (CursorLine) and
`AbNuiCodeMatch` (Search); matched characters in the results are
`AbNuiMatch` (Special) — on the selected row, the bar's complementary
colour; the dimmed background is
`AbNuiBackdrop`.

## Layout of the code

| File          | What                                                         |
| ------------- | ------------------------------------------------------------ |
| `lib.rs`      | `:AbstractSeek`, `require("seek")`, setup                    |
| `session.rs`  | one open window: state, searches, widget tree, actions       |
| `engine.rs`   | background threads: matcher, file walker, grep              |
| `fuzzy.rs`    | fzf-style matcher and query syntax                           |
| `preview.rs`  | what a result shows (file, buffer, line), into a `CodeView`  |
| `actions.rs`  | opening results, quickfix                                    |
| `config.rs`   | options                                                      |
| `helpers.lua` | Neovim API helpers (buffers, filetype, jumps, quickfix)      |

All of the UI — the modal window, frame, prompt, preview, icons — comes from
ab-nui (`Surface`, `Frame`, `BufferInput`, `CodeView`, `icons`).
