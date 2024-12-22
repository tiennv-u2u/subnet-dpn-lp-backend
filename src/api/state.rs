use crate::db::{repo::Repository, DbPool};
use std::sync::Arc;
// use tokio::sync::broadcast;

#[derive(Clone)]
pub struct AppState {
    pub db: DbPool,
    pub repo: Arc<Repository>,  // Add repository to state
    // pub redis: ConnectionManager,
    // pub ws_tx: broadcast::Sender<String>,
}

impl AppState {
    pub fn new(
        db: DbPool,
        // redis: ConnectionManager,
        // ws_tx: broadcast::Sender<String>,
    ) -> Arc<Self> {
        let repo = Arc::new(Repository::new(db.clone()));
        Arc::new(Self { db, repo })
    }
}

