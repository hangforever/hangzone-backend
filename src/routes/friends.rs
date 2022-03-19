use super::PaginationParams;
use crate::auth::Auth;
use crate::db;
use rocket::serde::json::{json, Value};
use rocket::State;
use sqlx::postgres::PgPool;

#[post("/friends/<friend_id>")]
pub async fn create_friend(auth: Auth, friend_id: i32, pool: &State<PgPool>) -> Value {
    let friend = db::friends::create_one(pool, auth.id, friend_id).await;
    if let Err(e) = friend {
        eprintln!("Err creating friend: {}", e);
    }
    json!({ "ok": true })
}

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
