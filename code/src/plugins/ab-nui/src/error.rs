//! Error type for the whole library.

/// All public fallible APIs return this.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// A call into the Lua/Neovim runtime failed.
    #[error("lua error: {0}")]
    Lua(#[from] mlua::Error),

    /// `ui::init()` was never called, so there is no `Lua` handle to use.
    #[error("ab-nui is not initialized — call `ui::init(lua)` once during plugin setup")]
    NotInitialized,

    /// A widget/operation was used in an invalid state (e.g. closed popup).
    #[error("{0}")]
    Invalid(String),
}

/// Convenience alias used across the crate.
pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    /// Build an [`Error::Invalid`] from anything string-like.
    pub fn invalid(msg: impl Into<String>) -> Self {
        Error::Invalid(msg.into())
    }
}
