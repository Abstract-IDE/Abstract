use std::cell::RefCell;

use nvim_oxi::api::{self, Buffer, Window, opts::ClearAutocmdsOpts};

use crate::config::Config;

// -----------------------------------------------------------------------------
// State Management
// -----------------------------------------------------------------------------

pub struct State {
    pub bufs: Vec<Buffer>,
    pub win: Option<Window>,
    pub current: usize,
    pub modes: Vec<String>,
    pub float_guard_group: Option<u32>,
    pub config: Config,
}

impl State {
    pub fn new(config: Config) -> Self {
        Self { bufs: Vec::new(), win: None, current: 0, modes: Vec::new(), float_guard_group: None, config }
    }

    /// Remove dead (invalid) buffers and adjust the current index.
    pub fn prune_buffers(&mut self) {
        let mut live_bufs = Vec::new();
        let mut live_modes = Vec::new();

        for (i, buf) in self.bufs.iter().enumerate() {
            if buf.is_valid() {
                live_bufs.push(buf.clone());
                live_modes.push(self.modes[i].clone());
            }
        }

        self.bufs = live_bufs;
        self.modes = live_modes;

        if self.bufs.is_empty() {
            self.current = 0;
        } else {
            self.current = self.current.min(self.bufs.len() - 1);
        }
    }

    /// Get the currently selected buffer, if any.
    pub fn current_buf(&self) -> Option<&Buffer> {
        self.bufs.get(self.current)
    }

    /// Get the mode string for the current terminal.
    pub fn current_mode(&self) -> &str {
        self.modes.get(self.current).map_or("n", |m| m.as_str())
    }

    /// Close the floating window and clear its guard autocmds.
    pub fn close_float(&mut self) {
        if let Some(win) = self.win.take()
            && win.is_valid()
        {
            let _ = win.close(true);
        }
        if let Some(group) = self.float_guard_group {
            let _ = api::clear_autocmds(&ClearAutocmdsOpts::builder().group(group).build());
        }
    }

    /// Total number of terminal buffers.
    pub fn len(&self) -> usize {
        self.bufs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bufs.is_empty()
    }
}

// -----------------------------------------------------------------------------
// Thread-Local State
// -----------------------------------------------------------------------------

thread_local! {
    pub static STATE: RefCell<Option<State>> = const { RefCell::new(None) };
}

/// Run a closure with mutable access to the global state.
/// Panics if `setup()` has not been called.
pub fn with_state<F, R>(f: F) -> R
where
    F: FnOnce(&mut State) -> R,
{
    STATE.with(|cell| {
        let mut borrow = cell.borrow_mut();
        let state = borrow.as_mut().expect("AbstractTerminal: setup() was not called");
        f(state)
    })
}

/// Initialise the global state with the given config.
pub fn init_state(config: Config) {
    STATE.with(|cell| {
        *cell.borrow_mut() = Some(State::new(config));
    });
}
