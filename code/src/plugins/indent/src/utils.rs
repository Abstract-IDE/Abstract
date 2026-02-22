use std::io::Error;

use nvim_oxi::api::{
    self, Buffer,
    opts::{CreateAugroupOpts, CreateAutocmdOpts, OptionOpts},
};

use wl_utils::neovim::types::events::Events;

use crate::core::{
    error::IndentError,
    indent::Indent,
    types::{IndentConfig, IndentStyle},
};

/// Detects the indentation style directly from an open Neovim buffer's live memory.
pub fn detect_buffer_indent(buffer: &Buffer, config: IndentConfig) -> Result<IndentStyle, IndentError> {
    // Extract lines from Neovim's memory using `..` to mean the entire buffer
    let lines = buffer
        .get_lines(.., false)
        .map_err(|e| -> IndentError { IndentError::Io(Error::other(e.to_string())) })?;

    let mut content = String::new();
    for line in lines {
        // to_string_lossy() converts nvim_oxi::String to standard Rust Cow<str> safely
        content.push_str(&line.to_string_lossy());
        content.push('\n');
    }

    let indent = Indent::new(config);
    indent.detect_from_bytes(content.as_bytes())
}

/// Applies the detected indentation style to the given Neovim buffer.
pub fn apply_buffer_indent(buffer: &Buffer, style: IndentStyle) -> Result<(), nvim_oxi::Error> {
    let opts = OptionOpts::builder().buffer(buffer.clone()).build();

    match style {
        IndentStyle::Space(n) => {
            api::set_option_value("expandtab", true, &opts)?;
            api::set_option_value("shiftwidth", n, &opts)?;
            api::set_option_value("tabstop", n, &opts)?;
            api::set_option_value("softtabstop", n, &opts)?;
        },
        IndentStyle::Tab => {
            api::set_option_value("expandtab", false, &opts)?;
            api::set_option_value("shiftwidth", 0, &opts)?;
            // TODO: make it configureable
            // 'tabstop' is left untouched to respect user's preferred visual width
        },
        IndentStyle::Mixed | IndentStyle::None | IndentStyle::Binary => {
            // Do nothing, fall back to Neovim defaults
        },
    }

    Ok(())
}

pub fn auto_detect_and_apply() {
    let cbuff = api::get_current_buf();
    let config = IndentConfig::default();

    match detect_buffer_indent(&cbuff, config) {
        Ok(style) => {
            if let Err(e) = apply_buffer_indent(&cbuff, style) {
                api::err_writeln(&format!("Indent: Failed to set options: {}", e));
            }
        },
        Err(e) => {
            api::err_writeln(&format!("Indent: Failed to detect indent: {:?}", e));
        },
    }
}

pub fn setup_indent_autocmds() -> Result<(), nvim_oxi::Error> {
    let group_opts = CreateAugroupOpts::builder().clear(true).build();
    let group_id = api::create_augroup("ABSTRACT_INDENT", &group_opts)?;

    let autocmd_opts = CreateAutocmdOpts::builder()
        .group(group_id)
        .callback(|_args| -> nvim_oxi::Result<bool> {
            auto_detect_and_apply();
            Ok(false)
        })
        .build();

    api::create_autocmd([Events::BufReadPost.as_ref()], &autocmd_opts)?;

    Ok(())
}
