use super::PaginationParams;
use crate::auth::Auth;
use crate::db;
use crate::models::notifications::NotificationType;
use rocket::http::Status;
use rocket::serde::json::{json, Value};
use rocket::State;
use sqlx::postgres::PgPool;

#[get("/friends?<pagination_params..>")]
pub async fn get_friends(
    auth: Auth,
    pagination_params: PaginationParams,
    pool: &State<PgPool>,
) -> Value {
    let friends = db::friends::find(pool, auth.id, pagination_params.page).await;
    json!({ "friends": friends })
}

#[delete("/friends/<friend_id>")]
pub async fn delete_friend(auth: Auth, friend_id: i32, pool: &State<PgPool>) -> Status {
    let res = db::friends::delete_one(pool, auth.id, friend_id)
        .await
        .is_ok();
    Status::Created
}
