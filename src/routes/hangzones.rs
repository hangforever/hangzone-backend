use crate::db;
use crate::db::hangzones::FindHangzones;
use rocket::serde::json::{json, Value};
use rocket::State;
use sqlx::postgres::PgPool;

#[get("/hangzones/<slug>")]
pub async fn get_hangzone(slug: String, pool: &State<PgPool>) -> Value {
    let hangzone = db::hangzones::find_one(pool, Some(&slug), None).await;

    if let Some(h) = hangzone {
        json!({ "hangzone": h })
    } else {
        json!({ "hangzone": null })
    }
}

#[get("/hangzones?<params..>")]
pub async fn get_hangzones(params: FindHangzones, pool: &State<PgPool>) -> Value {
    let hangzones = db::hangzones::find(pool, params).await;

    json!({ "hangzones": hangzones })
}
