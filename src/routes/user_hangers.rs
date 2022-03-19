use crate::config::AppState;
use crate::db;
use crate::db::user_hangers::UserBody;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::Deserialize;
use rocket::State;
use sqlx::postgres::PgPool;
use sqlx::FromRow;

#[get("/users/<user_hanger_id>")]
pub async fn get_user(user_hanger_id: i32, pool: &State<PgPool>) -> Value {
    let user_hanger = db::user_hangers::find_one(pool, user_hanger_id).await;

    if let Ok(h) = user_hanger {
        return json!({ "user_hanger": h });
    }
    json!({ "user_hanger": null })
}

#[post("/users", data = "<user_body>")]
pub async fn register_user(user_body: Json<UserBody>, pool: &State<PgPool>) -> Value {
    let user_hanger = db::user_hangers::create_one(pool, user_body.into_inner()).await;

    if let Ok(u) = user_hanger {
        return json!({ "user_hanger": u });
    }
    json!({ "user_hanger": null })
}

#[derive(Deserialize)]
pub struct LoginUser {
    user_hanger: LoginUserData,
}

#[derive(Deserialize)]
struct LoginUserData {
    email: String,
    password: String,
}

#[post("/login", data = "<login_user>")]
pub async fn post_login(
    login_user: Json<LoginUser>,
    pool: &State<PgPool>,
    state: &State<AppState>,
) -> Value {
    let login_user = login_user.into_inner().user_hanger;
    let secret = state.secret.clone();

    let user_hanger = db::user_hangers::login(pool, &login_user.email, &login_user.password).await;
    if let Some(user_hanger) = user_hanger {
        return json!({ "user_hanger": user_hanger.to_user_auth(&secret) });
    }
    json!({ "user_hanger": null })
}
