#![allow(unused)]

use std::{
    env,
    path::PathBuf, //
};

/// Return the user's home directory (cross-platform)
fn home_dir() -> PathBuf {
    if cfg!(windows) {
        env::var_os("USERPROFILE").map(PathBuf::from).expect("USERPROFILE not set")
    } else {
        env::var_os("HOME").map(PathBuf::from).expect("HOME not set")
    }
}

/// XDG helper with fallback
fn xdg(var: &str, fallback: &str) -> PathBuf {
    env::var_os(var).map(PathBuf::from).unwrap_or_else(|| home_dir().join(fallback))
}

/// Neovim-compatible stdpath implementation
pub fn nvim_stdpath(kind: &str) -> PathBuf {
    match kind {
        // === UNIX (Linux / macOS)  === //
        #[cfg(not(windows))]
        "config" => xdg("XDG_CONFIG_HOME", ".config").join("nvim"),

        #[cfg(not(windows))]
        "data" => xdg("XDG_DATA_HOME", ".local/share").join("nvim"),

        #[cfg(not(windows))]
        "cache" => xdg("XDG_CACHE_HOME", ".cache").join("nvim"),

        #[cfg(not(windows))]
        "state" => xdg("XDG_STATE_HOME", ".local/state").join("nvim"),

        // === WINDOWS === //
        #[cfg(windows)]
        "config" => env::var_os("LOCALAPPDATA").map(PathBuf::from).expect("LOCALAPPDATA not set").join("nvim"),

        #[cfg(windows)]
        "data" => env::var_os("LOCALAPPDATA").map(PathBuf::from).expect("LOCALAPPDATA not set").join("nvim-data"),

        #[cfg(windows)]
        "cache" => env::var_os("TEMP").map(PathBuf::from).expect("TEMP not set").join("nvim"),

        #[cfg(windows)]
        "state" => env::var_os("LOCALAPPDATA").map(PathBuf::from).expect("LOCALAPPDATA not set").join("nvim-state"),

        // === ALL PLATFORMS ===
        "runtime" => env::var_os("VIMRUNTIME")
            .map(PathBuf::from)
            .expect("VIMRUNTIME not set (must be run inside Neovim)"),

        _ => panic!("invalid stdpath: {kind}"),
    }
}

pub fn nvim_stdpath_str(kind: &str) -> String {
    nvim_stdpath(kind).to_string_lossy().into_owned()
}
