use nvim_oxi::api::{
    self, Buffer,
    opts::{ClearAutocmdsOpts, CreateAugroupOpts, CreateAutocmdOpts},
    types::AutocmdCallbackArgs,
};

use crate::state::State;

// -----------------------------------------------------------------------------
// Float Guard Autocmd
// -----------------------------------------------------------------------------

/// Set up a `BufWinEnter` guard so that the floating window always shows the
/// intended terminal buffer. Without this, commands like `:bnext` inside the
/// float could swap the displayed buffer.
pub fn guard_buf(state: &mut State, buf: Buffer) -> nvim_oxi::Result<()> {
    let group = match state.float_guard_group {
        Some(g) => g,
        None => {
            let opts = CreateAugroupOpts::builder().clear(true).build();
            let g = api::create_augroup("AbstractTerminalFloatGuard", &opts)?;
            state.float_guard_group = Some(g);
            g
        },
    };

    // Clear any previous guard autocmds in this group.
    api::clear_autocmds(&ClearAutocmdsOpts::builder().group(group).build())?;

    if let Some(win_id) = state.win.clone() {
        let buf_id = buf.clone();
        let guard_opts = CreateAutocmdOpts::builder()
            .group(group)
            .nested(true)
            .callback(move |args: AutocmdCallbackArgs| -> std::result::Result<bool, nvim_oxi::Error> {
                let current_win = api::get_current_win();
                if current_win == win_id && args.buffer != buf_id {
                    let mut win = win_id.clone();
                    let _ = win.set_buf(&buf_id);
                }
                Ok(false)
            })
            .build();

        api::create_autocmd(["BufWinEnter"], &guard_opts)?;
    }

    Ok(())
}
