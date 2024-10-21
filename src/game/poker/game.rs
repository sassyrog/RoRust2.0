use crate::game::{game_types::*, Room};
use async_trait::async_trait;
use serde_json::json;
use std::collections::HashSet;

pub struct PokerRoom {
    id: String,
    players: HashSet<String>,
    game_type: GameType,
}

#[async_trait]
#[allow(unused_variables)]
impl Room for PokerRoom {
    fn id(&self) -> &str {
        &self.id
    }

    fn game_type(&self) -> GameType {
        self.game_type.clone()
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

impl PokerRoom {
    pub fn new(id: String, game_type: GameType) -> PokerRoom {
        PokerRoom {
            id,
            players: HashSet::new(),
            game_type,
        }
    }
}
