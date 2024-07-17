use anyhow::Result;
use env_logger::Env;
use pear_daemon::init_service;

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

    let mut service = init_service().await.expect("Failed to initialize service");

    service.run().await
}
