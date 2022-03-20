use crate::auth::Auth;
use crate::config::AppState;
use crate::db;
use crate::db::user_hangers::UserBody;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::Deserialize;
use rocket::State;
use sqlx::postgres::PgPool;
use sqlx::FromRow;

#[derive(Deserialize)]
pub struct LoginUser {
    user_hanger: LoginUserData,
}

#[derive(Deserialize)]
struct LoginUserData {
    email: String,
    password: String,
}

#[post("/users/login", data = "<login_user>")]
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

#[post("/users", data = "<user_body>")]
pub async fn register_user(
    user_body: Json<UserBody>,
    pool: &State<PgPool>,
    state: &State<AppState>,
) -> Value {
    let user_hanger = db::user_hangers::create_one(pool, user_body.into_inner()).await;
    let secret = state.secret.clone();
    if let Ok(u) = user_hanger {
        return json!({ "user_hanger": u.to_user_auth(&secret) });
    }
    json!({ "user_hanger": null })
}

#[get("/users")]
pub async fn get_user(auth: Auth, pool: &State<PgPool>, state: &State<AppState>) -> Value {
    let user_hanger = db::user_hangers::find_one(pool, auth.id).await;
    let secret = state.secret.clone();
    if let Ok(u) = user_hanger {
        return json!({ "user_hanger": u.to_user_auth(&secret) });
    }
    json!({ "user_hanger": null })
}

// TODO: update user
