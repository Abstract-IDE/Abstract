use std::path::PathBuf;

use nvim_oxi::{
    self,
    api, //
};

use crate::utils::api::{
    append_opt,
    listchars, //
    set_opt,
};

/// Plugin independent Configs
///
/// This struct handles all Neovim configuration that is independent of any plugin.
pub struct Config {}

impl Config {
    pub fn init() -> nvim_oxi::Result<()> {
        // ----------------------------
        // Leader keys
        // ----------------------------
        // To see the current mapping for |<Leader>|, type :echo mapleader.
        // If it reports an undefined variable, it means the leader key is set to the default of '\'.
        api::set_var("mapleader", " ")?;
        api::set_var("maplocalleader", "|")?;

        // ----------------------------
        // UI options
        // ----------------------------
        set_opt("termguicolors", true)?; // Enable GUI colors for the terminal to get truecolor
        set_opt("list", true)?; // show whitespace
        set_opt(
            "listchars",
            listchars(&[
                ("nbsp", "⦸"),     // CIRCLED REVERSE SOLIDUS (U+29B8)
                ("extends", "»"),  // RIGHT-POINTING DOUBLE ANGLE QUOTATION MARK (U+00BB)
                ("precedes", "«"), // LEFT-POINTING DOUBLE ANGLE QUOTATION MARK (U+00AB)
                ("tab", "➔ "),     // WHITE RIGHT-POINTING TRIANGLE + BOX DRAWINGS HEAVY TRIPLE DASH HORIZONTAL
                ("trail", "•"),    // BULLET (U+2022)
                ("space", " "),    // space
            ]),
        )?;
        set_opt(
            "fillchars",
            listchars(&[
                ("diff", "∙"), // BULLET OPERATOR (U+2219)
                ("eob", " "),  // NO-BREAK SPACE to suppress ~ at EndOfBuffer
                ("fold", "·"), // MIDDLE DOT (U+00B7)
                ("vert", "│"), // window border when window splits vertically
            ]),
        )?;

        // ----------------------------
        // Backup related options
        // ----------------------------

        // Resolve stdpath("data") like Lua's `vim.fn.stdpath("data")`
        let backup_dir = {
            let path: String = api::eval("stdpath('data')")?;
            PathBuf::from(path).join(".cache")
        };

        set_opt("backup", true)?; // make backups before writing
        set_opt("undofile", false)?; // persistent undos
        set_opt("writebackup", true)?; // Make backup before overwriting current buffer
        set_opt("backupcopy", "yes")?; // Overwrite the original backup file
        set_opt("directory", backup_dir.join("swap").to_string_lossy().as_ref())?; // swap files directory
        set_opt("backupdir", backup_dir.join("backedUP").to_string_lossy().as_ref())?; // backup files
        set_opt("undodir", backup_dir.join("undos").to_string_lossy().as_ref())?; // undo files
        set_opt("viewdir", backup_dir.join("view").to_string_lossy().as_ref())?; // :mkview storage
        set_opt("shada", format!("'100,<50,f50,n{}", backup_dir.join("shada/shada").to_string_lossy()))?;

        // ----------------------------
        // Clipboard
        // ----------------------------
        append_opt("clipboard", "unnamedplus")?; // copy & paste

        // ----------------------------
        // Wrapping / matching
        // ----------------------------
        set_opt("wrap", false)?; // don't automatically wrap on load
        set_opt("showmatch", true)?; // show matching part of pairs [] {} ()

        // ----------------------------
        // Cursor / search
        // ----------------------------
        set_opt("cursorline", true)?; // highlight current line
        set_opt("number", true)?; // show line numbers
        set_opt("relativenumber", true)?; // show relative line number
        set_opt("incsearch", true)?; // incremental search
        set_opt("hlsearch", true)?; // highlighted search results
        set_opt("ignorecase", true)?; // ignore case while searching
        set_opt("smartcase", true)?; // smartcase

        // ----------------------------
        // Scrolling
        // ----------------------------
        set_opt("scrolloff", 1)?; // keep cursor 1 line away from screen border
        set_opt("sidescrolloff", 2)?; // keep 2 columns visible left/right of cursor

        // ----------------------------
        // Editing behavior
        // ----------------------------
        set_opt("backspace", "indent,start,eol")?; // backspace behaves normally
        set_opt("mouse", "a")?; // enable mouse interaction
        set_opt("mousescroll", "ver:3,hor:2")?; // scroll speed
        set_opt("updatetime", 500)?; // CursorHold interval

        // ----------------------------
        // Tabs / indentation
        // ----------------------------
        set_opt("softtabstop", 4)?; // soft tab stops
        set_opt("shiftwidth", 4)?; // shift width
        set_opt("tabstop", 4)?; // tab width
        set_opt("smarttab", true)?; // <tab>/<BS> in leading whitespace
        set_opt("autoindent", true)?; // maintain indent of current line
        // set_opt("expandtab", false)?; // don't expand tabs into spaces (commented out)
        set_opt("shiftround", true)?; // round indent on shift

        // ----------------------------
        // Splits / layout
        // ----------------------------
        set_opt("splitbelow", true)?; // horizontal splits below current window
        set_opt("splitright", true)?; // vertical splits to right
        set_opt("laststatus", 3)?; // always show status line (3 = global status)
        // set_opt("colorcolumn", "79")?; // vertical word limit line (commented out)
        set_opt("cmdheight", 1)?; // command height

        // ----------------------------
        // Buffers / commands
        // ----------------------------
        set_opt("hidden", true)?; // hide buffers with unsaved changes
        set_opt("inccommand", "split")?; // live preview of :s results
        set_opt("shell", "zsh")?; // shell for :! or system()
        // set_opt("lazyredraw", true)?; // faster scrolling (commented out)

        // ----------------------------
        // Wildignore / completion
        // ----------------------------
        append_opt("wildignore", "*.o")?;
        append_opt("wildignore", "*.rej")?;
        append_opt("wildignore", "*.so")?;
        set_opt("completeopt", "menuone,noselect,noinsert")?; // completion options

        // ----------------------------
        // Misc
        // ----------------------------
        set_opt("showmode", false)?; // disable insert/replace/visual mode messages
        // set_opt("cmdheight", 0)?; // command height (commented out)

        Ok(())
    }
}
