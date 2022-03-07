use crate::db;
use rocket::serde::json::{json, Value};
use rocket::State;
use sqlx::postgres::PgPool;

#[get("users/<slug>")]
pub async fn get_user(slug: String, pool: &State<PgPool>) -> Value {
    let hangzone = db::hangzones::find_one(pool, Some(&slug), None).await;

    if let Some(h) = hangzone {
        return json!({ "hangzone": h });
    }
    json!({ "hangzone": null })
}

#[get("users?<params..>")]
pub async fn get_users(params: FindHangzones, pool: &State<PgPool>) -> Value {
    let hangzones = db::hangzones::find(pool, params).await;

    json!({ "hangzones": hangzones })
}
