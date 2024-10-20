use crate::config;
use crate::game::{GameManager, GameType};
use crate::message::{
    parse_client_message, serialize_server_message, ClientMessage, ServerMessage,
};
use futures::{stream::SplitSink, SinkExt, StreamExt};
use serde_json::{json, Error};
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex as TokioMutex;
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message, WebSocketStream};

struct ConnectionContext {
    ws_sender: SplitSink<WebSocketStream<TcpStream>, Message>,
    game_manager: Arc<TokioMutex<GameManager>>,
    player_id: String,
    room_id: String,
}

async fn handle_connection(
    stream: TcpStream,
    game_manager: Arc<TokioMutex<GameManager>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let addr = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    let ws_stream = accept_async(stream).await?;
    let (ws_sender, mut ws_receiver) = ws_stream.split();

    println!("Connection established from {}", addr);

    let mut ctx = ConnectionContext {
        ws_sender,
        game_manager,
        player_id: String::from("player_random_id"),
        room_id: String::new(),
    };

    while let Some(Ok(msg)) = ws_receiver.next().await {
        if let Ok(text) = msg.to_text() {
            match parse_client_message(text) {
                Ok(client_message) => {
                    if !handle_client_message(&mut ctx, client_message, text).await? {
                        break;
                    }
                }
                Err(parse_error) => {
                    handle_parse_error(&mut ctx.ws_sender, parse_error, text).await?;
                }
            }
        }
    }

    Ok(())
}

async fn handle_client_message(
    ctx: &mut ConnectionContext,
    client_message: ClientMessage,
    text: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    match client_message {
        ClientMessage::Quit => handle_quit(ctx).await?,
        ClientMessage::SelectGame { game_type } => handle_select_game(ctx, game_type).await?,
        _ => handle_default(ctx, client_message, text).await?,
    }
    Ok(true)
}

async fn handle_quit(ctx: &mut ConnectionContext) -> Result<(), Box<dyn std::error::Error>> {
    let response = ServerMessage::Echo {
        message: "Goodbye!".into(),
    };
    ctx.ws_sender
        .send(serialize_server_message(&response)?.into())
        .await?;
    Ok(())
}

async fn handle_select_game(
    ctx: &mut ConnectionContext,
    game_type: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let game_type = match game_type.as_str() {
        "POKER" => GameType::Poker,
        "ROULETTE" => GameType::Roulette,
        _ => {
            let response = ServerMessage::Error {
                message: "Invalid game type".to_string(),
            };
            ctx.ws_sender
                .send(serialize_server_message(&response)?.into())
                .await?;
            return Ok(());
        }
    };

    ctx.room_id = {
        let game_manager = ctx.game_manager.lock().await;
        game_manager
            .assign_to_room(ctx.player_id.clone(), game_type)
            .await
    };

    let response = ServerMessage::GameAssigned {
        room_id: ctx.room_id.clone(),
        game_type: game_type.to_str().to_string(),
    };

    ctx.ws_sender
        .send(serialize_server_message(&response)?.into())
        .await?;

    Ok(())
}

async fn handle_default(
    ctx: &mut ConnectionContext,
    client_message: ClientMessage,
    text: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let response = ServerMessage::Echo {
        message: json!({
            "received": text,
            "parsed": client_message
        }),
    };
    ctx.ws_sender
        .send(serialize_server_message(&response)?.into())
        .await?;
    Ok(())
}

async fn handle_parse_error(
    ws_sender: &mut SplitSink<WebSocketStream<TcpStream>, Message>,
    parse_error: Error,
    text: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let error_response = ServerMessage::Error {
        message: format!(
            "Failed to parse message: {}. Raw message: {}",
            parse_error, text
        ),
    };
    ws_sender
        .send(serialize_server_message(&error_response)?.into())
        .await?;
    Ok(())
}

pub async fn start_server(config: config::Config) -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("{}:{}", config.server_address, config.server_port);
    let listener = TcpListener::bind(&addr).await?;
    println!("Server started and listening on {}", addr);

    let game_manager = Arc::new(TokioMutex::new(GameManager::new()));

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
