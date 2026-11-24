use serde::{
    Deserialize,
    de::{self, Error},
};
use types::{
    Function,
    Object,
    conversion::{self, FromObject},
    serde::Deserializer,
};

use super::{CommandAddr, CommandArgs, CommandNArgs, CommandRange};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
pub struct CommandInfos {
    /// TODO: docs
    pub addr: Option<CommandAddr>,

    /// Whether the command can take a `!` modifier.
    pub bang: bool,

    /// Whether the command can be followed by a `|` and another command.
    pub bar: bool,

    /// Callback triggered by the command.
    pub callback: Option<Function<CommandArgs, ()>>,

    /// Command completion strategy (e.g. `"file"`, `"custom"`).
    ///
    /// Since Neovim 0.12 a command whose completion is a Lua function reports
    /// that function here instead of a name; in that case this is `None`.
    #[serde(default, deserialize_with = "parse_complete")]
    pub complete: Option<String>,

    /// TODO: docs
    pub complete_arg: Option<String>,

    /// TODO: docs
    #[serde(deserialize_with = "parse_count")]
    pub count: Option<u32>,

    /// TODO: docs
    pub definition: Option<String>,

    /// Whether to use the invocation location as opposed to the definition
    /// location in verbose messages.
    pub keepscript: bool,

    /// The command name.
    pub name: String,

    /// The number of arguments the command can take.
    #[serde(default)]
    pub nargs: CommandNArgs,

    /// TODO: docs
    pub range: Option<CommandRange>,

    /// Whether the firrst argument to the command can be an optional register
    /// name (like `:del`, `:put` or `:yank`).
    pub register: bool,

    /// TODO: docs
    pub script_id: i32,
}

fn parse_count<'de, D>(deserializer: D) -> Result<Option<u32>, D::Error>
where
    D: de::Deserializer<'de>,
{
    Option::<String>::deserialize(deserializer)?
        .map(|count| {
            count.parse().map_err(|err: std::num::ParseIntError| {
                D::Error::custom(err.to_string())
            })
        })
        .transpose()
}

/// Accepts both a completion name and (since Neovim 0.12) a Lua function,
/// mapping the latter to `None`.
fn parse_complete<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: de::Deserializer<'de>,
{
    struct CompleteVisitor;

    impl<'de> de::Visitor<'de> for CompleteVisitor {
        type Value = Option<String>;

        fn expecting(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.write_str("a completion name, a Lua function or nil")
        }

        fn visit_str<E: de::Error>(self, name: &str) -> Result<Self::Value, E> {
            Ok(Some(name.to_owned()))
        }

        fn visit_string<E: de::Error>(
            self,
            name: String,
        ) -> Result<Self::Value, E> {
            Ok(Some(name))
        }

        // Lua references are visited as `f32`s.
        fn visit_f32<E: de::Error>(self, _: f32) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_some<D2>(self, deserializer: D2) -> Result<Self::Value, D2::Error>
        where
            D2: de::Deserializer<'de>,
        {
            deserializer.deserialize_any(self)
        }
    }

    deserializer.deserialize_any(CompleteVisitor)
}

impl FromObject for CommandInfos {
    fn from_object(obj: Object) -> Result<Self, conversion::Error> {
        Self::deserialize(Deserializer::new(obj)).map_err(Into::into)
    }
}
