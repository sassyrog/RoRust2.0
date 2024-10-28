pub mod models;
pub mod schema;

mod auth;
mod config;
mod db;
mod game;
mod queues;

mod message;
mod server;

use std::sync::Arc;
use tokio;

use crate::queues::db_queue;

#[tokio::main(flavor = "multi_thread", worker_threads = 3)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::Config::from_file("config/settings.ini")?;
    let db_pool = Arc::new(db::DbPool::new()?);

    let db_queue = Arc::new(db_queue::create_db_queue(db_pool.clone(), None).await?);

    let game_manager = game::GameManager::new(db_pool.clone(), db_queue.clone());

    server::start_server(config, game_manager).await?;

    Ok(())
}
