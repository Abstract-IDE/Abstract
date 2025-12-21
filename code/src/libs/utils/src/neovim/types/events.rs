use strum_macros::{
    AsRefStr,
    Display,
    EnumString, //
};

/// Neovim autocommand events.
///
/// These correspond to `:help autocmd-events`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, AsRefStr, EnumString, Display)]
#[strum(ascii_case_insensitive)]
pub enum Events {
    /// After a buffer is added to the buffer list.
    /// Happens before `BufEnter`.
    BufAdd,

    /// Before a buffer is deleted from the buffer list.
    BufDelete,

    /// After entering a buffer (existing or new).
    BufEnter,

    /// After changing the name of the current buffer.
    BufFilePost,

    /// Before changing the name of the current buffer.
    BufFilePre,

    /// Before a buffer becomes hidden (no windows display it).
    BufHidden,

    /// Before leaving the current buffer.
    BufLeave,

    /// After the `'modified'` flag of a buffer changes.
    BufModifiedSet,

    /// After creating a new buffer.
    BufNew,

    /// When starting to edit a file that does not exist.
    BufNewFile,

    /// After reading a file into a new buffer.
    BufReadPost,

    /// Before reading a file into a new buffer.
    BufReadPre,

    /// Before reading a file into a buffer; handler should read it.
    BufReadCmd,

    /// Before unloading a buffer and freeing its text.
    BufUnload,

    /// After a buffer is displayed in a window.
    BufWinEnter,

    /// Before a buffer is removed from a window.
    BufWinLeave,

    /// Before completely deleting a buffer.
    BufWipeout,

    /// Before writing the whole buffer to a file.
    BufWritePre,

    /// After writing the whole buffer to a file.
    BufWritePost,

    /// Before writing the buffer; handler should write it.
    BufWriteCmd,

    /// State of an RPC channel changed.
    ChanInfo,

    /// Just after a channel is opened.
    ChanOpen,

    /// When an undefined user command is invoked.
    CmdUndefined,

    /// After the command-line text changes.
    CmdlineChanged,

    /// After entering the command-line.
    CmdlineEnter,

    /// Before leaving the command-line.
    CmdlineLeave,

    /// After entering the command-line window.
    CmdwinEnter,

    /// Before leaving the command-line window.
    CmdwinLeave,

    /// After loading a colorscheme.
    ColorScheme,

    /// Before loading a colorscheme.
    ColorSchemePre,

    /// After the insert-mode completion menu changes.
    CompleteChanged,

    /// After insert-mode completion finishes (before cleanup).
    CompleteDonePre,

    /// After insert-mode completion finishes.
    CompleteDone,

    /// Triggered when the cursor is idle in Normal mode.
    CursorHold,

    /// Triggered when the cursor is idle in Insert mode.
    CursorHoldI,

    /// After the cursor moves in Normal or Visual mode.
    CursorMoved,

    /// After the cursor moves in Insert mode.
    CursorMovedI,

    /// After the cursor moves in the command-line.
    CursorMovedC,

    /// After diffs have been updated.
    DiffUpdated,

    /// After the working directory changes.
    DirChanged,

    /// Before the working directory changes.
    DirChangedPre,

    /// When Vim is about to exit due to a quit command.
    ExitPre,

    /// Before appending to a file; handler should append.
    FileAppendCmd,

    /// After appending to a file.
    FileAppendPost,

    /// Before appending to a file.
    FileAppendPre,

    /// Before modifying a read-only file.
    FileChangedRO,

    /// When Vim detects a file changed outside of Vim.
    FileChangedShell,

    /// After handling an external file change.
    FileChangedShellPost,

    /// Before reading a file via `:read`; handler should read.
    FileReadCmd,

    /// After reading a file via `:read`.
    FileReadPost,

    /// Before reading a file via `:read`.
    FileReadPre,

    /// When the `filetype` option is set.
    FileType,

    /// Before writing part of a buffer; handler should write.
    FileWriteCmd,

    /// After writing part of a buffer.
    FileWritePost,

    /// Before writing part of a buffer.
    FileWritePre,

    /// After reading from a filter command.
    FilterReadPost,

    /// Before reading from a filter command.
    FilterReadPre,

