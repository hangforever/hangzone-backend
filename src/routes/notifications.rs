use super::PaginationParams;
use crate::auth::Auth;
use crate::db::{self, notifications};
use rocket::response::status::BadRequest;
use rocket::serde::json::{json, Json, Value};
use rocket::State;
use serde::Deserialize;
use sqlx::postgres::PgPool;

#[get("/notifications?<pagination>")]
pub async fn get_notifications(
    auth: Auth,
    pagination: PaginationParams,
    pool: &State<PgPool>,
) -> Result<Value, BadRequest<String>> {
    db::notifications::find(pool, auth.id)
        .await
        .map(|notifications| json!({ "notifications": notifications }))
        .map_err(|e| BadRequest(Some(e.to_string())))
}

#[derive(Deserialize)]
pub struct NotificationReqData {
    ids: Vec<i32>,
}

#[put("/notifications/read", data = "<data>")]
pub async fn update_read(
    data: Json<NotificationReqData>,
    pool: &State<PgPool>,
) -> Result<(), String> {
    db::notifications::mark_as_read(pool, &data.ids)
        .await
        .map_err(|e| e.to_string())
}

#[put("/notifications/trash", data = "<data>")]
pub async fn update_trash(
    data: Json<NotificationReqData>,
    pool: &State<PgPool>,
) -> Result<(), String> {
    db::notifications::send_to_trash(pool, &data.ids)
        .await
        .map_err(|e| e.to_string())
}
