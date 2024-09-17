use log::info;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use crate::config::Config;

async fn process(socket: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    tokio::spawn(async move {
        let mut buffer = [0, 1024];
        loop {
            let n = match socket.read(&mut buffer).await {
                Ok(n) if n == 0 => return,
                Err(e) => {
                    info!("Failed to read from socket; error = {:?}", e);
                }
            };
        }
    });
    Ok(())
}

pub async fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_file("Config/Settings.ini")?;
    let server_address = format!("{}:{}", config.server_address, config.server_port);
    info!("Starting server at {}", server_address);
    let listener = TcpListener::bind(server_address).await?;

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}
