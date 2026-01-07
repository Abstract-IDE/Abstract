use nvim_oxi::{self};
use wp_autogood::{self};

use crate::{
    core::{
        configs::Config,
        keymaps::{
            KEYMAPS, //
            MapKey,
            MapLoader,
        },
    },
    plugins::lazy,
    utils::runtime, //
};

#[nvim_oxi::plugin]
fn libabstract() -> nvim_oxi::Result<()> {
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

    // Register builtin keymaps
    KEYMAPS.set_map(MapKey::Builtin)?;
    // NOTE: this must be called after initilizing PluginManager as mapping depends on external plugin key-map
    // setup which-key for builtin keymaps (keymaps that don't depends on 3rd parties plugins)
    MapLoader::register_all()?;

    Ok(())
}

async fn async_setup() -> anyhow::Result<String> {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    Ok("Hello from async setup!".into())
}
