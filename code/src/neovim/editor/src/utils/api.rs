use nvim_oxi::{
    self,
    api, //
};

/// Set an option value
pub fn set_opt<Opt>(name: &str, value: Opt) -> nvim_oxi::Result<()>
where
    Opt: nvim_oxi::conversion::ToObject,
{
    api::set_option_value(name, value, &Default::default())?;
    Ok(())
}

/// Build a `listchars` option value
pub fn listchars(pairs: &[(&str, &str)]) -> String {
    pairs.iter().map(|(k, v)| format!("{k}:{v}")).collect::<Vec<_>>().join(",")
}

/// Append to a comma-separated string option
pub fn append_opt(name: &str, suffix: &str) -> nvim_oxi::Result<()> {
    let current = api::get_option_value::<String>(name, &Default::default())?;
    let new = if current.is_empty() { suffix.to_string() } else { format!("{current},{suffix}") };
    api::set_option_value(name, new, &Default::default())?;
    Ok(())
}
