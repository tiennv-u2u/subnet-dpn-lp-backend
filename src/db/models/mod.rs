// models are defined in here

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub user_addr: String,
    pub start_time: i64,
    pub end_time: Option<i64>,
    pub earned_lp: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub addr: String,
    pub total_lp: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct CreateSession {
//     pub user_addr: String,
//     pub start_time: i64,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct UpdateSession {
//     pub end_time: i64,
//     pub earned_lp: i32,
// }
