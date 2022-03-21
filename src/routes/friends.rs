use super::PaginationParams;
use crate::auth::Auth;
use crate::db;
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
    if let Some(friends) = friends {
        return json!({ "friends": friends });
    }

    json!({ "friends": null })
}

// TODO: make 201 response
#[post("/friends/<friend_id>")]
pub async fn add_friend(auth: Auth, friend_id: i32, pool: &State<PgPool>) -> Value {
    let res = db::friends::create_one(pool, auth.id, friend_id)
        .await
        .map_err(|e| eprintln!("Err creating friend: {}", e))
        .is_ok();
    json!({ "ok": res })
}

// TODO: make 201 response
#[delete("/friends?<friend_id>")]
pub async fn delete_friend(auth: Auth, friend_id: i32, pool: &State<PgPool>) -> Value {
    let res = db::friends::delete_one(pool, auth.id, friend_id)
        .await
        .is_ok();
    json!({ "ok": res })
}
