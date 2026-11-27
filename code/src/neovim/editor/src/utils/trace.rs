use std::{fs, panic, path::PathBuf, sync::Once};

use nvim_oxi::mlua;
use tracing::{self};
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

/// Show one of Abstract's own messages as an ab-nui toast.
///
/// Falls back to `vim.notify` when ab-nui is not initialised yet (an error
/// during the earliest part of startup) or the toast cannot be shown.
pub fn vim_notify(msg: &str, level: NotifyLevel) {
    if toast(msg, &level) {
        return;
    }
    let lua = mlua::lua();
    let level_str = match level {
        NotifyLevel::Error => "vim.log.levels.ERROR",
        NotifyLevel::Warn => "vim.log.levels.WARN",
        NotifyLevel::Info => "vim.log.levels.INFO",
    };
    let _ = lua.load(format!("vim.notify([==[{msg}]==], {level_str})")).exec();
}

/// Show `msg` as an ab-nui toast. Returns whether it was shown.
fn toast(msg: &str, level: &NotifyLevel) -> bool {
    use wp_ui::view::{Level, NotifyOptions, notify};

    if !wp_ui::is_initialized() {
        return false;
    }
    let (level, timeout_ms) = match level {
        NotifyLevel::Error => (Level::Error, 10_000),
        NotifyLevel::Warn => (Level::Warn, 6_000),
        NotifyLevel::Info => (Level::Info, 4_000),
    };
    // The toast is already titled "Abstract".
    let msg = msg.trim().strip_prefix("[Abstract]").unwrap_or(msg).trim();
    let opts = NotifyOptions { title: Some("Abstract".into()), level, timeout_ms: Some(timeout_ms), width: 60 };
    notify(msg, opts).is_ok()
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

// Set a panic hook to capture and log panics before they kill Neovim
pub fn setup_err_hook() {
    panic::set_hook(Box::new(|panic_info| {
        let loc = panic_info
            .location()
            .map(|l| format!("{}:{}", l.file(), l.line()))
            .unwrap_or_else(|| "unknown".to_string());

        let msg = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            s.to_string()
        } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            s.clone()
        } else {
            "Unknown panic".to_string()
        };

        let backtrace = std::backtrace::Backtrace::force_capture();
        let full_msg = format!(
            "\n=== === ===\n\
             PANIC at {loc}\n\
             Message: {msg}\n\
             \n\
             Backtrace:\n\
             {backtrace}\n\
             === === ===\n"
        );

        // Log using tracing so it's in the standard log file
        tracing::error!("{full_msg}");

        // Write synchronously to a dedicated crash file in NVIM_DATA_HOME
        let crash_path = constants::NVIM_DATA_HOME.as_str();
        let crash_file = PathBuf::from(crash_path).join("abstract-crash.log");
        if let Some(parent) = crash_file.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let _ = fs::write(&crash_file, &full_msg);

        // The full report goes to `:messages`; a short toast points at it. The
        // toast is shown on the next tick rather than from inside the hook:
        // a second panic while this one is being handled would abort Neovim.
        vim_notify_error_report(&full_msg);
        if wp_ui::is_initialized() {
            let summary = format!("PANIC at {loc}: {msg}\nFull report in :messages and {}", crash_file.display());
            let _ = wp_ui::nvim::schedule(move || vim_notify(&summary, NotifyLevel::Error));
        }
    }));
}
