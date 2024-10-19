use crate::game::GameType;
use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
pub trait Room: Send + Sync {
    fn id(&self) -> &str;
    fn game_type(&self) -> GameType;
    fn add_player(&mut self, player_id: String);
    fn remove_player(&mut self, player_id: String);
    fn is_full(&self) -> bool;
    fn is_empty(&self) -> bool;
    fn handle_action(&self, player_id: String, action: String, params: Value) -> Value;
    async fn broadcast(&self, message: &str);
}
