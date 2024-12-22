use sqlx::PgPool;
use std::sync::Arc;

mod session;
mod user;

pub use session::SessionRepo;
pub use user::UserRepo;


#[derive(Clone)]
pub struct Repository {
    pub sessions: Arc<SessionRepo>,
    pub users: Arc<UserRepo>,
}

impl Repository {
    pub fn new(pool: PgPool) -> Self {
        Self {
            sessions: Arc::new(SessionRepo::new(pool.clone())),
            users: Arc::new(UserRepo::new(pool.clone())),
        }
    }
}