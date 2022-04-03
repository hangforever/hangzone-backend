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

#[post("/friends/request/<friend_id>")]
pub async fn accept_friend(auth: Auth, friend_id: i32, pool: &State<PgPool>) -> Status {
    //if let Some(friend_request) = db::
    // TODO: if you're already friends, delete request, upgrade to friend
    let res = db::friends::create_one(pool, auth.id, friend_id)
        .await
        .map_err(|e| eprintln!("Err creating friend: {}", e))
        .is_ok();
    handle_friend_added_notification(pool, friend_id, &auth.alias).await;
    Status::Created
}

async fn handle_friend_added_notification(pool: &State<PgPool>, friend_id: i32, alias: &str) {
    db::notifications::create_one(
        pool,
        friend_id,
        NotificationType::FriendAdded,
        &format!("{} added you as a friend!", alias),
    )
    .await
    .map_err(|e| eprintln!("Err creating friend: {}", e));
}

#[post("/friends/request/<friend_id>")]
pub async fn request_friend(auth: Auth, friend_id: i32, pool: &State<PgPool>) -> Status {
    //db::friends::find(pool, user_hanger_id, page)
    // TODO: create a friend request
    handle_friend_requested_notification(pool, friend_id, &auth.alias).await;
    Status::Created
}

async fn handle_friend_requested_notification(pool: &State<PgPool>, friend_id: i32, alias: &str) {
    db::notifications::create_one(
        pool,
        friend_id,
        NotificationType::FriendAdded,
        &format!("{} requested you as a friend!", alias),
    )
    .await
    .map_err(|e| eprintln!("Err creating friend: {}", e));
}

#[delete("/friends/<friend_id>")]
pub async fn delete_friend(auth: Auth, friend_id: i32, pool: &State<PgPool>) -> Status {
    let res = db::friends::delete_one(pool, auth.id, friend_id)
        .await
        .is_ok();
    Status::Created
}
