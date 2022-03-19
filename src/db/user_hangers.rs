use crate::models::user_hangers::{StatusHang, UserHanger};
use chrono::{DateTime, Utc};
use rocket::serde::Deserialize;
use scrypt::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Scrypt,
};
use sqlx::postgres::PgPool;
use sqlx::Row;

#[derive(Deserialize, Debug)]
pub struct UserBody {
    alias: String,
    email_address: String,
    password: String,
}

pub async fn find_one(pool: &PgPool, user_hanger_id: i32) -> Result<UserHanger, sqlx::Error> {
    sqlx::query("SELECT * FROM user_hangers WHERE id = $1")
        .bind(user_hanger_id)
        .map(|row| row_to_user_hanger_json(row))
        .fetch_one(pool)
        .await
}

pub async fn find(pool: &PgPool, hangzone_slug: &str) -> Result<Vec<UserHanger>, sqlx::Error> {
    sqlx::query("SELECT * FROM user_hangers INNER JOIN hangzones ON user_hangers.current_hangzone_id = hangzones.id WHERE hangzones.slug = $1")
        .bind(hangzone_slug)
        .map(|row| row_to_user_hanger_json(row))
        .fetch_all(pool)
        .await
}

pub async fn create_one(pool: &PgPool, user_body: UserBody) -> Result<(), sqlx::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Scrypt
        .hash_password(user_body.password.as_bytes(), &salt)
        .expect("hashing error")
        .to_string();
    sqlx::query!(
        "
insert into user_hangers
            (first_name,
             last_name,
             alias,
             email_address,
             status_hang,
             status_description,
             icon_url,
             hash,
             created_at,
             updated_at)
values      ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
    ",
        "Anonymous".to_string(),
        "Hanger".to_string(),
        user_body.alias,
        user_body.email_address,
        0,
        //user_body.status_hang as StatusHang,
        "No description".to_string(),
        // TODO: Have a default icon
        String::new(),
        hash,
        Utc::now(),
        Utc::now(),
    )
    .fetch_one(pool)
    .await?;
    Ok(())
}

pub async fn login(pool: &PgPool) -> Option<UserHanger> {
    None
}

pub fn row_to_user_hanger_json(row: sqlx::postgres::PgRow) -> UserHanger {
    UserHanger {
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
