#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate sqlx;

#[macro_use]
extern crate validator_derive;

use dotenv::dotenv;

use rocket_contrib::json::JsonValue;
use rocket_cors::Cors;

mod config;
mod routes;

#[catch(404)]
fn not_found() -> JsonValue {
  json!({
      "status": "error",
      "reason": "Resource was not found."
  })
}

fn cors_fairing() -> Cors {
  Cors::from_options(&Default::default()).expect("Cors fairing cannot be created")
}

pub fn rocket() -> rocket::Rocket {
  dotenv().ok();
  rocket::custom(config::from_env())
    .mount("/api", routes![routes::hangzones::get_hangzones])
    // .attach(db::Conn::fairing())
    // .attach(cors_fairing())
    // .attach(config::AppState::manage())
    .register(catchers![not_found])
}
