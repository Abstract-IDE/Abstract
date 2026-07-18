//! Buffer handle and operations.

use mlua::{FromLua, IntoLua};

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

    /// Replace text within a line range, character-precise (`nvim_buf_set_text`;
    /// `(row, col)` are 0-based, end-exclusive, cols in bytes).
    pub fn set_text<S: Into<String>>(
        self,
        start_row: i64,
        start_col: i64,
        end_row: i64,
        end_col: i64,
        lines: impl IntoIterator<Item = S>,
    ) -> Result<()> {
        let lines: Vec<String> = lines.into_iter().map(Into::into).collect();
        self.with_modifiable(|| {
            lua::call_api::<()>("nvim_buf_set_text", (self.0, start_row, start_col, end_row, end_col, lines))
        })
    }

    /// The buffer's full name (path).
    pub fn name(self) -> Result<String> {
        lua::call_api("nvim_buf_get_name", (self.0,))
    }

    /// Set the buffer's name.
    pub fn set_name(self, name: &str) -> Result<()> {
        lua::call_api("nvim_buf_set_name", (self.0, name.to_string()))
    }

    /// Set a buffer-local option (e.g. `"modifiable"`, `"filetype"`).
    pub fn set_option(self, name: &str, value: impl IntoLua) -> Result<()> {
        let scope = lua::table()?;
        scope.set("buf", self.0)?;
        lua::set_option_scoped(name, value, scope)
    }

    /// Read a buffer-local option.
    pub fn get_option<R: FromLua>(self, name: &str) -> Result<R> {
        let scope = lua::table()?;
        scope.set("buf", self.0)?;
        lua::get_option_scoped(name, scope)
    }

    /// Run `f` with the buffer temporarily writable, restoring `modifiable` and
    /// `readonly` afterwards. UI buffers are usually locked; this lets writes
    /// through transparently without tripping the `W10` readonly warning.
    fn with_modifiable<R>(self, f: impl FnOnce() -> Result<R>) -> Result<R> {
        let was_modifiable: bool = self.get_option("modifiable")?;
        let was_readonly: bool = self.get_option("readonly")?;
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
