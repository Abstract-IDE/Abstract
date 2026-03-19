use nvim_oxi::api::{self, opts::OptionOpts};

use crate::{state::with_state, window};

// -----------------------------------------------------------------------------
// Shell Resolution
// -----------------------------------------------------------------------------

/// Determine the shell to use for terminal spawning.
/// Priority: vim `shell` option → `$SHELL` env var → fallback to `"sh"`.
fn resolve_shell() -> String {
    // Try vim's &shell option first.
    if let Ok(shell) = api::get_option_value::<String>("shell", &OptionOpts::default())
        && !shell.is_empty()
        && api::call_function::<_, i64>("executable", (nvim_oxi::Object::from(nvim_oxi::String::from(shell.clone())),))
            .unwrap_or(0)
            == 1
    {
        return shell;
    }

    // Fall back to $SHELL.
    if let Ok(shell) = std::env::var("SHELL")
        && !shell.is_empty()
        && api::call_function::<_, i64>("executable", (nvim_oxi::Object::from(nvim_oxi::String::from(shell.clone())),))
            .unwrap_or(0)
            == 1
    {
        return shell;
    }

    // Ultimate fallback.
    "sh".to_string()
}

// -----------------------------------------------------------------------------
// Internal: Create a New Terminal Buffer
// -----------------------------------------------------------------------------

fn new_terminal_internal() -> nvim_oxi::Result<()> {
    with_state(|state| {
        // Close any existing float.
        state.close_float();

        // Create a fresh unlisted scratch buffer.
        let buf = api::create_buf(false, true)?;
        api::set_option_value("bufhidden", "hide", &OptionOpts::builder().buffer(buf.clone()).build())?;

        // Track it.
        state.bufs.push(buf.clone());
        state.current = state.bufs.len() - 1;
        state.modes.push("t".to_string());

        // Open the float.
        window::open_floating(state, &buf)?;

        if let Some(win) = &state.win {
            api::set_current_win(win)?;
        }
        api::set_current_buf(&buf)?;

        // Spawn a shell in the buffer.
        let shell = resolve_shell();
        let args = vec![nvim_oxi::Object::from(nvim_oxi::String::from(shell))].into_iter().collect::<nvim_oxi::Array>();
        api::call_function::<_, i64>("termopen", (args,))?;
        api::command("startinsert")?;

        // Removed `modifiable=false`. Terminal buffers natively reject standard text entry unless you are in Terminal mode. 
        // Setting it to false causes `startinsert` to throw E21, which forces Neovim to abort window focus during mouse clicks!

        // Bug 1: Auto-close buffer when terminal process exits
        let buf_clone = buf.clone();
        api::create_autocmd(
            ["TermClose"],
            &nvim_oxi::api::opts::CreateAutocmdOpts::builder()
                .buffer(buf.clone())
                .nested(true)
                .callback(move |_args| -> std::result::Result<bool, nvim_oxi::Error> {
                    // Check if Neovim is currently exiting. If so, do not try to open floats or startinsert.
                    if let Ok(v_exiting) = api::get_vvar::<nvim_oxi::Object>("exiting") {
                        if !v_exiting.is_nil() {
                            return Ok(false);
                        }
                    }

                    let buf_handle = buf_clone.handle();
                    nvim_oxi::schedule(move |()| {
                        if let Ok(v_exiting) = api::get_vvar::<nvim_oxi::Object>("exiting") {
                            if !v_exiting.is_nil() {
                                return;
                            }
                        }

                        crate::state::with_state(|state| {
                            let is_current = state.current_buf().map(|b| b.handle() == buf_handle).unwrap_or(false);
                            
                            let _ = api::command(&format!("bdelete! {}", buf_handle));
                            
                            state.prune_buffers();
                            
                            if state.is_empty() {
                                state.close_float();
                            } else if is_current {
                                if state.current >= state.len() {
                                    state.current = state.len() - 1;
                                }
                                let next_buf = state.bufs[state.current].clone();
                                let _ = crate::window::open_floating(state, &next_buf);
                                if let Some(win) = &state.win {
                                    let _ = api::set_current_win(win);
                                }
                                let _ = crate::window::update_title(state);
                                if state.current_mode() == "t" {
                                    let _ = api::command("startinsert");
                                }
                            } else {
                                let _ = crate::window::update_title(state);
                            }
                        });
                    });
                    Ok(false)
                })
                .build(),
        )?;

        // Enter insert mode when terminal window gains focus
        api::create_autocmd(
            ["BufEnter", "WinEnter"],
            &nvim_oxi::api::opts::CreateAutocmdOpts::builder()
                .buffer(buf.clone())
                .nested(true)
                .callback(|_args| -> std::result::Result<bool, nvim_oxi::Error> {
                    nvim_oxi::schedule(|()| {
                        if let Ok(m) = api::get_mode() {
                            if m.mode != "t" {
                                let _ = api::command("startinsert");
                            }
                        }
                    });
                    Ok(false)
                })
                .build(),
        )?;

        Ok(())
    })
}

