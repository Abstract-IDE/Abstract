use types::{Array, Object};

use crate::Buffer;
use crate::trait_utils::StringOrInt;

/// Options passed to [`get_autocmds()`](crate::get_autocmds).
#[derive(Clone, Debug, Default, macros::OptsBuilder)]
#[repr(C)]
pub struct GetAutocmdsOpts {
    #[builder(mask)]
    mask: u64,

    /// Get all the autocommands triggered by one or more of the specified
    /// events.
    #[builder(
        generics = "'a, E: IntoIterator<Item = &'a str>",
        method = "events",
        argtype = "E",
        inline = "Array::from_iter({0}).into()"
    )]
    event: Object,

    /// Only get the autocommands belonging to a specific augroup. The
    /// augroup can be specified by both id and name.
    #[builder(
        generics = "G: StringOrInt",
        method = "group",
        argtype = "G",
        inline = "{0}.to_object()"
    )]
    group: Object,

    /// Only get the autocommands that match specific patterns. For example, if
    /// you have `"*.py"` as a pattern for a particular autocommand, you must
    /// pass that exact pattern to clear it. Cannot be used together with
    /// `buffer`.
    #[builder(
        generics = "'a, P: IntoIterator<Item = &'a str>",
        method = "patterns",
        argtype = "P",
        inline = "Array::from_iter({0}).into()"
    )]
    pattern: Object,

    #[cfg(not(feature = "neovim-0-12"))] // Only on 0.11.
    /// Get the autocommands local to a specific `Buffer`. Cannot be used
    /// together with `patterns`.
    #[builder(argtype = "Buffer", inline = "{0}.into()")]
    buffer: Object,

    // Neovim 0.12.2 added `buf` next to the (now deprecated) `buffer` key.
    // Both fields must exist for the layout and the mask to match.
    #[cfg(feature = "neovim-0-12")] // On 0.12 and Nightly.
    #[builder(skip)]
    buffer: Object,

    #[cfg(feature = "neovim-0-12")] // On 0.12 and Nightly.
    /// Get the autocommands local to a specific `Buffer`. Cannot be used
    /// together with `patterns`.
    #[builder(method = "buffer", argtype = "Buffer", inline = "{0}.into()")]
    buf: Object,

    /// Only get the autocommand with this id.
    #[builder(argtype = "u32", inline = "{0} as types::Integer")]
    id: types::Integer,
}
