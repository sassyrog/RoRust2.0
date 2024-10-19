mod config;
mod game;
mod message;
mod server;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::Config::from_file("config/settings.ini")?;

    server::start_server(config).await?;

    Ok(())
}
