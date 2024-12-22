use sqlx::PgPool;

use crate::db::models::Session;

pub struct SessionRepo {
    pool: PgPool,
}

impl SessionRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_user_sessions(
        &self,
        user_addr: String,
        start_time: i64,
        end_date: i64,
    ) -> Result<Vec<Session>, sqlx::Error> {
        sqlx::query_as!(
            Session,
            r#"
            SELECT *
            FROM sessions
            WHERE user_addr = $1
            AND start_time BETWEEN $2 AND $3
            ORDER BY start_time DESC
            "#,
            user_addr,
            start_time,
            end_date,
        )
        .fetch_all(&self.pool)
        .await
    }
}
