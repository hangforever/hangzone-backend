use crate::models::hangzones::Hangzone;
use chrono::Utc;
use sqlx::postgres::{PgPool, PgRow};

pub async fn create_one(
    pool: &PgPool,
    user_hanger_id: i32,
    friend_id: i32,
) -> Result<PgRow, sqlx::Error> {
    sqlx::query(
        "
INSERT INTO hangzones
 (user_hanger_id,
  friend_user_hanger_id,
  created_at, 
  updated_at)
VALUES ($1, $2, $3, $4)
RETURNING id
    ",
    )
    .bind(user_hanger_id)
    .bind(friend_id)
    .bind(Utc::now())
    .bind(Utc::now())
    .fetch_one(pool)
    .await
}
