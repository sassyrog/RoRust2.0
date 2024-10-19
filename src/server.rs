use crate::config;
use crate::game::{GameManager, GameType};
use crate::message::{
    parse_client_message, serialize_server_message, ClientMessage, ServerMessage,
};
use futures::{SinkExt, StreamExt};
use serde_json::json;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tokio_tungstenite::accept_async;

pub async fn handle_connection(
    stream: TcpStream,
    game_manager: Arc<Mutex<GameManager>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let addr = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    let ws_stream = accept_async(stream).await?;
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    println!("Connection established from {}", addr);

    let mut room_id = String::new();
    let mut player_id = String::from("player_random_id");

    while let Some(Ok(msg)) = ws_receiver.next().await {
        if let Ok(text) = msg.to_text() {
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
                    ClientMessage::SelectGame { game_type } => {
                        let game_type = match game_type.as_str() {
                            "POKER" => GameType::Poker,
                            "ROULETTE" => GameType::Roulette,
                            _ => {
                                let response = ServerMessage::Error {
                                    message: "Invalid game type".to_string(),
                                };
                                ws_sender
                                    .send(serialize_server_message(&response)?.into())
                                    .await?;
                                continue;
                            }
                        };

                        room_id = {
                            let game_manager = game_manager.lock().await;
                            game_manager
                                .assign_to_room(player_id.clone(), game_type)
                                .await
                        };

                        // Create the response after releasing the lock
                        let response = ServerMessage::GameAssigned {
                            room_id: room_id.clone(),
                            game_type: game_type.to_str().to_string(),
                        };

                        ws_sender
                            .send(serialize_server_message(&response)?.into())
                            .await?;
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

    let game_manager = Arc::new(Mutex::new(GameManager::new()));

    while let Ok((stream, _)) = listener.accept().await {
        let game_manager = Arc::clone(&game_manager);

        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, game_manager).await {
                eprintln!("Error handling connection: {:?}", e);
            }
        });
    }

    Ok(())
}
