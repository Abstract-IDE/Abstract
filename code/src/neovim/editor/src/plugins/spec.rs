/// Plugin spec with source location info for error tracing.
///
/// Instead of returning a plain `&'static str` from `spec()`, plugins return
/// a `SpecInfo` which also carries the Rust source file and line number where
/// the spec string was defined. This allows error messages to point directly
/// to the correct line in the Rust source.
///
/// # Usage
/// ```rust
/// use crate::lua_spec;
///
/// pub fn spec() -> crate::plugins::spec::SpecInfo {
///     lua_spec!(r#"{
///         "author/plugin",
///         lazy = true,
///     }"#)
/// }
/// ```
pub struct SpecInfo {
    /// The Lua spec string
    pub spec: &'static str,
    /// Rust source file (from `file!()`)
    pub file: &'static str,
    /// Rust line number where `lua_spec!` was called (from `line!()`)
    pub spec_line: u32,
}

/// Macro to create a [`SpecInfo`] while capturing the Rust source location.
///
/// Place this macro at the line where your spec string (or `format!(...).leak()`) begins.
/// The captured `line!()` is used to compute the real Rust line number when a Lua
/// syntax error is reported.
///
/// Works with all spec patterns:
/// ```rust
/// // Direct string
/// lua_spec!(r#"{ "author/plugin", lazy = true }"#)
///
/// // Format + leak
/// lua_spec!(format!(r#"{{ "author/plugin", opts = {opts} }}"#).leak())
///
/// // Box::leak
/// lua_spec!(Box::leak(spec.into_boxed_str()))
/// ```
#[macro_export]
macro_rules! lua_spec {
    ($spec:expr) => {
        $crate::plugins::spec::SpecInfo {
            spec: $spec,
            file: file!(),
            spec_line: line!(),
        }
    };
}
