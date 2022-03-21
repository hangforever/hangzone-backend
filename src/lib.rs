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
pub mod db;
mod models;
mod routes;

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
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
                routes::hangzones::get_hangzone,
                routes::hangzones::get_hangzones,
                routes::hangzones::create_hangzone,
                routes::user_hangers::post_login,
                routes::user_hangers::register_user,
                routes::user_hangers::update_user,
                routes::friends::add_friend,
                routes::friends::get_friends,
                routes::friends::delete_friend,
            ],
        )
        .manage::<PgPool>(pool)
        .attach(cors_fairing())
        .attach(config::AppState::manage())
        .register("/", catchers![not_found])
}
