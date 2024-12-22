use std::sync::Arc;

use crate::api::state::AppState;
use crate::db::models::Session;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct GetSessionsQuery {
    user_addr: String,
    start_time: Option<i64>,
    end_time: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct SessionResponse {
    sessions: Vec<Session>,
}

pub async fn get_user_sessions(
    State(state): State<Arc<AppState>>,
    Query(query): Query<GetSessionsQuery>,
) -> Result<Json<SessionResponse>, StatusCode> {
    // Validate start_time
    if let Some(start_time) = query.start_time {
        if start_time < 0 {
            return Err(StatusCode::BAD_REQUEST);
        }
    }

    // Validate end_time
    if let Some(end_time) = query.end_time {
        if end_time < 0 {
            return Err(StatusCode::BAD_REQUEST);
        }
    }

    // Use repository directly from state
    let sessions = state
        .repo
        .sessions
        .get_user_sessions(
            query.user_addr.clone(),
            query.start_time.unwrap_or(0), // Provide a default if None
            query.end_time.unwrap_or(i64::MAX), // Provide a max range if None
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(SessionResponse { sessions }))
}
