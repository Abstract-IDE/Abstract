use types::Object;
use types::{Boolean, Integer, String as NvimString};

// https://github.com/neovim/neovim/blob/v0.12.6/src/nvim/api/keysets_defs.h#L174-L211
//
// NOTE: this is the layout used by Neovim 0.12.2 and later. Compared to 0.11
// the boolean attributes were sorted alphabetically, `blink`, `conceal`,
// `dim`, `overline` and `update` were added, `cterm` became a dictionary and
// `global_link` was renamed to `link_global`.
/// Options passed to [`set_hl()`](crate::set_hl).
#[cfg(feature = "neovim-0-12")] // On 0.12 and Nightly.
#[derive(Clone, Debug, Default, PartialEq, macros::OptsBuilder)]
#[repr(C)]
pub struct SetHighlightOpts {
    #[builder(mask)]
    mask: u64,

    #[builder(argtype = "bool")]
    altfont: Boolean,

    #[builder(argtype = "bool")]
    blink: Boolean,

    #[builder(argtype = "bool")]
    bold: Boolean,

    #[builder(argtype = "bool")]
    conceal: Boolean,

    #[builder(argtype = "bool")]
    dim: Boolean,

    #[builder(argtype = "bool")]
    italic: Boolean,

    #[builder(argtype = "bool")]
    nocombine: Boolean,

    #[builder(argtype = "bool")]
    overline: Boolean,

    #[builder(argtype = "bool")]
    reverse: Boolean,

    #[builder(argtype = "bool")]
    standout: Boolean,

    #[builder(argtype = "bool")]
    strikethrough: Boolean,

    #[builder(argtype = "bool")]
    undercurl: Boolean,

    #[builder(argtype = "bool")]
    underdashed: Boolean,

    #[builder(argtype = "bool")]
    underdotted: Boolean,

    #[builder(argtype = "bool")]
    underdouble: Boolean,

    #[builder(argtype = "bool")]
    underline: Boolean,

    // The C field is `default_`, its key is `default`. The setter keeps the
    // name it has on 0.11 (a method called `default` would shadow
    // `Default::default()` on the builder).
    #[builder(method = "builder", argtype = "bool")]
    default: Boolean,

    /// The `cterm` attribute map, e.g. `{ bold = true, underline = true }`.
    #[builder(
        generics = "D: Into<types::Dictionary>",
        argtype = "D",
        inline = "{0}.into()"
    )]
    cterm: types::Dictionary,

    #[builder(argtype = "&str", inline = "types::String::from({0}).into()")]
    foreground: Object,

    #[builder(skip)]
    fg: Object,

    #[builder(argtype = "&str", inline = "types::String::from({0}).into()")]
    background: Object,

    #[builder(skip)]
    bg: Object,

    #[builder(argtype = "&str", inline = "types::String::from({0}).into()")]
    ctermfg: Object,

    #[builder(argtype = "&str", inline = "types::String::from({0}).into()")]
    ctermbg: Object,

    #[builder(argtype = "&str", inline = "types::String::from({0}).into()")]
    special: Object,

    #[builder(skip)]
    sp: Object,

    #[builder(
        generics = "Hl: crate::HlGroup",
        argtype = "Hl",
        inline = r#"{ let Ok(hl_id) = {0}.to_hl_id() else { return self; }; hl_id }"#
    )]
    link: types::HlGroupId,

    #[builder(skip)]
    link_global: types::HlGroupId,

    #[builder(argtype = "bool")]
    fallback: Boolean,

    #[builder(argtype = "u8", inline = "{0} as Integer")]
    blend: Integer,

    #[builder(argtype = "bool")]
    fg_indexed: Boolean,

    #[builder(argtype = "bool")]
    bg_indexed: Boolean,

    #[builder(argtype = "bool")]
    force: Boolean,

    /// Update the existing highlight group instead of replacing it.
    #[builder(argtype = "bool")]
    update: Boolean,

    #[builder(skip)]
    url: NvimString,
}

// https://github.com/neovim/neovim/blob/v0.11.3/src/nvim/api/keysets_defs.h#L164-L196
#[cfg(not(feature = "neovim-0-12"))] // Only on 0.11.
#[derive(Clone, Debug, Default, PartialEq, macros::OptsBuilder)]
#[repr(C)]
pub struct SetHighlightOpts {
    #[builder(mask)]
    mask: u64,

    #[builder(argtype = "bool")]
    bold: Boolean,

    #[builder(argtype = "bool")]
    standout: Boolean,

    #[builder(argtype = "bool")]
    strikethrough: Boolean,

    #[builder(argtype = "bool")]
    underline: Boolean,

    #[builder(argtype = "bool")]
    undercurl: Boolean,

    #[builder(argtype = "bool")]
    underdouble: Boolean,

    #[builder(argtype = "bool")]
    underdotted: Boolean,

    #[builder(argtype = "bool")]
    underdashed: Boolean,

    #[builder(argtype = "bool")]
    italic: Boolean,

    #[builder(argtype = "bool")]
    reverse: Boolean,

    #[builder(argtype = "bool")]
    altfont: Boolean,

    #[builder(argtype = "bool")]
    nocombine: Boolean,

    #[builder(method = "builder", argtype = "bool")]
    // The field name is actually `default_`, but I think it somehow gets
    // converted to `default` at build time because the correct mask index
    // is obtained with `default`.
    default: Boolean,

    #[builder(argtype = "&str", inline = "types::String::from({0}).into()")]
    cterm: Object,

    #[builder(argtype = "&str", inline = "types::String::from({0}).into()")]
    foreground: Object,

    #[builder(skip)]
    fg: Object,

    #[builder(argtype = "&str", inline = "types::String::from({0}).into()")]
    background: Object,

    #[builder(skip)]
    bg: Object,

    #[builder(argtype = "&str", inline = "types::String::from({0}).into()")]
    ctermfg: Object,

    #[builder(argtype = "&str", inline = "types::String::from({0}).into()")]
    ctermbg: Object,

    #[builder(argtype = "&str", inline = "types::String::from({0}).into()")]
    special: Object,

    #[builder(skip)]
    sp: Object,

    #[builder(
        generics = "Hl: crate::HlGroup",
        argtype = "Hl",
        inline = r#"{ let Ok(hl_id) = {0}.to_hl_id() else { return self; }; hl_id }"#
    )]
    link: types::HlGroupId,

    #[builder(skip)]
    global_link: types::HlGroupId,

    #[builder(argtype = "bool")]
    fallback: Boolean,

    #[builder(argtype = "u8", inline = "{0} as Integer")]
    blend: Integer,

    #[builder(argtype = "bool")]
    fg_indexed: Boolean,

    #[builder(argtype = "bool")]
    bg_indexed: Boolean,

    #[builder(argtype = "bool")]
    force: Boolean,

    #[builder(skip)]
    url: NvimString,
}
