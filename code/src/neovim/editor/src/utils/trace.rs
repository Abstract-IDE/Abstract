use std::{fs, path::PathBuf, sync::Once};

use nvim_oxi::mlua;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

use crate::utils::constants;

static INIT: Once = Once::new();

/// Initialize the tracing/logging system.
///
/// - **File logging**: All levels written to `{NVIM_DATA_HOME}/abstract.log`
/// - **Env filter**: Respects `ABSTRACT_LOG` env var (defaults to `info`)
///
/// Safe to call multiple times — only initializes once.
pub fn init() {
    INIT.call_once(|| {
        let log_path = log_file_path();

        // Ensure the parent directory exists
        if let Some(parent) = log_path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        // Open log file (append mode)
        let file = match fs::OpenOptions::new().create(true).append(true).open(&log_path) {
            Ok(f) => f,
            Err(_) => {
                // If we can't open the log file, just skip file logging
                eprintln!("[Abstract] Failed to open log file: {}", log_path.display());
                return;
            },
        };

        // Env filter: ABSTRACT_LOG=debug, ABSTRACT_LOG=trace, etc.
        let filter = EnvFilter::try_from_env("ABSTRACT_LOG").unwrap_or_else(|_| EnvFilter::new("info"));

        let file_writer = std::sync::Mutex::new(file);

        let file_layer = fmt::layer()
            .with_writer(file_writer)
            .with_ansi(false)
            .with_target(true)
            .with_thread_ids(false)
            .with_thread_names(false);

        tracing_subscriber::registry().with(filter).with(file_layer).init();
    });
}

/// Get the log file path: `{NVIM_DATA_HOME}/abstract.log`
pub fn log_file_path() -> PathBuf {
    PathBuf::from(constants::NVIM_DATA_HOME.as_str()).join("abstract.log")
}

/// Send a notification to Neovim's `vim.notify`.
/// Uses Lua long strings `[==[...]==]` to safely handle any content.
pub fn vim_notify(msg: &str, level: NotifyLevel) {
    let lua = mlua::lua();
    let level_str = match level {
        NotifyLevel::Error => "vim.log.levels.ERROR",
        NotifyLevel::Warn => "vim.log.levels.WARN",
        NotifyLevel::Info => "vim.log.levels.INFO",
    };
    let _ = lua.load(format!("vim.notify([==[{msg}]==], {level_str})")).exec();
}

#[allow(unused)]
pub enum NotifyLevel {
    Error,
    Warn,
    Info,
}

/// Send a colored error report to Neovim using nvim_echo with highlight groups.
pub fn vim_notify_error_report(report: &str) {
    let lua = mlua::lua();

    let mut chunks = String::from("vim.api.nvim_echo({");
    for line in report.lines() {
        let escaped = line.replace('\\', "\\\\").replace('"', "\\\"");
        let hl = if line.starts_with("[Abstract]") {
            "ErrorMsg"
        } else if line.contains("-->") {
            "Directory"
        } else if line.starts_with('>') {
            "WarningMsg"
        } else if line.starts_with("  =") {
            "ErrorMsg"
        } else {
            "Comment"
        };
        chunks.push_str(&format!("{{\"{escaped}\\n\", \"{hl}\"}},"));
    }
    chunks.push_str("}, true, {})");

    let _ = lua.load(&chunks).exec();
}
