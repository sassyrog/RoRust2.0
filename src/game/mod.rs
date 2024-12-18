pub mod game_types;
mod player;
mod poker;
mod room;
mod roulette;

use serde_json::json;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::db::DbPool;
use crate::queues::db_queue::DbQueue;

pub use game_types::GameType;
pub use room::Room;

pub struct GameManager {
    rooms: RwLock<HashMap<String, Box<dyn Room + Send + Sync>>>,
    db_pool: Arc<DbPool>,
    db_queue: Arc<DbQueue>,
}

impl GameManager {
    pub fn new(db_pool: Arc<DbPool>, db_queue: Arc<DbQueue>) -> Self {
        GameManager {
            rooms: RwLock::new(HashMap::new()),
            db_pool,
            db_queue,
        }
    }

    pub async fn assign_to_room(&self, player_id: String, game_type: GameType) -> String {
        let mut rooms = self.rooms.write().await;
        let room = rooms
            .values_mut()
            .find(|r| r.game_type() == game_type && !r.is_full());

        match room {
            Some(room) => {
                room.add_player(player_id);
                room.id().to_string()
            }
            None => {
                let room_id = Uuid::new_v4().to_string();

                let new_room: Box<dyn Room + Send + Sync> = match game_type {
                    GameType::Poker(_) => {
                        Box::new(poker::PokerRoom::new(room_id.clone(), game_type.clone()))
                    }
                    GameType::Roulette(_) => Box::new(roulette::RouletteRoom::new(
                        room_id.clone(),
                        game_type.clone(),
                    )),
                    _ => panic!("Unsupported game type"),
                };
                // new_room.add_player(player_id); //TODO: Implement this method
                rooms.insert(room_id.clone(), new_room);
                room_id
            }
        }
    }

    pub async fn handle_action(
        &self,
        room_id: String,
        player_id: String,
        action: String,
        params: serde_json::Value,
    ) -> serde_json::Value {
        let rooms = self.rooms.read().await;
        if let Some(room) = rooms.get(&room_id) {
            room.handle_action(player_id, action, params)
        } else {
            json!({ "error": "Room not found" })
        }
    }

    pub async fn remove_player(&self, room_id: String, player_id: String) {
        let mut rooms = self.rooms.write().await;
        if let Some(room) = rooms.get_mut(&room_id) {
            room.remove_player(player_id);
            if room.is_empty() {
                rooms.remove(&room_id);
            }
        }
    }
}
