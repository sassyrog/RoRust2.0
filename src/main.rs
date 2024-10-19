mod config;
mod game;
mod message;
mod server;
use tokio;

#[tokio::main]
#[allow(unused_variables)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::Config::from_file("config/settings.ini")?;
    let game_manager = game::GameManager::new();

    server::start_server(config).await?;

    Ok(())
}
