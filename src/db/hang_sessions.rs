use super::pagination::Paginate;
use crate::constants::SEARCH_METERS;
use crate::models::hang_sessions::HangSession;
use crate::position::Position;
use chrono::{DateTime, Utc};
use rocket::serde::Deserialize;
use sqlx::postgres::PgPool;
use sqlx::Row;

pub async fn find_one(pool: &PgPool, hang_session_id: i32) -> Option<HangSession> {
    sqlx::query_as!(
        HangSession,
        "SELECT id, name, description, hangzone_id, starts_at FROM hang_sessions WHERE id = $1",
        hang_session_id
    )
    .fetch_one(pool)
    .await
    .ok()
}

pub async fn find(
    pool: &PgPool,
    pos: Option<Position>,
    search: Option<String>,
) -> Option<Vec<HangSession>> {
    if let Some(pos) = pos {
        return find_sessions_by_pos(pool, pos).await;
    }
    if let Some(search) = search {
        return find_sessions_by_search(pool, search).await;
    }
    None
}

async fn find_sessions_by_pos(pool: &PgPool, pos: Position) -> Option<Vec<HangSession>> {
    sqlx::query_as!(
        HangSession,
        "
        SELECT hs.id, hs.name, hs.description, hs.hangzone_id, hs.starts_at 
        FROM hang_sessions hs
        INNER JOIN hangzones ON hangzones.id = hs.hangzone_id
        WHERE ST_DWithin(geography, ST_SetSRID(ST_MakePoint($1, $2), 4326), $3)
        ",
        pos.lng,
        pos.lat,
        SEARCH_METERS,
    )
    .fetch_all(pool)
    .await
    .ok()
}

async fn find_sessions_by_search(pool: &PgPool, search: String) -> Option<Vec<HangSession>> {
    sqlx::query_as!(
        HangSession,
        "
        SELECT id, name, description, hangzone_id, starts_at
        FROM hang_sessions
        WHERE name ILIKE $1
        ",
        search,
    )
    .fetch_all(pool)
    .await
    .ok()
}

pub async fn create_one(
    pool: &PgPool,
    name: String,
    description: Option<String>,
    hangzone_id: i32,
    starts_at: Option<DateTime<Utc>>,
) -> Result<HangSession, sqlx::Error> {
    sqlx::query_as!(
        HangSession,
        "
        INSERT INTO hang_sessions 
        (name, description, hangzone_id, starts_at) 
        VALUES ($1, $2, $3, $4)
        RETURNING id, name, description, hangzone_id, starts_at
        ",
        name,
        description,
        hangzone_id,
        starts_at,
    )
    .fetch_one(pool)
    .await
}

pub async fn find_by_hangzone(
    pool: &PgPool,
    hangzone_id: i32,
) -> Result<Vec<HangSession>, sqlx::Error> {
    sqlx::query_as!(
        HangSession,
        "
        SELECT id, name, description, hangzone_id, starts_at 
        FROM hang_sessions hs
        WHERE hangzone_id = $1  
        ",
        hangzone_id
    )
    .fetch_all(pool)
    .await
}
