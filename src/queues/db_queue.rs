use crate::db::DbPool;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::info;

use crate::queues::{OperationProcessor, Queue, QueueConfig, QueueError};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DbOperation {
    CreatePlayer {
        id: String,
        username: String,
        password_hash: String,
    },
    UpdateGameProgress {
        player_id: String,
        game_type: String,
        state: String,
    },
}

pub struct DbOperationProcessor {
    db_pool: Arc<DbPool>,
}

#[async_trait]
impl OperationProcessor for DbOperationProcessor {
    type Operation = DbOperation;

    async fn process(&self, operation: DbOperation) -> Result<(), String> {
        let conn = self
            .db_pool
            .pool
            .get()
            .map_err(|e| format!("Database error: {}", e))?;

        match operation {
            DbOperation::CreatePlayer {
                id,
                username,
                password_hash,
            } => {
                info!("Creating player: {}", username);
                Ok(())
            }
            DbOperation::UpdateGameProgress {
                player_id,
                game_type,
                state,
            } => {
                info!("Updating game progress for player: {}", player_id);
                Ok(())
            }
        }
    }

    fn operation_type(&self) -> &'static str {
        "db_operation"
    }
}

// Type alias for DbQueue
pub type DbQueue = Queue<DbOperationProcessor>;

// Helper function to create DbQueue
pub async fn create_db_queue(
    db_pool: Arc<DbPool>,
    config: Option<QueueConfig>,
) -> Result<DbQueue, QueueError> {
    let config = config.unwrap_or_else(|| {
        QueueConfig::builder()
            .queue_name("db_operations".to_string())
            .exchange_name("game_operations".to_string())
            .consumer_tag("db_consumer".to_string())
            .build()
    });

    let processor = DbOperationProcessor { db_pool };
    Queue::new(processor, config).await
}
