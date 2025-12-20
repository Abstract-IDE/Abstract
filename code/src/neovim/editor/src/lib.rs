mod configs;
mod utils;

use nvim_oxi::{self};

use crate::{
    configs::Config,
    utils::runtime, //
};

#[nvim_oxi::plugin]
fn libabstract() -> nvim_oxi::Result<()> {
    Config::new();
    runtime::spawn(async {
        let _ = setup().await;
    });

    Ok(())
}

async fn setup() -> anyhow::Result<String> {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    Ok("Hello from async setup!".into())
}
