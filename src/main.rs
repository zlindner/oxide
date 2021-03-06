mod character;
mod client;
mod config;
mod crypto;
mod db;
mod handler;
mod login;
mod net;
mod shutdown;
mod world;

use log::LevelFilter;
use simple_logger::SimpleLogger;
use std::env;
use std::sync::Arc;
use tokio::{net::TcpListener, signal};
use world::World;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Shared {
    worlds: Vec<World>,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    SimpleLogger::new()
        .with_module_level("tokio_util", LevelFilter::Debug)
        .with_module_level("mio", LevelFilter::Debug)
        .env()
        .init()
        .unwrap();

    let shared = Arc::new(Shared {
        worlds: world::load_worlds(),
    });

    let listener = TcpListener::bind(&env::var("SERVER_ADDRESS").unwrap()).await?;
    login::server::start(listener, signal::ctrl_c(), &shared).await?;

    Ok(())
}
