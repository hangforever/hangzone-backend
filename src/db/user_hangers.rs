use crate::models::user_hangers::{StatusHang, UserHangerJson};
use rocket::serde::Deserialize;
use sqlx::postgres::PgPool;
use sqlx::Row;

pub async fn find_one(pool: &PgPool, user_hanger_id: i32) -> Option<UserHangerJson> {
    let row = sqlx::query("SELECT * FROM user_hangers WHERE id = $1")
        .bind(user_hanger_id)
        .fetch_one(pool)
        .await;
    if let Ok(r) = row {
        return Some(row_to_user_hanger_json(r));
    }
    None
}

#[derive(Deserialize)]
pub struct UserBody {
    first_name: String,
    last_name: String,
    alias: String,
    email_address: Option<String>,
    status_hang: StatusHang,
    status_description: Option<String>,
    icon_url: Option<String>,
    // geography: (f32, f32),
    current_hangzone_id: Option<i32>,
}

pub async fn create_one(pool: &PgPool, user_body: UserBody) -> Option<UserHangerJson> {
    None
}

fn row_to_user_hanger_json(row: sqlx::postgres::PgRow) -> UserHangerJson {
    UserHangerJson {
        id: row.get("id"),
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
