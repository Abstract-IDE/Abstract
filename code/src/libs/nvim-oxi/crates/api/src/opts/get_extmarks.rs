#[derive(Clone, Debug, Default, macros::OptsBuilder)]
#[repr(C)]
/// Options passed to
/// [`Buffer::get_extmarks()`](crate::Buffer::get_extmarks).
pub struct GetExtmarksOpts {
    #[builder(mask)]
    mask: u64,

    #[builder(
        method = "limits",
        argtype = "bool",
        inline = "{0} as types::Integer"
    )]
    limit: types::Integer,

    /// Whether to include the extmark's
    /// [`ExtmarkInfos`](crate::types::ExtmarkInfos) as the last element of
    /// the tuples returned by
    /// [`Buffer::get_extmarks()`](crate::Buffer::get_extmarks).
    #[builder(argtype = "bool")]
    details: types::Boolean,

    #[builder(argtype = "bool")]
    hl_name: types::Boolean,

    #[builder(argtype = "bool")]
    overlap: types::Boolean,

    // TODO: fix `Into`.
    // #[builder(Into)]
    #[builder(
        method = "ty",
        generics = "S: Into<types::String>",
        argtype = "S",
        inline = "{0}.into()"
    )]
    r#type: types::String,
}
