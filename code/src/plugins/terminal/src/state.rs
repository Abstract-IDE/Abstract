use std::cell::RefCell;
use std::mem::ManuallyDrop;

use wp_ui::nvim::Buffer;
use wp_ui::widget::Popup;
use wp_ui::{Error, Result};

use crate::config::Config;

// -----------------------------------------------------------------------------
// State
// -----------------------------------------------------------------------------

/// One terminal: its buffer and the popup that shows it.
pub struct Term {
    pub buf: Buffer,
    pub popup: Popup,
    /// Resume terminal (insert) mode when this terminal is shown again.
    pub insert: bool,
}

pub struct State {
    pub terms: Vec<Term>,
    /// The terminal that `toggle` shows; always a valid index unless `terms` is empty.
    pub current: usize,
    pub config: Config,
}

impl State {
    /// Forget terminals whose buffer is gone.
    pub fn prune(&mut self) {
        let mut i = 0;
        while i < self.terms.len() {
            if self.terms[i].buf.is_valid() {
                i += 1;
            } else {
                self.remove(i);
            }
        }
    }

    /// Remove terminal `idx` and close its popup. The buffer is left to the
    /// caller. `current` keeps pointing at the same terminal, or at the one
    /// that took the removed terminal's place.
    pub fn remove(&mut self, idx: usize) -> Term {
        let term = self.terms.remove(idx);
        term.popup.close();
        if idx < self.current {
            self.current -= 1;
        }
        self.current = self.current.min(self.terms.len().saturating_sub(1));
        term
    }

    /// The terminal whose float is open, if any.
    pub fn visible(&self) -> Option<usize> {
        self.terms.iter().position(|t| t.popup.is_open())
    }

    pub fn index_of(&self, buf: Buffer) -> Option<usize> {
        self.terms.iter().position(|t| t.buf == buf)
    }

    /// Border title for the current terminal.
    pub fn title(&self) -> String {
        self.config.title(self.current, self.terms.len())
    }
}

// -----------------------------------------------------------------------------
// Thread-Local State
// -----------------------------------------------------------------------------

thread_local! {
    // `ManuallyDrop`: never tear terminals down from a thread-exit destructor,
    // after Neovim has shut Lua down.
    static STATE: ManuallyDrop<RefCell<Option<State>>> = const { ManuallyDrop::new(RefCell::new(None)) };
}

/// Run `f` with the plugin state.
pub fn with_state<R>(f: impl FnOnce(&mut State) -> Result<R>) -> Result<R> {
    STATE.with(|cell| {
        let mut borrow = cell.try_borrow_mut().map_err(|_| Error::invalid("terminal state is already in use"))?;
        let state = borrow.as_mut().ok_or_else(|| Error::invalid("setup() was not called"))?;
        f(state)
    })
}

/// Create the state, or swap in a new config while keeping open terminals.
pub fn install(config: Config) {
    STATE.with(|cell| {
        let mut borrow = cell.borrow_mut();
        match borrow.as_mut() {
            Some(state) => state.config = config,
            None => *borrow = Some(State { terms: Vec::new(), current: 0, config }),
        }
    });
}
