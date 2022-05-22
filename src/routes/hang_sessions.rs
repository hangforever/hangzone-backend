use crate::auth::Auth;
use crate::db;
use crate::models::hang_sessions::HangSession;
use crate::position::Position;
use chrono::{DateTime, Utc};
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::Deserialize;
use rocket::State;
use sqlx::postgres::PgPool;

#[get("/hang_sessions?<search>&<pos>")]
pub async fn get_hang_sessions(
    pool: &State<PgPool>,
    search: Option<String>,
    pos: Option<Position>,
) -> Value {
    let hang_sessions = db::hang_sessions::find(pool, pos, search)
        .await
        .unwrap_or(vec![]);
    json!({ "hang_sessions": hang_sessions })
}

#[get("/hang_sessions/<id>")]
pub async fn get_hang_session(pool: &State<PgPool>, id: i32) -> Option<Value> {
    db::hang_sessions::find_one(pool, id)
        .await
        .map(|hs| json!({ "hang_session": hs }))
}

#[derive(Deserialize, FromForm, Debug)]
pub struct HangSessionCreateBody {
    pub name: String,
    pub description: Option<String>,
    pub hangzone_id: i32,
    pub starts_at: Option<String>,
}

#[post("/hang_sessions", data = "<body>")]
pub async fn create_hang_session(
    pool: &State<PgPool>,
    body: Json<HangSessionCreateBody>,
) -> Result<Status, &str> {
    let body = body.into_inner();
    let starts_at = body.starts_at.map(|s| {
        let dt = DateTime::parse_from_rfc3339(&s).unwrap();
        dt.with_timezone(&Utc)
    });
    match db::hang_sessions::create_one(
        pool,
        body.name,
        body.description,
        body.hangzone_id,
        starts_at,
    )
    .await
    {
        Ok(_res) => Ok(Status::Created),
        Err(e) => {
            eprintln!("Could not create hang session: {}", e);
            Err("Could not create hang session")
        }
    }
}

#[post("/hang_sessions/join")]
pub async fn join_hang_session(pool: &State<PgPool>) -> Result<Status, &str> {
    // TODO: join hang session
    Ok(Status::Created)
}

#[post("/hang_sessions/leave")]
pub async fn _hang_session(pool: &State<PgPool>) -> Result<Status, &str> {
    // TODO: join hang session
    Ok(Status::Created)
}
