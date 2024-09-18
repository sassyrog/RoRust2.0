#![allow(unused)]
use crate::config::Config;
use futures::{stream, TryStreamExt};
use futures::{SinkExt, StreamExt};
use log::info;
use tokio::{
    net::{TcpListener, TcpStream},
    sync::broadcast::{self, Sender},
};
use tokio_util::codec::{FramedRead, FramedWrite, LinesCodec};

async fn handle_connection(mut socket: TcpStream, tx: Sender<String>) -> anyhow::Result<()> {
    let (mut reader, mut writer) = socket.split();
    let mut stream = FramedRead::new(&mut reader, LinesCodec::new());
    let mut sink = FramedWrite::new(&mut writer, LinesCodec::new());

    let mut rx = tx.subscribe();

    sink.send("Welcome to the chat!".to_string()).await?;

    loop {
        tokio::select! {
            in_msg = stream.try_next() => {
                let in_msg = match in_msg {
                    Ok(Some(msg)) => msg,
                    Ok(None) => break,
                    Err(e) => return Err(e.into()),
                };

                if in_msg.starts_with("/quit") {
                    sink.send("Goodbye!".to_string()).await?;
                    break;
                } else {
                    sink.send(format!("Message: {}", in_msg));
                }
            }

            peer_msg = rx.recv() => {
                sink.send(peer_msg?).await?;
            },
        }
    }

    Ok(())
}

pub async fn start_server() -> anyhow::Result<()> {
    let config = Config::from_file("Config/settings.ini")?;
    let connection_string = format!("{}:{}", &config.server_address, &config.server_port);
    let listener = TcpListener::bind(connection_string).await?;

    let (tx, _rx) = broadcast::channel::<String>(32);

    info!("Server listening on {}", &config.server_address);

    loop {
        let (socket, _) = listener.accept().await?;
        let tx = tx.clone();
        tokio::spawn(handle_connection(socket, tx));
    }

    Ok(())
}
