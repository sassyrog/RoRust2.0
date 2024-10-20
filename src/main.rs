mod config;
mod db;
mod game;
mod message;
mod server;
use std::sync::Arc;

use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::Config::from_file("config/settings.ini")?;
    let db_pool = Arc::new(db::DbPool::new().await?);

    let game_manager = game::GameManager::new(db_pool.clone());

    server::start_server(config, game_manager).await?;

    Ok(())
}
