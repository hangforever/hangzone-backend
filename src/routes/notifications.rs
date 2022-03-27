use super::PaginationParams;
use crate::auth::Auth;
use crate::db;
//use crate::db::notifications;
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::State;
use sqlx::postgres::PgPool;

#[get("/notifications?<pagination>")]
pub async fn get_notifications(
    auth: Auth,
    pagination: PaginationParams,
    pool: &State<PgPool>,
) -> Value {
    let res = db::notifications::find(pool, auth.id).await;
    match res {
        Ok(notifications) => json!({ "notifications": notifications }),
        Err(e) => {
            eprintln!("Error getting notifications: {}", e);
            json!({})
        }
    }
}

#[get("/notifications/read/<notification_id>")]
pub async fn update_read(notification_id: i32, pool: &State<PgPool>) -> Value {
    // TODO: update notification
    json!({ "ok": true })
}

#[get("/notifications/trash/<notification_id>")]
pub async fn update_trash(notification_id: i32, pool: &State<PgPool>) -> Value {
    // TODO: update notification
    json!({ "ok": true })
}
