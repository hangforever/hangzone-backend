#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate sqlx;

#[macro_use]
extern crate validator_derive;

use dotenv::dotenv;

use rocket_cors::Cors;
use sqlx::PgPool;

mod config;
mod db;
mod models;
mod routes;

#[catch(404)]
fn not_found() -> String {
    // TODO: make 404
    String::new()
}

fn cors_fairing() -> Cors {
    Cors::from_options(&Default::default()).expect("Cors fairing cannot be created")
}

#[launch]
pub fn rocket() -> _ {
    dotenv().ok();
    let pool = db::get_pool().await;

    rocket::custom(config::from_env())
        .mount("/api", routes![routes::hangzones::get_hangzone])
        .manage::<PgPool>(pool)
    // .attach(db::Conn::fairing())
    // .attach(cors_fairing())
    // .attach(config::AppState::manage())

    // Register not available in 0.5?
    // .register(catchers![not_found])
}
