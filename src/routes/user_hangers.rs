use crate::auth::Auth;
use crate::config::AppState;
use crate::db;
use crate::db::user_hangers::{Position, UserBody};
use rocket::http::Status;
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

#[derive(Deserialize)]
pub struct UserHangerUpdate {
    first_name: Option<String>,
    last_name: Option<String>,
    alias: Option<String>,
    email_address: Option<String>,
    status_description: Option<String>,
    icon_url: Option<String>,
}

#[put("/users", data = "<update>")]
pub async fn update_user(
    auth: Auth,
    update: Json<UserHangerUpdate>,
    pool: &State<PgPool>,
    state: &State<AppState>,
) -> Value {
    if let Ok(mut hanger) = db::user_hangers::find_one(pool, auth.id).await {
        let update = update.into_inner();
        if let Some(first_name) = update.first_name {
            hanger.first_name = first_name;
        }
        if let Some(last_name) = update.last_name {
            hanger.last_name = last_name;
        }
        if let Some(alias) = update.alias {
            hanger.alias = alias;
        }
        if let Some(email_address) = &update.email_address {
            hanger.email_address = update.email_address;
        }
        if let Some(status_description) = &update.status_description {
            hanger.status_description = update.status_description;
        }
        if let Some(icon_url) = &update.icon_url {
            hanger.icon_url = update.icon_url;
        }
        let res = db::user_hangers::update(pool, hanger).await.is_ok();
        return json!({ "user_hanger": res });
    }
    json!({ "ok": false })
}

#[put("/users/location", data = "<pos>")]
pub async fn update_position(auth: Auth, pos: Json<Position>, pool: &State<PgPool>) -> Status {
    match db::user_hangers::update_geography(pool, pos.into_inner(), auth.id).await {
        Ok(_) => Status::NoContent,
        Err(e) => {
            eprintln!("Could not update position: {}", e);
            Status::UnprocessableEntity
        }
    }
}
