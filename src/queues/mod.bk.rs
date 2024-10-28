use crate::db::DbPool;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::mpsc;

#[derive(Debug, Serialize, Deserialize)]
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
    // TODO: More db operations to come :)
}

pub struct DbQueue {
    sender: mpsc::Sender<DbOperation>,
}

impl DbQueue {
    pub fn new(db_pool: Arc<DbPool>) -> Self {
        let (sender, mut receiver) = mpsc::channel(100); // Adjust buffer size as needed

        tokio::spawn(async move {
            while let Some(op) = receiver.recv().await {
                match op {
                    // TODO: Handle operations
                    _ => {
                        eprintln!("Unrecognized database operation: {:?}", op);
                    }
                }
            }
        });

        DbQueue { sender }
    }

    pub async fn queue_operation(&self, operation: DbOperation) {
        if let Err(e) = self.sender.send(operation).await {
            eprintln!("Failed to queue database operation: {}", e);
        }
    }
}