// -----------------------------------------------------------------------------
// Public API
// -----------------------------------------------------------------------------

/// Toggle the floating terminal. Creates one if none exist.
pub fn toggle() -> nvim_oxi::Result<()> {
    with_state(|state| {
        state.prune_buffers();

        if state.is_empty() {
            // State borrow is naturally released when this closure returns
            return Err(nvim_oxi::Error::from(nvim_oxi::lua::Error::RuntimeError("__create_new__".into())));
        }

        // If the float is currently open, save mode and close it.
        if let Some(win) = &state.win
            && win.is_valid()
        {
            let mode = api::get_mode()?.mode;
            state.modes[state.current] = mode.to_string();
            state.close_float();
            return Ok(());
        }

        // Float is closed — re-open it with the current buffer.
        let buf = state.bufs[state.current].clone();
        window::open_floating(state, &buf)?;

        if let Some(win) = &state.win {
            api::set_current_win(win)?;
        }

        if state.current_mode() == "t" {
            api::command("startinsert")?;
        }

        Ok(())
    })
    .or_else(|e| if e.to_string().contains("__create_new__") { new_terminal_internal() } else { Err(e) })
}

/// Create a new terminal buffer and display it in a float.
pub fn new_terminal() -> nvim_oxi::Result<()> {
    with_state(|state| state.prune_buffers());
    new_terminal_internal()
}

/// Cycle to the next or previous terminal.
pub fn cycle(direction: isize, wrap: bool) -> nvim_oxi::Result<()> {
    // Phase 1: save current mode and issue stopinsert if needed.
    with_state(|state| {
        state.prune_buffers();

        if state.len() <= 1 {
            return Ok::<(), nvim_oxi::Error>(());
        }

        let mode = api::get_mode()?.mode;
        state.modes[state.current] = mode.to_string();

        if mode == "t" {
            api::command("stopinsert")?;
        }

        Ok(())
    })?;

    // Phase 2: schedule the actual buffer swap (needed after mode change).
    nvim_oxi::schedule(move |()| {
        with_state(|state| {
            if state.win.as_ref().is_none_or(|w| !w.is_valid()) {
                return;
            }

            let len = state.len() as isize;
            let mut idx = state.current as isize + direction;

            if idx >= len {
                if !wrap {
                    return;
                }
                idx = 0;
            } else if idx < 0 {
                if !wrap {
                    return;
                }
                idx = len - 1;
            }

            // Close old float and open new one.
            state.close_float();
            state.current = idx as usize;

            let buf = state.bufs[state.current].clone();
            let _ = window::open_floating(state, &buf);

            if let Some(win) = &state.win {
                let _ = api::set_current_win(win);
            }
            let _ = window::update_title(state);

            if state.current_mode() == "t" {
                let _ = api::command("startinsert");
            }
        });
    });

    Ok(())
}

/// Close the current terminal buffer. Shows the next one, or hides the float
/// if it was the last terminal.
pub fn close() -> nvim_oxi::Result<()> {
    with_state(|state| {
        state.prune_buffers();

        if state.is_empty() {
            return Ok(());
        }

        // Exit terminal mode if we're in it.
        if let Some(win) = &state.win
            && win.is_valid()
        {
            api::set_current_win(win)?;
            if api::get_mode()?.mode == "t" {
                api::command("stopinsert")?;
            }
        }

        // Delete the buffer.
        let buf = state.bufs[state.current].clone();
        if buf.is_valid() {
            api::set_current_buf(&buf)?;
            api::command("bdelete!")?;
        }

        state.bufs.remove(state.current);
        state.modes.remove(state.current);

        // No more terminals → close everything.
        if state.is_empty() {
            state.close_float();
            state.current = 0;
            return Ok(());
        }

        // Adjust index and show the next terminal.
        if state.current >= state.len() {
            state.current = state.len() - 1;
        }

        let buf = state.bufs[state.current].clone();
        window::open_floating(state, &buf)?;

        if let Some(win) = &state.win {
            api::set_current_win(win)?;
        }
        window::update_title(state)?;

        if state.current_mode() == "t" {
            api::command("startinsert")?;
        }

        Ok(())
    })
}
