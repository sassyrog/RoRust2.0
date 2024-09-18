use log::info;
mod config;
mod server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    info!("Starting gambling game server...");
    server::start_server().await?;
    Ok(())
}
