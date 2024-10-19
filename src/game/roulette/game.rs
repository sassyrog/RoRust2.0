use crate::game::{GameType, Room};
use async_trait::async_trait;
use serde_json::json;
use std::collections::HashSet;

pub struct RouletteRoom {
    id: String,
    players: HashSet<String>,
}

#[async_trait]
#[allow(unused_variables)]
impl Room for RouletteRoom {
    fn id(&self) -> &str {
        &self.id
    }

    fn game_type(&self) -> GameType {
        GameType::Roulette
    }

    fn add_player(&mut self, player_id: String) {
        self.players.insert(player_id);
    }

    fn remove_player(&mut self, player_id: String) {
        self.players.remove(&player_id);
    }

    fn is_full(&self) -> bool {
        self.players.len() >= 6
    }

    fn is_empty(&self) -> bool {
        self.players.is_empty()
    }

    fn handle_action(
        &self,
        player_id: String,
        action: String,
        params: serde_json::Value,
    ) -> serde_json::Value {
        json!({})
    }

    async fn broadcast(&self, message: &str) {
        // for player in &self.players {
        //     // send message to player
        // }
    }
}

impl RouletteRoom {
    pub fn new(id: String) -> Self {
        RouletteRoom {
            id,
            players: HashSet::new(),
        }
    }
}
