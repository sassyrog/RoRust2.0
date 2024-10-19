use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum ClientMessage {
    Auth {
        username: String,
        password: String,
    },
    SelectGame {
        game_type: String,
    },
    GameAction {
        action: String,
        params: serde_json::Value,
    },
    Quit,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum ServerMessage {
    AuthSuccess { token: String },
    AuthFailed,
    GameAssigned { room_id: String, game_type: String },
    GameUpdate { state: serde_json::Value },
    Error { message: String },
    Echo { message: serde_json::Value },
}

pub fn parse_client_message(msg: &str) -> Result<ClientMessage, serde_json::Error> {
    serde_json::from_str(msg)
}

pub fn serialize_server_message(msg: &ServerMessage) -> Result<String, serde_json::Error> {
    serde_json::to_string(msg)
}
