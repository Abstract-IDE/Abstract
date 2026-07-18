use std::panic;

use nvim_oxi::{self};

use wl_utils::{panic::panic_test, safe_wrap};
use wp_autogood::{self};
use wp_indent::{self};
use wp_terminal::{self};

use crate::{
    core::{
        configs::Config,
        keymaps, //
    },
    plugins::lazy,
    utils::{
        runtime,
        trace::{self, NotifyLevel, setup_err_hook}, //
    },
};

#[nvim_oxi::plugin]
fn libabstract() -> nvim_oxi::Result<()> {
    // Initialize tracing first so all subsequent code can use tracing macros
    trace::init();
    tracing::info!("Abstract initializing...");

    // Catch any panics so they show as error messages instead of killing Neovim
    let result = panic::catch_unwind(plugins_init);

    match result {
        Ok(Ok(())) => {
            tracing::info!("Abstract initialized successfully");
            Ok(())
        },
        Ok(Err(e)) => {
            let msg = format!("[Abstract] {e}");
            tracing::error!("{msg}");
            trace::vim_notify(&msg, NotifyLevel::Error);
            Ok(())
        },
        Err(panic_info) => {
            let panic_msg = if let Some(s) = panic_info.downcast_ref::<&str>() {
                s.to_string()
            } else if let Some(s) = panic_info.downcast_ref::<String>() {
                s.clone()
            } else {
                "Unknown panic".to_string()
            };
            let log_path = trace::log_file_path();
            let msg = format!("[Abstract] PANIC: {panic_msg}. Check log at {} for backtrace.", log_path.display());
            tracing::error!("{msg}");
            trace::vim_notify(&msg, NotifyLevel::Error);
            Ok(())
        },
    }
}

fn plugins_init() -> nvim_oxi::Result<()> {
    setup_err_hook();
    Config::init()?;
    plugins_setup()?;
    runtime::spawn(async {
        let _ = async_setup().await;
    });

    Ok(())
}

fn plugins_setup() -> nvim_oxi::Result<()> {
    wp_autogood::Init::new().keymaps().autocmds();
    lazy::PluginManager::new()?;

    wp_indent::setup_indent_autocmds()?;
    wp_terminal::setup();

    // ab-nui: hand it the host's live Lua once, then the demo commands.
    wp_ui::init(&nvim_oxi::mlua::lua());
    if let Err(e) = crate::core::ui_demo::setup() {
        trace::vim_notify(&format!("[Abstract] ui_demo: {e}"), NotifyLevel::Error);
    }

    // Registration for testing our panic handler
    panic_test()?;
    // NOTE: this must be called after initilizing PluginManager as mapping depends on external plugin key-map
    // setup which-key for builtin keymaps (keymaps that don't depends on 3rd parties plugins)
    keymaps::MAPPING.register_all()?;

    Ok(())
}

async fn async_setup() -> anyhow::Result<String> {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    Ok("Hello from async setup!".into())
}
