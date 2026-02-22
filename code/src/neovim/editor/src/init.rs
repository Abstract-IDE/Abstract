use std::panic;

use nvim_oxi::{self};
use wc_indent::setup_indent_autocmds;
use wp_autogood::{self};

use crate::{
    core::{
        configs::Config,
        keymaps, //
    },
    plugins::lazy,
    utils::{
        runtime,
        trace::{self, NotifyLevel}, //
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
            let msg = format!("[Abstract] PANIC: {panic_msg}");
            tracing::error!("{msg}");
            trace::vim_notify(&msg, NotifyLevel::Error);
            Ok(())
        },
    }
}

fn plugins_init() -> nvim_oxi::Result<()> {
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

    // indent
    setup_indent_autocmds()?;

    // Register builtin keymaps
    keymaps::MAPPING.set_map(keymaps::Key::Builtin)?;
    // NOTE: this must be called after initilizing PluginManager as mapping depends on external plugin key-map
    // setup which-key for builtin keymaps (keymaps that don't depends on 3rd parties plugins)
    keymaps::MAPPING.register_all()?;

    Ok(())
}

async fn async_setup() -> anyhow::Result<String> {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    Ok("Hello from async setup!".into())
}
