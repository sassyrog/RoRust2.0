use crate::config;
use futures::{SinkExt, StreamExt};
use serde_json::json;
// use std::sync::Arc;
use crate::message::{
    parse_client_message, serialize_server_message, ClientMessage, ServerMessage,
};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;

pub async fn handle_connection(stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let addr = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    let ws_stream = accept_async(stream).await?;
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    println!("Connection established from {}", addr);

    while let Some(Ok(msg)) = ws_receiver.next().await {
        if let Ok(text) = msg.to_text() {
            // let client_message = parse_client_message(&text)?;

            match parse_client_message(text) {
                Ok(client_message) => match client_message {
                    ClientMessage::Quit => {
                        let response = ServerMessage::Echo {
                            message: "Goodbye!".into(),
                        };
                        ws_sender
                            .send(serialize_server_message(&response)?.into())
                            .await?;
                        break;
                    }
                    _ => {
                        let response = ServerMessage::Echo {
                            message: json!({
                                "received": text,
                                "parsed": client_message
                            }),
                        };
                        ws_sender
                            .send(serialize_server_message(&response)?.into())
                            .await?;
                    }
                },
                Err(parse_error) => {
                    let error_response = ServerMessage::Error {
                        message: format!(
                            "Failed to parse message: {}. Raw message: {}",
                            parse_error, text
                        ),
                    };
                    ws_sender
                        .send(serialize_server_message(&error_response)?.into())
                        .await?;
                }
            }
        }
    }

    Ok(())
}

pub async fn start_server(config: config::Config) -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("{}:{}", config.server_address, config.server_port);
    let listener = TcpListener::bind(&addr).await?;

    println!("Server started and listening on {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream).await {
                eprintln!("Error handling connection: {:?}", e);
            }
        });
    }

    Ok(())
}
