use mlua::{Table, Value};
use wp_ui::nvim::{self, Buffer, Window};
use wp_ui::view::{Level, NotifyOptions, notify};
use wp_ui::widget::Popup;
use wp_ui::{Error, Result, lua};

use crate::state::{State, Term, with_state};

// -----------------------------------------------------------------------------
// Neovim helpers
// -----------------------------------------------------------------------------

fn in_terminal_mode() -> bool {
    lua::call_api::<Table>("nvim_get_mode", ())
        .and_then(|t| Ok(t.get::<String>("mode")?))
        .is_ok_and(|mode| mode == "t")
}

fn current_win() -> Option<Window> {
    lua::call_api::<i64>("nvim_get_current_win", ()).ok().map(Window)
}

fn current_buf() -> Option<Buffer> {
    lua::call_api::<i64>("nvim_get_current_buf", ()).ok().map(Buffer)
}

/// Whether Neovim is shutting down (`v:exiting` is set).
fn exiting() -> bool {
    lua::call_api::<Value>("nvim_get_vvar", ("exiting",)).is_ok_and(|v| !v.is_nil())
}

/// Shell to run: the `shell` option, then `$SHELL`, then `sh`.
fn resolve_shell() -> String {
    let usable = |s: &str| !s.is_empty() && lua::call_fn::<i64>("executable", (s,)).unwrap_or(0) == 1;
    if let Ok(shell) = lua::get_option::<String>("shell")
        && usable(&shell)
    {
        return shell;
    }
    if let Ok(shell) = std::env::var("SHELL")
        && usable(&shell)
    {
        return shell;
    }
    "sh".to_string()
}

/// Show an error from a callback, where there is no caller to return it to.
pub fn report(res: Result<()>) {
    if let Err(e) = res {
        let opts = NotifyOptions { title: Some("Terminal".into()), level: Level::Error, ..Default::default() };
        if notify(e.to_string(), opts).is_err() {
            eprintln!("AbstractTerminal: {e}");
        }
    }
}

// -----------------------------------------------------------------------------
// Internals
// -----------------------------------------------------------------------------

/// Record whether terminal `idx` is in terminal mode. Only meaningful while
/// its float is the current window, so otherwise the old value is kept.
fn remember_mode(state: &mut State, idx: usize) {
    let term = &mut state.terms[idx];
    if term.popup.window().is_some_and(|w| Some(w) == current_win()) {
        term.insert = in_terminal_mode();
    }
}

/// Show terminal `idx` (hiding any other) and focus it.
fn show(state: &mut State, idx: usize) -> Result<()> {
    for (i, term) in state.terms.iter().enumerate() {
        if i != idx {
            term.popup.hide();
        }
    }
    state.current = idx;
    let title = state.title();
    let term = &state.terms[idx];
    term.popup.set_title(title)?;
    term.popup.open()?;
    if let Some(win) = term.popup.window() {
        win.focus()?;
    }
    if term.insert {
        lua::cmd("startinsert")?;
    }
    Ok(())
}

/// Buffer-local autocmds for a new terminal. They only schedule work, so they
/// never run while the state is borrowed.
fn watch(buf: Buffer) -> Result<()> {
    nvim::buf_autocmd(&["TermClose"], buf, None, move || {
        let _ = nvim::schedule(move || report(on_exit(buf)));
    })?;
    nvim::buf_autocmd(&["BufEnter", "WinEnter"], buf, None, move || {
        let _ = nvim::schedule(move || report(on_enter(buf)));
    })?;
    Ok(())
}

/// The shell exited: drop the terminal and, if it was on screen, show the next one.
fn on_exit(buf: Buffer) -> Result<()> {
    if exiting() {
        return Ok(());
    }
    with_state(|state| {
        // Already gone if it was closed with `close()`.
        let Some(idx) = state.index_of(buf) else { return Ok(()) };
        let was_visible = state.terms[idx].popup.is_open();
        state.remove(idx);
        if buf.is_valid() {
            buf.delete()?;
        }
        state.prune();
        if state.terms.is_empty() {
            return Ok(());
        }
        if was_visible {
            show(state, state.current)
        } else {
            // The count in the title changed.
            let title = state.title();
            state.terms[state.current].popup.set_title(title)
        }
    })
}

/// Entering a terminal window (a click, `<C-w>w`): resume terminal mode if
/// that is how the terminal was left.
fn on_enter(buf: Buffer) -> Result<()> {
    if current_buf() != Some(buf) || in_terminal_mode() {
        return Ok(());
    }
    let insert = with_state(|state| Ok(state.index_of(buf).is_some_and(|i| state.terms[i].insert)))?;
    if insert {
        lua::cmd("startinsert")?;
    }
    Ok(())
}

// -----------------------------------------------------------------------------
// Public API
// -----------------------------------------------------------------------------

/// Toggle the floating terminal. Creates one if none exist.
pub fn toggle() -> Result<()> {
    let none = with_state(|state| {
        state.prune();
        if state.terms.is_empty() {
            return Ok(true);
        }
        match state.visible() {
            Some(i) => {
                remember_mode(state, i);
                state.terms[i].popup.hide();
            },
            None => show(state, state.current)?,
        }
        Ok(false)
    })?;
    if none { new_terminal() } else { Ok(()) }
}

/// Create a new terminal and show it.
pub fn new_terminal() -> Result<()> {
    with_state(|state| {
        state.prune();
        if let Some(i) = state.visible() {
            remember_mode(state, i);
        }

        let buf = Buffer::scratch()?;
        // Keep the terminal running while its float is hidden.
        buf.set_option("bufhidden", "hide")?;
        let popup = Popup::with_buffer(buf, state.config.popup_options())?;
        state.terms.push(Term { buf, popup, insert: false });
        let idx = state.terms.len() - 1;
        show(state, idx)?;

        // `jobstart(.., { term = true })` turns the current buffer into a terminal.
        let opts = lua::table()?;
        opts.set("term", true)?;
        let job = lua::call_fn::<i64>("jobstart", (vec![resolve_shell()], opts));
        if !matches!(job, Ok(id) if id > 0) {
            state.remove(idx);
            let _ = buf.delete();
            return Err(match job {
                Err(e) => e,
                Ok(_) => Error::invalid("could not start the shell"),
            });
        }

        watch(buf)?;
        state.terms[idx].insert = true;
        lua::cmd("startinsert")
    })
}

/// Show the next (`step = 1`) or previous (`step = -1`) terminal. Does
/// nothing unless a terminal float is on screen.
pub fn cycle(step: isize, wrap: bool) -> Result<()> {
    with_state(|state| {
        state.prune();
        let len = state.terms.len() as isize;
        let Some(visible) = state.visible() else { return Ok(()) };
        if len < 2 {
            return Ok(());
        }
        let next = visible as isize + step;
        let next = if wrap {
            next.rem_euclid(len)
        } else if (0..len).contains(&next) {
            next
        } else {
            return Ok(());
        };
        remember_mode(state, visible);
        show(state, next as usize)
    })
}

/// Close the current terminal (killing its shell). If it was on screen, the
/// next terminal takes its place.
pub fn close() -> Result<()> {
    with_state(|state| {
        state.prune();
        if state.terms.is_empty() {
            return Ok(());
        }
        let was_visible = state.terms[state.current].popup.is_open();
        let term = state.remove(state.current);
        // Kills the job; its TermClose handler then finds nothing left to do.
        term.buf.delete()?;
        if was_visible && !state.terms.is_empty() {
            show(state, state.current)?;
        }
        Ok(())
    })
}
