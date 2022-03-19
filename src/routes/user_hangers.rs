use crate::db;
use crate::db::user_hangers::UserBody;
use rocket::serde::json::{json, Json, Value};
use rocket::State;
use sqlx::postgres::PgPool;

#[get("/users/<user_hanger_id>")]
pub async fn get_user(user_hanger_id: i32, pool: &State<PgPool>) -> Value {
    let user_hanger = db::user_hangers::find_one(pool, user_hanger_id).await;

    if let Ok(h) = user_hanger {
        return json!({ "user_hanger": h });
    }
    json!({ "user_hanger": null })
}

#[post("/users", data = "<user_body>")]
pub async fn create_user(user_body: Json<UserBody>, pool: &State<PgPool>) -> Value {
    let user_hanger = db::user_hangers::create_one(pool, user_body.into_inner()).await;

    if let Ok(u) = user_hanger {
        return json!({ "user_hanger": u });
    }
    json!({ "user_hanger": null })
}
