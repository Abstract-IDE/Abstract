use serde::Deserialize;
use types::{
    Object,
    conversion::{self, FromObject},
    serde::Deserializer,
};

/// Attributes related to a highlight group.
#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Default)]
pub struct HighlightInfos {
    pub altfont: Option<bool>,
    #[serde(rename = "bg")]
    pub background: Option<u32>,
    pub bg_indexed: Option<bool>,
    pub blend: Option<u32>,
    pub blink: Option<bool>,
    pub bold: Option<bool>,
    pub conceal: Option<bool>,
    pub ctermbg: Option<u32>,
    pub ctermfg: Option<u32>,
    pub default: Option<bool>,
    pub dim: Option<bool>,
    pub fallback: Option<bool>,
    pub fg_indexed: Option<bool>,
    pub force: Option<bool>,
    #[serde(rename = "fg")]
    pub foreground: Option<u32>,
    pub italic: Option<bool>,
    /// The name of the group this one links to.
    pub link: Option<String>,
    pub nocombine: Option<bool>,
    pub overline: Option<bool>,
    pub reverse: Option<bool>,
    #[serde(rename = "sp")]
    pub special: Option<u32>,
    pub standout: Option<bool>,
    pub strikethrough: Option<bool>,
    pub undercurl: Option<bool>,
    /// Never set: Neovim calls this attribute `underdashed`.
    #[deprecated(note = "use `underdashed` instead")]
    pub underdash: Option<bool>,
    pub underdashed: Option<bool>,
    /// Never set: Neovim calls this attribute `underdotted`.
    #[deprecated(note = "use `underdotted` instead")]
    pub underdot: Option<bool>,
    pub underdotted: Option<bool>,
    pub underdouble: Option<bool>,
    pub underline: Option<bool>,
    /// Never set: Neovim calls this attribute `underdouble`.
    #[deprecated(note = "use `underdouble` instead")]
    pub underlineline: Option<bool>,
}

impl FromObject for HighlightInfos {
    fn from_object(obj: Object) -> Result<Self, conversion::Error> {
        Self::deserialize(Deserializer::new(obj)).map_err(Into::into)
    }
}
