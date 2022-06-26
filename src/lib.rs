#[macro_use]
extern crate rocket;
use rocket::serde::json::{json, Value};

extern crate sqlx;
extern crate validator_derive;

extern crate rocket_cors;
use rocket_cors::{Cors, CorsOptions};

use dotenv::dotenv;

use sqlx::PgPool;

mod auth;
mod config;
mod constants;
pub mod db;
mod errors;
mod models;
mod position;
mod routes;

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

#[catch(422)]
fn unprocessable_entity() -> Value {
    json!({
        "status": "error",
        "reason": "You did something bad."
    })
}

fn cors_fairing() -> Cors {
    CorsOptions::default()
        .to_cors()
        .expect("Cors fairing cannot be created")
}

#[launch]
pub async fn rocket() -> _ {
    dotenv().ok();
    let pool = db::get_pool().await.unwrap();

    rocket::custom(config::from_env())
        .mount(
            "/api",
            routes![
                routes::friend_requests::accept_friend,
                routes::friend_requests::cancel_friend,
                routes::friend_requests::decline_friend,
                routes::friend_requests::get_friend_requests,
                routes::friend_requests::request_friend,
                routes::friends::delete_friend,
                routes::friends::get_friends,
                routes::hang_requests::accept_hang,
                routes::hang_requests::cancel_hang,
                routes::hang_requests::decline_hang,
                routes::hang_requests::get_hang_requests,
                routes::hang_requests::request_hang,
                routes::hangzones::create_hangzone,
                routes::hangzones::get_hangzone,
                routes::hangzones::get_hangzones,
                routes::hang_sessions::create_hang_session,
                routes::hang_sessions::get_hang_session,
                routes::hang_sessions::get_hang_sessions,
                routes::hang_sessions::join_hang_session,
                routes::hang_sessions::leave_hang_session,
                routes::notifications::get_notifications,
                routes::notifications::update_read,
                routes::notifications::update_trash,
                routes::user_hangers::login,
                routes::user_hangers::register_user,
                routes::user_hangers::get_user,
                routes::user_hangers::update_position,
                routes::user_hangers::update_user,
            ],
        )
        .manage::<PgPool>(pool)
        .attach(cors_fairing())
        .attach(config::AppState::manage())
        .register("/", catchers![not_found, unprocessable_entity])
}
