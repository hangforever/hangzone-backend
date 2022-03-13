use crate::models::friends::Friend;
use chrono::Utc;
use sqlx::postgres::{PgPool, PgRow};
use sqlx::Row;

pub async fn find(pool: &PgPool, user_hanger_id: i32) -> Option<Vec<Friend>> {
    // TODO: change to compile time checked query!
    let friends = sqlx::query(
        "
        SELECT * 
        FROM user_hangers 
        INNER JOIN friends ON user_hangers.id = friends.user_hanger_id
        WHERE friends.user_hanger_id = $1 
    ",
    )
    .bind(user_hanger_id)
    .map(|row| row_to_friend(row))
    .fetch_all(pool)
    .await;

    match friends {
        Ok(friends) => return Some(friends),
        Err(e) => eprintln!("{}", e),
    }
    None
}
pub async fn create_one(
    pool: &PgPool,
    user_hanger_id: i32,
    friend_id: i32,
) -> Result<PgRow, sqlx::Error> {
    sqlx::query(
        "
INSERT INTO friends
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

fn row_to_friend(row: sqlx::postgres::PgRow) -> Friend {
    Friend {
        first_name: row.get("first_name"),
        last_name: row.get("last_name"),
        alias: row.get("alias"),
        email_address: row.get("email_address"),
        status_hang: row.get("status_hang"),
        status_description: row.get("status_description"),
        icon_url: row.get("icon_url"),
        current_hangzone_id: row.get("current_hangzone_id"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}
