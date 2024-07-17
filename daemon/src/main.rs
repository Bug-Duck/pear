use anyhow::{Ok, Result};
use dht::init_dht;
use env_logger::Env;

mod dht;

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() -> Result<()> {
    // TODO: use tracing?
    // let _ = tracing_subscriber::fmt()
    //     .with_env_filter(EnvFilter::from_default_env())
    //     .try_init();

    env_logger::Builder::from_env(Env::default().default_filter_or("trace")).init();

    let mut dht = init_dht("alice".to_string()).await?;
    dht.main_loop().await;

    Ok(())
}