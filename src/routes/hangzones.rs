use crate::db;
use crate::db::hangzones::{FindHangzones, HangzoneBody};
use rocket::serde::json::{json, Json, Value};
use rocket::State;
use sqlx::postgres::PgPool;

#[get("/hangzones/<slug>")]
pub async fn get_hangzone(slug: String, pool: &State<PgPool>) -> Value {
    let hangzone = db::hangzones::find_one(pool, Some(&slug), None).await;

    if let Some(h) = hangzone {
        return json!({ "hangzone": h });
    }
    json!({ "hangzone": null })
}

#[get("/hangzones?<params..>")]
pub async fn get_hangzones(params: FindHangzones, pool: &State<PgPool>) -> Value {
    let hangzones = db::hangzones::find(pool, params).await;

    json!({ "hangzones": hangzones })
}

#[post("/hangzones", data = "<hangzone_body>")]
pub async fn create_hangzone(hangzone_body: Json<HangzoneBody>, pool: &State<PgPool>) -> Value {
    let hangzone = db::hangzones::create_one(pool, hangzone_body.into_inner()).await;

    if let Err(e) = hangzone {
        eprintln!("Error creating hangzone: {}", e);
    }
    json!({ "hangzones": null })
}