    /// After writing to a filter command.
    FilterWritePost,

    /// Before writing to a filter command.
    FilterWritePre,

    /// When Neovim gains focus.
    FocusGained,

    /// When Neovim loses focus.
    FocusLost,

    /// When an undefined user function is called.
    FuncUndefined,

    /// After a UI attaches to Neovim.
    UIEnter,

    /// After a UI detaches from Neovim.
    UILeave,

    /// When toggling Insert/Replace mode via `<Insert>`.
    InsertChange,

    /// Before inserting a character in Insert mode.
    InsertCharPre,

    /// Just before entering Insert or Replace mode.
    InsertEnter,

    /// Just before leaving Insert mode.
    InsertLeavePre,

    /// Just after leaving Insert mode.
    InsertLeave,

    /// When an LSP client attaches to a buffer.
    LspAttach,

    /// When an LSP client detaches from a buffer.
    LspDetach,

    /// When an LSP notification is received.
    LspNotify,

    /// When LSP progress updates are reported.
    LspProgress,

    /// When an LSP request is sent.
    LspRequest,

    /// When semantic tokens are updated by LSP.
    LspTokenUpdate,

    /// Before showing the popup menu (right-click).
    MenuPopup,

    /// After the editor mode changes.
    ModeChanged,

    /// After an option is set.
    OptionSet,

    /// Before running a quickfix command.
    QuickFixCmdPre,

    /// After running a quickfix command.
    QuickFixCmdPost,

    /// Before quitting a window or Vim.
    QuitPre,

    /// When a reply from a remote Vim server is received.
    RemoteReply,

    /// When a search wraps around the file.
    SearchWrapped,

    /// When macro recording starts.
    RecordingEnter,

    /// When macro recording stops.
    RecordingLeave,

    /// When Neovim reaches a safe idle state.
    SafeState,

    /// After loading a session file.
    SessionLoadPost,

    /// After writing a session file.
    SessionWritePost,

    /// After executing a shell command.
    ShellCmdPost,

    /// After executing a shell filter command.
    ShellFilterPost,

    /// Before sourcing a script file.
    SourcePre,

    /// After sourcing a script file.
    SourcePost,

    /// When sourcing a script file; handler must source it.
    SourceCmd,

    /// When a spell file is missing.
    SpellFileMissing,

    /// After reading stdin during startup.
    StdinReadPost,

    /// Before reading stdin during startup.
    StdinReadPre,

    /// When an existing swap file is detected.
    SwapExists,

    /// When the `syntax` option is set.
    Syntax,

    /// After entering a tab page.
    TabEnter,

    /// Before leaving a tab page.
    TabLeave,

    /// When creating a new tab page.
    TabNew,

    /// After entering a newly created tab page.
    TabNewEntered,

    /// After closing a tab page.
    TabClosed,

    /// When a terminal job starts.
    TermOpen,

    /// After entering Terminal mode.
    TermEnter,

    /// After leaving Terminal mode.
    TermLeave,

    /// When a terminal job exits.
    TermClose,

    /// When a terminal child emits an OSC/DCS/APC sequence.
    TermRequest,

    /// When the host terminal responds to an OSC/DCS query.
    TermResponse,

    /// After text changes in Normal mode.
    TextChanged,

    /// After text changes in Insert mode.
    TextChangedI,

    /// After text changes in Insert mode with popup menu visible.
    TextChangedP,

    /// After text changes in Terminal mode.
    TextChangedT,

    /// After a yank or delete operation.
    TextYankPost,

    /// User-defined custom event (triggered via `:doautocmd`).
    User,

    /// Fired after Vim finishes startup.
    VimEnter,

    /// Before Vim exits.
    VimLeave,

    /// Before Vim exits, prior to writing shada.
    VimLeavePre,

    /// After Vim window is resized.
    VimResized,

    /// After Vim resumes from suspend.
    VimResume,

    /// Before Vim suspends.
    VimSuspend,

    /// When a window is closed.
    WinClosed,

    /// After entering a window.
    WinEnter,

    /// Before leaving a window.
    WinLeave,

    /// When a new window is created.
    WinNew,

    /// After a window scrolls or changes size.
    WinScrolled,

    /// After a window is resized.
    WinResized,
}
