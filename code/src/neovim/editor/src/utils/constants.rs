#![allow(unused)]

use std::sync::LazyLock;

use crate::utils::nvim_path;
use nvim_oxi::mlua;

fn vim_global(key: &str) -> Option<String> {
    let lua = mlua::lua();
    lua.load(format!("return vim.g.{key}")).eval::<String>().ok()
}

pub static ABSTRACT_ROOT: LazyLock<String> =
    LazyLock::new(|| vim_global("ABSTRACT_ROOT").unwrap_or_else(|| nvim_path::nvim_stdpath_str("abstract-repo")));

pub static ABSTRACT_DATA: LazyLock<String> =
    LazyLock::new(|| vim_global("ABSTRACT_DATA").unwrap_or_else(|| nvim_path::nvim_stdpath_str("data")));

pub static NVIM_DATA_HOME: LazyLock<String> = LazyLock::new(|| format!("{}/data", ABSTRACT_DATA.as_str()));

pub static NVIM_TREESITTER_HOME: LazyLock<String> = LazyLock::new(|| {
    vim_global("ABSTRACT_TREESITTER_HOME").unwrap_or_else(|| format!("{}/treesitter", NVIM_DATA_HOME.as_str()))
});

pub static NVIM_PM_INSTALL_HOME: LazyLock<String> = LazyLock::new(|| {
    vim_global("ABSTRACT_PM_INSTALL_HOME").unwrap_or_else(|| format!("{}/plugins", NVIM_DATA_HOME.as_str()))
});

pub static NVIM_PM_LOCK: LazyLock<String> =
    LazyLock::new(|| vim_global("ABSTRACT_PM_LOCK").unwrap_or_else(|| env!("CARGO_MANIFEST_DIR").to_string()));
