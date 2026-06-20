//! Buffer handle and operations.

use mlua::IntoLua;

use crate::error::Result;
use crate::lua;

/// A Neovim buffer, identified by its number.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Buffer(pub i64);

impl Buffer {
    /// Create a scratch buffer (`nvim_create_buf(false, true)`): unlisted and
    /// throwaway — the right kind for UI.
    pub fn scratch() -> Result<Self> {
        let id: i64 = lua::call_api("nvim_create_buf", (false, true))?;
        Ok(Buffer(id))
    }

    /// The raw buffer number.
    pub fn id(self) -> i64 {
        self.0
    }

    /// Whether the buffer still exists.
    pub fn is_valid(self) -> bool {
        lua::call_api::<bool>("nvim_buf_is_valid", (self.0,)).unwrap_or(false)
    }

    /// Number of lines in the buffer.
    pub fn line_count(self) -> Result<i64> {
        lua::call_api("nvim_buf_line_count", (self.0,))
    }

    /// Replace lines in `[start, end)` (end-exclusive; `-1` means last).
    pub fn set_lines<S: Into<String>>(self, start: i64, end: i64, lines: impl IntoIterator<Item = S>) -> Result<()> {
        let lines: Vec<String> = lines.into_iter().map(Into::into).collect();
        self.with_modifiable(|| lua::call_api::<()>("nvim_buf_set_lines", (self.0, start, end, false, lines)))
    }

    /// Replace the entire buffer contents.
    pub fn set_all<S: Into<String>>(self, lines: impl IntoIterator<Item = S>) -> Result<()> {
        self.set_lines(0, -1, lines)
    }

    /// Read lines in `[start, end)` (end-exclusive; `-1` means last).
    pub fn get_lines(self, start: i64, end: i64) -> Result<Vec<String>> {
        lua::call_api("nvim_buf_get_lines", (self.0, start, end, false))
    }

    /// The first line, or empty string if none.
    pub fn first_line(self) -> Result<String> {
        Ok(self.get_lines(0, 1)?.into_iter().next().unwrap_or_default())
    }

    /// Set a buffer-local option (e.g. `"modifiable"`, `"filetype"`).
    pub fn set_option(self, name: &str, value: impl IntoLua) -> Result<()> {
        let scope = lua::table()?;
        scope.set("buf", self.0)?;
        lua::set_option_scoped(name, value, scope)
    }

    fn get_bool_option(self, name: &str) -> Result<bool> {
        let scope = lua::table()?;
        scope.set("buf", self.0)?;
        lua::call_api("nvim_get_option_value", (name.to_string(), scope))
    }

    /// Run `f` with the buffer temporarily writable, restoring `modifiable` and
    /// `readonly` afterwards. UI buffers are usually locked; this lets writes
    /// through transparently without tripping the `W10` readonly warning.
    fn with_modifiable<R>(self, f: impl FnOnce() -> Result<R>) -> Result<R> {
        let was_modifiable = self.get_bool_option("modifiable")?;
        let was_readonly = self.get_bool_option("readonly")?;
        if !was_modifiable {
            self.set_option("modifiable", true)?;
        }
        if was_readonly {
            self.set_option("readonly", false)?;
        }
        let result = f();
        if !was_modifiable {
            self.set_option("modifiable", false)?;
        }
        if was_readonly {
            self.set_option("readonly", true)?;
        }
        result
    }

    /// Lock the buffer against user edits (used by display-only widgets).
    pub fn lock(self) -> Result<()> {
        self.set_option("modifiable", false)?;
        self.set_option("readonly", true)
    }

    /// Delete the buffer (`force = true`).
    pub fn delete(self) -> Result<()> {
        let opts = lua::table()?;
        opts.set("force", true)?;
        lua::call_api::<()>("nvim_buf_delete", (self.0, opts))
    }
}
