use std::sync::LazyLock;

use crate::utils::nvim_path::nvim_stdpath;

pub static NVIM_CONFIG_HOME: LazyLock<String> =
    LazyLock::new(|| format!("{}/rust", nvim_stdpath("config").to_string_lossy()));

pub static NVIM_DATA_HOME: LazyLock<String> =
    LazyLock::new(|| format!("{}/rust", nvim_stdpath("data").to_string_lossy()));

pub static NVIM_PLUGINS_HOME: LazyLock<String> = LazyLock::new(|| format!("{}/plugins", NVIM_DATA_HOME.as_str()));
pub static NVIM_TREESITTER_HOME: LazyLock<String> = LazyLock::new(|| format!("{}/treesitter", NVIM_DATA_HOME.as_str()));
