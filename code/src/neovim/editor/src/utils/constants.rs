#![allow(unused)]

use std::{env, sync::LazyLock};

use crate::utils::nvim_path::nvim_stdpath;

pub static NVIM_CONFIG_HOME: LazyLock<String> = LazyLock::new(|| nvim_stdpath("config").to_string_lossy().to_string());
pub static NVIM_DATA_HOME: LazyLock<String> =
    LazyLock::new(|| format!("{}/rust", nvim_stdpath("data").to_string_lossy()));

pub static NVIM_TREESITTER_HOME: LazyLock<String> = LazyLock::new(|| format!("{}/treesitter", NVIM_DATA_HOME.as_str()));

//
//
// MP = Plugin Manager
//

pub static NVIM_PM_INSTALL_HOME: LazyLock<String> = LazyLock::new(|| format!("{}/plugins", NVIM_DATA_HOME.as_str()));
pub static NVIM_PM_LOCK: LazyLock<String> = LazyLock::new(|| env!("CARGO_MANIFEST_DIR").to_string());
