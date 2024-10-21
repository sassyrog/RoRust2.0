pub mod models;
pub mod schema;

mod config;
mod db;
mod db_queue;
mod game;

mod message;
mod server;

use std::sync::Arc;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::Config::from_file("config/settings.ini")?;
    let db_pool = Arc::new(db::DbPool::new()?);
    let db_queue = Arc::new(db_queue::DbQueue::new(db_pool.clone()));

    let game_manager = game::GameManager::new(db_pool.clone(), db_queue.clone());

    server::start_server(config, game_manager).await?;

    Ok(())
}
