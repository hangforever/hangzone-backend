use super::PaginationParams;
use crate::auth::Auth;
use crate::db;
//use crate::db::notifications;
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::State;
use sqlx::postgres::PgPool;

#[get("/notifications&<pagination>")]
pub async fn get_notifications(pagination: PaginationParams, pool: &State<PgPool>) -> Value {
    // TODO: get from DB
    let notifications = vec![];
    json!({ "notifications": notifications })
}

#[get("/notifications/read")]
pub async fn update_read(slug: String, pool: &State<PgPool>) -> Value {
    // TODO: update notification
    json!({ "ok": true })
}

#[get("/notifications/trash")]
pub async fn update_trash(slug: String, pool: &State<PgPool>) -> Value {
    // TODO: update notification
    json!({ "ok": true })
}
