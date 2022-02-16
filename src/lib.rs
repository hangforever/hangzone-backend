#[macro_use]
extern crate rocket;
extern crate sqlx;
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
pub async fn rocket() -> _ {
    dotenv().ok();
    let pool = db::get_pool().await.unwrap();

    rocket::custom(config::from_env())
        .mount("/api", routes![routes::hangzones::get_hangzone])
        .manage::<PgPool>(pool)
        .attach(cors_fairing())
        .attach(config::AppState::manage())

    // Register not available in 0.5?
    // .register(catchers![not_found])
}
