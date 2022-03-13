use crate::db;
use rocket::serde::json::{json, Value};
use rocket::State;
use sqlx::postgres::PgPool;

#[post("/friends/<user_hanger_id>/<friend_id>")]
pub async fn create_friend(user_hanger_id: i32, friend_id: i32, pool: &State<PgPool>) -> Value {
    let friend = db::friends::create_one(pool, user_hanger_id, friend_id).await;

    json!({ "ok": true })
}
