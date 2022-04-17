use crate::models::user_hangers::{StatusHang, UserHanger};
use crate::position::Position;
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

pub async fn create_one(pool: &PgPool, user_body: UserBody) -> Result<UserHanger, sqlx::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Scrypt
        .hash_password(user_body.password.as_bytes(), &salt)
        .expect("hashing error")
        .to_string();
    sqlx::query_as!(
        UserHanger,
        r#"
        INSERT INTO user_hangers
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
        VALUES      ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        RETURNING id, first_name, last_name, alias, email_address, status_hang as "status_hang!: StatusHang", status_description, icon_url, hash, current_hangzone_id, created_at, updated_at
    "#,
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
    .await
}

pub async fn login(pool: &PgPool, email: &str, password: &str) -> Option<UserHanger> {
    let user_hanger = sqlx::query("SELECT * FROM user_hangers WHERE email_address = $1")
        .bind(email)
        .map(|row| row_to_user_hanger_json(row))
        .fetch_one(pool)
        .await;
    if let Ok(user_hanger) = user_hanger {
        let parsed_hash = PasswordHash::new(&user_hanger.hash).unwrap();
        let password_match = Scrypt
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_err(|e| eprintln!("login_user: scrypt_check: {}", e))
            .is_ok();
        if password_match {
            return Some(user_hanger);
        }
    }
    None
}

pub async fn update(pool: &PgPool, user_hanger: UserHanger) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "
        UPDATE user_hangers 
        SET 
          first_name = $1,
          last_name = $2,
          alias = $3,
          email_address = $4,
          status_hang = $5,
          status_description = $6,
          icon_url = $7,
          hash = $8,
          current_hangzone_id = $9
        WHERE id = $10
        RETURNING id
        ",
        user_hanger.first_name,
        user_hanger.last_name,
        user_hanger.alias,
        user_hanger.email_address,
        user_hanger.status_hang as StatusHang,
        user_hanger.status_description,
        user_hanger.icon_url,
        user_hanger.hash,
        user_hanger.current_hangzone_id,
        user_hanger.id,
    )
    .fetch_one(pool)
    .await?;
    Ok(())
}

pub async fn update_hangzone_id(
    pool: &PgPool,
    id: i32,
    hangzone_id: i32,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "
        UPDATE user_hangers 
        SET 
          current_hangzone_id = $1
        WHERE id = $2
        RETURNING id
        ",
        hangzone_id,
        id,
    )
    .fetch_one(pool)
    .await?;
    Ok(())
}

pub async fn update_geography(
    pool: &PgPool,
    pos: Position,
    user_hanger_id: i32,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "
        UPDATE user_hangers 
        SET geography = ST_MakePoint($1, $2) 
        WHERE id = $3
        RETURNING id
        ",
    )
    .bind(pos.lng)
    .bind(pos.lat)
    .bind(user_hanger_id)
    .fetch_one(pool)
    .await?;
    Ok(())
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
        hash: row.get("hash"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}
