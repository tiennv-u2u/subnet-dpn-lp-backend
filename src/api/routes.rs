
use std::sync::Arc;
use axum::{
    routing::get,
    Router,
};

use super::{handlers::get_user_sessions, AppState};
// use crate::api::{handlers::sessions::*, AppState};

pub fn sessions_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/api/sessions", get(get_user_sessions))
        // .route("/api/sessions/active", get(get_active_session))
}