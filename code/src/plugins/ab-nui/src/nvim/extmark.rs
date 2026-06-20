//! Highlight namespaces and extmark-based highlighting.

use crate::error::Result;
use crate::lua;
use crate::nvim::buffer::Buffer;

/// A highlight namespace — an isolated layer of extmarks we can clear wholesale.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Namespace(pub i64);

impl Namespace {
    /// Create (or fetch) a named namespace.
    pub fn create(name: &str) -> Result<Self> {
        let id: i64 = lua::call_api("nvim_create_namespace", (name.to_string(),))?;
        Ok(Namespace(id))
    }

    pub fn id(self) -> i64 {
        self.0
    }

    /// Remove all extmarks this namespace owns in `buf`.
    pub fn clear(self, buf: Buffer) -> Result<()> {
        lua::call_api::<()>("nvim_buf_clear_namespace", (buf.id(), self.0, 0i64, -1i64))
    }

    /// Highlight a byte range on `row` (0-based) with `group`.
    pub fn highlight(self, buf: Buffer, row: i64, start_col: i64, end_col: i64, group: &str) -> Result<()> {
        let opts = lua::table()?;
        opts.set("end_col", end_col)?;
        opts.set("hl_group", group.to_string())?;
        lua::call_api::<i64>("nvim_buf_set_extmark", (buf.id(), self.0, row, start_col, opts))?;
        Ok(())
    }
}
