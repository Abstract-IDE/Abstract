use nvim_oxi::{self};

use wsp_autogood::{self};

use crate::{
    configs::Config,
    utils::runtime, //
};

#[nvim_oxi::plugin]
fn libabstract() -> nvim_oxi::Result<()> {
    Config::init()?;
    plugins_setup();
    runtime::spawn(async {
        let _ = async_setup().await;
    });

    Ok(())
}

fn plugins_setup() {
    wsp_autogood::Init::new().keymaps().autocmds();
}

async fn async_setup() -> anyhow::Result<String> {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    Ok("Hello from async setup!".into())
}
